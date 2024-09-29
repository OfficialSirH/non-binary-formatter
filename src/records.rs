use std::io::Read;

use member_reference::ObjectNull;

use crate::{
    common::{
        data_types::{
            Char, ClassTypeInfo, DateTime, Decimal, Double, LengthPrefixedString, Single, TimeSpan,
        },
        enumerations::{BinaryTypeEnumeration, PrimitiveTypeEnumeration},
    },
    deserializer::from_reader,
    errors::Error,
    readers::read_bytes,
};

pub mod array;
pub mod class;
pub mod member_reference;
pub mod method_invocation;
pub mod other;

#[derive(Debug)]
pub enum AdditionalTypeInfo {
    Primitive(PrimitiveTypeEnumeration),
    SystemClass(LengthPrefixedString),
    Class(ClassTypeInfo),
    PrimitiveArray(PrimitiveTypeEnumeration),
    None,
}

impl<R: Read> TryFrom<(&mut R, &BinaryTypeEnumeration)> for AdditionalTypeInfo {
    type Error = Error;

    fn try_from(
        (reader, binary_type_enum): (&mut R, &BinaryTypeEnumeration),
    ) -> Result<Self, Self::Error> {
        let res = match binary_type_enum {
            BinaryTypeEnumeration::Primitive => AdditionalTypeInfo::Primitive(from_reader(reader)?),
            BinaryTypeEnumeration::SystemClass => {
                AdditionalTypeInfo::SystemClass(from_reader(reader)?)
            }
            BinaryTypeEnumeration::Class => {
                AdditionalTypeInfo::Class(ClassTypeInfo::deserialize(reader)?)
            }
            BinaryTypeEnumeration::PrimitiveArray => {
                AdditionalTypeInfo::PrimitiveArray(from_reader(reader)?)
            }
            BinaryTypeEnumeration::String
            | BinaryTypeEnumeration::Object
            | BinaryTypeEnumeration::ObjectArray
            | BinaryTypeEnumeration::StringArray => AdditionalTypeInfo::None,
        };

        Ok(res)
    }
}

/// Part of the Remoting Data Model. A [Primitive Value](PrimitiveValue) is an instance of a
/// Primitive Type.
#[derive(Debug)]
pub enum PrimitiveValue {
    Boolean(bool),
    Byte(u8),
    Char(Char),
    Decimal(Decimal),
    Double(Double),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    SByte(i8),
    Single(Single),
    TimeSpan(TimeSpan),
    DateTime(DateTime),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Null(ObjectNull),
    String(LengthPrefixedString),
}

impl<R: Read> TryFrom<(&mut R, &PrimitiveTypeEnumeration)> for PrimitiveValue {
    type Error = Error;

    fn try_from(
        (reader, primitive_type_enum): (&mut R, &PrimitiveTypeEnumeration),
    ) -> Result<Self, Self::Error> {
        let res = match primitive_type_enum {
            PrimitiveTypeEnumeration::Boolean => {
                let boolean: u8 = read_bytes(reader)?;

                PrimitiveValue::Boolean(boolean > 0)
            }
            PrimitiveTypeEnumeration::Byte => PrimitiveValue::Byte(read_bytes(reader)?),
            PrimitiveTypeEnumeration::Char => PrimitiveValue::Char(from_reader(reader)?),
            PrimitiveTypeEnumeration::Decimal => {
                PrimitiveValue::Decimal(Decimal::deserialize(reader)?)
            }
            PrimitiveTypeEnumeration::Double => {
                PrimitiveValue::Double(Double::deserialize(reader)?)
            }
            PrimitiveTypeEnumeration::Int16 => PrimitiveValue::Int16(read_bytes(reader)?),
            PrimitiveTypeEnumeration::Int32 => PrimitiveValue::Int32(read_bytes(reader)?),
            PrimitiveTypeEnumeration::Int64 => PrimitiveValue::Int64(read_bytes(reader)?),
            PrimitiveTypeEnumeration::SByte => PrimitiveValue::SByte(read_bytes(reader)?),
            PrimitiveTypeEnumeration::Single => {
                PrimitiveValue::Single(Single::deserialize(reader)?)
            }
            PrimitiveTypeEnumeration::TimeSpan => {
                PrimitiveValue::TimeSpan(TimeSpan::deserialize(reader)?)
            }
            PrimitiveTypeEnumeration::DateTime => {
                PrimitiveValue::DateTime(DateTime::deserialize(reader)?)
            }
            PrimitiveTypeEnumeration::UInt16 => PrimitiveValue::UInt16(read_bytes(reader)?),
            PrimitiveTypeEnumeration::UInt32 => PrimitiveValue::UInt32(read_bytes(reader)?),
            PrimitiveTypeEnumeration::UInt64 => PrimitiveValue::UInt64(read_bytes(reader)?),
            PrimitiveTypeEnumeration::Null => PrimitiveValue::Null(ObjectNull {}),
            PrimitiveTypeEnumeration::String => PrimitiveValue::String(from_reader(reader)?),
        };

        Ok(res)
    }
}

// TODO: values for the rest of the enums
#[derive(Debug)]
pub enum BinaryValue {
    Primitive(PrimitiveValue),
    String(LengthPrefixedString),
    Object,
    SystemClass,
    Class,
    ObjectArray,
    StringArray,
    PrimitiveArray,
}

impl<R: Read> TryFrom<(&mut R, &BinaryTypeEnumeration)> for BinaryValue {
    type Error = Error;

    fn try_from(
        (reader, binary_type_enum): (&mut R, &BinaryTypeEnumeration),
    ) -> Result<Self, Self::Error> {
        let res = match binary_type_enum {
            BinaryTypeEnumeration::Primitive => {
                let primitive_type_enum: PrimitiveTypeEnumeration = from_reader(reader)?;

                BinaryValue::Primitive(PrimitiveValue::try_from((reader, &primitive_type_enum))?)
            }
            BinaryTypeEnumeration::String => BinaryValue::String(from_reader(reader)?),
            BinaryTypeEnumeration::Object => BinaryValue::Object,
            BinaryTypeEnumeration::SystemClass => BinaryValue::SystemClass,
            BinaryTypeEnumeration::Class => BinaryValue::Class,
            BinaryTypeEnumeration::ObjectArray => BinaryValue::ObjectArray,
            BinaryTypeEnumeration::StringArray => BinaryValue::StringArray,
            BinaryTypeEnumeration::PrimitiveArray => BinaryValue::PrimitiveArray,
        };

        Ok(res)
    }
}
