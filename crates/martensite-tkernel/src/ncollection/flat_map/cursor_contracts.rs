//! Iterator assignment and increment return values keep slot identity.
use super::{Failure, FlatMapIter};

impl<K> FlatMapIter<'_, K> {
    /// Returns the current slot address; `cursor.key_ptr()?`.
    pub fn key_ptr(&self) -> Result<*const K, Failure> {
        self.key().map(std::ptr::from_ref)
    }
    /// Advances and returns this cursor, as prefix increment; `cursor.advance_returning().key()`.
    pub fn advance_returning(&mut self) -> &mut Self {
        self.advance();
        self
    }
    /// Returns the previous cursor and then advances, as postfix increment; `cursor.advance_post().key()`.
    pub fn advance_post(&mut self) -> Self {
        let previous = self.clone();
        self.advance();
        previous
    }
    /// Copies the slot/index state and returns this cursor; `cursor.assign_from(&source)`.
    pub fn assign_from(&mut self, source: &Self) -> &mut Self {
        self.items = source.items.clone();
        self
    }
    /// Preserves assignment to the same iterator without aliased borrows; `cursor.assign_self()`.
    pub fn assign_self(&mut self) -> &mut Self {
        self
    }
    /// Compares slot identity, treating all exhausted iterators as equal; `first.is_equal(&second)`.
    pub fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
