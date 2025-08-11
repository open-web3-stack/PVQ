//! This crate defines the primitive types for the PVQ system.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use parity_scale_codec::{Decode, Encode};
#[cfg(not(feature = "std"))]
use scale_info::TypeInfo;

/// The result of a PVQ query.
pub type PvqResult = Result<PvqResponse, PvqError>;

/// The response of a PVQ query.
pub type PvqResponse = Vec<u8>;

/// The error of a PVQ query.
#[cfg(feature = "std")]
pub type PvqError = String;

/// The error of a PVQ query.
#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum PvqError {
    /// Failed to decode the query.
    FailedToDecode,
    /// The PVQ program format is invalid.
    InvalidPvqProgramFormat,
    /// The query exceeds the weight limit.
    QueryExceedsWeightLimit,
    /// A trap occurred during execution.
    Trap,
    /// A memory access error occurred.
    MemoryAccessError,
    /// A host call error occurred.
    HostCallError,
    /// An other error occurred.
    Other,
}
