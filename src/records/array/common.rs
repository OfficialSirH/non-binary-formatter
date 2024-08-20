use std::io::Read;

use crate::{errors::NrbfError, readers::read_bytes};

/// The [`ArrayInfo`] is a common structure that is used by Array records.
#[derive(Debug)]
pub struct ArrayInfo {
    pub object_id: i32,
    pub length: i32,
}

impl ArrayInfo {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_bytes(reader)?;

        let length = read_bytes(reader)?;

        Ok(ArrayInfo { object_id, length })
    }
}
