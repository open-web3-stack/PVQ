#![no_std]
#![no_main]

// REVIEW: The `extension_id` is a magic number. It would be better to define it as a constant with a descriptive name.
#[pvq_program::program]
mod sum_balance_percent {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 1)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 0)]
    fn total_supply(asset: AssetId) -> Balance {}

    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        // REVIEW: The entrypoint should be renamed to `sum_balance_percent` to reflect that it returns a percentage.
        // Also, the percentage calculation `sum_balance * 100 / total_supply(asset)` can be misleading.
        // It should be clarified that this is an integer division and might not be precise.
        let mut sum_balance = 0;
        for account in accounts {
            sum_balance += balance(asset, account);
        }
        sum_balance * 100 / total_supply(asset)
    }
}
