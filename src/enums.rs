use crate::errors::NrbfError;

#[derive(Debug)]
pub enum BinaryTypeEnum {
    String = 0x01,
    Object = 0x02,
    Boolean = 0x03,
    Int32 = 0x04,
    // Add other types as needed
}

impl BinaryTypeEnum {
    pub fn from_u8(value: u8) -> Result<Self, NrbfError> {
        match value {
            0x01 => Ok(BinaryTypeEnum::String),
            0x02 => Ok(BinaryTypeEnum::Object),
            0x03 => Ok(BinaryTypeEnum::Boolean),
            0x04 => Ok(BinaryTypeEnum::Int32),
            // Add other cases
            _ => Err(NrbfError::UnexpectedRecordType),
        }
    }
}

#[derive(Debug)]
pub enum RecordTypeEnum {
    SerializedStreamHeader = 0x00,
    ClassWithMembers = 0x01,
    ClassWithMembersAndTypes = 0x05,
    BinaryObjectString = 0x06,
    MessageEnd = 0x0B,
    // Add other types as needed
}
