use std::fmt;

use serde::{de::Visitor, Deserialize};

use super::ArrayInfo;

/// The [`ArraySingleObject`] record contains a single-dimensional Array in which each Member record MAY
/// contain any Data Value.
#[derive(Debug)]
pub struct ArraySingleObject {
    pub array_info: ArrayInfo,
}

// region: ArraySingleObject Deserialization
impl<'de> Deserialize<'de> for ArraySingleObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArraySingleObjectVisitor;

        impl<'de> Visitor<'de> for ArraySingleObjectVisitor {
            type Value = ArraySingleObject;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArraySingleObject")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let array_info = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(ArraySingleObject { array_info })
            }
        }

        const FIELDS: &[&str] = &["array_info"];
        deserializer.deserialize_struct("ArraySingleObject", FIELDS, ArraySingleObjectVisitor)
    }
}
// endregion
