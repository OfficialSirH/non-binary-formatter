use ::f128::f128;
use std::io::Read;

use crate::errors::NrbfError;

use super::LengthPrefixedString;

pub struct Decimal {
    pub string_value: LengthPrefixedString,
    pub value: f128,
}

impl Decimal {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let string_value = LengthPrefixedString::deserialize(reader)?;
        let value = f128::parse(string_value.value.as_str()).unwrap();

        Ok(Decimal {
            string_value,
            value,
        })
    }
}

// TODO: figure out how to actually test this (I'm using Windows and Windows is too quirky for me to get f128 to work)
#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::Decimal;

    #[test]
    fn test_decimal_deserialize() {
        let mut encoded_decimal = [
            0x0B, // Length: 11
        ]
        .to_vec();
        encoded_decimal.extend_from_slice(b"69420.13377");

        let mut reader = Cursor::new(&encoded_decimal);
        let result = Decimal::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!("69420.13377", value.string_value.value);
        assert_eq!(69420.13377, value.value);
    }
}
