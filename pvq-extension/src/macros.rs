use crate::{CallData, ExtensionError, ExtensionIdTy};
use fortuples::fortuples;
use scale_info::prelude::vec::Vec;

/// Trait for a tuple of extension call data types
pub trait CallDataTuple {
    /// Dispatch a call to an extension
    ///
    /// # Arguments
    ///
    /// * `extension_id` - The ID of the extension to call
    /// * `data` - The encoded call data
    ///
    /// # Returns
    ///
    /// The encoded response data or an error
    fn dispatch(extension_id: ExtensionIdTy, data: &[u8]) -> Result<Vec<u8>, ExtensionError>;
}

impl<Member0> CallDataTuple for Member0
where
    Member0: CallData,
{
    fn dispatch(extension_id: ExtensionIdTy, mut data: &[u8]) -> Result<Vec<u8>, ExtensionError> {
        if extension_id == Member0::EXTENSION_ID {
            return Member0::decode(&mut data)
                .map_err(ExtensionError::DecodeError)?
                .dispatch()
                .map_err(ExtensionError::DispatchError);
        }
        Err(ExtensionError::UnsupportedExtension)
    }
}

fortuples! {
    #[tuples::min_size(1)]
    impl CallDataTuple for #Tuple where #(#Member: CallData),*{
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        fn dispatch(extension_id: ExtensionIdTy, mut call: &[u8]) -> Result<Vec<u8>, ExtensionError> {
            #(
                if extension_id == #Member::EXTENSION_ID {
                    return #Member::decode(&mut call).map_err(ExtensionError::DecodeError)?.dispatch().map_err(ExtensionError::DispatchError);
                }
            )*
            Err(ExtensionError::UnsupportedExtension)
        }
    }
}
