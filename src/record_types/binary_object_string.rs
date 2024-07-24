use std::io::Read;

use crate::{
    enums::RecordTypeEnum,
    errors::NrbfError,
    readers::{read_i32, read_string},
};

#[derive(Debug)]
pub struct BinaryObjectString {
    pub object_id: i32,
    pub value: String,
}

impl BinaryObjectString {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        if record_type[0] != RecordTypeEnum::BinaryObjectString as u8 {
            // 0x06 is the RecordTypeEnum for BinaryObjectString
            return Err(NrbfError::UnexpectedRecordType);
        }

        let object_id = read_i32(reader)?;
        let value = read_string(reader)?;
        Ok(BinaryObjectString { object_id, value })
    }
}
