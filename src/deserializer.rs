use std::io::Read;

use num_traits::FromBytes;
use serde::{
    de::{self, Visitor},
    Deserialize,
};

use crate::errors::{Error, Result};

pub struct Deserializer<'de, R: Read> {
    reader: &'de mut R,
}

impl<'de, R: Read> Deserializer<'de, R> {
    pub fn from_reader(reader: &'de mut R) -> Self {
        Deserializer { reader }
    }
}

pub fn from_reader<'a, T>(r: &'a mut impl Read) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_reader(r);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.reader.bytes().count() == 0 {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

impl<'de, R: Read> Deserializer<'de, R> {
    /// Will read the amount of bytes necessary for any type that implements [`FromBytes`]
    ///
    /// [`read_bytes`] has an overloaded return type. If you do not specify the return type it may produce a surprising type to satisfy inference.
    pub fn read_bytes<const N: usize, T: FromBytes<Bytes = [u8; N]>>(&mut self) -> Result<T> {
        let mut buffer = [0u8; N];
        self.reader.read_exact(&mut buffer)?;

        Ok(T::from_le_bytes(&buffer))
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    /// # UNSUPPORTED OPERATION
    /// this is a non-self describing format so this can't be used
    fn deserialize_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: This is a non-self describing format")
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let boolean: u8 = self.read_bytes()?;
        visitor.visit_bool(boolean > 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.read_bytes()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.read_bytes()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.read_bytes()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.read_bytes()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.read_bytes()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.read_bytes()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_bytes()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.read_bytes()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.read_bytes()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.read_bytes()?)
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_char<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No char deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_str<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No str deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_string<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No string deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_bytes<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No bytes deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No byte buf deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_option<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No option deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_unit<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No unit deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No unit struct deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_seq<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No seq deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_map<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No map deserialization")
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No enum deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_identifier<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No identifier deserialization")
    }

    /// # UNSUPPORTED OPERATION
    fn deserialize_ignored_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Unsupported Method: No ignored any deserialization")
    }
}

#[test]
fn test_record_type() {
    let data = [
        0x05, // RecordTypeEnum: ClassWithMembersAndTypes
    ];

    use crate::common::enumerations::RecordTypeEnumeration;

    let mut cursor = std::io::Cursor::new(data);
    assert_eq!(
        RecordTypeEnumeration::ClassWithMembersAndTypes,
        from_reader(&mut cursor).unwrap()
    );
}
