//! Constructors build into an owned Vec allocation.
use super::LinearVector;

impl<T> LinearVector<T> {
    /// Reserve `capacity`, then let `initialize` publish the elements. A panicking element
    /// constructor drops the fresh vector (Vec frees it): a leaked allocation and
    /// completed prefix are never left behind.
    pub(super) fn construct_owned(capacity: usize, initialize: impl FnOnce(&mut Self)) -> Self {
        let mut destination = Self::with_capacity(capacity);
        initialize(&mut destination);
        destination
    }
}
