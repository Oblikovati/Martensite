//! A borrowed job handed to pool threads,
//! and the conversion of a failing job into the `Standard_ProgramError` the launcher raises.
//!
//! Scoped dispatch keeps each borrow alive until all participants finish (#106).
#![forbid(unsafe_code)]
use crate::standard::Failure;
use std::{
    any::Any,
    panic::{AssertUnwindSafe, catch_unwind},
};

/// The job body: called once per participating thread with that thread's index.
pub(super) type JobBody<'a> = dyn Fn(i32) + Sync + 'a;

/// Captures one participant's failure without unwinding through dispatch.
pub(super) fn perform(body: &JobBody<'_>, thread_index: i32) -> Option<Failure> {
    catch_unwind(AssertUnwindSafe(|| body(thread_index)))
        .err()
        .map(|payload| program_error(payload.as_ref()))
}

/// Failure capture: a `Failure` payload keeps "Type: message" and its stack, a
/// text panic reads "panic: <text>" (the `std::exception` clause prints the type name),
/// any other payload is "Error: Unknown exception".
fn program_error(payload: &(dyn Any + Send)) -> Failure {
    if let Some(failure) = payload.downcast_ref::<Failure>() {
        let message: String = format!("{}: {}", failure.exception_type(), failure.message());
        return standard_program_error(&message, failure.stack_string());
    }
    let text: Option<&str> = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
    match text {
        Some(text) => standard_program_error(&format!("panic: {text}"), ""),
        None => standard_program_error("Error: Unknown exception", ""),
    }
}

/// `Standard_ProgramError(message, stack)`; `standard_program_error("bad", "")`.
pub(super) fn standard_program_error(message: &str, stack: &str) -> Failure {
    Failure::with_type("Standard_ProgramError", message, stack)
}
