use crate::records::BinaryValue;

use super::{ArrayOfValueWithCode, StringValueWithCode, ValueWithCode};

// TODO: confirm the actual types, spec doesn't make it super obvious
pub struct MethodReturnCallArray {
    pub return_value: Option<ValueWithCode>,
    pub output_arguments: Option<ArrayOfValueWithCode>,
    pub exception: Option<BinaryValue>,
    pub call_context: Option<StringValueWithCode>,
    pub message_properties: Option<BinaryValue>,
}
