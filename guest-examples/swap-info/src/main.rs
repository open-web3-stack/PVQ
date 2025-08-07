//! A guest program that provides information about asset swaps.
#![no_std]
#![no_main]

/// A guest program that provides information about asset swaps.
#[pvq_program::program]
mod swap_info {

    /// Represents a unique identifier for an asset.
    type AssetId = alloc::vec::Vec<u8>;
    /// Represents the balance of an asset.
    type Balance = u128;
    /// Represents information about an asset.
    #[derive(
        Debug, Clone, PartialEq, Eq, parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo,
    )]
    pub struct AssetInfo {
        /// The unique identifier of the asset.
        pub asset_id: AssetId,
        /// The name of the asset.
        pub name: alloc::vec::Vec<u8>,
        /// The symbol of the asset.
        pub symbol: alloc::vec::Vec<u8>,
        /// The number of decimals the asset has.
        pub decimals: u8,
    }

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
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 0)]
    fn quote_price_tokens_for_exact_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

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
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 1)]
    fn quote_price_exact_tokens_for_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

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
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 2)]
    fn get_liquidity_pool(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {}

    /// List all available liquidity pools.
    ///
    /// # Returns
    ///
    /// A list of tuples, where each tuple represents a liquidity pool and contains the identifiers of the two assets in the pool.
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 3)]
    fn list_pools() -> alloc::vec::Vec<(AssetId, AssetId)> {}

    /// Get information about a specific asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: The identifier of the asset.
    ///
    /// # Returns
    ///
    /// Information about the asset, or `None` if the asset does not exist.
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 4)]
    fn asset_info(asset: AssetId) -> Option<AssetInfo> {}

    /// Get information about all assets.
    ///
    /// # Returns
    ///
    /// A map of asset identifiers to asset information.
    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 5)]
    fn assets_info() -> alloc::collections::BTreeMap<AssetId, AssetInfo> {}

    /// Get the quote price of `asset2` in terms of `asset1` for a given amount of `asset1`.
    ///
    /// # Arguments
    ///
    /// * `asset1`: The identifier of the asset to quote against.
    /// * `asset2`: The identifier of the asset to be quoted.
    /// * `amount`: The amount of `asset1`.
    ///
    /// # Returns
    ///
    /// The quote price of `asset2` in terms of `asset1`, or `None` if the quote is not available.
    #[program::entrypoint]
    fn entrypoint_quote_price_exact_tokens_for_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
    ) -> Option<Balance> {
        quote_price_exact_tokens_for_tokens(asset1, asset2, amount, true)
    }

    /// Get the quote price of `asset1` in terms of `asset2` for a given amount of `asset2`.
    ///
    /// # Arguments
    ///
    /// * `asset1`: The identifier of the asset to be quoted.
    /// * `asset2`: The identifier of the asset to quote against.
    /// * `amount`: The amount of `asset2`.
    ///
    /// # Returns
    ///
    /// The quote price of `asset1` in terms of `asset2`, or `None` if the quote is not available.
    #[program::entrypoint]
    fn entrypoint_quote_price_tokens_for_exact_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
    ) -> Option<Balance> {
        quote_price_tokens_for_exact_tokens(asset1, asset2, amount, true)
    }

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
    #[program::entrypoint]
    fn entrypoint_get_liquidity_pool(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {
        get_liquidity_pool(asset1, asset2)
    }

    /// List all available liquidity pools with their asset information.
    ///
    /// # Returns
    ///
    /// A list of tuples, where each tuple represents a liquidity pool and contains the information of the two assets in the pool.
    #[program::entrypoint]
    fn entrypoint_list_pools() -> alloc::vec::Vec<(AssetInfo, AssetInfo)> {
        let pools = list_pools();
        let mut result = alloc::vec::Vec::new();
        let assets_info = assets_info();
        for pool in pools {
            let asset1_info = assets_info.get(&pool.0).cloned();
            let asset2_info = assets_info.get(&pool.1).cloned();
            if let (Some(a1), Some(a2)) = (asset1_info, asset2_info) {
                result.push((a1, a2));
            }
        }
        result
    }
}
