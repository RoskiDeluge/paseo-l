## Project: Paseo-L

Paseo-L is a lightweight agent runtime built on WebAssembly. It allows for the creation and execution of small, self-contained agents (pods) in a WASM-backed environment.

### Core Concepts

- **Agent**: A WASM-compiled module that handles input, maintains local memory, and calls host functions.
- **Host**: A native Rust runtime using `wasmtime` that loads and executes pods.
- **Pod**: A minimal unit of digital presence, consisting of a WASM agent and a `Paseofile`.
- **Paseofile**: A declarative text file that defines how to run a pod, including its identity, agent, and hooks.

### Project Structure

The project is a Rust workspace with two main components:

- `agent/`: The WASM agent, written in Rust.
- `host/`: The native Rust runtime that hosts and executes the agents.

### Development Workflow

- **Agent**: To build the agent, navigate to the `agent/` directory and run `cargo build --target wasm32-wasip1`.
- **Host**: To build and run the host, navigate to the `host/` directory and run `cargo run`.

### Example `Paseofile`

```paseo
ENTITY "Midtown Community Garden"
AGENT "./agent.wasm"
MEMORY "./state.json"
HOOK onMessage "handle_message"
HOOK onSync "sync_state"
CAPABILITY "chat"
```
