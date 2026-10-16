use super::{BaseSparseArrayBasicIterator, BasicSparseArraySource, BasicSparseArraySourceMut};
impl<C: BasicSparseArraySource<V>, V, const M: bool> BaseSparseArrayBasicIterator<'_, C, V, M> {
    unsafe fn item_pointer(&self) -> *mut V {
        let source = self
            .source
            .expect("SparseArray iterator detached; expected a live occupied item");
        let block = self
            .block
            .expect("SparseArray iterator without block; expected a live occupied item");
        // SAFETY: callers guarantee the cursor rests on a live occupied item of `source`, which
        // sparse_item requires.
        unsafe { source.sparse_item(block, self.offset) }
    }
    /// Borrows the read-only value, as typed wrappers do; `unsafe { cursor.value() }`.
    /// # Safety
    /// Current slot/projection must be live and readable, without mutation through aliases for the borrow.
    pub unsafe fn value(&self) -> &V {
        // SAFETY: the caller guarantees the current item is live and readable with no mutable
        // alias for the borrow.
        unsafe { &*self.item_pointer() }
    }
}
impl<C: BasicSparseArraySource<V>, V> BaseSparseArrayBasicIterator<'_, C, V, false> {
    /// Const dereference with custom item projection; `unsafe { cursor.dereference() }`.
    /// # Safety
    /// Same valid-slot and returned-borrow requirements as `value`.
    pub unsafe fn dereference(&self) -> &V {
        // SAFETY: the caller upholds value's valid-slot and returned-borrow contract.
        unsafe { self.value() }
    }
    /// Read-only arrow; `unsafe { cursor.arrow() }`.
    /// # Safety
    /// Current occupied slot and custom item projection must remain live for this call.
    pub unsafe fn arrow(&self) -> *const V {
        // SAFETY: the caller guarantees the current occupied slot and its projection stay live
        // for this call.
        unsafe { self.item_pointer().cast_const() }
    }
}
impl<C: BasicSparseArraySourceMut<V>, V> BaseSparseArrayBasicIterator<'_, C, V, true> {
    /// Borrows the writable value, as typed wrappers do; `unsafe { cursor.change_value() }`.
    /// # Safety
    /// The projected object must be live and exclusively accessible for the returned borrow, including copied cursors.
    pub unsafe fn change_value(&mut self) -> &mut V {
        // SAFETY: the caller guarantees the projected item is live and exclusively accessible
        // for the returned borrow.
        unsafe { &mut *self.item_pointer() }
    }
    /// Writable dereference; `unsafe { cursor.dereference() }`.
    /// # Safety
    /// Same exclusive-object requirements as `change_value`.
    pub unsafe fn dereference(&mut self) -> &mut V {
        // SAFETY: the caller upholds change_value's exclusive-object contract.
        unsafe { self.change_value() }
    }
    /// Writable arrow without manufacturing a reference; `unsafe { cursor.arrow() }`.
    /// # Safety
    /// Current occupied slot and writable projection must be live; dereference additionally requires exclusivity.
    pub unsafe fn arrow(&self) -> *mut V {
        // SAFETY: the caller guarantees the current occupied slot and its projection stay live
        // for this call.
        unsafe { self.item_pointer() }
    }
}
