//! This module defines the permission controller for extensions.
use crate::ExtensionIdTy;

/// The source of an extension invocation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InvokeSource {
    /// The invocation is from a runtime API.
    RuntimeAPI,

    /// The invocation is from an XCM (Cross-Consensus Message).
    XCM,

    /// The invocation is from an extrinsic.
    Extrinsic,

    /// The invocation is from the runtime itself.
    Runtime,
}

/// A controller for extension permissions.
///
/// This trait is used to control access to extensions based on the extension ID,
/// call data, and invocation source.
pub trait PermissionController {
    /// Checks if a call to an extension is allowed.
    ///
    /// # Arguments
    ///
    /// * `extension_id`: The identifier of the extension.
    /// * `call`: The encoded call data.
    /// * `source`: The source of the invocation.
    ///
    /// # Returns
    ///
    /// `true` if the call is allowed, `false` otherwise.
    fn is_allowed(extension_id: ExtensionIdTy, call: &[u8], source: InvokeSource) -> bool;
}

/// A default permission controller that allows all calls.
impl PermissionController for () {
    fn is_allowed(_extension_id: ExtensionIdTy, _call: &[u8], _source: InvokeSource) -> bool {
        true
    }
}
