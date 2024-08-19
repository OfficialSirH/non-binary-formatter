use std::io::Read;

use crate::errors::NrbfError;

use super::ArrayInfo;

/// The [`ArraySingleString`] record contains a single-dimensional Array whose items are String values.
#[derive(Debug)]
pub struct ArraySingleString {
    pub array_info: ArrayInfo,
}

impl ArraySingleString {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let array_info = ArrayInfo::deserialize(reader)?;

        Ok(ArraySingleString { array_info })
    }
}
