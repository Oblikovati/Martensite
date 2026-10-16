//! Behavior of the extended string and shared extended string API.
//!
//! Expected values are the reviewed reference values;
//! each module covers one group.
use martensite_tkernel::{standard::Failure, tcollection::ExtendedString};
#[path = "extended_string_api_behavior/construction.rs"]
mod construction;
#[path = "extended_string_api_behavior/editing.rs"]
mod editing;
#[path = "extended_string_api_behavior/handle.rs"]
mod handle;
#[path = "extended_string_api_behavior/queries.rs"]
mod queries;

/// UTF-16 units of a Rust string; `units("ab") == [97, 98]`.
fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// An explicit-length string; `string("ab")`.
fn string(text: &str) -> ExtendedString {
    ExtendedString::from_raw(&units(text))
}

/// The exception class of a failed call; `class(result) == "Standard_OutOfRange"`.
fn class<T: std::fmt::Debug>(result: Result<T, Failure>) -> &'static str {
    result.unwrap_err().exception_type()
}
