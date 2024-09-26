use std::io::Read;

use crate::{
    common::{data_types::LengthPrefixedString, enumerations::PrimitiveTypeEnumeration},
    deserializer::from_reader,
    errors::Error,
};

// Documentation Types Import
#[allow(unused)]
pub use crate::records::method_invocation::ValueWithCode;

/// The [`StringValueWithCode`] structure is a [`ValueWithCode`] where [`PrimitiveTypeEnumeration`] is
/// [String (18)](PrimitiveTypeEnumeration::String).
#[derive(Debug)]
pub struct StringValueWithCode {
    pub primitive_type_enum: PrimitiveTypeEnumeration,
    pub value: LengthPrefixedString,
}

impl StringValueWithCode {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let primitive_type_enum: PrimitiveTypeEnumeration = from_reader(reader)?;

        let value = LengthPrefixedString::deserialize(reader)?;

        Ok(StringValueWithCode {
            primitive_type_enum,
            value,
        })
    }
}
