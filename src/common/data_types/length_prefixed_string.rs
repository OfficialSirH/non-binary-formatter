use std::io::Read;

use crate::{errors::NrbfError, readers::read_u8};

/// The number of bits to shift for each chunk magnitude to calculate the length of the string
pub const LENGTH_CHUNK_BIT_STEP: u32 = 7;

#[derive(Debug)]
pub struct LengthPrefixedString {
    pub value: String,
}

impl LengthPrefixedString {
    /// Deserializes a LengthPrefixedString from the binary data
    /// # Errors
    /// Returns an error if the string is invalid
    /// # Examples
    /// ```
    /// use std::io::Cursor;
    /// use nrbf::common::data_types::LengthPrefixedString;
    ///
    /// let encoded_string = [
    ///    0b00001011, // Length: 11
    ///   0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, // "Hello World"
    /// ];
    /// let mut reader = Cursor::new(&encoded_string);
    /// let result = LengthPrefixedString::deserialize(&mut reader);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!("Hello World", result.unwrap().value);
    /// ```
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut string_length: u32 = 0;
        for i in 0..5 {
            let (continues_chunk, value) = Self::read_chunk(reader)?;
            string_length += match i {
                0 => value,
                1..=4 => {
                    let magnitude = i * LENGTH_CHUNK_BIT_STEP;
                    value * 2_u32.pow(magnitude as u32)
                }
                _ => 0,
            };
            if !continues_chunk {
                break;
            }
        }

        let mut buffer = vec![0u8; string_length as usize];
        reader.read_exact(&mut buffer)?;
        let value = String::from_utf8(buffer).map_err(|_| NrbfError::InvalidString)?;

        Ok(LengthPrefixedString { value })
    }

    /// returns a tuple of `(continues_to_next_chunk, value)`
    ///
    /// `continues_to_next_chunk` is a boolean that indicates if the next byte is part of the length
    ///
    /// `value` is the value of the current chunk
    /// # Errors
    /// Returns an error if it can't read the next byte
    /// # Examples
    /// ```
    /// use std::io::Cursor;
    /// use nrbf::common::data_types::LengthPrefixedString;
    /// use nrbf::readers::read_u8;
    ///
    /// let encoded_string = [
    ///    0b00001011, // Length: 11
    ///  0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, // "Hello World"
    /// ];
    ///
    /// let mut reader = Cursor::new(&encoded_string);
    /// let result = LengthPrefixedString::read_chunk(&mut reader);
    ///
    /// assert!(result.is_ok());
    /// let (continues_chunk, value) = result.unwrap();
    /// assert_eq!(false, continues_chunk);
    /// assert_eq!(11, value);
    /// ```
    pub fn read_chunk<R: Read>(reader: &mut R) -> Result<(bool, u32), NrbfError> {
        let byte = read_u8(reader)?;

        let continues_chunk = (byte >> 7) & 1 == 1;
        let value = byte << 1 >> 1;

        Ok((continues_chunk, value as u32))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::data_types::LengthPrefixedString;

    #[test]
    /// Test the deserialization of an example LengthPrefixedString
    fn hello_world_test() {
        let encoded_string = [
            0b00001011, // Length: 11
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, // "Hello World"
        ];

        let mut reader = std::io::Cursor::new(&encoded_string);
        let result = LengthPrefixedString::deserialize(&mut reader);

        assert!(result.is_ok());
        assert_eq!("Hello World", result.unwrap().value);
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
        let result = LengthPrefixedString::deserialize(&mut reader);

        assert!(result.is_ok());
        assert_eq!(2_u32.pow(31) - 1, result.unwrap().value.len() as u32);
    }
}
