use std::io::Read;

use crate::errors::NrbfError;

#[derive(Debug, PartialEq)]
pub enum DateTimeKind {
    Unspecified,
    Utc,
    Local,
}

impl TryFrom<u8> for DateTimeKind {
    type Error = NrbfError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DateTimeKind::Unspecified),
            1 => Ok(DateTimeKind::Utc),
            2 => Ok(DateTimeKind::Local),
            _ => Err(NrbfError::InvalidDateTimeKind),
        }
    }
}

pub struct DateTime {
    /// first 62 bits are the number of 100-nanosecond intervals that have
    /// elapsed since 12:00:00 midnight, January 1, 0001
    pub value: i64,
    /// last 2 bits are the kind of date time
    pub kind: DateTimeKind,
}

impl DateTime {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;

        let kind = DateTimeKind::try_from(buffer[7] & 0b0000_0011)?;
        buffer[7] &= 0b1111_1100;
        let value = i64::from_le_bytes(buffer);

        Ok(DateTime { value, kind })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::data_types::{DateTime, DateTimeKind};

    #[test]
    fn test_date_time_deserialize() {
        let mut encoded_date_time = 10_i64.to_le_bytes();
        encoded_date_time[7] += 0b0000_0010; // Local

        let mut reader = Cursor::new(&encoded_date_time);
        let result = DateTime::deserialize(&mut reader);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(10, value.value);
        assert_eq!(DateTimeKind::Local, value.kind);
    }
}
