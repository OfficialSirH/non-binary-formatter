use std::io::Read;

use strum::FromRepr;

use crate::{errors::NrbfError, readers::read_i32};

// this is for the docs
#[allow(unused)]
use crate::records::method_invocation::{
    BinaryMethodCall, BinaryMethodReturn, MethodReturnCallArray,
};

#[derive(Debug, FromRepr, PartialEq)]
#[repr(i32)]
pub enum MessageFlags {
    /// The record contains no arguments. It is in the Arg category.
    NoArgs = 0x00000001,
    /// The Arguments Array is in the `Args`([`BinaryMethodCall`](BinaryMethodCall::args) / [`BinaryMethodReturn`](BinaryMethodReturn::args)) field of the Method record. It is in the Arg category.
    ArgsInline = 0x00000002,
    /// Each argument is an item in a separate Call Array record. It is in the Arg category.
    ArgsIsArray = 0x00000004,
    /// The Arguments Array is an item in a separate Call Array record. It is in the Arg category.
    ArgsInArray = 0x00000008,
    /// The record does not contain a Call Context value. It is in the Context category.
    NoContext = 0x00000010,
    /// Call Context contains only a Logical Call ID value and is in the `CallContext`([`BinaryMethodCall`](BinaryMethodCall::call_context) / [`BinaryMethodReturn`](BinaryMethodReturn::call_context)) field of the Method record. It is in the Context category.
    ContextInline = 0x00000020,
    /// CallContext values are contained in an array that is contained in the Call Array record. It is in the Context category.
    ContextInArray = 0x00000040,
    /// The Method Signature is contained in the Call Array record. It is in the Signature category.
    MethodSignatureInArray = 0x00000080,
    /// Message Properties is contained in the Call Array record. It is in the Property category.
    PropertiesInArray = 0x00000100,
    /// The Return Value is a Null object. It is in the Return category.
    NoReturnValue = 0x00000200,
    /// The method has no Return Value. It is in the Return category.
    ReturnValueVoid = 0x00000400,
    /// The Return Value is in the [`ReturnValue`](MethodReturnCallArray::return_value) field of the [`MethodReturnCallArray`] record. It is in the Return category.
    ReturnValueInline = 0x00000800,
    /// The Return Value is contained in the [`MethodReturnCallArray`] record. It is in the Return category.
    ReturnValueInArray = 0x00001000,
    /// An Exception is contained in the [`MethodReturnCallArray`] record. It is in the Exception category.
    ExceptionInArray = 0x00002000,
    /// The Remote Method is generic and the actual Remoting Types for the Generic Arguments are contained in the Call Array. It is in the Generic category.
    GenericMethod = 0x00008000,
}

// CURRENT TODO: implement the valid combinations
impl MessageFlags {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<MessageFlags, NrbfError> {
        let value = read_i32(reader)?;

        MessageFlags::from_repr(value).ok_or(NrbfError::InvalidEnum)
    }
}
