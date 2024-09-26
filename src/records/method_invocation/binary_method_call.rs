use std::io::Read;

use crate::errors::Error;

use super::{ArrayOfValueWithCode, MessageFlags, StringValueWithCode};

pub struct BinaryMethodCall {
    pub message_enum: MessageFlags,
    pub method_name: StringValueWithCode,
    pub type_name: StringValueWithCode,
    pub call_context: StringValueWithCode,
    pub args: ArrayOfValueWithCode,
}

impl BinaryMethodCall {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let message_enum = MessageFlags::deserialize(reader)?;

        let method_name = StringValueWithCode::deserialize(reader)?;

        let type_name = StringValueWithCode::deserialize(reader)?;

        let call_context = StringValueWithCode::deserialize(reader)?;

        let args = ArrayOfValueWithCode::deserialize(reader)?;

        Ok(BinaryMethodCall {
            message_enum,
            method_name,
            type_name,
            call_context,
            args,
        })
    }
}
