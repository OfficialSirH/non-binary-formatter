use std::io::Read;

use crate::{
    common::data_types::LengthPrefixedString, deserializer::from_reader, errors::Error,
    readers::read_bytes,
};

#[derive(Debug)]
pub struct ClassInfo {
    pub object_id: i32,
    pub name: LengthPrefixedString,
    pub member_names: Vec<LengthPrefixedString>,
}

impl ClassInfo {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let object_id = read_bytes(reader)?;
        let name: LengthPrefixedString = from_reader(reader)?;

        // Read member names
        let member_count = read_bytes(reader)?;
        let mut member_names = Vec::with_capacity(member_count as usize);
        for _ in 0..member_count {
            member_names.push(from_reader(reader)?);
        }

        Ok(ClassInfo {
            object_id,
            name,
            member_names,
        })
    }
}
