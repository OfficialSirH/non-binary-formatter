use std::fmt;

use serde::{de::Visitor, Deserialize};
use strum::FromRepr;

#[derive(Debug, FromRepr, PartialEq)]
#[repr(u8)]
pub enum DateTimeKind {
    Unspecified = 0,
    Utc = 1,
    Local = 2,
}

#[derive(Debug)]
pub struct DateTime {
    /// first 62 bits are the number of 100-nanosecond intervals that have
    /// elapsed since 12:00:00 midnight, January 1, 0001
    pub value: i64,
    /// last 2 bits are the kind of date time
    pub kind: DateTimeKind,
}

// region: DateTime Deserialization
struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct DateTime")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut buffer: [u8; 8] = v.try_into().map_err(|_| E::custom("Invalid Buffer"))?;

        let kind_repr = buffer[7] & 0b0000_0011;
        let kind = DateTimeKind::from_repr(kind_repr).ok_or(E::custom(format!(
            "u8 value doesn't map to any DateTimeKind variant: {}",
            kind_repr
        )))?;

        buffer[7] &= 0b1111_1100;
        let value = i64::from_le_bytes(buffer);

        Ok(DateTime { value, kind })
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(&v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut buffer: [u8; 8] = v
            .to_le_bytes()
            .try_into()
            .map_err(|_| E::custom("Invalid Buffer"))?;

        let kind_repr = buffer[7] & 0b0000_0011;
        let kind = DateTimeKind::from_repr(kind_repr).ok_or(E::custom(format!(
            "u8 value doesn't map to any DateTimeKind variant: {}",
            kind_repr
        )))?;

        buffer[7] &= 0b1111_1100;
        let value = i64::from_le_bytes(buffer);

        Ok(DateTime { value, kind })
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(DateTimeVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        common::data_types::{DateTime, DateTimeKind},
        deserializer::from_reader,
    };

    #[test]
    fn test_date_time() {
        let mut encoded_date_time = 10_i64.to_le_bytes();
        encoded_date_time[7] += 0b0000_0010; // Local

        let mut reader = Cursor::new(&encoded_date_time);
        let date_time: DateTime = from_reader(&mut reader).unwrap();

        assert_eq!(10, date_time.value);
        assert_eq!(DateTimeKind::Local, date_time.kind);
    }
}
