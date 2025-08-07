#![no_std]
#![no_main]

// REVIEW: The `extension_id` is a magic number. It would be better to define it as a constant with a descriptive name.
#[pvq_program::program]
mod query_total_supply {
    #[program::extension_fn(extension_id = 4071833530116166512u64, fn_index = 0)]
    fn total_supply(asset: u32) -> u64 {}

    #[program::entrypoint]
    fn get_total_supply(asset: u32) -> u64 {
        total_supply(asset)
    }
}
