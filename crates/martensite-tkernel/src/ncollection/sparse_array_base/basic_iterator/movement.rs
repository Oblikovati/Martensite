use super::{BaseSparseArrayBasicIterator, BasicSparseArraySource};
impl<C: BasicSparseArraySource<V>, V, const M: bool> BaseSparseArrayBasicIterator<'_, C, V, M> {
    /// Restarts with the originally captured end span; `unsafe { cursor.restart() }`.
    /// # Safety
    /// Container storage and occupied objects must remain valid, with no incompatible mutation.
    pub unsafe fn restart(&mut self) {
        self.index = 0;
        // SAFETY: the caller guarantees the container storage stays valid, which skip_unset
        // needs.
        unsafe { self.skip_unset() };
    }
    /// Advances, leaving exhausted indices unchanged; `unsafe { cursor.advance() }`.
    /// # Safety
    /// Cached blocks must remain live and storage geometry must be unchanged.
    pub unsafe fn advance(&mut self) {
        if !self.more {
            return;
        }
        self.index += 1;
        self.offset += 1;
        // SAFETY: the caller guarantees the container storage stays valid, which find_next
        // needs.
        unsafe { self.find_next() };
    }
    /// Returns this iterator after prefix increment; `unsafe { cursor.advance_returning() }`.
    /// # Safety
    /// Same live-block requirements as `advance`.
    pub unsafe fn advance_returning(&mut self) -> &mut Self {
        // SAFETY: the caller upholds advance's live-block contract.
        unsafe { self.advance() };
        self
    }
    /// Returns a shallow mutable or const copy before advancing; `unsafe { cursor.advance_post() }`.
    /// # Safety
    /// Same requirements as `advance`; copied mutable cursors do not establish exclusive references.
    pub unsafe fn advance_post(&mut self) -> Self {
        let previous = *self;
        // SAFETY: the caller upholds advance's live-block contract.
        unsafe { self.advance() };
        previous
    }
    pub(super) unsafe fn skip_unset(&mut self) {
        self.more = false;
        let Some(source) = self.source else {
            return;
        };
        if self.index >= self.end {
            return;
        }
        let size = source.sparse_storage().block_size();
        self.block_number = self.index / size;
        self.offset = self.index % size;
        // SAFETY: the caller guarantees the container storage stays valid, which find_next
        // needs.
        unsafe { self.find_next() };
    }
    unsafe fn find_next(&mut self) {
        self.more = false;
        let Some(source) = self.source else {
            self.index = self.end;
            return;
        };
        let base = source.sparse_storage();
        while self.index < self.end && self.block_number < base.block_slots() {
            let address = base.block_address(self.block_number);
            // SAFETY: `address` is a non-null block of the live container, which scan_block
            // reads.
            if !address.is_null() && unsafe { self.scan_block(address) } {
                return;
            }
            self.block_number += 1;
            self.offset = 0;
            self.index = self.block_number * base.block_size();
        }
        self.index = self.end;
    }
    unsafe fn scan_block(&mut self, address: *mut std::ffi::c_void) -> bool {
        let base = self.source.unwrap().sparse_storage();
        // SAFETY: callers pass a non-null block address of this live container, which has its
        // stride and block size.
        let block = unsafe { base.get_block(address) };
        self.block = Some(block);
        // Inner scanning intentionally does not check the custom captured end span.
        while self.offset < base.block_size() {
            // SAFETY: `block` is a live block with an initialized bitmap and `self.offset` is
            // below its block size.
            if unsafe { block.is_set(self.offset) } != 0 {
                self.more = true;
                return true;
            }
            self.offset += 1;
            self.index += 1;
        }
        false
    }
}
