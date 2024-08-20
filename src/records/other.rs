use std::io::Read;

use crate::{common::data_types::LengthPrefixedString, errors::NrbfError, readers::read_bytes};

#[derive(Debug)]
pub struct MessageEnd {}

#[derive(Debug)]
pub struct SerializationHeaderRecord {
    pub root_id: i32,
    pub header_id: i32,
    pub major_version: i32,
    pub minor_version: i32,
}

impl Default for SerializationHeaderRecord {
    fn default() -> Self {
        SerializationHeaderRecord {
            root_id: 0,
            header_id: 0,
            major_version: 0,
            minor_version: 0,
        }
    }
}

impl SerializationHeaderRecord {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let root_id = read_bytes(reader)?;
        let header_id = read_bytes(reader)?;
        let major_version = read_bytes(reader)?;
        let minor_version = read_bytes(reader)?;

        Ok(SerializationHeaderRecord {
            root_id,
            header_id,
            major_version,
            minor_version,
        })
    }
}

/// The [`BinaryLibrary`] record associates an INT32 ID (as specified in [MS-DTYP] section 2.2.22) with a
/// Library name. This allows other records to reference the Library name by using the ID. This approach
/// reduces the wire size when there are multiple records that reference the same Library name
pub struct BinaryLibrary {
    pub library_id: i32,
    pub library_name: LengthPrefixedString,
}

impl BinaryLibrary {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let library_id = read_bytes(reader)?;

        let library_name = LengthPrefixedString::deserialize(reader)?;

        Ok(BinaryLibrary {
            library_id,
            library_name,
        })
    }
}
