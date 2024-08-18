use super::{ArrayOfValueWithCode, MessageFlags, StringValueWithCode};

pub struct BinaryMethodCall {
    pub message_enum: MessageFlags,
    pub method_name: StringValueWithCode,
    pub type_name: StringValueWithCode,
    pub call_context: StringValueWithCode,
    pub args: ArrayOfValueWithCode,
}
