use std::io::Read;

use crate::{enums::RecordTypeEnum, errors::NrbfError};

#[derive(Debug)]
pub struct MessageEnd {}

impl MessageEnd {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        if record_type[0] != RecordTypeEnum::MessageEnd as u8 {
            return Err(NrbfError::UnexpectedRecordType);
        }

        Ok(MessageEnd {})
    }
}
