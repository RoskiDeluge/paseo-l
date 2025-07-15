# AGENTS.md

# Overview
This repository implements Paseo-L, a lightweight runtime for WebAssembly-based agents called "pods".
Agents are WASM modules that run inside a host process exposing a minimal ABI.

# Paseo Pod Components
Each Paseo pod contains:
- An agent WASM module
- A Paseofile configuration
- Optional state memory

# Paseofile Syntax
The Paseofile is a plain text file with directives:
```
ENTITY "Entity Name"
AGENT "path/to/agent.wasm"
MEMORY "path/to/state.json"
HOOK onMessage "handle_message"
HOOK onSync "sync_state"
CAPABILITY "chat"
```

# Host Runtime
The host, written in Rust, uses Wasmtime to instantiate and run the WASM agent.
It provides a WASI environment and injects host functions like `log` and `write_memory` for communication.

# Agent Requirements
- Import from `"env"` module functions (e.g., `log`)
- Export entrypoints matching hook names (`handle_message`, etc.)
- Compatible with WASI environment

# Interaction
- The host reads the Paseofile, loads the WASM agent, and runs exported hooks.
- Hooks handle incoming messages or sync events from the pod environment.

---  

This document should help orient anyone writing or working with Paseo-L agents and pods in this repo.
If you'd like, I can help you further implement features or write code related to this system.
