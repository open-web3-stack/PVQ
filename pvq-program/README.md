# PVQ Program

Procedural macro for declaring PVQ (PolkaVM Query) guest programs. This crate exposes one attribute, `#[pvq_program::program]`, which turns an inline Rust module into a PolkaVM guest with entrypoints and host extension calls.

The README reflects the real implementation in this crate and the working examples in `guest-examples/`.

## Usage overview

- Annotate an inline module with `#[pvq_program::program]`.
- Mark extension functions with `#[program::extension_fn(extension_id = <u64>, fn_index = <u32>)]`.
- Mark one or more entrypoints with `#[program::entrypoint]`.
- Use SCALE-encodable types for all arguments and return values.

## Simple example

A minimal module with one extension function and one entrypoint:

```rust
#![no_std]
#![no_main]

#[pvq_program::program]
mod my_program {
    type AssetId = u32;
    type Balance = u64;

    // Replace extension_id/fn_index with values that match your runtime
    #[program::extension_fn(extension_id = 123u64, fn_index = 0)]
    fn total_supply(asset: AssetId) -> Balance {}

    #[program::entrypoint]
    fn query(asset: AssetId) -> Balance {
        total_supply(asset)
    }
}
```

## Authoring rules

- Use on an inline module only: `#[pvq_program::program] mod my_program { ... }`.
- Mark extension functions with `#[program::extension_fn(extension_id = <u64>, fn_index = <u32>)]`.
  - Both attributes are required.
  - Functions must be free functions (no `self`/receiver).
- Mark one or more entrypoints with `#[program::entrypoint]`.
  - At least one entrypoint is required.
  - Entrypoints must be free functions (no receivers).
  - Entrypoints’ arguments and return types must be SCALE `Encode`/`Decode`.

## Types and encoding

All inputs/outputs are SCALE encoded. For custom structs/enums, always derive
`parity_scale_codec::{Encode, Decode}` (for host/guest serialization) and
`scale_info::TypeInfo` (for metadata generation).

Example:

```rust
#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo)]
pub struct MyData { pub x: u32, pub y: u64 }

#[program::entrypoint]
fn do_something(d: MyData) -> Option<u64> { Some(d.x as u64 + d.y) }
```

## Building and metadata

Guest programs are `no_std` PolkaVM binaries. Typical Cargo dependencies (see `guest-examples/*/Cargo.toml`):

```toml
[dependencies]
polkavm-derive = { workspace = true }
pvq-program = { workspace = true }
parity-scale-codec = { workspace = true }
# For richer metadata types:
scale-info = { workspace = true, optional = true }
```

To emit program metadata during build (as used in all examples), use a `build.rs`
that runs the generator and writes to a directory provided by the environment:

```rust
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let output_dir = PathBuf::from(env::var("METADATA_OUTPUT_DIR").expect("METADATA_OUTPUT_DIR is not set"));

    let status = Command::new("pvq-program-metadata-gen")
        .arg("--crate-path").arg(&current_dir)
        .arg("--output-dir").arg(&output_dir)
        .env("RUST_LOG", "info")
        .status()
        .expect("Failed to execute pvq-program-metadata-gen");
    if !status.success() { panic!("Failed to generate program metadata"); }
}
```

## Not supported (by design)

- Per-function macro options such as optimization levels, timeouts, custom stack/memory, or built-in testing/debug/profiling helpers are not part of this crate.
- Methods with `self`/`&self` receivers cannot be entrypoints or extension functions.

## See also

- `guest-examples/sum-balance`
- `guest-examples/sum-balance-percent`
- `guest-examples/total-supply`
- `guest-examples/swap-info`

---

See `pvq-program/src/lib.rs` for the re-export and `pvq-program/procedural` for internals.
