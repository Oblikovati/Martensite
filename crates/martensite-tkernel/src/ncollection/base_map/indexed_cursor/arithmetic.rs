use super::BaseMapIndexedIterator;
use std::cmp::Ordering;

impl<C, const M: bool> BaseMapIndexedIterator<C, M> {
    /// Advances unconditionally and returns this iterator; `cursor.advance_returning()`.
    pub(crate) fn advance_returning(&mut self) -> &mut Self {
        self.index = self.index.wrapping_add(1);
        self
    }
    /// Returns the old position before advancing; `let previous = cursor.advance_post()`.
    pub(crate) fn advance_post(&mut self) -> Self {
        let previous = *self;
        self.advance_returning();
        previous
    }
    /// Retreats unconditionally, including unsigned underflow; `cursor.retreat_returning()`.
    pub(crate) fn retreat_returning(&mut self) -> &mut Self {
        self.index = self.index.wrapping_sub(1);
        self
    }
    /// Returns the old position before retreating; `let previous = cursor.retreat_post()`.
    pub(crate) fn retreat_post(&mut self) -> Self {
        let previous = *self;
        self.retreat_returning();
        previous
    }
    /// Applies a signed index displacement; `cursor.offset_returning(-1)`.
    pub(crate) fn offset_returning(&mut self, offset: isize) -> &mut Self {
        self.index = self.index.wrapping_add_signed(offset);
        self
    }
    /// Subtracts a signed index displacement; `cursor.offset_sub_returning(1)`.
    pub(crate) fn offset_sub_returning(&mut self, offset: isize) -> &mut Self {
        self.index = self.index.wrapping_sub_signed(offset);
        self
    }
    /// Returns a displaced copy, for either addition order; `cursor.shifted(2)`.
    pub(crate) fn shifted(&self, offset: isize) -> Self {
        let mut shifted = *self;
        shifted.offset_returning(offset);
        shifted
    }
    /// Returns a copy displaced backwards; `cursor.shifted_back(2)`.
    pub(crate) fn shifted_back(&self, offset: isize) -> Self {
        let mut shifted = *self;
        shifted.offset_sub_returning(offset);
        shifted
    }
    /// Subtracts indices regardless of owners; `cursor.distance(&other)`.
    pub(crate) fn distance<const OTHER: bool>(
        &self,
        other: &BaseMapIndexedIterator<C, OTHER>,
    ) -> isize {
        (self.index as isize).wrapping_sub(other.index as isize)
    }
    /// Compares only indices; `cursor.compare_position(&other)`.
    /// Kept separate from Rust ordering traits because end equality ignores indices.
    pub(crate) fn compare_position<const OTHER: bool>(
        &self,
        other: &BaseMapIndexedIterator<C, OTHER>,
    ) -> Ordering {
        self.distance(other).cmp(&0)
    }
}
