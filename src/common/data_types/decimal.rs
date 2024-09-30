use ::f128::f128;
use serde::{de::Visitor, Deserialize};
use std::fmt;

use super::LengthPrefixedString;

#[derive(Debug)]
pub struct Decimal {
    pub string_value: LengthPrefixedString,
    pub value: f128,
}

// region: Decimal Deserialization
struct DecimalVisitor;

impl<'de> Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Decimal")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let string_value = LengthPrefixedString { value };
        let value = f128::parse(string_value.value.as_str()).unwrap();

        Ok(Decimal {
            string_value,
            value,
        })
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DecimalVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use ::f128::f128;

    use crate::{common::data_types::Decimal, deserializer::from_reader};

    #[test]
    fn test_decimal_deserialize() {
        let mut encoded_decimal = [
            0x0B, // Length: 11
        ]
        .to_vec();
        encoded_decimal.extend_from_slice(b"69420.13377");

        let mut reader = Cursor::new(&encoded_decimal);
        let value: Decimal = from_reader(&mut reader).unwrap();

        assert_eq!("69420.13377", value.string_value.value);
        // I would prefer comparing them by their actual values than just a stringified version but it doesn't seem to take too kindly to that
        assert_eq!(f128::from(69420.13377).to_string(), value.value.to_string());
    }
}
