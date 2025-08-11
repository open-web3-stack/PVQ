# PVQ PoC Runtime

FRAME runtime demonstrating PVQ integration used in this repository.

### What it exposes

- Runtime API `PvqApi` with two methods:
  - `execute_query(...)`: runs a PVQ program with input bytes and an optional gas limit; if not provided, a default ~2s limit is applied.
  - `metadata()`: returns serialized extension metadata bytes (portable type registry + extension functions).

### How it’s wired

- Uses the generated `extensions` module to connect extension traits to runtime pallets.
- Executes programs via `ExtensionsExecutor` with invocation source set to runtime API.

### Build

```bash
cargo build -p poc-runtime --release
```
