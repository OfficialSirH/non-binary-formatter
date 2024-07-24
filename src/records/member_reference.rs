use std::io::Read;

use crate::{
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
        let object_id = read_i32(reader)?;
        let value = read_string(reader)?;
        Ok(BinaryObjectString { object_id, value })
    }
}
