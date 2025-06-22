# Paseo-l

**Paseo-l** is a lightweight agent runtime built on WebAssembly. Inspired by the structural elegance of biological viruses, Paseo agents are small, self-contained, and catalytic—they assume a capable host and connect entities through minimal, declarative pods.

Paseo-l provides a foundation for a future in which every real-world entity—person, place, idea, organization—can have a living counterpart in a WASM-backed environment.

---

## 🧬 What Is a Paseo Pod?

A **pod** is a minimal unit of digital presence. It consists of:

- A WASM agent (compiled from Rust or other languages)
- A simple `Paseofile` that declares:
  - The pod’s identity
  - Which host functions it uses
  - What hooks it exposes (e.g., `init`, `handle_message`)
  - Paths for state, memory, or capabilities

Pods are composable, portable, and runtime-agnostic—they can run in any WASM host that conforms to the Paseo ABI.

---

## 🧠 Core Concepts

- **Agent**: A WASM-compiled module that handles input, maintains local memory, and calls host functions (like `log`, `write_memory`).
- **Host**: A native Rust runtime using `wasmtime` that loads and executes pods, provides WASI, and exposes host functions to the agent.
- **ABI Contract**: Agents import a minimal interface from the `"env"` module (`log`, `write_memory`, etc.) and export simple entrypoints (`init`, `handle_message`, etc.).
- **Paseofile**: A declarative text file that defines how to run a pod.

---

## 🚀 Current Capabilities

- Execute standalone WASM agents with memory access and logging
- Define and run pods using a minimal `Paseofile`
- Bridge guest-to-host communication via a custom ABI
- Run on any machine with a WASM runtime and Rust installed

---

## 🛠 Example: `Paseofile`

```paseo
ENTITY "Midtown Community Garden"
AGENT "./agent.wasm"
MEMORY "./state.json"
HOOK onMessage "handle_message"
HOOK onSync "sync_state"
CAPABILITY "chat"

