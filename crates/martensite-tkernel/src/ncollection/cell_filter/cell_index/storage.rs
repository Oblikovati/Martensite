//! Free-by-address allocator metadata keeps empty heap ownership after moves.
use super::{BaseAllocator, CellIndex};
impl CellIndex {
    pub(super) fn allocate(length: usize) -> Self {
        Self {
            inline: [0; 10],
            heap: (length > 10).then(|| allocate_heap(length)),
            length,
        }
    }
    pub(in crate::ncollection::cell_filter) fn move_construct(source: &mut Self) -> Self {
        let mut index = Self::allocate(0);
        index.move_from(source);
        index
    }
    pub(in crate::ncollection::cell_filter) fn move_from(&mut self, source: &mut Self) {
        if source.heap.is_none() {
            self.release_heap();
            self.inline[..source.length].copy_from_slice(source.as_slice());
        } else {
            // When both are heap-backed, the empty donor retains our previous allocation until Drop.
            std::mem::swap(&mut self.heap, &mut source.heap);
        }
        self.length = std::mem::replace(&mut source.length, 0);
    }
    pub(super) fn release_heap(&mut self) {
        if let Some(heap) = self.heap.take() {
            // SAFETY: `heap` came from the common allocator and this index owns it uniquely; take() ensures it is freed once. Common Free is infallible for this uniquely owned pointer, even when length is
            // zero: its private header retains the real allocation layout and capacity.
            unsafe { BaseAllocator::common().free_raw(heap.cast()).unwrap() };
        }
    }
}

fn allocate_heap(length: usize) -> std::ptr::NonNull<i32> {
    let bytes = length
        .checked_mul(std::mem::size_of::<i32>())
        .unwrap_or_else(|| {
            panic!("CellIndex length {length}: expected byte size representable by usize")
        });
    // Index storage uses the global allocator independently of the filter's arena.
    // The existing common policy's private header retains the layout after a source is emptied.
    super::BaseAllocator::common()
        .allocate_raw(bytes)
        .unwrap_or_else(|| panic!("CellIndex allocation {bytes}: expected available heap bytes"))
        .cast()
}
