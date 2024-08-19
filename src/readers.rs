use std::io::Read;

use crate::errors::NrbfError;

pub fn read_u8<R: Read>(reader: &mut R) -> Result<u8, NrbfError> {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn read_i32<R: Read>(reader: &mut R) -> Result<i32, NrbfError> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}
