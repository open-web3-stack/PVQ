use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if the source file changes
    // println!("cargo:rerun-if-changed=src/main.rs");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // Determine the output directory for the metadata
    let output_dir = PathBuf::from(env::var("METADATA_OUTPUT_DIR").expect("METADATA_OUTPUT_DIR is not set"))
        .expect("Failed to get output directory");

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
