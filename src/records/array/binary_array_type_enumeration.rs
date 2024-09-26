use std::io::Read;

use strum::FromRepr;

use crate::{errors::Error, readers::read_bytes};

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

impl BinaryArrayTypeEnumeration {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let value = read_bytes(reader)?;

        BinaryArrayTypeEnumeration::from_repr(value).ok_or(Error::InvalidEnum)
    }
}
