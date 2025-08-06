use pvq_extension::metadata::Metadata;
use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};

#[extensions_impl]
pub mod extensions {
    use frame::deps::scale_info::prelude::vec::Vec;
    use frame::token::fungibles;

    #[extensions_impl::impl_struct]
    pub struct ExtensionImpl;

    #[extensions_impl::extension]
    impl pvq_extension_core::extension::ExtensionCore for ExtensionImpl {
        type ExtensionId = u64;
        fn has_extension(id: Self::ExtensionId) -> bool {
            id == pvq_extension_core::extension::extension_id()
                || id == pvq_extension_fungibles::extension::extension_id()
        }
    }

    #[extensions_impl::extension]
    impl pvq_extension_fungibles::extension::ExtensionFungibles for ExtensionImpl {
        type AccountId = crate::interface::AccountId;
        type Balance = crate::interface::Balance;
        type AssetId = crate::interface::AssetId;
        fn name(asset: Self::AssetId) -> Vec<u8> {
            <crate::Assets as fungibles::metadata::Inspect<crate::interface::AccountId>>::name(asset)
        }
        fn symbol(asset: Self::AssetId) -> Vec<u8> {
            <crate::Assets as fungibles::metadata::Inspect<crate::interface::AccountId>>::symbol(asset)
        }
        fn decimals(asset: Self::AssetId) -> u8 {
            <crate::Assets as fungibles::metadata::Inspect<crate::interface::AccountId>>::decimals(asset)
        }
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance {
            <crate::Assets as fungibles::Inspect<crate::interface::AccountId>>::balance(asset, &who)
        }
        fn total_supply(asset: Self::AssetId) -> Self::Balance {
            <crate::Assets as fungibles::Inspect<crate::interface::AccountId>>::total_issuance(asset)
        }
        fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
            <crate::Assets as fungibles::Inspect<crate::interface::AccountId>>::minimum_balance(asset)
        }
        fn asset_exists(asset: Self::AssetId) -> bool {
            <crate::Assets as fungibles::Inspect<crate::interface::AccountId>>::asset_exists(asset)
        }
    }
}

pub fn execute_query(program: &[u8], args: &[u8], gas_limit: i64) -> pvq_primitives::PvqResult {
    let mut executor = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let (result, _) = executor.execute(program, args, Some(gas_limit));
    result
}

pub fn metadata() -> Metadata {
    extensions::metadata()
}
