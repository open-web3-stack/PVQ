//! This crate provides procedural macros for the PVQ extension system.
use proc_macro::TokenStream;
mod extension_decl;
mod extensions_impl;
pub(crate) mod utils;

/// A procedural macro for declaring an extension.
#[proc_macro_attribute]
pub fn extension_decl(attr: TokenStream, item: TokenStream) -> TokenStream {
    extension_decl::extension_decl(attr, item)
}

/// A procedural macro for implementing extensions.
#[proc_macro_attribute]
pub fn extensions_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    extensions_impl::extensions_impl(attr, item)
}
