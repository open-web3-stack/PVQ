// REVIEW: The crate lacks any form of testing, which is critical for ensuring its correctness and stability.
// Unit tests should be added.
// REVIEW: There is no `README.md` file in the crate, making it difficult for new contributors to understand its purpose and usage.
// A `README.md` file should be created.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloc::vec::Vec;
pub use polkavm::{Caller, Config, Engine, Linker, Module, ProgramBlob};

mod context;
mod error;
mod executor;

pub use context::PvqExecutorContext;
pub use error::PvqExecutorError;
pub use executor::PvqExecutor;
