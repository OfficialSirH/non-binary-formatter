use std::{collections::HashMap, io::Read};

use crate::{
    common::enumerations::BinaryTypeEnum,
    errors::NrbfError,
    readers::{read_i32, read_string},
    records::member_reference::BinaryObjectString,
};

#[derive(Debug)]
pub struct ClassWithMembersAndTypes {
    pub object_id: i32,
    pub name: String,
    pub member_names: Vec<String>,
    pub member_types: Vec<BinaryTypeEnum>,
    pub library_id: i32,
    pub member_values: Vec<MemberValue>,
}

#[derive(Debug)]
pub enum MemberValue {
    String(BinaryObjectString),
    /// The i32 is the object ID of the object being referenced
    Object(i32),
    Boolean(bool),
    Int32(i32),
}

impl ClassWithMembersAndTypes {
    pub fn deserialize<R: Read>(
        reader: &mut R,
        _libraries: &HashMap<i32, String>, // TODO: Implement library deserialization
    ) -> Result<Self, NrbfError> {
        let object_id = read_i32(reader)?;
        let name = read_string(reader)?;

        // Read member names
        let member_count = read_i32(reader)?;
        let mut member_names = Vec::with_capacity(member_count as usize);
        for _ in 0..member_count {
            member_names.push(read_string(reader)?);
        }

        // Read member types
        let mut member_types = Vec::with_capacity(member_count as usize);
        for _ in 0..member_count {
            let mut type_byte = [0u8; 1];
            reader.read_exact(&mut type_byte)?;
            member_types.push(BinaryTypeEnum::from_u8(type_byte[0])?);
        }

        let library_id = read_i32(reader)?;

        // Read member values
        let mut member_values = Vec::with_capacity(member_count as usize);
        for type_enum in &member_types {
            let value = match type_enum {
                BinaryTypeEnum::String => {
                    MemberValue::String(BinaryObjectString::deserialize(reader)?)
                }
                BinaryTypeEnum::Object => MemberValue::Object(read_i32(reader)?),
                BinaryTypeEnum::Boolean => {
                    let mut bool_byte = [0u8; 1];
                    reader.read_exact(&mut bool_byte)?;
                    MemberValue::Boolean(bool_byte[0] != 0)
                }
                BinaryTypeEnum::Int32 => MemberValue::Int32(read_i32(reader)?),
            };
            member_values.push(value);
        }

        Ok(ClassWithMembersAndTypes {
            object_id,
            name,
            member_names,
            member_types,
            library_id,
            member_values,
        })
    }
}
