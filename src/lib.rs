#![feature(f128)]

pub mod common;
pub mod deserializer;
pub mod errors;
pub mod readers;
pub mod records;

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

// pub fn deserialize_nrbf<R: std::io::Read>(reader: &mut R) -> Result<(), errors::Error> {
//     let mut deserialized_data = NrbfDeserialized::default();

//     let record_type = RecordTypeEnumeration::deserialize(reader)?;
//     // ensure first record type is SerializedStreamHeader
//     if record_type != RecordTypeEnumeration::SerializedStreamHeader {
//         return Err(errors::Error::UnexpectedRecordType);
//     }
//     deserialized_data.serialized_stream_header = SerializationHeaderRecord::deserialize(reader)?;

//     let record_type = RecordTypeEnumeration::deserialize(reader)?;
//     loop {
//         match record_type {
//             RecordTypeEnumeration::ClassWithMembersAndTypes => {
//                 deserialized_data
//                     .class_with_members_and_types
//                     .push(ClassWithMembersAndTypes::deserialize(reader)?);
//             }
//             RecordTypeEnumeration::BinaryObjectString => {
//                 deserialized_data
//                     .binary_object_string
//                     .push(BinaryObjectString::deserialize(reader)?);
//             }
//             RecordTypeEnumeration::MessageEnd => {
//                 break;
//             }
//             _ => return Err(errors::Error::UnexpectedRecordType),
//         }
//     }

//     Ok(())
// }
