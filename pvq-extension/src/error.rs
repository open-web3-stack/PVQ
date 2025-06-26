// TODO: contain source error
use crate::DispatchError;
use parity_scale_codec::Error as CodecError;

/// Errors that can occur when working with extensions
// Typically will be used as a UserError
#[derive(Debug, thiserror::Error)]
pub enum ExtensionError {
    /// Permission denied for the requested operation
    #[error("Permission denied")]
    PermissionError,

    /// Failed to allocate memory
    #[error("Failed to allocate memory")]
    MemoryAllocationError,

    /// Error accessing memory
    #[error("Memory access error: {0}")]
    MemoryAccessError(polkavm::MemoryAccessError),

    /// Error decoding data
    #[error("Decode error: {0}")]
    DecodeError(CodecError),

    /// Error dispatching a call
    #[error("Dispatch error: {0:?}")]
    DispatchError(#[from] DispatchError),

    /// The requested extension is not supported
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
