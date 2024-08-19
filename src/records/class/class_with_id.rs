use std::io::Read;

use crate::{errors::NrbfError, readers::read_i32};

pub struct ClassWithId {
    pub object_id: i32,
    pub metadata_id: i32,
}

impl ClassWithId {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_i32(reader)?;

        let metadata_id = read_i32(reader)?;

        Ok(ClassWithId {
            object_id,
            metadata_id,
        })
    }
}