#![no_std]
#![no_main]

#[pvq_program::program]
mod query_pools {

    type AssetId = alloc::vec::Vec<u8>;
    type Balance = u128;
    const UNITS: Balance = 1_000_000_000_000;

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
    fn test_swap_extension(asset1: AssetId, asset2: AssetId) -> Option<(Balance, Balance)> {
        // Check quote prices
        let amount_in = quote_price_tokens_for_exact_tokens(asset1.clone(), asset2.clone(), 10 * UNITS, false)
            .expect("Quote price exists");

        assert!(amount_in == 20 * UNITS);
        let amount_out = quote_price_exact_tokens_for_tokens(asset1.clone(), asset2.clone(), 20 * UNITS, false)
            .expect("Quote price exists");
        assert!(amount_out == 10 * UNITS);

        // // Check list_pools
        let pools = list_pools();
        assert!(pools.len() == 1);
        let (pool_asset1, pool_asset2, pool_balance1, pool_balance2) = &pools[0];
        assert!(pool_asset1 == &asset1);
        assert!(pool_asset2 == &asset2);
        assert!(*pool_balance1 == 100 * UNITS);
        assert!(*pool_balance2 == 50 * UNITS);

        // Check get_liquidity_pool
        let (balance1, balance2) = get_liquidity_pool(asset1, asset2).expect("Pool exists");
        Some((balance1, balance2))
    }
}
