use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////
///

#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueDetail {
    signed_value_data: SignedValueData,
    descriptor: Option<SignedValueDescriptor>,
}

impl ValueDetail {
    pub fn new(
        signed_value_data: SignedValueData,
        descriptor: Option<SignedValueDescriptor>,
    ) -> Self {
        Self {
            signed_value_data,
            descriptor,
        }
    }

    pub fn validate(
        &self,
        last_descriptor: Option<&SignedValueDescriptor>,
        subkey: ValueSubkey,
        vcrypto: CryptoSystemVersion,
    ) -> Result<(), VeilidAPIError> {
        // Get descriptor to validate with
        let descriptor = if let Some(descriptor) = &self.descriptor {
            if let Some(last_descriptor) = last_descriptor {
                if descriptor.cmp_no_sig(&last_descriptor) != cmp::Ordering::Equal {
                    return Err(VeilidAPIError::generic(
                        "value detail descriptor does not match last descriptor",
                    ));
                }
            }
            descriptor
        } else {
            let Some(descriptor) = last_descriptor else {
                return Err(VeilidAPIError::generic(
                    "no last descriptor, requires a descriptor",
                ));
            };
            descriptor
        };

        // Ensure the descriptor itself validates
        descriptor.validate(vcrypto.clone())?;

        // And the signed value data
        self.signed_value_data
            .validate(descriptor.owner(), subkey, vcrypto)
    }

    pub fn signed_value_data(&self) -> &SignedValueData {
        &self.signed_value_data
    }
    pub fn descriptor(&self) -> Option<&SignedValueDescriptor> {
        self.descriptor.as_ref()
    }
}
