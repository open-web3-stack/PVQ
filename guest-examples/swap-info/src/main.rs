#![no_std]
#![no_main]

#[pvq_program::program]
mod swap_info {

    cfg_if::cfg_if! {
        if #[cfg(feature = "asset-hub")] {
            // Actually AssetHub uses xcm::Location as AssetId, but we use opaque Vec<u8> because some compilation issues.
            type AssetId = alloc::vec::Vec<u8>;
            type Balance = u128;
        } else if #[cfg(feature = "acala")] {
            type AssetId = alloc::vec::Vec<u8>;
            type Balance = u128;
        } else {
            type AssetId = alloc::vec::Vec<u8>;
            type Balance = u128;
        }
    }

    #[program::extension_fn(extension_id = 13206387959972970661u64, fn_index = 0)]
    fn quote_price_tokens_for_exact_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

    #[program::extension_fn(extension_id = 13206387959972970661u64, fn_index = 1)]
    fn quote_price_exact_tokens_for_tokens(
        asset1: AssetId,
        asset2: AssetId,
        amount: Balance,
        include_fee: bool,
    ) -> Option<Balance> {
    }

    #[program::extension_fn(extension_id = 13206387959972970661u64, fn_index = 2)]
    fn get_liquidity_pool(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {}

    #[program::extension_fn(extension_id = 13206387959972970661u64, fn_index = 3)]
    fn list_pools() -> alloc::vec::Vec<(AssetId, AssetId, Balance, Balance)> {}

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
    fn entrypoint_list_pools() -> alloc::vec::Vec<(AssetId, AssetId, Balance, Balance)> {
        list_pools()
    }
}
