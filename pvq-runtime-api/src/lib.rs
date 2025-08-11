//! This crate defines the runtime API for the PVQ module.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use pvq_primitives::PvqResult;
use sp_api::decl_runtime_apis;

decl_runtime_apis! {
    /// The runtime API for the PVQ module.
    pub trait PvqApi {
        /// Executes a PVQ query.
        ///
        /// # Arguments
        ///
        /// * `program`: The PVQ program binary.
        /// * `args`: The SCALE-encoded query arguments.
        /// * `gas_limit`: An optional gas limit for the query execution. If `None`, the execution is constrained by the default time boundary.
        ///
        /// # Returns
        ///
        /// The result of the PVQ query.
        fn execute_query(program: Vec<u8>, args: Vec<u8>, gas_limit: Option<i64>) -> PvqResult;
        /// Returns the metadata of the PVQ extensions.
        fn metadata() -> Vec<u8>;
    }
}
