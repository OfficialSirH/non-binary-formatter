use std::io::Read;

use strum::FromRepr;

use crate::{errors::NrbfError, readers::read_u8};

#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
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

#[cfg(test)]
mod binary_type_enum_tests {
    use crate::common::enumerations::BinaryTypeEnum;

    #[test]
    fn from_u8_repr() {
        assert_eq!(BinaryTypeEnum::from_repr(4).unwrap(), BinaryTypeEnum::Class)
    }
}

#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
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
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        RecordTypeEnum::from_repr(record_type[0]).ok_or(NrbfError::InvalidEnum)
    }
}

#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
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
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<PrimitiveTypeEnum, NrbfError> {
        let value = read_u8(reader)?;

        PrimitiveTypeEnum::from_repr(value).ok_or(NrbfError::InvalidEnum)
    }
}
