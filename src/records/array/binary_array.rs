use std::io::Read;

use crate::{
    common::enumerations::BinaryTypeEnumeration, errors::NrbfError, readers::read_bytes,
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
    pub type_enum: BinaryTypeEnumeration,
    pub additional_type_info: AdditionalTypeInfo,
}

impl BinaryArray {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_bytes(reader)?;

        let binary_array_type_enumeration = BinaryArrayTypeEnumeration::deserialize(reader)?;

        let rank = read_bytes(reader)?;

        let lengths = vec![0i32; rank as usize]
            .into_iter()
            .map(|_| read_bytes(reader))
            .collect::<Result<Vec<i32>, NrbfError>>()?;

        let lower_bounds = match binary_array_type_enumeration {
            BinaryArrayTypeEnumeration::SingleOffset
            | BinaryArrayTypeEnumeration::JaggedOffset
            | BinaryArrayTypeEnumeration::RectangularOffset => {
                let lower_bounds = vec![0i32; rank as usize]
                    .into_iter()
                    .map(|_| read_bytes(reader))
                    .collect::<Result<Vec<i32>, NrbfError>>()?;
                Some(lower_bounds)
            }
            _ => None,
        };

        let type_enum =
            BinaryTypeEnumeration::from_repr(read_bytes(reader)?).ok_or(NrbfError::InvalidEnum)?;

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
