//! Keep the assignment ordering separate from Rust's existing replacement-clone extension.
use super::SparseArray;
impl<T> SparseArray<T> {
    /// Preserves self-assignment without conflicting Rust borrows; `array.assign_self()`.
    pub fn assign_self(&mut self) -> &mut Self {
        self
    }
}
impl<T: Clone> SparseArray<T> {
    /// Copies in ascending index order, preserving completed copies on panic; `target.assign(&source)`.
    pub fn assign(&mut self, source: &Self) -> &mut Self {
        self.storage.assign(&source.storage);
        self
    }
}
impl<T: Clone> Clone for SparseArray<T> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
    // The default clone_from first completes clone(), then replaces self. Assignment instead
    // reuses live slots and retains the target's table capacity.
}
