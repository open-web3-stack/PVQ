//! This crate provides a tool for generating metadata for PVQ programs.
pub type ExtensionId = u64;
pub type FnIndex = u8;
mod features;
mod helper;
pub use features::{extract_features, get_active_features};
mod manifest;
pub use manifest::create_manifest;
mod metadata_gen;
pub use metadata_gen::metadata_gen_src;
