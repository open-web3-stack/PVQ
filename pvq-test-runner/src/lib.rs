use parity_scale_codec::Encode;
use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};
use pvq_primitives::PvqResult;
use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_core::hexdisplay::HexDisplay;
use staging_xcm::v5::*;

#[derive(Encode)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum ExtensionFungiblesFunctions {
    #[codec(index = 0)]
    total_supply { asset: u32 },
    #[codec(index = 1)]
    balance { asset: u32, who: [u8; 32] },
}

#[extensions_impl]
pub mod extensions {
    use parity_scale_codec::Decode;
    #[extensions_impl::impl_struct]
    pub struct ExtensionsImpl;

    #[extensions_impl::extension]
    impl pvq_extension_core::extension::ExtensionCore for ExtensionsImpl {
        type ExtensionId = u64;
        fn has_extension(id: Self::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    #[extensions_impl::extension]
    impl pvq_extension_fungibles::extension::ExtensionFungibles for ExtensionsImpl {
        type AssetId = u32;
        type AccountId = [u8; 32];
        type Balance = u64;
        fn total_supply(_asset: Self::AssetId) -> Self::Balance {
            100
        }
        fn balance(_asset: Self::AssetId, _who: Self::AccountId) -> Self::Balance {
            100
        }
    }
    #[extensions_impl::extension]
    impl pvq_extension_swap::extension::ExtensionSwap for ExtensionsImpl {
        type AssetId = Vec<u8>;
        type Balance = u128;
        fn quote_price_tokens_for_exact_tokens(
            _asset1: Self::AssetId,
            _asset2: Self::AssetId,
            _amount: Self::Balance,
            _include_fee: bool,
        ) -> Option<Self::Balance> {
            None
        }

        fn quote_price_exact_tokens_for_tokens(
            _asset1: Self::AssetId,
            _asset2: Self::AssetId,
            _amount: Self::Balance,
            _include_fee: bool,
        ) -> Option<Self::Balance> {
            None
        }

        fn get_liquidity_pool(asset1: Self::AssetId, asset2: Self::AssetId) -> Option<(Self::Balance, Self::Balance)> {
            let _asset1 = u32::decode(&mut &asset1[..]).expect("Failed to decode asset1");
            let _asset2 = u32::decode(&mut &asset2[..]).expect("Failed to decode asset2");
            Some((100, 100))
        }

        fn list_pools() -> Vec<(Self::AssetId, Self::AssetId, Self::Balance, Self::Balance)> {
            vec![]
        }
    }
}

pub struct TestRunner {
    executor: ExtensionsExecutor<extensions::Extensions, ()>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            executor: ExtensionsExecutor::new(InvokeSource::RuntimeAPI),
        }
    }

    pub fn prepare_input_data(program_path: &str, chain: &str) -> Vec<u8> {
        let mut input_data = Vec::new();

        if program_path.contains("sum-balance") {
            if chain == "poc" {
                input_data.extend_from_slice(&[0u8]);
                input_data.extend_from_slice(&21u32.encode());
                let alice_account: [u8; 32] =
                    AccountId32::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
                        .expect("Failed to decode Alice's address")
                        .into();
                input_data.extend_from_slice(&vec![alice_account].encode());
            }
        } else if program_path.contains("total-supply") {
            if chain == "poc" {
                input_data.extend_from_slice(&[0u8]);
                input_data.extend_from_slice(&21u32.encode());
            }
        } else if program_path.contains("swap-info") {
            if chain == "acala" {
                input_data.extend_from_slice(&[2u8]);
                let asset1 = u32::encode(&21);
                let asset2 = u32::encode(&22);
                input_data.extend_from_slice(&asset1.encode());
                input_data.extend_from_slice(&asset2.encode());
            } else if chain == "ah" {
                input_data.extend_from_slice(&[2u8]);
                let native_location = staging_xcm::v5::Location::parent();
                let assets_2511_location =
                    staging_xcm::v5::Location::new(0, (Junction::PalletInstance(50), Junction::GeneralIndex(2511)));
                input_data.extend_from_slice(&native_location.encode().encode());
                input_data.extend_from_slice(&assets_2511_location.encode().encode());
            }
        };
        tracing::info!("Input data (hex): {}", HexDisplay::from(&input_data));
        tracing::info!("Using chain: {}", chain);
        input_data
    }

    pub fn expected_result(program_path: &str, chain: &str, entrypoint_idx: u8) -> Vec<u8> {
        // TODO: add more entrypoints
        if program_path.contains("swap-info") {
            if chain == "poc" {
                return Vec::new();
            } else if chain == "ah" {
                if entrypoint_idx == 2 {
                    return (10_235_709_412_325u128, 12_117_819_770_919u128).encode();
                }
            }
        } else if program_path.contains("sum-balance") {
            return Vec::new();
        } else if program_path.contains("total-supply") {
            return Vec::new();
        }

        // Default empty result
        Vec::new()
    }

    pub fn execute_program(&mut self, program_blob: &[u8], input_data: &[u8]) -> pvq_primitives::PvqResult {
        let (result, _) = self.executor.execute(program_blob, input_data, None);
        result
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}
