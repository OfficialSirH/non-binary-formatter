use std::io::Read;

use crate::{
    common::{
        data_types::{ClassTypeInfo, LengthPrefixedString},
        enumerations::{BinaryTypeEnum, PrimitiveTypeEnum},
    },
    errors::NrbfError,
};

#[derive(Debug)]
pub enum AdditionalInfo {
    Primitive(PrimitiveTypeEnum),
    SystemClass(LengthPrefixedString),
    Class(ClassTypeInfo),
    PrimitiveArray(PrimitiveTypeEnum),
}

#[derive(Debug)]
pub struct MemberTypeInfo {
    pub binary_type_enums: Vec<BinaryTypeEnum>,
    pub additional_infos: Vec<Option<AdditionalInfo>>,
}

impl MemberTypeInfo {
    pub fn deserialize<R: Read>(reader: &mut R, member_count: usize) -> Result<Self, NrbfError> {
        // Read member types
        let mut binary_type_enums = Vec::with_capacity(member_count);
        for _ in 0..member_count {
            let mut type_byte = [0u8; 1];
            reader.read_exact(&mut type_byte)?;
            binary_type_enums
                .push(BinaryTypeEnum::from_repr(type_byte[0]).ok_or(NrbfError::InvalidEnum)?);
        }

        // Read additional info
        let mut additional_infos = Vec::with_capacity(member_count);
        for binary_type_enum in &binary_type_enums {
            additional_infos.push(match binary_type_enum {
                BinaryTypeEnum::Primitive => Some(AdditionalInfo::Primitive(
                    PrimitiveTypeEnum::deserialize(reader)?,
                )),
                BinaryTypeEnum::SystemClass => Some(AdditionalInfo::SystemClass(
                    LengthPrefixedString::deserialize(reader)?,
                )),
                BinaryTypeEnum::Class => {
                    Some(AdditionalInfo::Class(ClassTypeInfo::deserialize(reader)?))
                }
                BinaryTypeEnum::PrimitiveArray => Some(AdditionalInfo::PrimitiveArray(
                    PrimitiveTypeEnum::deserialize(reader)?,
                )),
                BinaryTypeEnum::String
                | BinaryTypeEnum::Object
                | BinaryTypeEnum::ObjectArray
                | BinaryTypeEnum::StringArray => None,
            });
        }

        Ok(MemberTypeInfo {
            binary_type_enums,
            additional_infos,
        })
    }
}
