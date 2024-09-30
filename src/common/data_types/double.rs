use std::fmt;

use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub struct Double {
    pub value: f64,
}

// region: Double Deserialization
struct DoubleVisitor;

impl<'de> Visitor<'de> for DoubleVisitor {
    type Value = Double;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Double")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Double { value })
    }
}

impl<'de> Deserialize<'de> for Double {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_f64(DoubleVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{common::data_types::Double, deserializer::from_reader};

    #[test]
    fn test_double_deserialize() {
        let encoded_double = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x40, // 10.0
        ];

        let mut reader = Cursor::new(&encoded_double);
        let double: Double = from_reader(&mut reader).unwrap();

        assert_eq!(10.0, double.value);
    }
}
