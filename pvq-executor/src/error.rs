use pvq_primitives::PvqError;
#[derive(Debug, thiserror::Error)]
pub enum PvqExecutorError<UserError> {
    #[error("Invalid PVQ program format")]
    InvalidProgramFormat,
    #[error("Memory access error: {0}")]
    MemoryAccessError(polkavm::MemoryAccessError),
    // Extract from the PVM CallError
    #[error("Trap")]
    Trap,
    // Extract from the PVM CallError
    #[error("Not enough gas")]
    NotEnoughGas,
    // Usually a custom error type from the extension system definition
    #[error("User error: {0}")]
    User(UserError),
    // Other errors directly from the PVM
    #[error("Other PVM error: {0}")]
    OtherPvmError(polkavm::Error),
}

impl<UserError> From<polkavm::CallError<UserError>> for PvqExecutorError<UserError> {
    fn from(err: polkavm::CallError<UserError>) -> Self {
        match err {
            polkavm::CallError::Trap => Self::Trap,
            polkavm::CallError::NotEnoughGas => Self::NotEnoughGas,
            polkavm::CallError::Error(e) => Self::OtherPvmError(e),
            polkavm::CallError::User(e) => Self::User(e),
        }
    }
}

impl<UserError> From<polkavm::Error> for PvqExecutorError<UserError> {
    fn from(e: polkavm::Error) -> Self {
        Self::OtherPvmError(e)
    }
}

impl<UserError> From<polkavm::MemoryAccessError> for PvqExecutorError<UserError> {
    fn from(e: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(e)
    }
}

#[cfg(feature = "std")]
impl<UserError: core::fmt::Debug> From<PvqExecutorError<UserError>> for PvqError {
    fn from(e: PvqExecutorError<UserError>) -> PvqError {
        match e {
            PvqExecutorError::InvalidProgramFormat => "Invalid PVQ program format".to_string(),
            PvqExecutorError::MemoryAccessError(_) => "Memory access error".to_string(),
            PvqExecutorError::Trap => "Trap".to_string(),
            PvqExecutorError::NotEnoughGas => "Not enough gas".to_string(),
            PvqExecutorError::User(user_error) => format!("Host call error: {user_error:?}"),
            PvqExecutorError::OtherPvmError(pvm_error) => format!("Other error: {pvm_error:?}"),
        }
    }
}

#[cfg(not(feature = "std"))]
impl<UserError> From<PvqExecutorError<UserError>> for PvqError {
    fn from(e: PvqExecutorError<UserError>) -> PvqError {
        match e {
            PvqExecutorError::InvalidProgramFormat => PvqError::InvalidPvqProgramFormat,
            PvqExecutorError::MemoryAccessError(_) => PvqError::MemoryAccessError,
            PvqExecutorError::Trap => PvqError::Trap,
            PvqExecutorError::NotEnoughGas => PvqError::QueryExceedsWeightLimit,
            PvqExecutorError::User(_) => PvqError::HostCallError,
            PvqExecutorError::OtherPvmError(_) => PvqError::Other,
        }
    }
}
