use std::io::Read;

use crate::errors::NrbfError;

use super::{ClassInfo, MemberTypeInfo};

#[derive(Debug)]
pub struct SystemClassWithMembersAndTypes {
    pub class_info: ClassInfo,
    pub member_type_info: MemberTypeInfo,
}

impl SystemClassWithMembersAndTypes {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let class_info = ClassInfo::deserialize(reader)?;

        let member_type_info = MemberTypeInfo::deserialize(reader, class_info.member_names.len())?;

        Ok(SystemClassWithMembersAndTypes {
            class_info,
            member_type_info,
        })
    }
}
