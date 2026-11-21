//! Internal SetValue sources are constructed after allocation and gap initialization.
use super::DynamicArray;
#[cfg(test)]
use super::{ArrayError, checked_index};
#[cfg(test)]
use std::fmt::Debug;

impl<T: Default> DynamicArray<T> {
    /// Copies an internal logical source into a new item or an assignment temporary.
    /// `array.set_value_from_index_with_assignment(2, 0, |s| *s, |t, s| *t = *s)?`.
    /// Existing targets, including the source itself, receive assignment from a distinct temporary.
    /// New targets construct gaps first. Callbacks must leave their inputs live on unwind.
    #[cfg(test)]
    pub(crate) fn set_value_from_index_with_assignment<
        I: TryInto<usize> + Copy + Debug,
        S: TryInto<usize> + Copy + Debug,
    >(
        &mut self,
        index: I,
        source_index: S,
        copy_construct: impl FnOnce(&T) -> T,
        move_assign: impl FnOnce(&mut T, &mut T),
    ) -> Result<&mut T, ArrayError> {
        self.set_internal_with_assignment(index, source_index, |s| copy_construct(s), move_assign)
    }

    /// Moves from an internal donor that stays live, constructing after gaps and allocation.
    /// `array.set_value_moved_from_index_with_assignment(2, 0, std::mem::take, std::mem::swap)?`.
    /// A self-source first constructs a temporary; assignment therefore never aliases its inputs.
    /// A constructor panic retains donor mutations and the old logical size.
    #[cfg(test)]
    pub(crate) fn set_value_moved_from_index_with_assignment<
        I: TryInto<usize> + Copy + Debug,
        S: TryInto<usize> + Copy + Debug,
    >(
        &mut self,
        index: I,
        source_index: S,
        move_construct: impl FnOnce(&mut T) -> T,
        move_assign: impl FnOnce(&mut T, &mut T),
    ) -> Result<&mut T, ArrayError> {
        self.set_internal_with_assignment(index, source_index, move_construct, move_assign)
    }

    #[cfg(test)]
    fn set_internal_with_assignment<
        I: TryInto<usize> + Copy + Debug,
        S: TryInto<usize> + Copy + Debug,
    >(
        &mut self,
        index: I,
        source_index: S,
        construct: impl FnOnce(&mut T) -> T,
        assign: impl FnOnce(&mut T, &mut T),
    ) -> Result<&mut T, ArrayError> {
        self.set_internal_constructed(
            index,
            source_index,
            construct,
            |array, index, mut temporary| {
                assign(array.change_value(index)?, &mut temporary);
                Ok(())
            },
        )
    }
    #[cfg(test)]
    pub(super) fn set_internal_constructed<
        I: TryInto<usize> + Copy + Debug,
        S: TryInto<usize> + Copy + Debug,
    >(
        &mut self,
        index: I,
        source_index: S,
        construct: impl FnOnce(&mut T) -> T,
        replace: impl FnOnce(&mut Self, usize, T) -> Result<(), ArrayError>,
    ) -> Result<&mut T, ArrayError> {
        let index = checked_index(index)?;
        let source = self.logical_source_index(source_index)?;
        self.ensure_storage_for_index(index);
        let constructor = |array: &mut Self| {
            let (block, offset) = array.element_coordinates(source);
            construct(&mut array.blocks[block][offset])
        };
        if index >= self.used {
            self.emplace_new(index, constructor);
        } else {
            let temporary = constructor(self);
            replace(self, index, temporary)?;
        }
        self.change_value(index)
    }
}
