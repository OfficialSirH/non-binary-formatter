use std::io::Read;

use crate::{
    common::enumerations::PrimitiveTypeEnumeration, errors::NrbfError, records::PrimitiveValue,
};

/// The [`ValueWithCode`] structure is used to associate a [Primitive Value](PrimitiveValue) with an Enum that identifies the
/// Primitive Type of the [Primitive Value](PrimitiveValue).
#[derive(Debug)]
pub struct ValueWithCode {
    pub primitive_type_enum: PrimitiveTypeEnumeration,
    pub value: Option<PrimitiveValue>,
}

impl ValueWithCode {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let primitive_type_enum = PrimitiveTypeEnumeration::deserialize(reader)?;

        let value = match primitive_type_enum {
            PrimitiveTypeEnumeration::Null => None,
            _ => Some(PrimitiveValue::try_from((reader, &primitive_type_enum))?),
        };

        Ok(ValueWithCode {
            primitive_type_enum,
            value,
        })
    }
}
