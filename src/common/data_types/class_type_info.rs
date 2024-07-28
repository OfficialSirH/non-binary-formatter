use std::io::Read;

use crate::{errors::NrbfError, readers::read_i32};

use super::LengthPrefixedString;

pub struct ClassTypeInfo {
    pub type_name: LengthPrefixedString,
    pub library_id: i32,
}

impl ClassTypeInfo {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let type_name = LengthPrefixedString::deserialize(reader)?;
        let library_id = read_i32(reader)?;

        Ok(ClassTypeInfo {
            type_name,
            library_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::ClassTypeInfo;

    #[test]
    fn test_class_type_info_deserialize() {
        let mut encoded_class_type_info = [
            0b00001011, // Length: 11
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, // "Hello World"
        ]
        .to_vec();
        // library ID
        encoded_class_type_info.extend_from_slice(&69420_i32.to_le_bytes());

        let mut reader = Cursor::new(&encoded_class_type_info);
        let result = ClassTypeInfo::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!("Hello World", value.type_name.value);
        assert_eq!(69420, value.library_id);
    }
}
