//! This module defines the traits and types for handling extension call data.
use parity_scale_codec::Decode;
use scale_info::prelude::vec::Vec;

/// The type for extension identifiers.
pub type ExtensionIdTy = u64;

/// A trait for identifying extensions.
pub trait ExtensionId {
    /// The unique identifier of the extension.
    const EXTENSION_ID: ExtensionIdTy;
}

/// A trait for dispatching extension calls.
pub trait Dispatchable {
    /// Dispatches the extension call.
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

/// The error type for dispatch operations.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    /// A phantom data error.
    #[error("PhantomData")]
    PhantomData,
}

/// A trait for extension call data.
///
/// This trait combines several traits that are required for extension call data:
/// - `Dispatchable`: Allows dispatching calls to the extension functions.
/// - `ExtensionId`: Identifies the extension.
/// - `Decode`: Allows decoding the call data.
pub trait CallData: Dispatchable + ExtensionId + Decode {}
impl<T> CallData for T where T: Dispatchable + ExtensionId + Decode {}
