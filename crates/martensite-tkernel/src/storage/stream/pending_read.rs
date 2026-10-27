//! An interrupted device read cannot publish its resized scratch buffer (#318).
use super::state;

pub(super) struct PendingRead<'a> {
    bytes: &'a mut Vec<u8>,
    cursor: &'a mut usize,
    state: &'a mut u8,
    complete: bool,
}
impl<'a> PendingRead<'a> {
    /// Guard scratch space across an injected read; `PendingRead::new(bytes, cursor, state)`.
    pub(super) fn new(bytes: &'a mut Vec<u8>, cursor: &'a mut usize, state: &'a mut u8) -> Self {
        *cursor = 0;
        Self {
            bytes,
            cursor,
            state,
            complete: false,
        }
    }
    /// Expose scratch space without claiming a received length; `guard.receive(4096)`.
    pub(super) fn receive(&mut self, size: usize) -> &mut [u8] {
        self.bytes.resize(size, 0);
        self.bytes.as_mut_slice()
    }
    /// Publish only a completed device result; `guard.complete(count)`.
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
    }
}
