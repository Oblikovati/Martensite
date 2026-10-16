use super::super::BasicSequenceNodeMut;
use super::{BaseSequenceBasicIterator, BasicSequenceNode, SeqNode};

impl<N: BasicSequenceNode, const M: bool> BaseSequenceBasicIterator<N, M> {
    /// Reads the value, even for a writable iterator; `unsafe { cursor.value() }`.
    /// # Safety
    /// Current must be a live embedded link in N. The node and returned item must remain
    /// readable and unmodified through all aliases for the lifetime of the returned borrow.
    pub unsafe fn value(&self) -> &N::Item {
        // SAFETY: caller supplies the live concrete node and reference lifetime guarantees.
        unsafe { (&*N::from_sequence_link(self.raw.current_node())).sequence_value() }
    }
    /// Returns the read-only value identity; `unsafe { cursor.value_ptr() }`.
    /// # Safety
    /// Current must satisfy value's live, readable concrete-node contract during this call.
    pub unsafe fn value_ptr(&self) -> *const N::Item {
        // SAFETY: the caller upholds value's contract for the current node, as this function's
        // `# Safety` section requires.
        std::ptr::from_ref(unsafe { self.value() })
    }
}
impl<N: BasicSequenceNode> BaseSequenceBasicIterator<N, false> {
    /// Returns the read-only node without dereferencing it; `cursor.node()`.
    pub fn node(&self) -> *const SeqNode {
        self.current_node()
    }
    /// Const dereference via the value read; `unsafe { cursor.dereference() }`.
    /// # Safety
    /// The node must satisfy value's readable-node and returned-reference contract.
    pub unsafe fn dereference(&self) -> &N::Item {
        // SAFETY: the caller upholds value's contract for the current node, as this function's
        // `# Safety` section requires.
        unsafe { self.value() }
    }
    /// Applies a custom address-of after the value read; `unsafe { cursor.arrow_with(address) }`.
    /// # Safety
    /// The node must satisfy value's live readable-node contract for policy dispatch.
    /// The policy must preserve that borrow. Returned raw-pointer validity, provenance and
    /// read permissions must be established separately before dereference.
    pub unsafe fn arrow_with(
        &self,
        address: impl FnOnce(&N::Item) -> *const N::Item,
    ) -> *const N::Item {
        // SAFETY: caller guarantees the node remains shared-readable during dispatch.
        address(unsafe { self.dereference() })
    }
    /// Const arrow via the value read; `unsafe { cursor.arrow() }`.
    /// # Safety
    /// The node must satisfy value's readable-node contract during this call.
    pub unsafe fn arrow(&self) -> *const N::Item {
        // SAFETY: the caller upholds value's readable-node contract, which arrow_with relies
        // on.
        unsafe { self.arrow_with(std::ptr::from_ref) }
    }
}
impl<N: BasicSequenceNode> BaseSequenceBasicIterator<N, true> {
    /// Returns the mutable node without dereferencing it; `cursor.node()`.
    pub fn node(&self) -> *mut SeqNode {
        self.raw.current_node()
    }
    /// Returns the mutable node identity; `cursor.mutable_node()`.
    pub fn mutable_node(&self) -> *mut SeqNode {
        self.node()
    }
    /// Returns the mutable previous-node identity; `cursor.mutable_previous_node()`.
    pub fn mutable_previous_node(&self) -> *mut SeqNode {
        self.raw.previous_node()
    }
}
impl<N: BasicSequenceNodeMut> BaseSequenceBasicIterator<N, true> {
    /// Writes through the concrete node; `*unsafe { cursor.change_value() } = item`.
    /// # Safety
    /// Current must be a live embedded link in writable N. The entire concrete node must
    /// permit exclusive borrowing; no aliases may access it during the returned item borrow.
    pub unsafe fn change_value(&mut self) -> &mut N::Item {
        // SAFETY: caller guarantees projection validity and exclusive access through copied cursors.
        unsafe { (&mut *N::from_sequence_link(self.raw.current_node())).sequence_change_value() }
    }
    /// Mutable dereference through the writable value; `unsafe { cursor.dereference() }`.
    /// # Safety
    /// The node must satisfy change_value's exclusive-node and returned-reference contract.
    pub unsafe fn dereference(&mut self) -> &mut N::Item {
        // SAFETY: the caller upholds change_value's exclusive-node contract, as this function's
        // `# Safety` section requires.
        unsafe { self.change_value() }
    }
    /// Applies mutable overloaded address-of after ChangeValue; `unsafe { cursor.arrow_with(address) }`.
    /// # Safety
    /// The node must satisfy change_value's live exclusive-node contract for policy dispatch.
    /// The policy must preserve that borrow. Returned raw-pointer validity, provenance and
    /// write permissions must be established separately before dereference.
    pub unsafe fn arrow_with(
        &mut self,
        address: impl FnOnce(&mut N::Item) -> *mut N::Item,
    ) -> *mut N::Item {
        // SAFETY: caller guarantees exclusive access while the address policy runs.
        address(unsafe { self.dereference() })
    }
    /// Mutable arrow through the writable value; `unsafe { cursor.arrow() }`.
    /// # Safety
    /// The node must satisfy change_value's exclusive-node contract for this call.
    pub unsafe fn arrow(&mut self) -> *mut N::Item {
        // SAFETY: the caller upholds change_value's exclusive-node contract, which arrow_with
        // relies on.
        unsafe { self.arrow_with(std::ptr::from_mut) }
    }
}
