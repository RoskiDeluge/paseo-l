use std::path::PathBuf;

mod paseofile;
mod pod;

use paseofile::parse_paseofile;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

fn main() -> anyhow::Result<()> {
    // 🔍 Path to Paseofile
    let paseofile_path = PathBuf::from("pod-a/Paseofile");
    let paseofile_dir = paseofile_path.parent().unwrap();

    println!("🔍 Looking for Paseofile at: {}", paseofile_path.display());

    // 🧾 Parse Paseofile
    let spec = parse_paseofile(paseofile_path.to_str().unwrap())?;
    println!("Running pod for entity: {}", spec.entity);
    println!("Hook map: {:?}", spec.hooks);

    // 📍 Resolve agent.wasm path relative to Paseofile
    let resolved_agent_path = paseofile_dir.join(&spec.agent_path);
    println!("Resolved agent path: {}", resolved_agent_path.display());

    // 🔧 Set up WASM engine and linker
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    // 🧠 Register WASI context (enables println! etc. in WASM)
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio() // this allows WASM to print to your terminal
        .build();
    let mut store = Store::new(&engine, wasi);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // 🔌 Host: log function
    linker.func_wrap(
        "env",
        "log",
        |mut caller: Caller<'_, WasiCtx>, ptr: i32, len: i32| {
            let memory = caller
                .get_export("memory")
                .and_then(|e| e.into_memory())
                .expect("failed to find memory export");

            let data = memory
                .data(&caller)
                .get(ptr as usize..(ptr + len) as usize)
                .expect("memory out of bounds");

            let msg = std::str::from_utf8(data).expect("invalid utf-8");
            println!("[AGENT LOG]: {}", msg);
        },
    )?;

    // 🔌 Host: write_memory stub (just logs the call for now)
    linker.func_wrap(
        "env",
        "write_memory",
        |_caller: Caller<'_, WasiCtx>, ptr: i32, len: i32| {
            println!(
                "[HOST] Agent wrote {} bytes starting at memory offset {}",
                len, ptr
            );
            Ok(())
        },
    )?;

    // 🔁 Load and instantiate the module
    let module = Module::from_file(&engine, resolved_agent_path)?;
    let instance = linker.instantiate(&mut store, &module)?;

    // 🏁 Run init (optional)
    if let Some(init_name) = spec.hooks.get("init") {
        if let Ok(init_func) = instance.get_typed_func::<(), i32>(&mut store, init_name) {
            init_func.call(&mut store, ())?;
        }
    } else if let Ok(init_func) = instance.get_typed_func::<(), i32>(&mut store, "init") {
        init_func.call(&mut store, ())?;
    }

    // ✉️ Send a message to the agent
    let message = "Hello from host!";
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let ptr = 1024;
    memory.write(&mut store, ptr, message.as_bytes())?;

    if let Some(handler_name) = spec.hooks.get("onMessage") {
        let handler = instance.get_typed_func::<(i32, i32), i32>(&mut store, handler_name)?;
        handler.call(&mut store, (ptr as i32, message.len() as i32))?;
    } else {
        println!("No onMessage hook defined.");
    }

    Ok(())
}
