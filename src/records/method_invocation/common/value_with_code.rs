use crate::common::enumerations::PrimitiveTypeEnum;

pub struct ValueWithCode {
    pub primitive_type_enum: PrimitiveTypeEnum,
    // TODO: change `String` to an enum type that encapsulates all primitive types
    pub value: Option<String>,
}
