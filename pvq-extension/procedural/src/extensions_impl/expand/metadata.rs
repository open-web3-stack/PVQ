use crate::extensions_impl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// generate the `metadata` function in the #[extensions_impl] module
pub fn expand_metadata(def: &Def) -> TokenStream2 {
    let pvq_extension = &def.pvq_extension;
    let scale_info = &def.scale_info;
    let mut extension_id_call_list = Vec::new();
    let mut extension_metadata_call_list = Vec::new();

    for impl_ in &def.extension_impls {
        let mut trait_path = impl_.trait_path.clone();
        trait_path.segments.pop();

        // Replace trait_path with a call to the metadata function with the impl struct as generic parameter
        let impl_struct_ident = &def.impl_struct.ident;

        let extension_id_call = quote!(
            #trait_path extension_id()
        );
        // Create a method call expression instead of a path
        let method_call = quote!(
            #trait_path metadata::<#impl_struct_ident>()
        );

        extension_id_call_list.push(extension_id_call);
        extension_metadata_call_list.push(method_call);
    }

    let metadata = quote! {
        pub fn metadata() -> #pvq_extension::metadata::Metadata {
            #pvq_extension::metadata::Metadata::new(
                #scale_info::prelude::collections::BTreeMap::from([ #( (#extension_id_call_list, #extension_metadata_call_list), )* ]),
            )
        }
    };
    metadata
}
