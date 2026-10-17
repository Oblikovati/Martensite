//! Typed owners retain the full allocation even after the logical index is emptied.
use super::{AllocBox, CellIndex};
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
        self.heap = None;
    }
}

fn allocate_heap(length: usize) -> AllocBox<Vec<i32>> {
    let bytes = length
        .checked_mul(std::mem::size_of::<i32>())
        .unwrap_or_else(|| {
            panic!("CellIndex length {length}: expected byte size representable by usize")
        });
    // The common allocator uses Rust's global allocator, independently of the filter arena.
    // Typed ownership needs neither its raw header nor its virtual-dispatch bookkeeping (#82).
    let mut heap = Vec::new();
    heap.try_reserve_exact(length)
        .unwrap_or_else(|_| panic!("CellIndex allocation {bytes}: expected available heap bytes"));
    heap.resize(length, 0);
    AllocBox::try_new(heap)
        .unwrap_or_else(|_| panic!("CellIndex allocation {bytes}: expected available heap bytes"))
}
