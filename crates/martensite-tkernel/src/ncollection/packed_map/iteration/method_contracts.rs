//! Iterator expressions and the distinct checked legacy / cached STL key domains.
#[cfg(test)]
use super::PackedMap;
use super::{PackedInteger, PackedMapIter};
use crate::standard::Failure;
impl<'a, T: PackedInteger> PackedMapIter<'a, T> {
    /// Constructs an iterator at the first packed bit; `let cursor = PackedMapIter::new(&map)`.
    #[cfg(test)]
    pub(crate) fn new(map: &'a PackedMap<T>) -> Self {
        map.iter()
    }
    /// Returns the legacy key or its error at end; `cursor.key_checked()?`.
    pub fn key_checked(&self) -> Result<&T, Failure> {
        self.key().ok_or_else(|| {
            // The error carries only a message, including configured stack capture.
            Failure::with_message(
                "Standard_NoSuchObject",
                "NCollection_PackedMap::Iterator::Key",
            )
        })
    }
    /// Returns legacy Value with the same checked domain as Key; `cursor.value_checked()?`.
    pub fn value_checked(&self) -> Result<&T, Failure> {
        self.key_checked()
    }
    /// Borrows the STL cached key, defined even after exhaustion; `cursor.cached_value()`.
    pub fn cached_value(&self) -> &T {
        &self.cached_key
    }
    /// Returns the STL arrow pointer to iterator-owned cached storage; `cursor.value_ptr()`.
    #[cfg(test)]
    pub(crate) fn value_ptr(&self) -> *const T {
        std::ptr::from_ref(&self.cached_key)
    }
    /// Copies the iterator state and returns its destination; `cursor.assign(&other)`.
    #[cfg(test)]
    pub(crate) fn assign(&mut self, other: &Self) -> &mut Self {
        self.clone_from(other);
        self
    }
    /// Returns the destination of prefix increment; `cursor.advance_returning()`.
    #[cfg(test)]
    pub(crate) fn advance_returning(&mut self) -> &mut Self {
        self.advance();
        self
    }
    /// Copies the old state before incrementing; `let previous = cursor.post_advance()`.
    #[cfg(test)]
    pub(crate) fn post_advance(&mut self) -> Self {
        let previous = self.clone();
        self.advance();
        previous
    }
    /// Compares iterator state, treating all end iterators as equal; `cursor.is_equal(&other)`.
    #[cfg(test)]
    pub(crate) fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
