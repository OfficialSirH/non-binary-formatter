use std::io::Read;

use crate::errors::NrbfError;

pub struct Char {
    pub value: char,
}

impl Char {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let length_encoded_byte = buffer[0];

        let length = match length_encoded_byte {
            0b00000000..=0b01111111 => 1,
            0b11000000..=0b11011111 => 2,
            0b11100000..=0b11101111 => 3,
            0b11110000..=0b11110111 => 4,
            _ => return Err(NrbfError::InvalidString),
        };

        let mut buffer = vec![0u8; length];
        reader.read_exact(&mut buffer)?;
        let value = String::from_utf8(buffer)
            .map_err(|_| NrbfError::InvalidString)?
            .chars()
            .next()
            .unwrap();

        Ok(Char { value })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::Char;

    #[test]
    fn test_char_deserialize() {
        let encoded_char = [
            0b11000001, // Length: 2
            0xC3, 0xA1, // UTF-8: 0xC3A1 -> Unicode: 0xE1 -> "á"
        ];

        let mut reader = Cursor::new(&encoded_char);
        let result = Char::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!('á', value.value);
    }
}
