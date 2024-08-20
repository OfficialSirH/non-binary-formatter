use std::io::Read;

use crate::{common::enumerations::PrimitiveTypeEnumeration, errors::NrbfError};

use super::ArrayInfo;

/// The [`ArraySinglePrimitive`] record contains a single-dimensional Array in which all Members are
/// Primitive Value.
#[derive(Debug)]
pub struct ArraySinglePrimitive {
    pub array_info: ArrayInfo,
    pub primitive_type_enum: PrimitiveTypeEnumeration,
}

impl ArraySinglePrimitive {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let array_info = ArrayInfo::deserialize(reader)?;

        let primitive_type_enum = PrimitiveTypeEnumeration::deserialize(reader)?;

        Ok(ArraySinglePrimitive {
            array_info,
            primitive_type_enum,
        })
    }
}