use std::io::Read;

use crate::errors::Error;

#[derive(Debug)]
pub struct Single {
    pub value: f32,
}

impl Single {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let value = f32::from_le_bytes(buffer);

        Ok(Single { value })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::Single;

    #[test]
    fn test_single_deserialize() {
        let encoded_single = [
            0x00, 0x00, 0x20, 0x41, // 10.0
        ];

        let mut reader = Cursor::new(&encoded_single);
        let result = Single::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(10.0, value.value);
    }
}
