use std::io::Error as IoError;

#[derive(Debug)]
pub enum NrbfError {
    IoError(IoError),
    UnexpectedRecordType,
    InvalidString,
    // Add other error types as needed
}

impl From<IoError> for NrbfError {
    fn from(error: IoError) -> Self {
        NrbfError::IoError(error)
    }
}
