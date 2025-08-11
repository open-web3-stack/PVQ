//! The swap PVQ extension.
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pvq_extension::extension_decl;

/// The swap PVQ extension.
#[extension_decl]
pub mod extension {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;

    /// The swap PVQ extension trait.
    #[extension_decl::extension]
    pub trait ExtensionSwap {
        /// The asset identifier type.
        type AssetId;
        /// The balance type.
        type Balance;
        /// The asset info type.
        type AssetInfo;

        /// Get the quote price of `asset1` in terms of `asset2` for a given amount of `asset2`.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The identifier of the asset to be quoted.
        /// * `asset2`: The identifier of the asset to quote against.
        /// * `amount`: The amount of `asset2`.
        /// * `include_fee`: Whether to include the fee in the quote.
        ///
        /// # Returns
        ///
        /// The quote price of `asset1` in terms of `asset2`, or `None` if the quote is not available.
        fn quote_price_tokens_for_exact_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        /// Get the quote price of `asset2` in terms of `asset1` for a given amount of `asset1`.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The identifier of the asset to quote against.
        /// * `asset2`: The identifier of the asset to be quoted.
        /// * `amount`: The amount of `asset1`.
        /// * `include_fee`: Whether to include the fee in the quote.
        ///
        /// # Returns
        ///
        /// The quote price of `asset2` in terms of `asset1`, or `None` if the quote is not available.
        fn quote_price_exact_tokens_for_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        /// Get the liquidity pool of two assets.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The identifier of the first asset.
        /// * `asset2`: The identifier of the second asset.
        ///
        /// # Returns
        ///
        /// A tuple containing the balance of each asset in the liquidity pool, or `None` if the pool does not exist.
        fn get_liquidity_pool(asset1: Self::AssetId, asset2: Self::AssetId) -> Option<(Self::Balance, Self::Balance)>;

        /// List all available liquidity pools.
        ///
        /// # Returns
        ///
        /// A list of tuples, where each tuple represents a liquidity pool and contains the identifiers of the two assets in the pool.
        fn list_pools() -> Vec<(Self::AssetId, Self::AssetId)>;

        /// Get information about a specific asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// Information about the asset, or `None` if the asset does not exist.
        fn asset_info(asset: Self::AssetId) -> Option<Self::AssetInfo>;

        /// Get information about all assets.
        ///
        /// # Returns
        ///
        /// A map of asset identifiers to asset information.
        fn assets_info() -> BTreeMap<Self::AssetId, Self::AssetInfo>;
    }
}
