/// Declare the calls used in PVQ program
/// ```ignore
/// #[program]
/// mod query_fungibles {
///     // The types to be used in the program, which matches the runtime implementation
///     type AssetId = u32;
///     type AccountId = [u8; 32];
///     type Balance = u64;
///
///     #[program::extension_fn(extension_id = 123456u64, fn_index = 1u8)]
///     fn balance(asset: AssetId, who: AccountId) -> Balance;
///
///     #[program::entrypoint]
///     fn sum_balance(accounts: Vec<AccountId>) -> Balance {
///         let mut sum = 0;
///         for account in accounts {
///             sum += balance(0, account);
///         }
///         sum
///     }
/// }
/// ```
///
// REVIEW: The procedural macro lacks tests. UI tests should be added to ensure the macro generates correct code and provides good error messages for invalid input.
// REVIEW: A `README.md` file in the `procedural` crate's directory would be beneficial to explain its usage and how it works.
mod program;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn program(attr: TokenStream, item: TokenStream) -> TokenStream {
    program::program(attr, item)
}
