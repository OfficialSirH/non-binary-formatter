use std::io::Read;

use crate::errors::Error;

use super::{ArrayOfValueWithCode, MessageFlags, StringValueWithCode, ValueWithCode};

pub struct BinaryMethodReturn {
    pub message_enum: MessageFlags,
    pub return_value: ValueWithCode,
    pub call_context: StringValueWithCode,
    pub args: ArrayOfValueWithCode,
}

impl BinaryMethodReturn {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let message_enum = MessageFlags::deserialize(reader)?;

        let return_value = ValueWithCode::deserialize(reader)?;

        let call_context = StringValueWithCode::deserialize(reader)?;

        let args = ArrayOfValueWithCode::deserialize(reader)?;

        Ok(BinaryMethodReturn {
            message_enum,
            return_value,
            call_context,
            args,
        })
    }
}
