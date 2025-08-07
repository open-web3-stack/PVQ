//! The fungibles PVQ extension.
#![cfg_attr(not(feature = "std"), no_std)]
use pvq_extension::extension_decl;

/// The fungibles PVQ extension.
#[extension_decl]
pub mod extension {
    use scale_info::prelude::vec::Vec;
    /// The fungibles PVQ extension trait.
    #[extension_decl::extension]
    pub trait ExtensionFungibles {
        /// The asset identifier type.
        type AssetId;
        /// The balance type.
        type Balance;
        /// The account identifier type.
        type AccountId;
        /// Check if an asset exists.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// `true` if the asset exists, `false` otherwise.
        fn asset_exists(asset: Self::AssetId) -> bool;
        /// Get the name of an asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// The name of the asset.
        fn name(asset: Self::AssetId) -> Vec<u8>;
        /// Get the symbol of an asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// The symbol of the asset.
        fn symbol(asset: Self::AssetId) -> Vec<u8>;
        /// Get the decimals of an asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// The decimals of the asset.
        fn decimals(asset: Self::AssetId) -> u8;
        /// Get the minimum balance of an asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// The minimum balance of the asset.
        fn minimum_balance(asset: Self::AssetId) -> Self::Balance;
        /// Get the total supply of an asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// The total supply of the asset.
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        /// Get the balance of an asset for a specific account.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        /// * `who`: The account identifier.
        ///
        /// # Returns
        ///
        /// The balance of the asset for the specified account.
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }
}
