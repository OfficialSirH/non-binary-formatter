use std::io::Read;

use crate::errors::Error;

use super::ClassInfo;

#[derive(Debug)]
pub struct SystemClassWithMembers {
    pub class_info: ClassInfo,
}

impl SystemClassWithMembers {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let class_info = ClassInfo::deserialize(reader)?;

        Ok(SystemClassWithMembers { class_info })
    }
}
