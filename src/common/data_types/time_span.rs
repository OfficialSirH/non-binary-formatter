use std::io::Read;

use crate::errors::NrbfError;

pub struct TimeSpan {
    pub value: i64,
}

impl TimeSpan {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let value = i64::from_le_bytes(buffer);

        Ok(TimeSpan { value })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::TimeSpan;

    #[test]
    fn test_time_span_deserialize() {
        let encoded_time_span = 1_i64.to_le_bytes();

        let mut reader = Cursor::new(&encoded_time_span);
        let result = TimeSpan::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(1, value.value);
    }
}
