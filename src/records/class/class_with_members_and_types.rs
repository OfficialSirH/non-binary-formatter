use std::io::Read;

use crate::{errors::Error, readers::read_bytes};

use super::{ClassInfo, MemberTypeInfo};

#[derive(Debug)]
pub struct ClassWithMembersAndTypes {
    pub class_info: ClassInfo,
    pub member_type_info: MemberTypeInfo,
    pub library_id: i32,
}

impl ClassWithMembersAndTypes {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let class_info = ClassInfo::deserialize(reader)?;

        let member_type_info = MemberTypeInfo::deserialize(reader, class_info.member_names.len())?;

        let library_id = read_bytes(reader)?;

        Ok(ClassWithMembersAndTypes {
            class_info,
            member_type_info,
            library_id,
        })
    }
}
