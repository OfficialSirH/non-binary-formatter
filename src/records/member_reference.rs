use std::io::Read;

use crate::{
    common::{data_types::LengthPrefixedString, enumerations::PrimitiveTypeEnumeration},
    errors::NrbfError,
    readers::read_bytes,
};

use super::PrimitiveValue;

/// The [`MemberPrimitiveTyped`] record contains a Primitive Type value other than String. The mechanism
/// to serialize a Primitive Value is described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.8.
#[derive(Debug)]
pub struct MemberPrimitiveTyped {
    pub primitive_type_enum: PrimitiveTypeEnumeration,
    pub value: PrimitiveValue,
}

impl MemberPrimitiveTyped {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let primitive_type_enum = PrimitiveTypeEnumeration::deserialize(reader)?;

        let value = PrimitiveValue::try_from((reader, &primitive_type_enum))?;

        Ok(MemberPrimitiveTyped {
            primitive_type_enum,
            value,
        })
    }
}

/// The [`MemberPrimitiveUnTyped`] record is the most compact record to represent a Primitive Type
/// value. This type of record does not have a RecordTypeEnum to indicate the record type. The record
/// MUST be used when a Class Member or Array item is a Primitive Type. Because the containing Class
/// or Array record specifies the Primitive Type of each Member, the Primitive Type is not respecified
/// along with the value. Also, the Primitive Values cannot be referenced by any other record; therefore
/// it does not require an ObjectId. This record has no field besides the value. The mechanism to
/// serialize a Primitive Value is described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.8.
#[derive(Debug)]
pub struct MemberPrimitiveUnTyped {
    pub value: PrimitiveValue,
}

impl<R: Read> TryFrom<(&mut R, &PrimitiveTypeEnumeration)> for MemberPrimitiveUnTyped {
    type Error = NrbfError;

    fn try_from(
        (reader, primitive_type_enum): (&mut R, &PrimitiveTypeEnumeration),
    ) -> Result<Self, Self::Error> {
        let value = PrimitiveValue::try_from((reader, primitive_type_enum))?;

        Ok(MemberPrimitiveUnTyped { value })
    }
}

/// The [`MemberReference`] record contains a reference to another record that contains the actual value.
/// The record is used to serialize values of a Class Member and Array items. The mechanism to
/// serialize a Class instance is described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.6. The mechanism to serialize an
/// Array instance is described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.7.
#[derive(Debug)]
pub struct MemberReference {
    pub id_ref: i32,
}

impl MemberReference {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let id_ref = read_bytes(reader)?;

        Ok(MemberReference { id_ref })
    }
}

/// The [`ObjectNull`] record contains a Null Object. The mechanism to serialize a Null Object is described
/// in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.12.
#[derive(Debug)]
pub struct ObjectNull {}

/// The [`ObjectNullMultiple`] record provides a more compact form for multiple consecutive Null records
/// than using individual [`ObjectNull`] records. The mechanism to serialize a Null Object is described in
/// [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.12.
#[derive(Debug)]
pub struct ObjectNullMultiple {
    pub null_count: i32,
}

impl ObjectNullMultiple {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let null_count = read_bytes(reader)?;

        Ok(ObjectNullMultiple { null_count })
    }
}

/// The [`ObjectNullMultiple256`] record provides the most compact form for multiple, consecutive Null
/// records when the count of Null records is less than 256. The mechanism to serialize a Null Object is
/// described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.12.
#[derive(Debug)]
pub struct ObjectNullMultiple256 {
    pub null_count: u8,
}

impl ObjectNullMultiple256 {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let null_count = read_bytes(reader)?;

        Ok(ObjectNullMultiple256 { null_count })
    }
}

/// The [`BinaryObjectString`] record identifies an object as a String object, and contains information about
/// it. The mechanism to serialize a string is described in [\[MS-NRTP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp/3acb31b0-b873-4aaf-8503-9727ec40fbec) section 3.1.5.1.11.
#[derive(Debug)]
pub struct BinaryObjectString {
    pub object_id: i32,
    pub value: LengthPrefixedString,
}

impl BinaryObjectString {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_bytes(reader)?;
        let value = LengthPrefixedString::deserialize(reader)?;
        Ok(BinaryObjectString { object_id, value })
    }
}
