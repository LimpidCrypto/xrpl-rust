//! General XRPL Model Exception.

use alloc::string::String;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplModelException<'a> {
    #[error("`{model_type:?}`: The value of `{field:?}` is too high (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooHigh {
        model_type: &'a str,
        field: &'a str,
        max: u32,
        found: u32,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field:?}` is too low (min {min:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooLow {
        model_type: &'a str,
        field: &'a str,
        min: u32,
        found: u32,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field:?}` is too long (max length {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooLong {
        model_type: &'a str,
        field: &'a str,
        max: u32,
        found: u32,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field:?}` cannot be empty. If the field is optional, define it to be `None`. For more information see: {resource:?}")]
    ValueEmpty { field: &'a str, resource: &'a str },
    #[error("The value of `{field1:?}` must not be identical to `{field2:?}`. For more information see: {resource:?}")]
    ValuesIdentical {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field1:?}` must not be greater than `{field2:?}`. For more information see: {resource:?}")]
    ValueGreaterValue {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field1:?}` must not be smaller than `{field2:?}`. For more information see: {resource:?}")]
    ValueLowerValue {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The value of `{field:?}` has an invalid format (expected {format:?}, found {value:?}). For more information see: {resource:?}")]
    ValueFormatInvalid {
        model_type: &'a str,
        field: &'a str,
        value: &'a str,
        format: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The field `{field1}` requiers field `{field2}` to be defined. For more information see: {resource:?}")]
    FieldRequiresField {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error(
        "`{model_type:?}`: The field `{field1:?}` is not allowed to be defined in combination with `{field2:?}`. For more information see: {resource:?}"
    )]
    IllegalFieldCombo {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: Define at least one field of `{field1:?}` and `{field2:?}`. For more information see: {resource:?}")]
    DefineOneOf {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: Define exactly one field of `{field1:?}` and `{field2:?}`. For more information see: {resource:?}")]
    DefineExactlyOneOf {
        model_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The field `{field:?}` must not be zero. For more information see: {resource:?}")]
    ValueZero {
        model_type: &'a str,
        field: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The optional field `{field:?}` is required for {context:?}. For more informations see: {resource:?}")]
    OptionRequired {
        model_type: &'a str,
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
    #[error("`{model_type:?}`: The optional field `{field:?}` is no allowed for {context:?}. For more informations see: {resource:?}")]
    IllegalOption {
        model_type: &'a str,
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum XRPLRequestException {
    ChannelAuthorizeError(ChannelAuthorizeException),
    SignAndSubmitError(SignAndSubmitException),
    SignForError(SignForException),
    SignError(SignException),
    LedgerEntryError(LedgerEntryException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ChannelAuthorizeException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum LedgerEntryException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignAndSubmitException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignForException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JSONRPCException {
    code: i32,
    message: String,
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLModelException {}
