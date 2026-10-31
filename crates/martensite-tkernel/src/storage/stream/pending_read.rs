//! A failed read cannot publish its resized scratch space as received input.
use super::{super::state, borrowed_failure::StreamRejoinCause, borrowed_session::BorrowedSession};

pub(super) struct PendingRead<'a> {
    bytes: &'a mut Vec<u8>,
    cursor: &'a mut usize,
    state: &'a mut u8,
    complete: bool,
    session: Option<&'a BorrowedSession>,
}

impl<'a> PendingRead<'a> {
    /// Guard scratch input across one actual read; `PendingRead::new(bytes, cursor, state, session)`.
    pub(super) fn new(
        bytes: &'a mut Vec<u8>,
        cursor: &'a mut usize,
        state: &'a mut u8,
        session: Option<&'a BorrowedSession>,
    ) -> Self {
        *cursor = 0;
        Self {
            bytes,
            cursor,
            state,
            complete: false,
            session,
        }
    }
    /// Expose only this read's scratch space; `guard.receive(4096)`.
    pub(super) fn receive(&mut self, size: usize) -> &mut [u8] {
        self.bytes.resize(size, 0);
        self.bytes.as_mut_slice()
    }
    /// Publish only the completed read length; `guard.complete(count)`.
    pub(super) fn complete(mut self, count: usize) -> bool {
        self.bytes.truncate(count);
        self.complete = true;
        count > 0
    }
}

impl Drop for PendingRead<'_> {
    fn drop(&mut self) {
        if self.complete {
            return;
        }
        self.bytes.clear();
        *self.cursor = 0;
        *self.state |= state::BAD;
        if let Some(session) = self.session {
            session.record(StreamRejoinCause::ReadInterrupted);
        }
    }
}
