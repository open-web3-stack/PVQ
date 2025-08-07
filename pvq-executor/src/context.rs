use polkavm::Linker;

/// The context for the PVQ executor.
pub trait PvqExecutorContext {
    /// The user data.
    type UserData;
    /// The user error.
    type UserError;
    /// Registers the host functions.
    fn register_host_functions(&mut self, linker: &mut Linker<Self::UserData, Self::UserError>);
    /// Returns a mutable reference to the user data.
    fn data(&mut self) -> &mut Self::UserData;
}
