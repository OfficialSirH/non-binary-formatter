use std::io::Read;

use crate::{enums::RecordTypeEnum, errors::NrbfError, readers::read_i32};

#[derive(Debug)]
pub struct SerializationHeaderRecord {
    pub root_id: i32,
    pub header_id: i32,
    pub major_version: i32,
    pub minor_version: i32,
}

impl SerializationHeaderRecord {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut record_type = [0u8; 1];
        reader.read_exact(&mut record_type)?;

        if record_type[0] != RecordTypeEnum::SerializedStreamHeader as u8 {
            return Err(NrbfError::UnexpectedRecordType);
        }

        let root_id = read_i32(reader)?;
        let header_id = read_i32(reader)?;
        let major_version = read_i32(reader)?;
        let minor_version = read_i32(reader)?;

        Ok(SerializationHeaderRecord {
            root_id,
            header_id,
            major_version,
            minor_version,
        })
    }
}
