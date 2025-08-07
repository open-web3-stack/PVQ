#![no_std]
#![no_main]

// REVIEW: The `extension_id` used in the `extension_fn` attributes is a magic number. It would be better to define it as a constant with a descriptive name.
#[pvq_program::program]
mod swap_info {

    type AssetId = alloc::vec::Vec<u8>;
    type Balance = u128;
    #[derive(
        Debug, Clone, PartialEq, Eq, parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo,
    )]
    pub struct AssetInfo {
        pub asset_id: AssetId,
        pub name: alloc::vec::Vec<u8>,
        pub symbol: alloc::vec::Vec<u8>,
        pub decimals: u8,
    }

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 0)]
    fn quote_price_tokens_for_exact_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 1)]
    fn quote_price_exact_tokens_for_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 2)]
    fn get_liquidity_pool(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {}

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 3)]
    fn list_pools() -> alloc::vec::Vec<(AssetId, AssetId)> {}

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 4)]
    fn asset_info(asset: AssetId) -> Option<AssetInfo> {}

    #[program::extension_fn(extension_id = 15900548380266538526u64, fn_index = 5)]
    fn assets_info() -> alloc::collections::BTreeMap<AssetId, AssetInfo> {}

    #[program::entrypoint]
    fn entrypoint_quote_price_exact_tokens_for_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
    ) -> Option<Balance> {
        quote_price_exact_tokens_for_tokens(asset1, asset2, amount, true)
    }

    #[program::entrypoint]
    fn entrypoint_quote_price_tokens_for_exact_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
    ) -> Option<Balance> {
        quote_price_tokens_for_exact_tokens(asset1, asset2, amount, true)
    }

    #[program::entrypoint]
    fn entrypoint_get_liquidity_pool(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {
        get_liquidity_pool(asset1, asset2)
    }

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
