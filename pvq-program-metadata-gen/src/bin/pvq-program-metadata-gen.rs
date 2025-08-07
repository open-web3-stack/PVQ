use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(author, version, about = "PVQ Program Metadata Generator")]
struct Args {
    /// Path to the crate directory containing a PVQ program
    #[arg(short, long)]
    crate_path: PathBuf,

    #[arg(short, long)]
    output_dir: PathBuf,
}

// REVIEW: The `main` function is overly long and complex. It should be broken down into smaller,
// more focused functions to improve readability and maintainability.
fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Logging arguments
    info!("Generating metadata for program at: {}", args.crate_path.display());
    info!("Output dir: {}", args.output_dir.display());

    // Create a temp crate for the metadata generation
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_crate_path = temp_dir.path();
    fs::create_dir_all(temp_crate_path).expect("Failed to create `temp_crate` directory");
    info!("Temp crate path: {}", temp_crate_path.display());

    // Extract features section from the original manifest
    let original_manifest_content =
        std::fs::read_to_string(args.crate_path.join("Cargo.toml")).expect("Failed to read original Cargo.toml");
    let optional_features = pvq_program_metadata_gen::extract_features(&original_manifest_content)
        .expect("Failed to extract features section from the original Cargo.toml");
    debug!("Features section: {:?}", optional_features);

    // Create Cargo.toml with features from the original crate
    let manifest = pvq_program_metadata_gen::create_manifest(optional_features.as_ref());
    debug!("Manifest: {}", manifest);
    std::fs::write(temp_crate_path.join("Cargo.toml"), manifest).expect("Failed to write Cargo.toml");

    // Add active features to the cargo command
    let active_features = pvq_program_metadata_gen::get_active_features(optional_features.as_ref())
        .expect("Failed to get active features");
    debug!("Active features: {:?}", active_features);

    // Read the program source
    let source = fs::read_to_string(args.crate_path.join("src/main.rs"))
        .expect("Failed to read pvq program source file, expected `src/main.rs`");

    let pkg_name = std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");

    // Generate the metadata generator source codes
    let metadata_gen_src =
        pvq_program_metadata_gen::metadata_gen_src(&source, &pkg_name, args.output_dir.to_string_lossy().as_ref())
            .expect("Failed to generate metadata generator source code");
    debug!("Metadata generator source code: {}", metadata_gen_src);

    // Create src directory and write main.rs
    fs::create_dir_all(temp_crate_path.join("src")).expect("Failed to create `temp_crate/src directory");
    fs::write(temp_crate_path.join("src/main.rs"), metadata_gen_src.to_string())
        .expect("Failed to write metadata generator source code");

    // Compile and run the metadata generator in one step
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.current_dir(temp_crate_path).args(["run"]);
    for feature in active_features {
        cargo_cmd.arg("--features").arg(feature);
    }
    info!("Compiling and running metadata generator...");
    let status = cargo_cmd.status().expect("Failed to run metadata generator");
    if !status.success() {
        panic!("Failed to generate metadata");
    }
    info!("Metadata generation successful!");
}

// REVIEW: The `main` function uses `expect()` in multiple places, which can cause the
// program to panic. Consider returning a `Result` from `main` and using the `?` operator to
// propagate errors. This will make the error handling more robust.
