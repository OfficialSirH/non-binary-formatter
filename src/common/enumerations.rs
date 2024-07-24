use std::io::Read;

use crate::errors::NrbfError;

#[derive(Debug)]
pub enum BinaryTypeEnum {
    String = 0x01,
    Object = 0x02,
    Boolean = 0x03,
    Int32 = 0x04,
}

impl BinaryTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        match value {
            0x01 => Ok(BinaryTypeEnum::String),
            0x02 => Ok(BinaryTypeEnum::Object),
            0x03 => Ok(BinaryTypeEnum::Boolean),
            0x04 => Ok(BinaryTypeEnum::Int32),
            _ => Err(NrbfError::UnexpectedRecordType),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RecordTypeEnum {
    SerializedStreamHeader = 0x00,
    ClassWithMembers = 0x01,
    ClassWithMembersAndTypes = 0x05,
    BinaryObjectString = 0x06,
    MessageEnd = 0x0B,
}

impl RecordTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        match value {
            0x00 => Ok(RecordTypeEnum::SerializedStreamHeader),
            0x01 => Ok(RecordTypeEnum::ClassWithMembers),
            0x05 => Ok(RecordTypeEnum::ClassWithMembersAndTypes),
            0x06 => Ok(RecordTypeEnum::BinaryObjectString),
            0x0B => Ok(RecordTypeEnum::MessageEnd),
            _ => Err(NrbfError::UnexpectedRecordType),
        }
    }

    pub fn read_record_type<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        RecordTypeEnum::from_u8(record_type[0])
    }
}
