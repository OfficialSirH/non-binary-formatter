use std::fmt;

use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub struct Single {
    pub value: f32,
}

// region: Single Deserialization
struct SingleVisitor;

impl<'de> Visitor<'de> for SingleVisitor {
    type Value = Single;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Single")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Single { value })
    }
}

impl<'de> Deserialize<'de> for Single {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_f32(SingleVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{common::data_types::Single, deserializer::from_reader};

    #[test]
    fn test_single_deserialize() {
        let encoded_single = [
            0x00, 0x00, 0x20, 0x41, // 10.0
        ];

        let mut reader = Cursor::new(&encoded_single);
        let single: Single = from_reader(&mut reader).unwrap();

        assert_eq!(10.0, single.value);
    }
}
