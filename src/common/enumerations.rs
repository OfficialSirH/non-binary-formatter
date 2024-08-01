use std::io::Read;

use crate::errors::NrbfError;

#[derive(Debug)]
pub enum BinaryTypeEnum {
    Primitive = 0,
    String = 1,
    Object = 2,
    SystemClass = 3,
    Class = 4,
    ObjectArray = 5,
    StringArray = 6,
    PrimitiveArray = 7,
}

impl BinaryTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        Ok(match value {
            1 => BinaryTypeEnum::String,
            2 => BinaryTypeEnum::Object,
            3 => BinaryTypeEnum::SystemClass,
            4 => BinaryTypeEnum::Class,
            5 => BinaryTypeEnum::ObjectArray,
            6 => BinaryTypeEnum::StringArray,
            7 => BinaryTypeEnum::PrimitiveArray,
            _ => return Err(NrbfError::UnexpectedRecordType),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum RecordTypeEnum {
    SerializedStreamHeader = 0,
    ClassWithId = 1,
    SystemClassWithMembers = 2,
    ClassWithMembers = 3,
    SystemClassWithMembersAndTypes = 4,
    ClassWithMembersAndTypes = 5,
    BinaryObjectString = 6,
    BinaryArray = 7,
    MemberPrimitiveTyped = 8,
    MemberReference = 9,
    ObjectNull = 10,
    MessageEnd = 11,
    BinaryLibrary = 12,
    ObjectNullMultiple256 = 13,
    ObjectNullMultiple = 14,
    ArraySinglePrimitive = 15,
    ArraySingleObject = 16,
    ArraySingleString = 17,
    MethodCall = 21,
    MethodReturn = 22,
}

impl RecordTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        Ok(match value {
            0 => RecordTypeEnum::SerializedStreamHeader,
            1 => RecordTypeEnum::ClassWithId,
            2 => RecordTypeEnum::SystemClassWithMembers,
            3 => RecordTypeEnum::ClassWithMembers,
            4 => RecordTypeEnum::SystemClassWithMembersAndTypes,
            5 => RecordTypeEnum::ClassWithMembersAndTypes,
            6 => RecordTypeEnum::BinaryObjectString,
            11 => RecordTypeEnum::MessageEnd,
            7..=22 => todo!("Do the rest of these (probably just create a derive to do this)"),
            _ => return Err(NrbfError::UnexpectedRecordType),
        })
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        RecordTypeEnum::from_u8(record_type[0])
    }
}

#[derive(Debug)]
pub enum PrimitiveTypeEnum {
    Boolean = 1,
    Byte = 2,
    Char = 3,
    Decimal = 5,
    Double = 6,
    Int16 = 7,
    Int32 = 8,
    Int64 = 9,
    SByte = 10,
    Single = 11,
    TimeSpan = 12,
    DateTime = 13,
    UInt16 = 14,
    UInt32 = 15,
    UInt64 = 16,
    Null = 17,
    String = 18,
}

impl PrimitiveTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        Ok(match value {
            1 => PrimitiveTypeEnum::Boolean,
            2 => PrimitiveTypeEnum::Byte,
            3 => PrimitiveTypeEnum::Char,
            5 => PrimitiveTypeEnum::Decimal,
            6 => PrimitiveTypeEnum::Double,
            7 => PrimitiveTypeEnum::Int16,
            8 => PrimitiveTypeEnum::Int32,
            9 => PrimitiveTypeEnum::Int64,
            10 => PrimitiveTypeEnum::SByte,
            11 => PrimitiveTypeEnum::Single,
            12 => PrimitiveTypeEnum::TimeSpan,
            13 => PrimitiveTypeEnum::DateTime,
            14 => PrimitiveTypeEnum::UInt16,
            15 => PrimitiveTypeEnum::UInt32,
            16 => PrimitiveTypeEnum::UInt64,
            17 => PrimitiveTypeEnum::Null,
            18 => PrimitiveTypeEnum::String,
            _ => return Err(NrbfError::UnexpectedRecordType),
        })
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<PrimitiveTypeEnum, NrbfError> {
        let mut buffer = vec![0u8, 1];
        reader.read_exact(&mut buffer)?;

        PrimitiveTypeEnum::from_u8(buffer[0])
    }
}
