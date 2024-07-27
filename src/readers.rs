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

pub fn read_string<R: Read>(reader: &mut R) -> Result<String, NrbfError> {
    let length = read_i32(reader)?;
    let mut buffer = vec![0u8; length as usize];
    reader.read_exact(&mut buffer)?;
    String::from_utf8(buffer).map_err(|_| NrbfError::InvalidString)
}
