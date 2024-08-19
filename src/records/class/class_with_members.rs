use std::io::Read;

use crate::{errors::NrbfError, readers::read_i32};

use super::ClassInfo;

#[derive(Debug)]
pub struct ClassWithMembers {
    pub class_info: ClassInfo,
    pub library_id: i32,
}

impl ClassWithMembers {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let class_info = ClassInfo::deserialize(reader)?;

        let library_id = read_i32(reader)?;

        Ok(ClassWithMembers {
            class_info,
            library_id,
        })
    }
}
