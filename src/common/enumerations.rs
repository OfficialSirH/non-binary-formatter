use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use strum::FromRepr;

/// The [`BinaryTypeEnumeration`] identifies the Remoting Type of a Class (2) Member or an Array item.
/// The size of the enumeration is a BYTE.
#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
pub enum BinaryTypeEnumeration {
    /// The Remoting Type is defined in [`PrimitiveTypeEnumeration`] and the Remoting Type is not a
    /// string
    Primitive = 0,
    /// The Remoting Type is a [`LengthPrefixedString`].
    String = 1,
    /// The Remoting Type is System.Object.
    Object = 2,
    /// The Remoting Type is one of the following:
    ///
    /// - A Class (2) in the System Library
    ///
    /// - An Array whose Ultimate Array Item Type is a Class (2) in the System Library
    ///
    /// - An Array whose Ultimate Array Item Type is System.Object, String, or a Primitive Type
    /// but does not meet the definition of ObjectArray, StringArray, or PrimitiveArray.
    SystemClass = 3,
    /// The Remoting Type is a Class (2) or an Array whose Ultimate Array Item Type is a Class (2)
    /// that is not in the System Library.
    Class = 4,
    /// The Remoting Type is a single-dimensional Array of System.Object with a lower bound of 0.
    ObjectArray = 5,
    /// The Remoting Type is a single-dimensional Array of String with a lower bound of 0.
    StringArray = 6,
    /// The Remoting Type is a single-dimensional Array of a Primitive Type with a lower bound of 0.
    PrimitiveArray = 7,
}

//////////////////////////////////////////
// BinaryTypeEnumeration Deserializer ///
////////////////////////////////////////
struct BinaryTypeEnumerationVisitor;

impl<'de> Visitor<'de> for BinaryTypeEnumerationVisitor {
    type Value = BinaryTypeEnumeration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 127")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BinaryTypeEnumeration::from_repr(v).ok_or(E::custom(format!(
            "u8 value doesn't map to any BinaryTypeEnumeration variant: {}",
            v
        )))
    }
}

impl<'de> Deserialize<'de> for BinaryTypeEnumeration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(BinaryTypeEnumerationVisitor)
    }
}
//////////////////////////////////////////
// BinaryTypeEnumeration Deserializer ///
////////////////////////////////////////

#[test]
fn binary_type_enum_test() {
    use crate::common::enumerations::BinaryTypeEnumeration;

    assert_eq!(
        BinaryTypeEnumeration::from_repr(4).unwrap(),
        BinaryTypeEnumeration::Class
    )
}

#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordTypeEnumeration {
    SerializedStreamHeader = 0,
    ClassWithId = 1,
    SystemClassWithMembers = 2,
    ClassWithMembers = 3,
    SystemClassWithMembersAndTypes = 4,
    ClassWithMembersAndTypes = 5,
    BinaryObjectString = 6,
    BinaryArray = 7,
    MemberPrimitiveTyped = 8,
    MemberReference = 9,
    ObjectNull = 10,
    MessageEnd = 11,
    BinaryLibrary = 12,
    ObjectNullMultiple256 = 13,
    ObjectNullMultiple = 14,
    ArraySinglePrimitive = 15,
    ArraySingleObject = 16,
    ArraySingleString = 17,
    MethodCall = 21,
    MethodReturn = 22,
}

//////////////////////////////////////////
// RecordTypeEnumeration Deserializer ///
////////////////////////////////////////
struct RecordTypeEnumerationVisitor;

impl<'de> Visitor<'de> for RecordTypeEnumerationVisitor {
    type Value = RecordTypeEnumeration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 127")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RecordTypeEnumeration::from_repr(v).ok_or(E::custom(format!(
            "u8 value doesn't map to any RecordTypeEnumeration variant: {}",
            v
        )))
    }
}

impl<'de> Deserialize<'de> for RecordTypeEnumeration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(RecordTypeEnumerationVisitor)
    }
}
//////////////////////////////////////////
// RecordTypeEnumeration Deserializer ///
////////////////////////////////////////

// Documentation Types Import
#[allow(unused)]
use crate::common::data_types::{
    Char, DateTime, Decimal, Double, LengthPrefixedString, Single, TimeSpan,
};

/// The [`PrimitiveTypeEnumeration`] identifies a Primitive Type value. The size of the enumeration is a
/// BYTE.
#[derive(Debug, FromRepr, PartialEq, Eq)]
#[repr(u8)]
pub enum PrimitiveTypeEnumeration {
    /// Identifies a BOOLEAN as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.4.
    Boolean = 1,
    /// Identifies a BYTE as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.6.
    Byte = 2,
    /// Identifies a [`Char`] (section 2.1.1.1) type.
    Char = 3,
    /// Identifies a [`Decimal`] (section 2.1.1.7).
    Decimal = 5,
    /// Identifies a [`Double`] (section 2.1.1.2).
    Double = 6,
    /// Identifies an INT16 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.21.
    Int16 = 7,
    /// Identifies an INT32 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.22.
    Int32 = 8,
    /// Identifies an INT64 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.23.
    Int64 = 9,
    /// Identifies an INT8 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.20.
    SByte = 10,
    /// Identifies a [`Single`] (section 2.1.1.3).
    Single = 11,
    /// Identifies a [`TimeSpan`] (section 2.1.1.4).
    TimeSpan = 12,
    /// Identifies a [`DateTime`] (section 2.1.1.5).
    DateTime = 13,
    /// Identifies a UINT16 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.48.
    UInt16 = 14,
    /// Identifies a UINT32 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.49.
    UInt32 = 15,
    /// Identifies a UINT64 as specified in [\[MS-DTYP\]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/cca27429-5689-4a16-b2b4-9325d93e4ba2) section 2.2.50.
    UInt64 = 16,
    /// Identifies a Null Object.
    Null = 17,
    /// Identifies a [`LengthPrefixedString`] (section 2.1.1.6) value.
    String = 18,
}

/////////////////////////////////////////////
// PrimitiveTypeEnumeration Deserializer ///
///////////////////////////////////////////
struct PrimitiveTypeEnumerationVisitor;

impl<'de> Visitor<'de> for PrimitiveTypeEnumerationVisitor {
    type Value = PrimitiveTypeEnumeration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 127")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrimitiveTypeEnumeration::from_repr(v).ok_or(E::custom(format!(
            "u8 value doesn't map to any PrimitiveTypeEnumeration variant: {}",
            v
        )))
    }
}

impl<'de> Deserialize<'de> for PrimitiveTypeEnumeration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(PrimitiveTypeEnumerationVisitor)
    }
}
/////////////////////////////////////////////
// PrimitiveTypeEnumeration Deserializer ///
///////////////////////////////////////////
