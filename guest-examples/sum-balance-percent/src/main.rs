#![no_std]
#![no_main]

#[pvq_program::program]
mod sum_balance_percent {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
    #[program::extension_fn(extension_id = 1248491991627109725u64, fn_index = 6)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}
    #[program::extension_fn(extension_id = 1248491991627109725u64, fn_index = 5)]
    fn total_supply(asset: AssetId) -> Balance {}

    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        let mut sum_balance = 0;
        for account in accounts {
            sum_balance += balance(asset, account);
        }
        sum_balance * 100 / total_supply(asset)
    }
}
