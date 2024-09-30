use std::{fmt, io::Read};

use serde::{de::Visitor, Deserialize};

use crate::{deserializer::from_reader, errors::Error};

use super::ArrayInfo;

/// The [`ArraySingleString`] record contains a single-dimensional Array whose items are String values.
#[derive(Debug)]
pub struct ArraySingleString {
    pub array_info: ArrayInfo,
}

impl ArraySingleString {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let array_info: ArrayInfo = from_reader(reader)?;

        Ok(ArraySingleString { array_info })
    }
}

// region: ArraySingleString Deserialization
impl<'de> Deserialize<'de> for ArraySingleString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArraySingleStringVisitor;

        impl<'de> Visitor<'de> for ArraySingleStringVisitor {
            type Value = ArraySingleString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArraySingleString")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let array_info = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(ArraySingleString { array_info })
            }
        }

        const FIELDS: &[&str] = &["array_info"];
        deserializer.deserialize_struct("ArraySingleString", FIELDS, ArraySingleStringVisitor)
    }
}
// endregion
