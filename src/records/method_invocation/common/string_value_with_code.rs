use crate::common::{data_types::LengthPrefixedString, enumerations::PrimitiveTypeEnum};

pub struct StringValueWithCode {
    pub primitive_type_enum: PrimitiveTypeEnum,
    pub value: LengthPrefixedString,
}
