use super::{ArrayOfValueWithCode, MessageFlags, StringValueWithCode, ValueWithCode};

pub struct BinaryMethodReturn {
    pub message_enum: MessageFlags,
    pub return_value: ValueWithCode,
    pub call_context: StringValueWithCode,
    pub args: ArrayOfValueWithCode,
}
