use std::io::Read;

use crate::{errors::Error, readers::read_bytes};

use super::ValueWithCode;

/// The [`ArrayOfValueWithCode`] structure contains a list of [`ValueWithCode`] records. The list is prefixed with
/// the length of the Array.
#[derive(Debug)]
pub struct ArrayOfValueWithCode {
    pub list_of_value_with_code: Vec<ValueWithCode>,
}

impl ArrayOfValueWithCode {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length: usize = read_bytes(reader)?;

        let mut list_of_value_with_code: Vec<ValueWithCode> = Vec::with_capacity(length);
        for _ in 0..list_of_value_with_code.capacity() {
            list_of_value_with_code.push(ValueWithCode::deserialize(reader)?);
        }

        Ok(ArrayOfValueWithCode {
            list_of_value_with_code,
        })
    }
}
