use std::io::Read;

use crate::errors::Error;

#[derive(Debug)]
pub struct Double {
    pub value: f64,
}

impl Double {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let value = f64::from_le_bytes(buffer);

        Ok(Double { value })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::Double;

    #[test]
    fn test_double_deserialize() {
        let encoded_double = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x40, // 10.0
        ];

        let mut reader = Cursor::new(&encoded_double);
        let result = Double::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(10.0, value.value);
    }
}
