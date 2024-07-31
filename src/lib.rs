#![feature(f128)]

pub mod common;
pub mod errors;
pub mod readers;
pub mod records;

use common::enumerations::RecordTypeEnum;
use records::{
    class::ClassWithMembersAndTypes, member_reference::BinaryObjectString,
    other::SerializationHeaderRecord,
};

/// TODO: ***don't keep this, this will be something else entirely later on***
pub struct NrbfDeserialized {
    pub serialized_stream_header: SerializationHeaderRecord,
    pub class_with_members_and_types: Vec<ClassWithMembersAndTypes>,
    pub binary_object_string: Vec<BinaryObjectString>,
}

impl Default for NrbfDeserialized {
    fn default() -> Self {
        NrbfDeserialized {
            serialized_stream_header: SerializationHeaderRecord::default(),
            class_with_members_and_types: Vec::new(),
            binary_object_string: Vec::new(),
        }
    }
}

pub fn deserialize_nrbf<R: std::io::Read>(reader: &mut R) -> Result<(), errors::NrbfError> {
    let mut deserialized_data = NrbfDeserialized::default();

    let record_type = RecordTypeEnum::read_record_type(reader)?;
    // ensure first record type is SerializedStreamHeader
    if record_type != RecordTypeEnum::SerializedStreamHeader {
        return Err(errors::NrbfError::UnexpectedRecordType);
    }
    deserialized_data.serialized_stream_header = SerializationHeaderRecord::deserialize(reader)?;

    let record_type = RecordTypeEnum::read_record_type(reader)?;
    loop {
        match record_type {
            RecordTypeEnum::ClassWithMembersAndTypes => {
                deserialized_data.class_with_members_and_types.push(
                    ClassWithMembersAndTypes::deserialize(
                        reader,
                        &std::collections::HashMap::new(),
                    )?,
                );
            }
            RecordTypeEnum::BinaryObjectString => {
                deserialized_data
                    .binary_object_string
                    .push(BinaryObjectString::deserialize(reader)?);
            }
            RecordTypeEnum::MessageEnd => {
                break;
            }
            _ => return Err(errors::NrbfError::UnexpectedRecordType),
        }
    }

    Ok(())
}
