#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pvq_extension::extension_decl;

#[extension_decl]
pub mod extension {
    use alloc::vec::Vec;

    #[extension_decl::extension]
    pub trait ExtensionSwap {
        type AssetId;
        type Balance;

        fn quote_price_tokens_for_exact_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        fn quote_price_exact_tokens_for_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        fn get_liquidity_pool(asset1: Self::AssetId, asset2: Self::AssetId) -> Option<(Self::Balance, Self::Balance)>;

        #[allow(clippy::type_complexity)]
        fn list_pools() -> Vec<(Self::AssetId, Self::AssetId, Self::Balance, Self::Balance)>;
    }
}
