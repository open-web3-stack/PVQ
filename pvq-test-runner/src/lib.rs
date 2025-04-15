use parity_scale_codec::Encode;
use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};
use sp_core::crypto::{AccountId32, Ss58Codec};

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

    pub fn prepare_input_data(program_path: &str) -> Vec<u8> {
        let mut input_data = Vec::new();

        if program_path.contains("sum-balance") {
            input_data.extend_from_slice(&21u32.encode());

            let alice_account: [u8; 32] =
                AccountId32::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
                    .expect("Failed to decode Alice's address")
                    .into();
            input_data.extend_from_slice(&vec![alice_account].encode());
        } else if program_path.contains("total-supply") {
            input_data.extend_from_slice(&21u32.encode());
        } else if program_path.contains("transparent-call") {
            input_data.extend_from_slice(&4071833530116166512u64.encode());
            let alice_account = AccountId32::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
                .expect("Failed to decode Alice's address");
            input_data.extend_from_slice(
                &ExtensionFungiblesFunctions::balance {
                    asset: 21u32,
                    who: alice_account.into(),
                }
                .encode(),
            );
        }
        tracing::info!("Input data (hex): {}", hex::encode(&input_data));
        input_data
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
