//! Raw self-splice drops metadata after mutating links; typed Sequence forbids this path.
use super::BaseSequence;
use crate::standard::Failure;

impl BaseSequence {
    /// Raw self-append, retaining the resulting cycle in caller-owned nodes.
    /// Example: `unsafe { base.append_self() }; assert!(base.is_empty())`.
    /// # Safety
    /// Nodes must be live, exclusively managed and well formed. Caller must retain their
    /// allocation owners and restore/delete them separately after this operation loses all links.
    pub unsafe fn append_self(&mut self) {
        if self.size == 0 {
            return;
        }
        let (head, tail) = (self.first, self.last);
        // SAFETY: the base is nonempty, so both endpoints are live nodes, and the caller keeps
        // every allocation and accepts the resulting cycle.
        unsafe {
            self.guarded_link_mut(tail).set_next(head);
            self.guarded_link_mut(head).set_previous(tail);
        }
        self.nullify();
    }
    /// Raw self-prepend; `unsafe { base.prepend_self() }`.
    /// # Safety
    /// The same live-node, exclusive-access and retained-ownership requirements as append_self apply.
    pub unsafe fn prepend_self(&mut self) {
        // SAFETY: this function's caller gives append_self's guarantees, as its `# Safety`
        // section states.
        unsafe { self.append_self() };
    }
    /// Raw self-insertion after an index, including its interior cycles.
    /// Example: `unsafe { base.insert_self_after(1)? }; assert!(base.is_empty())`.
    /// # Safety
    /// All node/cache links must be live and consistent. Caller retains every allocation and
    /// must restore/delete nodes separately after this operation clears base ownership metadata.
    pub unsafe fn insert_self_after(&mut self, index: usize) -> Result<(), Failure> {
        self.check_insertion(index)?;
        if self.size == 0 {
            return Ok(());
        }
        if index == 0 {
            // SAFETY: this function's caller gives prepend_self's guarantees, as its `# Safety`
            // section states.
            unsafe { self.prepend_self() };
            return Ok(());
        }
        // SAFETY: check_insertion proved `index` is in 1..=size, and the caller guarantees the
        // links and cache are live and consistent.
        let before = unsafe { self.find(index) };
        // SAFETY: `before` and both endpoints are live nodes of this base; the caller keeps
        // every allocation and accepts the resulting cycle.
        unsafe { self.link_after(before, self.first, self.last) };
        self.nullify();
        Ok(())
    }
}
