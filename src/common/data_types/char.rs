use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize,
};

#[derive(Debug)]
pub struct Char {
    pub value: char,
}

// region: Char Deserialization
struct CharVisitor;

impl<'de> Visitor<'de> for CharVisitor {
    type Value = Char;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a char")
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Char { value: v })
    }
}

impl<'de> Deserialize<'de> for Char {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_char(CharVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{common::data_types::Char, deserializer::from_reader};

    #[test]
    fn test_char() {
        let encoded_char = [
            0b11000001, // Length: 2
            0xC3, 0xA1, // UTF-8: 0xC3A1 -> Unicode: 0xE1 -> "á"
        ];

        let mut reader = Cursor::new(&encoded_char);
        let result: Char = from_reader(&mut reader).unwrap();

        assert_eq!('á', result.value);
    }
}
