use std::io::Read;

use crate::{
    common::{
        data_types::{ClassTypeInfo, LengthPrefixedString},
        enumerations::{BinaryTypeEnum, PrimitiveTypeEnum},
    },
    errors::NrbfError,
};

pub mod array;
pub mod class;
pub mod member_reference;
pub mod method_invocation;
pub mod other;

#[derive(Debug)]
pub enum AdditionalTypeInfo {
    Primitive(PrimitiveTypeEnum),
    SystemClass(LengthPrefixedString),
    Class(ClassTypeInfo),
    PrimitiveArray(PrimitiveTypeEnum),
    None,
}

impl<R: Read> TryFrom<(&mut R, &BinaryTypeEnum)> for AdditionalTypeInfo {
    type Error = NrbfError;

    fn try_from(
        (reader, binary_type_enum): (&mut R, &BinaryTypeEnum),
    ) -> Result<Self, Self::Error> {
        match binary_type_enum {
            BinaryTypeEnum::Primitive => Ok(AdditionalTypeInfo::Primitive(
                PrimitiveTypeEnum::deserialize(reader)?,
            )),
            BinaryTypeEnum::SystemClass => Ok(AdditionalTypeInfo::SystemClass(
                LengthPrefixedString::deserialize(reader)?,
            )),
            BinaryTypeEnum::Class => Ok(AdditionalTypeInfo::Class(ClassTypeInfo::deserialize(
                reader,
            )?)),
            BinaryTypeEnum::PrimitiveArray => Ok(AdditionalTypeInfo::PrimitiveArray(
                PrimitiveTypeEnum::deserialize(reader)?,
            )),
            BinaryTypeEnum::String
            | BinaryTypeEnum::Object
            | BinaryTypeEnum::ObjectArray
            | BinaryTypeEnum::StringArray => Ok(AdditionalTypeInfo::None),
        }
    }
}
