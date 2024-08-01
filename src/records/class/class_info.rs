use std::io::Read;

use crate::{common::data_types::LengthPrefixedString, errors::NrbfError, readers::read_i32};

#[derive(Debug)]
pub struct ClassInfo {
    pub object_id: i32,
    pub name: LengthPrefixedString,
    pub member_names: Vec<LengthPrefixedString>,
}

impl ClassInfo {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let object_id = read_i32(reader)?;
        let name = LengthPrefixedString::deserialize(reader)?;

        // Read member names
        let member_count = read_i32(reader)?;
        let mut member_names = Vec::with_capacity(member_count as usize);
        for _ in 0..member_count {
            member_names.push(LengthPrefixedString::deserialize(reader)?);
        }

        Ok(ClassInfo {
            object_id,
            name,
            member_names,
        })
    }
}
