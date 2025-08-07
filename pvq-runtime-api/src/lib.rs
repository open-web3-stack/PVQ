// REVIEW: The crate lacks any form of testing, which is critical for ensuring its correctness and stability.
// Unit tests should be added.
// REVIEW: There is no `README.md` file in the crate, making it difficult for new contributors to understand its purpose and usage.
// A `README.md` file should be created.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use pvq_primitives::PvqResult;
use sp_api::decl_runtime_apis;

// The runtime API for the PVQ module.
//   - `program`: PVQ binary.
//   - `args`: Query arguments that is SCALE-encoded.
//   - `gas_limit`: Optional gas limit for query execution. When set to `None`, execution is constrained by the default time boundary.
decl_runtime_apis! {
    pub trait PvqApi {
        // REVIEW: The `program` and `args` parameters are passed as `Vec<u8>`, which implies ownership.
        // Consider using `&[u8]` to avoid unnecessary allocations when passing data to the runtime API.
        fn execute_query(program: Vec<u8>, args: Vec<u8>, gas_limit: Option<i64>) -> PvqResult;
        // REVIEW: The `metadata` function returns a `Vec<u8>`. If the metadata is static,
        // consider returning a `&'static [u8]` to avoid allocations.
        fn metadata() -> Vec<u8>;
    }
}
