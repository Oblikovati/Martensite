//! Behavior of the complete ASCII string, shared ASCII string and
//! string helper API, including the deliberate quirks.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{
    AsciiString, ExtendedString, HAsciiString, HExtendedString, next_prime_for_map,
};

fn bytes(text: &AsciiString) -> &[u8] {
    text.as_bytes()
}

fn class(failure: Failure) -> &'static str {
    failure.exception_type()
}

include!("ascii_string_api_behavior/construction.rs");
include!("ascii_string_api_behavior/editing.rs");
include!("ascii_string_api_behavior/queries.rs");
include!("ascii_string_api_behavior/c_string_search.rs");
include!("ascii_string_api_behavior/c_string_editing.rs");
include!("ascii_string_api_behavior/handles.rs");
include!("ascii_string_api_behavior/concatenation_arrays.rs");
