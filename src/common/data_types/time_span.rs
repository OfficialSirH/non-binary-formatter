use std::fmt;

use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub struct TimeSpan {
    pub value: i64,
}

// region: TimeSpan Deserialization
struct TimeSpanVisitor;

impl<'de> Visitor<'de> for TimeSpanVisitor {
    type Value = TimeSpan;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct TimeSpan")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TimeSpan { value })
    }
}

impl<'de> Deserialize<'de> for TimeSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(TimeSpanVisitor)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{common::data_types::TimeSpan, deserializer::from_reader};

    #[test]
    fn test_time_span_deserialize() {
        let encoded_time_span = 1_i64.to_le_bytes();

        let mut reader = Cursor::new(&encoded_time_span);
        let time_span: TimeSpan = from_reader(&mut reader).unwrap();

        assert_eq!(1, time_span.value);
    }
}
