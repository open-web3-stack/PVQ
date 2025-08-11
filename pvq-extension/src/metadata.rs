//! This module defines the metadata structures for extensions.
use crate::ExtensionIdTy;
use scale_info::prelude::string::{String, ToString};

/// A trait for retrieving extension implementation metadata.
pub trait ExtensionImplMetadata {
    /// Returns the metadata for a given extension.
    fn extension_metadata(extension_id: ExtensionIdTy) -> ExtensionMetadata;
}

use parity_scale_codec::Encode;
use scale_info::{
    form::{Form, MetaForm, PortableForm},
    prelude::collections::BTreeMap,
    prelude::vec::Vec,
    IntoPortable, PortableRegistry, Registry,
};
use serde::Serialize;
/// The metadata of all extensions.
#[derive(Clone, PartialEq, Eq, Encode, Debug, Serialize)]
pub struct Metadata {
    /// The portable type registry.
    pub types: PortableRegistry,
    /// A map of extension identifiers to their metadata.
    pub extensions: BTreeMap<String, ExtensionMetadata<PortableForm>>,
}

impl Metadata {
    /// Creates a new `Metadata` instance.
    pub fn new(extensions: BTreeMap<ExtensionIdTy, ExtensionMetadata>) -> Self {
        let mut registry = Registry::new();
        let extensions = extensions
            .into_iter()
            .map(|(id, metadata)| (id.to_string(), metadata.into_portable(&mut registry)))
            .collect();
        Self {
            types: registry.into(),
            extensions,
        }
    }
}

/// The metadata of an extension.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct ExtensionMetadata<T: Form = MetaForm> {
    /// The name of the extension.
    pub name: T::String,
    /// The functions of the extension.
    pub functions: Vec<FunctionMetadata<T>>,
}

impl IntoPortable for ExtensionMetadata {
    type Output = ExtensionMetadata<PortableForm>;

    fn into_portable(self, registry: &mut Registry) -> Self::Output {
        ExtensionMetadata {
            name: self.name.into_portable(registry),
            functions: registry.map_into_portable(self.functions),
        }
    }
}

impl Serialize for ExtensionMetadata<PortableForm> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ExtensionMetadata", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("functions", &self.functions)?;
        state.end()
    }
}

/// The metadata of a runtime function.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct FunctionMetadata<T: Form = MetaForm> {
    /// The name of the function.
    pub name: T::String,
    /// The parameters of the function.
    pub inputs: Vec<FunctionParamMetadata<T>>,
    /// The output of the function.
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

impl Serialize for FunctionMetadata<PortableForm> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FunctionMetadata", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("output", &self.output)?;
        state.end()
    }
}

/// The metadata of a runtime function parameter.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
pub struct FunctionParamMetadata<T: Form = MetaForm> {
    /// The name of the parameter.
    pub name: T::String,
    /// The type of the parameter.
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

impl Serialize for FunctionParamMetadata<PortableForm> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FunctionParamMetadata", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("ty", &self.ty)?;
        state.end()
    }
}
