use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use strum::FromRepr;

/// The [`BinaryArrayTypeEnumeration`] is used to denote the type of an Array. The size of the enumeration
/// is 1 byte. It is used by the Array records.
#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
pub enum BinaryArrayTypeEnumeration {
    /// A single-dimensional Array.
    Single = 0,
    /// An Array whose elements are Arrays. The elements of a jagged Array can be of different
    /// dimensions and sizes.
    Jagged = 1,
    /// A multi-dimensional rectangular Array.
    Rectangular = 2,
    /// A single-dimensional offset.
    SingleOffset = 3,
    /// A jagged Array where the lower bound index is greater than 0.
    JaggedOffset = 4,
    /// Multi-dimensional Arrays where the lower bound index of at least one of the dimensions is
    /// greater than 0.
    RectangularOffset = 5,
}

// region: BinaryArrayTypeEnumeration Deserialization
struct BinaryArrayTypeEnumerationVisitor;

impl<'de> Visitor<'de> for BinaryArrayTypeEnumerationVisitor {
    type Value = BinaryArrayTypeEnumeration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 5")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BinaryArrayTypeEnumeration::from_repr(v).ok_or(E::custom(format!(
            "u8 value doesn't map to any BinaryArrayTypeEnumeration variant: {}",
            v
        )))
    }
}

impl<'de> Deserialize<'de> for BinaryArrayTypeEnumeration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(BinaryArrayTypeEnumerationVisitor)
    }
}
// endregion
