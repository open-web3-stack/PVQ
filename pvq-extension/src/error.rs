//! This module defines the error types for the extension system.
// TODO: contain source error
use crate::DispatchError;
use parity_scale_codec::Error as CodecError;

/// The error type for the extension system.
// Typically will be used as a UserError
#[derive(Debug, thiserror::Error)]
pub enum ExtensionError {
    /// Permission to perform the requested operation was denied.
    #[error("Permission denied")]
    PermissionError,

    /// Failed to allocate memory.
    #[error("Failed to allocate memory")]
    MemoryAllocationError,

    /// An error occurred while accessing memory.
    #[error("Memory access error: {0}")]
    MemoryAccessError(polkavm::MemoryAccessError),

    /// An error occurred while decoding data.
    #[error("Decode error: {0}")]
    DecodeError(CodecError),

    /// An error occurred while dispatching a call.
    #[error("Dispatch error: {0:?}")]
    DispatchError(#[from] DispatchError),

    /// The requested extension is not supported.
    #[error("Unsupported extension")]
    UnsupportedExtension,
}

impl From<polkavm::MemoryAccessError> for ExtensionError {
    fn from(e: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(e)
    }
}

impl From<CodecError> for ExtensionError {
    fn from(e: CodecError) -> Self {
        Self::DecodeError(e)
    }
}
