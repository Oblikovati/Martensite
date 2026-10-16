//! Callback boundaries keep the observable occupancy and count publication order.
use super::{
    SparseArrayBase, SparseArrayBlock, SparseCopyItemFunc, SparseCreateItemFunc,
    SparseDestroyItemFunc,
};
use std::ffi::c_void;
impl SparseArrayBase {
    /// Constructs or assigns through callbacks; `unsafe { base.set_value(5, source, create, copy) }`.
    /// # Safety
    /// Source/callbacks must match item stride/alignment and stored destructor. No concurrent or
    /// reentrant mutations or outstanding item references are allowed. Destructors must not unwind.
    pub unsafe fn set_value(
        &self,
        index: usize,
        source: *mut c_void,
        create: SparseCreateItemFunc,
        copy: SparseCopyItemFunc,
    ) -> *mut c_void {
        // SAFETY: the caller upholds set_value's contract, which is set_value_with's.
        unsafe { self.set_value_with(index, source, |a, b| create(a, b), |a, b| copy(a, b)) }
    }
    /// Adapts construction/assignment callbacks with borrowed context; `unsafe { base.set_value_with(5, source, create, copy) }`.
    /// # Safety
    /// Same requirements as `set_value`; callbacks may observe valid base state but must not mutate it.
    pub unsafe fn set_value_with(
        &self,
        index: usize,
        source: *mut c_void,
        mut create: impl FnMut(*mut c_void, *mut c_void),
        mut copy: impl FnMut(*mut c_void, *mut c_void),
    ) -> *mut c_void {
        let number = index / self.block_size();
        // SAFETY: the caller guarantees no reentrant mutation or outstanding item reference, so
        // growing the table aliases nothing.
        unsafe { self.ensure_block(number) };
        // SAFETY: ensure_block just allocated block `number` of this base.
        let block = unsafe { self.get_block(self.block_address(number)) };
        let offset = index % self.block_size();
        // SAFETY: `block` is live and `offset` is below its block size.
        let item = unsafe { self.get_item(block, offset) };
        // SAFETY: `block` is live with an initialized bitmap and `offset` is below its block
        // size.
        if unsafe { block.is_set(offset) } != 0 {
            copy(item, source);
            return item;
        }
        create(item, source);
        // SAFETY: `create` just constructed the slot at `offset` of the live `block`.
        unsafe { self.publish_slot(block, offset) };
        item
    }
    /// Removes a constructed item through its destructor; `unsafe { base.unset_value(5, destroy) }`.
    /// # Safety
    /// No references may alias mutation; destructor must match live objects and must not unwind or mutate the base.
    pub unsafe fn unset_value(&self, index: usize, destroy: SparseDestroyItemFunc) -> bool {
        // SAFETY: the caller upholds unset_value's contract, which is unset_with's.
        unsafe { self.unset_with(index, |item| destroy(item)) }
    }
    /// Adapts removal with borrowed callback context; `unsafe { base.unset_with(5, destroy) }`.
    /// # Safety
    /// Same requirements as `unset_value`.
    pub unsafe fn unset_with(&self, index: usize, mut destroy: impl FnMut(*mut c_void)) -> bool {
        if !self.has_value(index) {
            return false;
        }
        let number = index / self.block_size();
        // SAFETY: has_value proved the block of `index` is allocated, so it is a live block of
        // this base.
        let block = unsafe { self.get_block(self.block_address(number)) };
        let offset = index % self.block_size();
        // SAFETY: `offset` is in the live block, and the caller guarantees no alias observes
        // the bitmap.
        unsafe { block.unset(offset) };
        // SAFETY: `block` is live and `offset` is below its block size; the destructor runs on
        // the item that was just unpublished.
        destroy(unsafe { self.get_item(block, offset) });
        // SAFETY: `block.count` is the live block's header, and the single-threaded state has
        // no outstanding reference.
        unsafe {
            *block.count -= 1;
            (*self.state.get()).size -= 1;
        }
        // SAFETY: `block.count` is the live block's header.
        if unsafe { *block.count } == 0 {
            // SAFETY: the destructor matches every live item of this base (the caller's
            // contract).
            unsafe { self.free_block_with(number, &mut destroy) };
        }
        true
    }
    /// Clears all objects and the table; `unsafe { base.clear_items(destroy) }`.
    /// # Safety
    /// Destructor must match every live object, must not unwind or mutate this base, and no live references may remain.
    pub unsafe fn clear_items(&self, destroy: SparseDestroyItemFunc) {
        // SAFETY: the caller upholds clear_items' contract, which is clear_with's.
        unsafe { self.clear_with(|item| destroy(item)) };
    }
    /// Clears through a context-carrying callback; `unsafe { base.clear_with(destroy) }`.
    /// # Safety
    /// Same requirements as `clear_items`.
    pub unsafe fn clear_with(&self, mut destroy: impl FnMut(*mut c_void)) {
        for number in 0..self.block_slots() {
            // SAFETY: `number < block_slots`, and the caller guarantees the destructor matches
            // every live item.
            unsafe { self.free_block_with(number, &mut destroy) };
        }
        // SAFETY: every block was freed above, so the table this base allocated with
        // `block_slots` pointers is no longer read.
        unsafe {
            self.release_storage(
                (*self.state.get()).table.cast(),
                self.block_slots() * size_of::<*mut c_void>(),
            )
        };
        // SAFETY: the single-threaded state has no outstanding reference while its fields are
        // reset.
        unsafe {
            (*self.state.get()).table = std::ptr::null_mut();
            (*self.state.get()).block_slots = 0;
            (*self.state.get()).size = 0;
        }
    }
    pub(super) unsafe fn free_block_with(
        &self,
        number: usize,
        destroy: &mut impl FnMut(*mut c_void),
    ) {
        let address = self.block_address(number);
        if address.is_null() {
            return;
        }
        // SAFETY: `address` is a non-null block of this base, with its stride and block size.
        let block = unsafe { self.get_block(address) };
        // SAFETY: `block` is a live block of this base, and the destructor matches its live
        // items (the caller's contract).
        unsafe { self.destroy_block_objects(block, destroy) };
        // SAFETY: the block's items were destroyed, so its storage is released once and its
        // table slot `number < block_slots` is cleared.
        unsafe {
            self.release_storage(address.cast(), self.checked_block_bytes());
            *(*self.state.get()).table.add(number) = std::ptr::null_mut();
        }
    }
    pub(super) unsafe fn publish_slot(&self, block: SparseArrayBlock, offset: usize) {
        // SAFETY: callers pass a live block of this base and an `offset` below its block size
        // whose item was just constructed.
        unsafe {
            block.set(offset);
            *block.count += 1;
            (*self.state.get()).size += 1;
        }
    }
    unsafe fn destroy_block_objects(
        &self,
        block: SparseArrayBlock,
        destroy: &mut impl FnMut(*mut c_void),
    ) {
        for offset in 0..self.block_size() {
            // SAFETY: `block` is live with an initialized bitmap and `offset` is below its
            // block size.
            if unsafe { block.is_set(offset) } == 0 {
                continue;
            }
            // SAFETY: the slot at `offset` of the live `block` is occupied, and the destructor
            // matches it.
            destroy(unsafe { self.get_item(block, offset) });
            // SAFETY: the single-threaded state has no outstanding reference while the size is
            // decremented.
            unsafe {
                (*self.state.get()).size -= 1;
            }
        }
    }
}
