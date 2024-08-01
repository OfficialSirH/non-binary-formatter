use std::io::Read;

use crate::{errors::NrbfError, readers::read_i32};

use super::{ClassInfo, MemberTypeInfo};

#[derive(Debug)]
pub struct ClassWithMembersAndTypes {
    pub class_info: ClassInfo,
    pub member_type_info: MemberTypeInfo,
    pub library_id: i32,
}

impl ClassWithMembersAndTypes {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let class_info = ClassInfo::deserialize(reader)?;

        let library_id = read_i32(reader)?;

        let member_type_info = MemberTypeInfo::deserialize(reader, class_info.member_names.len())?;

        Ok(ClassWithMembersAndTypes {
            class_info,
            member_type_info,
            library_id,
        })
    }
}
