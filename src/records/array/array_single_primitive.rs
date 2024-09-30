use std::fmt;

use serde::{de::Visitor, Deserialize};

use crate::common::enumerations::PrimitiveTypeEnumeration;

use super::ArrayInfo;

/// The [`ArraySinglePrimitive`] record contains a single-dimensional Array in which all Members are
/// Primitive Value.
#[derive(Debug)]
pub struct ArraySinglePrimitive {
    pub array_info: ArrayInfo,
    pub primitive_type_enum: PrimitiveTypeEnumeration,
}

// region: ArraySinglePrimitive Deserialization
impl<'de> Deserialize<'de> for ArraySinglePrimitive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArraySinglePrimitiveVisitor;

        impl<'de> Visitor<'de> for ArraySinglePrimitiveVisitor {
            type Value = ArraySinglePrimitive;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArraySinglePrimitive")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let array_info = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let primitive_type_enum = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(ArraySinglePrimitive {
                    array_info,
                    primitive_type_enum,
                })
            }
        }

        const FIELDS: &[&str] = &["array_info", "primitive_type_enum"];
        deserializer.deserialize_struct("ArraySinglePrimitive", FIELDS, ArraySinglePrimitiveVisitor)
    }
}
// endregion
