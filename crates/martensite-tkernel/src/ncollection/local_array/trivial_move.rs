use super::LocalArray;

impl<T: Copy, const N: usize> LocalArray<T, N> {
    /// Uses trivial-element move assignment; `destination.move_from_trivial(&mut source)`.
    /// `Copy` selects this policy explicitly; it is not inferred from the element type.
    pub fn move_from_trivial(&mut self, source: &mut Self) -> &mut Self {
        if !source.elements.spilled() {
            self.deallocate();
            self.elements.extend_from_slice(source.as_slice());
            source.elements.clear();
            return self;
        }
        if self.elements.spilled() {
            // Both heap pointers are swapped before zeroing the source's logical size.
            std::mem::swap(&mut self.elements, &mut source.elements);
            source.elements.clear();
            return self;
        }
        self.move_from(source);
        self
    }

    /// Preserves storage for self move assignment; `array.move_self_trivial()`.
    pub fn move_self_trivial(&mut self) -> &mut Self {
        self.move_self()
    }
}
