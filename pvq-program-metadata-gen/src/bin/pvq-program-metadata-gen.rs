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

    #[arg(short, long)]
    manifest_path: Option<PathBuf>,

    /// Target triple to build for (optional)
    #[arg(long)]
    target: Option<String>,
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Logging arguments
    info!("Generating metadata for program at: {}", args.crate_path.display());
    info!("Output dir: {}", args.output_dir.display());
    if let Some(ref manifest_path) = args.manifest_path {
        info!("Manifest path: {}", manifest_path.display());
    } else {
        info!("Using default manifest (no manifest path provided)");
    }

    // Create a temp crate for the metadata generation
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_crate_path = temp_dir.path();
    fs::create_dir_all(temp_crate_path).expect("Failed to create `temp_crate` directory");
    info!("Temp crate path: {}", temp_crate_path.display());

    // Read or create the manifest content
    let (manifest_content, optional_features) = if let Some(ref manifest_path) = args.manifest_path {
        // Read the manifest from the provided manifest path
        let content = std::fs::read_to_string(manifest_path)
            .unwrap_or_else(|_| panic!("Failed to read manifest file: {}", manifest_path.display()));
        debug!("Manifest content: {}", content);

        // Extract features section from the manifest for active features determination
        let features = pvq_program_metadata_gen::extract_features(&content)
            .expect("Failed to extract features section from the manifest");
        debug!("Features section: {:?}", features);

        (content, features)
    } else {
        // Use the default manifest from create_manifest
        let content = pvq_program_metadata_gen::create_manifest(None);
        debug!("Generated default manifest content: {}", content);
        (content, None)
    };

    // Copy the manifest to temp directory
    std::fs::write(temp_crate_path.join("Cargo.toml"), &manifest_content)
        .expect("Failed to write Cargo.toml to temp directory");

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

    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.current_dir(temp_crate_path).args(["run"]);

    // Add target if specified
    if let Some(ref target) = args.target {
        info!("Using explicit target: {}", target);
        cargo_cmd.arg("--target").arg(target);
    }

    if !active_features.is_empty() {
        cargo_cmd.arg("--features");
        for feature in active_features {
            cargo_cmd.arg(feature);
        }
    }
    info!("Compiling and running metadata generator...");
    let status = cargo_cmd.status().expect("Failed to run metadata generator");
    if !status.success() {
        panic!("Failed to generate metadata");
    }
    info!("Metadata generation successful!");
}
