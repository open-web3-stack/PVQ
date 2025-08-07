#![no_std]
#![no_main]

// REVIEW: The `extension_id` is a magic number. It would be better to define it as a constant with a descriptive name.
#[pvq_program::program]
mod sum_balance {

    cfg_if::cfg_if! {
        if #[cfg(feature = "option_version_1")] {
            type AccountId = [u8; 64];
            type AssetId = u64;
            type Balance = u128;
        } else if #[cfg(feature = "option_version_2")] {
            type AccountId = [u8; 32];
            type AssetId = u32;
            type Balance = u64;
        } else {
            type AccountId = [u8; 32];
            type AssetId = u32;
            type Balance = u64;
        }
    }

    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 1)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}

    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        let mut sum = 0;
        for account in accounts {
            sum += balance(asset, account);
        }
        sum
    }
}
