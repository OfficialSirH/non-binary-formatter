use std::fmt;

use serde::{de::Visitor, Deserialize};

use super::LengthPrefixedString;

#[derive(Debug)]
pub struct ClassTypeInfo {
    pub type_name: LengthPrefixedString,
    pub library_id: i32,
}

// region: ClassTypeInfo Deserialization
impl<'de> Deserialize<'de> for ClassTypeInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ClassTypeInfoVisitor;

        impl<'de> Visitor<'de> for ClassTypeInfoVisitor {
            type Value = ClassTypeInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ClassTypeInfo")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let type_name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let library_id = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(ClassTypeInfo {
                    type_name,
                    library_id,
                })
            }
        }

        const FIELDS: &[&str] = &["type_name", "library_id"];
        deserializer.deserialize_struct("ClassTypeInfo", FIELDS, ClassTypeInfoVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{common::data_types::ClassTypeInfo, deserializer::from_reader};

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
        let value: ClassTypeInfo = from_reader(&mut reader).unwrap();

        assert_eq!("Hello World", value.type_name.value);
        assert_eq!(69420, value.library_id);
    }
}
