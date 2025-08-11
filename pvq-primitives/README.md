# PVQ Primitives

Lightweight types shared across PVQ crates.

- PvqResponse: raw bytes returned from a PVQ program
- PvqResult: success bytes or error
- PvqError: in `std` it is a string message; in `no_std` it is a compact enum

### Behavior

- No helpers or codecs are provided here; this crate only defines the basic types used by executors, extensions, and runtime APIs.

### Build

```bash
cargo build -p pvq-primitives
```
