//! A guest program that sums the balances of a set of accounts for a given asset and returns the percentage of the total supply.
#![no_std]
#![no_main]

/// The primary module for the sum-balance-percent guest program.
///
/// This module defines the necessary types, extension functions, and the main entrypoint
/// for calculating the percentage of total supply represented by the sum of balances
/// for a given set of accounts.
#[pvq_program::program]
mod sum_balance_percent {
    /// A type alias for the asset identifier.
    type AssetId = u32;
    /// A type alias for the account identifier.
    type AccountId = [u8; 32];
    /// A type alias for the balance of an account.
    type Balance = u64;

    /// Retrieves the balance of a specific account for a given asset.
    ///
    /// This is an extension function that calls into the runtime.
    #[program::extension_fn(extension_id = 1248491991627109725u64, fn_index = 6)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}

    /// Retrieves the total supply of a given asset.
    ///
    /// This is an extension function that calls into the runtime.
    #[program::extension_fn(extension_id = 1248491991627109725u64, fn_index = 5)]
    fn total_supply(asset: AssetId) -> Balance {}

    /// The entrypoint of the program.
    ///
    /// It takes an asset ID and a vector of account IDs, calculates the sum of their balances,
    /// and returns the percentage of this sum with respect to the total supply of the asset.
    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        let mut sum_balance = 0;
        for account in accounts {
            sum_balance += balance(asset, account);
        }
        sum_balance * 100 / total_supply(asset)
    }
}
