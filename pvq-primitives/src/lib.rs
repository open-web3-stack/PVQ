#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use parity_scale_codec::{Decode, Encode};
#[cfg(not(feature = "std"))]
use scale_info::TypeInfo;

pub type PvqResult = Result<PvqResponse, PvqError>;

pub type PvqResponse = Vec<u8>;

#[cfg(feature = "std")]
pub type PvqError = String;

#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum PvqError {
    FailedToDecode,
    InvalidPvqProgramFormat,
    QueryExceedsWeightLimit,
    Trap,
    MemoryAccessError,
    HostCallError,
    Other,
}
