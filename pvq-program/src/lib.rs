// REVIEW: The crate lacks any form of testing, which is critical for ensuring its correctness and stability.
// Unit tests should be added.
// REVIEW: There is no `README.md` file in the crate, making it difficult for new contributors to understand its purpose and usage.
// A `README.md` file should be created.
#![cfg_attr(not(feature = "std"), no_std)]
pub use pvq_program_procedural::program;
