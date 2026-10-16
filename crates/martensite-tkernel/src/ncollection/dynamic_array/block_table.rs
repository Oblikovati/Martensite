//! Block descriptor growth and release share the LinearVector policy.
use super::DynamicArray;

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        // Dropping clears with release; freeing earlier blocks must not precede later drops.
        self.clear(true);
    }
}

#[cfg(test)]
mod tests;
