//! Borrowed assignment keeps item identity and invokes copy-assignment hooks.
use super::Sequence;
use crate::standard::Failure;

impl<T: Clone> Sequence<T> {
    /// Copies into an existing item through its assignment hook; `sequence.set_value_ref(1, &item)?`.
    pub fn set_value_ref(&mut self, index: usize, value: &T) -> Result<(), Failure> {
        self.change_value(index)?.clone_from(value);
        Ok(())
    }

    /// Copies another indexed item, with an explicit hook for aliasing assignment.
    /// Example: `sequence.set_value_from_index_with(1, 2, |item| item.assign_self())?`.
    /// Rust cannot call clone_from with aliased mutable/shared references. On equal indices,
    /// the hook represents the item's self-assignment; different indices use clone_from.
    pub fn set_value_from_index_with(
        &mut self,
        destination: usize,
        source: usize,
        assign_self: impl FnOnce(&mut T),
    ) -> Result<(), Failure> {
        // The source value is read before entering the assignment.
        let source = self.checked_node("Value", source, true)?;
        let destination = self.checked_node("ChangeValue", destination, true)?;
        if source == destination {
            // SAFETY: checked_node returned a live node owned by this sequence, and `&mut self`
            // bounds the exclusive item borrow.
            assign_self(unsafe { super::owner::item_mut(destination) });
            return Ok(());
        }
        // SAFETY: checked_node returned two live nodes owned by this sequence, and they differ, so Distinct live owned nodes provide disjoint item borrows without aliasing their links.
        unsafe {
            super::owner::item_mut::<T>(destination).clone_from(super::owner::item_ref(source))
        };
        Ok(())
    }
}
