//! A guest program that sums the balance of a given asset for a list of accounts.
#![no_std]
#![no_main]

/// A guest program that sums the balance of a given asset for a list of accounts.
#[pvq_program::program]
mod sum_balance {

    cfg_if::cfg_if! {
        if #[cfg(feature = "option_version_1")] {
            /// Represents a unique identifier for an account.
            type AccountId = [u8; 64];
            /// Represents a unique identifier for an asset.
            type AssetId = u64;
            /// Represents the balance of an asset.
            type Balance = u128;
        } else if #[cfg(feature = "option_version_2")] {
            /// Represents a unique identifier for an account.
            type AccountId = [u8; 32];
            /// Represents a unique identifier for an asset.
            type AssetId = u32;
            /// Represents the balance of an asset.
            type Balance = u64;
        } else {
            /// Represents a unique identifier for an account.
            type AccountId = [u8; 32];
            /// Represents a unique identifier for an asset.
            type AssetId = u32;
            /// Represents the balance of an asset.
            type Balance = u64;
        }
    }

    /// Get the balance of a given asset for a specific account.
    ///
    /// # Arguments
    ///
    /// * `asset`: The identifier of the asset.
    /// * `who`: The account identifier.
    ///
    /// # Returns
    ///
    /// The balance of the asset for the specified account.
    #[program::extension_fn(extension_id = 1248491991627109725u64, fn_index = 6)]
    fn balance(asset: AssetId, who: AccountId) -> Balance {}

    /// Sums the balance of a given asset for a list of accounts.
    ///
    /// # Arguments
    ///
    /// * `asset`: The identifier of the asset.
    /// * `accounts`: A list of account identifiers.
    ///
    /// # Returns
    ///
    /// The total balance of the asset for all the specified accounts.
    #[program::entrypoint]
    fn sum_balance(asset: AssetId, accounts: alloc::vec::Vec<AccountId>) -> Balance {
        let mut sum = 0;
        for account in accounts {
            sum += balance(asset, account);
        }
        sum
    }
}
