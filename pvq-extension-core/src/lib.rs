//! The core PVQ extension.
#![cfg_attr(not(feature = "std"), no_std)]

use pvq_extension::extension_decl;

/// The core PVQ extension.
#[extension_decl]
pub mod extension {
    /// The core PVQ extension trait.
    #[extension_decl::extension]
    pub trait ExtensionCore {
        /// The extension identifier type.
        type ExtensionId;
        /// Check if an extension is enabled.
        ///
        /// # Arguments
        ///
        /// * `id`: The identifier of the extension.
        ///
        /// # Returns
        ///
        /// `true` if the extension is enabled, `false` otherwise.
        fn has_extension(id: Self::ExtensionId) -> bool;
        // crypto functions
        // fn blake2_64(data: Vec<u8>) -> [u8; 8];
        // fn blake2_128(data: Vec<u8>) -> [u8; 16];
        // fn blake2_256(data: Vec<u8>) -> [u8; 32];
        // fn twox_64(data: Vec<u8>) -> [u8; 8];
        // fn read_storage(key: Vec<u8>) -> Option<Vec<u8>>;
    }
}
