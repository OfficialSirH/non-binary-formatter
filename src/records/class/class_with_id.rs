use std::io::Read;

use crate::{errors::Error, readers::read_bytes};

pub struct ClassWithId {
    pub object_id: i32,
    pub metadata_id: i32,
}

impl ClassWithId {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let object_id = read_bytes(reader)?;

        let metadata_id = read_bytes(reader)?;

        Ok(ClassWithId {
            object_id,
            metadata_id,
        })
    }
}
