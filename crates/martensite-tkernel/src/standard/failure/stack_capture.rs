//! `Standard_Failure::captureStackTrace` and its process-wide default length.
use super::{c_string, text::FailureText};
use crate::standard::stack_trace;
use std::sync::atomic::{AtomicI32, Ordering};

/// `Standard_Failure_DefaultStackTraceLength`: 0 means no stack trace.
static DEFAULT_STACK_TRACE_LENGTH: AtomicI32 = AtomicI32::new(0);
/// Smallest and largest trace buffer, in bytes.
const TRACE_BUFFER_BOUNDS: (i32, i32) = (2048, 65536);
/// Bytes reserved per requested frame.
const TRACE_BYTES_PER_FRAME: i32 = 200;

pub(super) fn default_length() -> i32 {
    DEFAULT_STACK_TRACE_LENGTH.load(Ordering::SeqCst)
}
pub(super) fn set_default_length(length: i32) {
    DEFAULT_STACK_TRACE_LENGTH.store(length, Ordering::SeqCst);
}
/// The trace a message constructor stores, empty when capture is off or fails.
pub(super) fn capture_stack_trace() -> FailureText {
    let length: i32 = default_length();
    if length <= 0 {
        return c_string("");
    }
    // The length is multiplied as `i32`; saturation replaces overflow for huge lengths.
    let buffer_size: i32 = length
        .saturating_mul(TRACE_BYTES_PER_FRAME)
        .clamp(TRACE_BUFFER_BOUNDS.0, TRACE_BUFFER_BOUNDS.1);
    let mut trace: String = String::new();
    // Skip one more frame, the constructor's.
    if !stack_trace(&mut trace, buffer_size, length, 1) {
        return c_string("");
    }
    c_string(&trace)
}
