use parity_scale_codec::Encode;
use scale_info::{
    form::{Form, MetaForm, PortableForm},
    prelude::vec::Vec,
    IntoPortable, PortableRegistry, Registry,
};
type ExtensionId = u64;
type FnIndex = u8;
/// Metadata of extensions
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
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

#[derive(Clone, PartialEq, Eq, Encode, Debug)]
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

#[derive(Clone, PartialEq, Eq, Encode, Debug)]
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
