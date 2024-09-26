use std::io::Read;

use crate::{errors::Error, readers::read_bytes};

use super::ClassInfo;

#[derive(Debug)]
pub struct ClassWithMembers {
    pub class_info: ClassInfo,
    pub library_id: i32,
}

impl ClassWithMembers {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let class_info = ClassInfo::deserialize(reader)?;

        let library_id = read_bytes(reader)?;

        Ok(ClassWithMembers {
            class_info,
            library_id,
        })
    }
}
