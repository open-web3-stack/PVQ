# PVQ Program Metadata Generator

A command-line tool for generating metadata for PVQ programs. This tool extracts metadata from your PVQ program source code, allowing the UI to know about your program's metadata.

## Installation

You can install the tool globally using Cargo:

```bash
cargo install --path /path/to/pvq-program-metadata-gen
```

## Usage

The basic usage is as follows:

```bash
pvq-program-metadata-gen --crate-path /path/to/your/crate --output-dir /path/to/output/dir
```

### Arguments

- `--crate-path, -c`: Path to the crate directory containing a PVQ program
- `--output-dir, -o`: Output directory for the metadata file, typically `OUT_DIR` specified in your `build.rs` file

## Integration with Build Scripts

You can integrate this tool into your crate's `build.rs` file:

```rust
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if the source file changes
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // Determine the output directory for the metadata
    let output_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));

    // Build and run the command
    let status = Command::new("pvq-program-metadata-gen")
        .arg("--crate-path")
        .arg(&current_dir)
        .arg("--output-dir")
        .arg(&output_dir)
        .env("RUST_LOG", "info")
        .status()
        .expect("Failed to execute pvq-program-metadata-gen");

    if !status.success() {
        panic!("Failed to generate program metadata");
    }
}

```

## How It Works

The tool:

1. Reads the source code of your PVQ program
2. Generates metadata generation code
3. Creates a temporary crate that store the metadata generation code
4. Compiles and runs the temporary crate using the same conditions as your original crate

The metadata includes information about function names, parameter types, and return types, allowing the UI to know about your program's metadata.
