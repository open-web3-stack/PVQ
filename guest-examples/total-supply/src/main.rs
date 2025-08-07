//! A guest program that queries the total supply of a given asset.
#![no_std]
#![no_main]

/// A guest program that queries the total supply of a given asset.
#[pvq_program::program]
mod query_total_supply {
    /// Get the total supply of a given asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: The identifier of the asset.
    ///
    /// # Returns
    ///
    /// The total supply of the asset.
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 0)]
    fn total_supply(asset: u32) -> u64 {}

    /// Get the total supply of a given asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: The identifier of the asset.
    ///
    /// # Returns
    ///
    /// The total supply of the asset.
    #[program::entrypoint]
    fn get_total_supply(asset: u32) -> u64 {
        total_supply(asset)
    }
}
