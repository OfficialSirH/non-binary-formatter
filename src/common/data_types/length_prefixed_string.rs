use std::fmt;

use serde::{de::Visitor, Deserialize};

/// The number of bits to shift for each chunk magnitude to calculate the length of the string
pub const LENGTH_CHUNK_BIT_STEP: u32 = 7;

#[derive(Debug)]
pub struct LengthPrefixedString {
    pub value: String,
}

// region: LengthPrefixedString Deserialization
struct LengthPrefixedStringVisitor;

impl<'de> Visitor<'de> for LengthPrefixedStringVisitor {
    type Value = LengthPrefixedString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(LengthPrefixedString {
            value: v.to_owned(),
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(LengthPrefixedString { value: v })
    }
}

impl<'de> Deserialize<'de> for LengthPrefixedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(LengthPrefixedStringVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use crate::{common::data_types::LengthPrefixedString, deserializer::from_reader};

    #[test]
    /// Test the deserialization of an example LengthPrefixedString
    fn hello_world_test() {
        let encoded_string = [
            0b00001011, // Length: 11
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, // "Hello World"
        ];

        let mut reader = std::io::Cursor::new(&encoded_string);
        let result: LengthPrefixedString = from_reader(&mut reader).unwrap();

        assert_eq!("Hello World", result.value);
    }

    #[ignore = "This thing is a memory monster that takes up quite some time to run"]
    #[test]
    /// Test the maximum length of a length prefixed string by generating a string of 2^31 - 1 bytes
    fn max_length_test() {
        let mut encoded_string = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000111, // Length: 2^31 - 1
        ];

        for _ in 1..2_u32.pow(31) {
            encoded_string.push(0x41); // 'A'
        }

        let mut reader = std::io::Cursor::new(&encoded_string);
        let result: LengthPrefixedString = from_reader(&mut reader).unwrap();

        assert_eq!(2_u32.pow(31) - 1, result.value.len() as u32);
    }
}
