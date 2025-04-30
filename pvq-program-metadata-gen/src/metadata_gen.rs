use quote::{quote, ToTokens};
use syn::spanned::Spanned;

type ExtensionId = u64;
type FnIndex = u8;

pub fn metadata_gen_src(source: &str, pkg_name: &str, output_dir: &str) -> syn::Result<proc_macro2::TokenStream> {
    // Parse the source code
    let mut syntax = syn::parse_file(source)?;

    // Find the program module
    // Find the index of the program module
    let program_mod_idx = syntax
        .items
        .iter()
        .position(|item| matches!(item, syn::Item::Mod(m) if m.attrs.iter().any(|attr|attr.path().segments.last().is_some_and(|last|last.ident == "program"))))
        .ok_or(syn::Error::new(
            proc_macro2::Span::call_site(),
            "No program module found",
        ))?;

    // Remove the program module from syntax.items
    let mut program_mod = match syntax.items.remove(program_mod_idx) {
        syn::Item::Mod(m) => m,
        _ => unreachable!("We already checked this is a module"),
    };

    // Remove the program attr
    program_mod.attrs.clear();
    let program_mod_items = &mut program_mod.content.as_mut().expect("This is checked before").1;

    // Find entrypoint and extension functions
    let mut entrypoint_metadata = None;
    let mut extension_fns_metadata = Vec::new();

    for i in (0..program_mod_items.len()).rev() {
        let item = &mut program_mod_items[i];
        if let Some(attr) = crate::helper::take_first_program_attr(item)? {
            if let Some(last_segment) = attr.path().segments.last() {
                if last_segment.ident == "extension_fn" {
                    let mut extension_id = None;
                    let mut fn_index = None;
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("extension_id") {
                            let value = meta.value()?;
                            extension_id = Some(value.parse::<syn::LitInt>()?.base10_parse::<ExtensionId>()?);
                        } else if meta.path.is_ident("fn_index") {
                            let value = meta.value()?;
                            fn_index = Some(value.parse::<syn::LitInt>()?.base10_parse::<FnIndex>()?);
                        } else {
                            return Err(syn::Error::new(
                                meta.path.span(),
                                "Invalid attribute meta, expected `extension_id` or `fn_index`",
                            ));
                        }
                        Ok(())
                    })?;
                    let removed_item = program_mod_items.remove(i);
                    if extension_id.is_none() || fn_index.is_none() {
                        return Err(syn::Error::new(
                            attr.span(),
                            "Extension ID and function index are required",
                        ));
                    }
                    let extension_id =
                        extension_id.ok_or_else(|| syn::Error::new(attr.span(), "Extension ID is required"))?;
                    let fn_index =
                        fn_index.ok_or_else(|| syn::Error::new(attr.span(), "Function index is required"))?;
                    let extension_fn_metadata = generate_extension_fn_metadata(removed_item, extension_id, fn_index)?;
                    extension_fns_metadata.push(extension_fn_metadata);
                } else if last_segment.ident == "entrypoint" {
                    if entrypoint_metadata.is_some() {
                        return Err(syn::Error::new(attr.span(), "Multiple entrypoint functions found"));
                    }
                    let removed_item = program_mod_items.remove(i);
                    entrypoint_metadata = Some(generate_entrypoint_metadata(removed_item)?);
                } else {
                    return Err(syn::Error::new(
                        attr.span(),
                        "Invalid attribute, expected `#[program::extension_fn]` or `#[program::entrypoint]`",
                    ));
                }
            }
        }
    }

    let entrypoint_metadata = entrypoint_metadata
        .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "No entrypoint function found"))?;

    let metadata_defs = metadata_defs();
    let import_packages = import_packages();

    let new_items = quote! {
        #(#program_mod_items)*
        #import_packages
        #metadata_defs
        fn main() {
            let extension_fns = vec![ #( #extension_fns_metadata, )* ];
            let entrypoint = #entrypoint_metadata;
            let metadata = Metadata::new(extension_fns, entrypoint);
            // Serialize to both formats
            let encoded = parity_scale_codec::Encode::encode(&metadata);
            let json = serde_json::to_string(&metadata).expect("Failed to serialize metadata to JSON");

            let bin_path = Path::new(#output_dir).join(format!("{}-metadata.bin", #pkg_name));
            let json_path = Path::new(#output_dir).join(format!("{}-metadata.json", #pkg_name));

            // Write the binary format
            std::fs::write(bin_path, &encoded).expect("Failed to write binary metadata");

            // Write the JSON format
            std::fs::write(json_path, json).expect("Failed to write JSON metadata");
        }
    };

    // Remove #![no_main] and #![no_std] attributes if present
    syntax.attrs.retain(|attr| {
        if let Some(segment) = attr.path().segments.last() {
            let ident = &segment.ident;
            !(ident == "no_main" || ident == "no_std")
        } else {
            true
        }
    });

    syntax.items.push(syn::Item::Verbatim(new_items));

    Ok(syntax.into_token_stream())
}

fn generate_extension_fn_metadata(
    f: syn::Item,
    extension_id: ExtensionId,
    fn_index: FnIndex,
) -> syn::Result<proc_macro2::TokenStream> {
    if let syn::Item::Fn(f) = f {
        let fn_name = f.sig.ident.to_string();
        let mut inputs = Vec::new();
        for input in &f.sig.inputs {
            if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = input {
                if let syn::Pat::Ident(pat_ident) = &**pat {
                    let name = pat_ident.ident.to_string();
                    inputs.push(quote!(
                    FunctionParamMetadata {
                        name: #name,
                            ty: scale_info::meta_type::<#ty>(),
                        }
                    ));
                } else {
                    return Err(syn::Error::new(input.span(), "Expected a typed argument"));
                }
            } else {
                return Err(syn::Error::new(input.span(), "Expected a typed argument"));
            }
        }
        let output = match &f.sig.output {
            syn::ReturnType::Default => quote!(scale_info::meta_type::<()>()),
            syn::ReturnType::Type(_, ty) => {
                quote!(scale_info::meta_type::<#ty>())
            }
        };
        Ok(quote! {
            (#extension_id, #fn_index, FunctionMetadata {
                name: #fn_name,
                inputs: vec![#(#inputs,)*],
                output: #output
            })
        })
    } else {
        Err(syn::Error::new(f.span(), "Expected a function"))
    }
}
fn generate_entrypoint_metadata(f: syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    if let syn::Item::Fn(f) = f {
        let name = f.sig.ident.to_string();
        let mut inputs = Vec::new();
        for input in &f.sig.inputs {
            if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = input {
                if let syn::Pat::Ident(pat_ident) = &**pat {
                    let name = pat_ident.ident.to_string();
                    inputs.push(quote!(
                    FunctionParamMetadata {
                        name: #name,
                            ty: scale_info::meta_type::<#ty>(),
                        }
                    ));
                } else {
                    return Err(syn::Error::new(input.span(), "Expected a typed argument"));
                }
            } else {
                return Err(syn::Error::new(input.span(), "Expected a typed argument"));
            }
        }
        let output = match &f.sig.output {
            syn::ReturnType::Default => quote!(scale_info::meta_type::<()>()),
            syn::ReturnType::Type(_, ty) => {
                quote!(scale_info::meta_type::<#ty>())
            }
        };
        Ok(quote! {
            FunctionMetadata {
                name: #name,
                inputs: vec![#(#inputs,)*],
                output: #output
            }
        })
    } else {
        Err(syn::Error::new(f.span(), "Expected a function"))
    }
}

fn import_packages() -> proc_macro2::TokenStream {
    quote! {
        extern crate alloc;
        use std::path::Path;
        use serde::Serialize;
        use parity_scale_codec::Encode;
        use scale_info::{
            form::{Form, MetaForm, PortableForm},
            prelude::vec::Vec,
            IntoPortable, PortableRegistry, Registry,
        };
    }
}
fn metadata_defs() -> proc_macro2::TokenStream {
    quote! {
        type ExtensionId = u64;
        type FnIndex = u8;
        /// Metadata of extensions
        #[derive(Clone, PartialEq, Eq, Encode, Debug, Serialize)]
        pub struct Metadata {
            pub types: PortableRegistry,
            pub extension_fns: Vec<(ExtensionId, FnIndex, FunctionMetadata<PortableForm>)>,
            pub entrypoint: FunctionMetadata<PortableForm>,
        }

        impl Metadata {
            pub fn new(extension_fns: Vec<(ExtensionId, FnIndex, FunctionMetadata)>, entrypoint: FunctionMetadata) -> Self {
                let mut registry = Registry::new();
                let extension_fns = extension_fns
                    .into_iter()
                    .map(|(id, index, metadata)| (id, index, metadata.into_portable(&mut registry)))
                    .collect();
                let entrypoint = entrypoint.into_portable(&mut registry);
                Self {
                    types: registry.into(),
                    extension_fns,
                    entrypoint,
                }
            }
        }

        #[derive(Clone, PartialEq, Eq, Encode, Debug, Serialize)]
        pub struct FunctionMetadata<T: Form = MetaForm> {
            pub name: T::String,
            pub inputs: Vec<FunctionParamMetadata<T>>,
            pub output: T::Type,
        }

        impl IntoPortable for FunctionMetadata {
            type Output = FunctionMetadata<PortableForm>;

            fn into_portable(self, registry: &mut Registry) -> Self::Output {
                FunctionMetadata {
                    name: self.name.into_portable(registry),
                    inputs: registry.map_into_portable(self.inputs),
                    output: registry.register_type(&self.output),
                }
            }
        }

        #[derive(Clone, PartialEq, Eq, Encode, Debug, Serialize)]
        pub struct FunctionParamMetadata<T: Form = MetaForm> {
            pub name: T::String,
            pub ty: T::Type,
        }

        impl IntoPortable for FunctionParamMetadata {
            type Output = FunctionParamMetadata<PortableForm>;

            fn into_portable(self, registry: &mut Registry) -> Self::Output {
                FunctionParamMetadata {
                    name: self.name.into_portable(registry),
                    ty: registry.register_type(&self.ty),
                }
            }
        }
    }
}
