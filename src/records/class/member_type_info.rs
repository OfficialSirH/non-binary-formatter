use std::io::Read;

use crate::{
    common::enumerations::BinaryTypeEnumeration, errors::Error, records::AdditionalTypeInfo,
};

#[derive(Debug)]
pub struct MemberTypeInfo {
    pub binary_type_enums: Vec<BinaryTypeEnumeration>,
    pub additional_infos: Vec<AdditionalTypeInfo>,
}

impl MemberTypeInfo {
    pub fn deserialize<R: Read>(reader: &mut R, member_count: usize) -> Result<Self, Error> {
        // Read member types
        let mut binary_type_enums = Vec::with_capacity(member_count);
        for _ in 0..member_count {
            let mut type_byte = [0u8; 1];
            reader.read_exact(&mut type_byte)?;
            binary_type_enums
                .push(BinaryTypeEnumeration::from_repr(type_byte[0]).ok_or(Error::InvalidEnum)?);
        }

        // Read additional info
        let mut additional_infos = Vec::with_capacity(member_count);
        for binary_type_enum in &binary_type_enums {
            additional_infos.push(AdditionalTypeInfo::try_from((
                reader.by_ref(),
                binary_type_enum,
            ))?);
        }

        Ok(MemberTypeInfo {
            binary_type_enums,
            additional_infos,
        })
    }
}
