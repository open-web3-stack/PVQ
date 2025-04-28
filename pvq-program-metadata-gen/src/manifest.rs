/// Creates a Cargo.toml file for the metadata generator
pub fn create_manifest(features: Option<&toml::Table>) -> String {
    // Create a basic Cargo.toml for the temp crate
    format!(
        r#"[package]
name = "metadata_gen"
version = "0.1.0"
edition = "2021"

[dependencies]
scale-info = {{ version = "2.0.0", features = ["derive","serde"] }}
parity-scale-codec = {{ version = "3.0.0", features = ["derive"] }}
serde = {{ version = "1", features = ["derive" ] }}
serde_json = "1"
cfg-if = "1.0"
{0}
"#,
        features.map_or_else(String::new, |features| format!(
            "\n[features]\n{}",
            toml::to_string(&features).expect("Should be checked in parsing")
        ))
    )
}
