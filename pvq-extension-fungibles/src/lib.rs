// REVIEW: The crate lacks any form of testing, which is critical for ensuring its correctness and stability.
// Unit tests should be added.
// REVIEW: There is no `README.md` file in the crate, making it difficult for new contributors to understand its purpose and usage.
// A `README.md` file should be created.
#![cfg_attr(not(feature = "std"), no_std)]
use pvq_extension::extension_decl;

#[extension_decl]
pub mod extension {
    #[extension_decl::extension]
    pub trait ExtensionFungibles {
        type AssetId;
        type Balance;
        type AccountId;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }
}
