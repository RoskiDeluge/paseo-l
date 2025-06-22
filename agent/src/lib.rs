#[no_mangle]
pub extern "C" fn init() -> i32 {
    log_str("Agent initialized.");
    0
}

#[no_mangle]
pub extern "C" fn handle_message(ptr: i32, len: i32) -> i32 {
    let msg = read_input(ptr, len);
    let reply = format!("Agent received: {}", msg);
    write_output(&reply);
    0
}

#[no_mangle]
pub extern "C" fn sync_state() -> i32 {
    log_str("Sync state called.");
    0
}

// Host functions we import
#[link(wasm_import_module = "env")]
extern "C" {
    fn read_memory() -> i32;
    fn write_memory(ptr: i32, len: i32);
    fn log(ptr: i32, len: i32);
}

// Helpers
fn log_str(s: &str) {
    unsafe {
        log(s.as_ptr() as i32, s.len() as i32);
    }
}

fn read_input(ptr: i32, len: i32) -> String {
    let mem = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    String::from_utf8_lossy(mem).to_string()
}

fn write_output(s: &str) {
    unsafe {
        write_memory(s.as_ptr() as i32, s.len() as i32);
    }
}
