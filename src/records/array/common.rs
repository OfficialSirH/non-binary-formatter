use std::fmt;

use serde::{de::Visitor, Deserialize};

/// The [`ArrayInfo`] is a common structure that is used by Array records.
#[derive(Debug)]
pub struct ArrayInfo {
    pub object_id: i32,
    pub length: i32,
}

// region: ArrayInfo Deserialization
impl<'de> Deserialize<'de> for ArrayInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArrayInfoVisitor;

        impl<'de> Visitor<'de> for ArrayInfoVisitor {
            type Value = ArrayInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArrayInfo")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let object_id = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let length = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(ArrayInfo { object_id, length })
            }
        }

        const FIELDS: &[&str] = &["object_id", "length"];
        deserializer.deserialize_struct("ArrayInfo", FIELDS, ArrayInfoVisitor)
    }
}
// endregion
