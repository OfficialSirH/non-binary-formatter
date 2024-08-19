use std::io::Read;

use crate::{
    common::enumerations::BinaryTypeEnum,
    errors::NrbfError,
    readers::{read_i32, read_u8},
    records::AdditionalTypeInfo,
};

use super::BinaryArrayTypeEnumeration;

/// [`BinaryArray`] is the most general form of Array records. The record is more verbose than the other
/// Array records.
#[derive(Debug)]
pub struct BinaryArray {
    pub object_id: i32,
    pub binary_array_type_enumeration: BinaryArrayTypeEnumeration,
    pub rank: i32,
    pub lengths: Vec<i32>,
    pub lower_bounds: Option<Vec<i32>>,
    pub type_enum: BinaryTypeEnum,
    pub additional_type_info: AdditionalTypeInfo,
}

impl BinaryArray {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_i32(reader)?;

        let binary_array_type_enumeration = BinaryArrayTypeEnumeration::deserialize(reader)?;

        let rank = read_i32(reader)?;

        let lengths = vec![0i32; rank as usize]
            .into_iter()
            .map(|_| read_i32(reader))
            .collect::<Result<Vec<i32>, NrbfError>>()?;

        let lower_bounds = match binary_array_type_enumeration {
            BinaryArrayTypeEnumeration::SingleOffset
            | BinaryArrayTypeEnumeration::JaggedOffset
            | BinaryArrayTypeEnumeration::RectangularOffset => {
                let lower_bounds = vec![0i32; rank as usize]
                    .into_iter()
                    .map(|_| read_i32(reader))
                    .collect::<Result<Vec<i32>, NrbfError>>()?;
                Some(lower_bounds)
            }
            _ => None,
        };

        let type_enum =
            BinaryTypeEnum::from_repr(read_u8(reader)?).ok_or(NrbfError::InvalidEnum)?;

        let additional_type_info = AdditionalTypeInfo::try_from((reader, &type_enum))?;

        Ok(BinaryArray {
            object_id,
            binary_array_type_enumeration,
            rank,
            lengths,
            lower_bounds,
            type_enum,
            additional_type_info,
        })
    }
}
