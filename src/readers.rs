use std::io::Read;

use num_traits::FromBytes;

use crate::errors::NrbfError;

/// Will read the amount of bytes necessary for any type that implements [`FromBytes`]
///
/// [`read_bytes`] has an overloaded return type. If you do not specify the return type it may produce a surprising type to satisfy inference.
pub fn read_bytes<const N: usize, T: FromBytes<Bytes = [u8; N]>, R: Read>(
    reader: &mut R,
) -> Result<T, NrbfError> {
    let mut buffer = [0u8; N];
    reader.read_exact(&mut buffer)?;

    Ok(T::from_le_bytes(&buffer))
}
