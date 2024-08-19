use std::io::Read;

use crate::errors::NrbfError;

use super::ArrayInfo;

/// The [`ArraySingleObject`] record contains a single-dimensional Array in which each Member record MAY
/// contain any Data Value.
#[derive(Debug)]
pub struct ArraySingleObject {
    pub array_info: ArrayInfo,
}

impl ArraySingleObject {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, NrbfError> {
        let array_info = ArrayInfo::deserialize(reader)?;

        Ok(ArraySingleObject { array_info })
    }
}
