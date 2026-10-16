use super::{BaseSequenceBasicIterator, BasicSequenceNode};

impl<N: BasicSequenceNode, const M: bool> BaseSequenceBasicIterator<N, M> {
    /// Advances, including an exhausted no-op; `unsafe { cursor.next() }`.
    /// # Safety
    /// Current must be null or a live readable SeqNode through this call.
    pub unsafe fn next(&mut self) {
        // SAFETY: the caller guarantees the current node is null or live and readable, which
        // the raw next needs.
        unsafe { self.raw.next() };
    }
    /// Steps back, including end-to-tail movement; `unsafe { cursor.previous() }`.
    /// # Safety
    /// Previous must be null or a live readable SeqNode through this call.
    pub unsafe fn previous(&mut self) {
        // SAFETY: the caller guarantees the remembered previous node is null or live and
        // readable, which the raw previous needs.
        unsafe { self.raw.previous() };
    }
    /// Implements prefix increment returning this cursor; `unsafe { cursor.advance() }`.
    /// # Safety
    /// Current must satisfy next's live-link contract.
    pub unsafe fn advance(&mut self) -> &mut Self {
        // SAFETY: the caller upholds next's live-link contract.
        unsafe { self.next() };
        self
    }
    /// Implements postfix increment returning the old cursor; `unsafe { cursor.post_advance() }`.
    /// # Safety
    /// Current must satisfy next's live-link contract.
    pub unsafe fn post_advance(&mut self) -> Self {
        let old = *self;
        // SAFETY: the caller upholds next's live-link contract.
        unsafe { self.next() };
        old
    }
    /// Implements prefix decrement returning this cursor; `unsafe { cursor.retreat() }`.
    /// # Safety
    /// Previous must satisfy previous's live-link contract.
    pub unsafe fn retreat(&mut self) -> &mut Self {
        // SAFETY: the caller upholds previous's live-link contract.
        unsafe { self.previous() };
        self
    }
    /// Implements postfix decrement returning the old cursor; `unsafe { cursor.post_retreat() }`.
    /// # Safety
    /// Previous must satisfy previous's live-link contract.
    pub unsafe fn post_retreat(&mut self) -> Self {
        let old = *self;
        // SAFETY: the caller upholds previous's live-link contract.
        unsafe { self.previous() };
        old
    }
}
