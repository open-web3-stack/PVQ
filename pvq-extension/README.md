# PVQ Extension

Extension framework for PVQ with declarative interfaces, metadata generation, execution, and permissions.

### What it provides

- Declarative extension traits and implementations via macros
- Generated dispatch and metadata for extensions
- Execution wrapper to run PVQ programs against your extensions
- Pluggable permission controller (allow‑all by default)

### Typical use

1. Declare an extension trait and implement it for your runtime.
2. Generate an `extensions` module (via the macros) and expose its `metadata()`.
3. Initialize `ExtensionsExecutor` with your generated `extensions` and an invocation source.
4. Optionally implement a custom `PermissionController`.

### Metadata

- A portable type registry plus per‑extension function signatures (names, inputs, output). Useful to serve from your runtime’s `metadata()`.

### Build

```bash
cargo build -p pvq-extension
```
