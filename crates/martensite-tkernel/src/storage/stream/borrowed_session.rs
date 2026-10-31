//! Sticky faults belong to the whole input borrow, independently of stream flags.
use super::borrowed_failure::StreamRejoinCause;
use std::sync::Mutex;

#[derive(Default)]
pub(super) struct BorrowedSession {
    fault: Mutex<Option<StreamRejoinCause>>,
}
impl BorrowedSession {
    /// Retain the first loss of progress; `session.record(cause)` follows device I/O.
    pub(super) fn record(&self, cause: StreamRejoinCause) {
        let mut fault = self
            .fault
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if fault.is_none() {
            *fault = Some(cause);
        }
    }
    /// Transfer the recorded cause at rejoin; `session.take_fault()` never invokes I/O.
    pub(super) fn take_fault(&self) -> Option<StreamRejoinCause> {
        let mut fault = self
            .fault
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        fault.take()
    }
}
