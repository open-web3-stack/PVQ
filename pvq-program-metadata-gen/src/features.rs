use std::collections::HashSet;
pub fn get_active_features(optional_features: Option<&toml::Table>) -> Result<Vec<String>, String> {
    let features_env = std::env::vars()
        .filter(|(var, _)| var.starts_with("CARGO_FEATURE_"))
        .map(|(var, _)| var)
        .collect::<HashSet<_>>();
    if features_env.is_empty() {
        Ok(vec![])
    } else {
        let features = optional_features
            .as_ref()
            .ok_or_else(|| "Some features are set, but there is no features section in the manifest".to_string())?;
        Ok(features
            .keys()
            .filter(|feature| {
                features_env.contains(&format!("CARGO_FEATURE_{}", feature.to_uppercase().replace("-", "_")))
            })
            .map(|feature| feature.to_string())
            .collect())
    }
}

/// Extracts features from the original crate's Cargo.toml
pub fn extract_features(original_manifest_content: &str) -> Result<Option<toml::Table>, String> {
    match toml::from_str::<toml::Table>(original_manifest_content) {
        Ok(manifest) => {
            // Extract features section if it exists
            if let Some(features) = manifest.get("features") {
                if let toml::Value::Table(features) = features {
                    Ok(Some(features.clone()))
                } else {
                    Err("features section is not a table".to_string())
                }
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
