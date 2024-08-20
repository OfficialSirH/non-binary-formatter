use crate::records::BinaryValue;

use super::ArrayOfValueWithCode;

// TODO: confirm the actual types, spec doesn't make it super obvious
pub struct MethodCallArray {
    pub input_arguments: Option<ArrayOfValueWithCode>,
    pub generic_type_arguments: Option<BinaryValue>,
    pub method_signature: Option<BinaryValue>,
    pub call_context: Option<BinaryValue>,
    pub message_properties: Option<BinaryValue>,
}
