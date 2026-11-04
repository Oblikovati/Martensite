//! First-fit free ranges of one device-memory block; pure bookkeeping without Vulkan calls.

/// Free byte ranges of a block, sorted by offset and never adjacent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BlockSpace {
    size: u64,
    free: Vec<(u64, u64)>,
}

impl BlockSpace {
    /// An entirely free block; `BlockSpace::new(16 << 20)`.
    pub fn new(size: u64) -> Self {
        Self {
            size,
            free: vec![(0, size)],
        }
    }

    /// Reserve `size` bytes at a multiple of `alignment` (a power of two); `space.reserve(256, 64)`.
    pub fn reserve(&mut self, size: u64, alignment: u64) -> Option<u64> {
        let (index, offset) =
            self.free
                .iter()
                .enumerate()
                .find_map(|(index, &(start, length))| {
                    let offset = start.next_multiple_of(alignment);
                    (offset + size <= start + length).then_some((index, offset))
                })?;
        let (start, length) = self.free.remove(index);
        let tail = (offset + size, start + length - offset - size);
        for range in [(start, offset - start), tail].into_iter().rev() {
            if range.1 > 0 {
                self.free.insert(index, range);
            }
        }
        Some(offset)
    }

    /// Return a reserved range and merge it with free neighbours; `space.release(offset, size)`.
    pub fn release(&mut self, offset: u64, size: u64) {
        let index = self.free.partition_point(|&(start, _)| start < offset);
        self.free.insert(index, (offset, size));
        if index + 1 < self.free.len() && offset + size == self.free[index + 1].0 {
            self.free[index].1 += self.free.remove(index + 1).1;
        }
        if index > 0 && self.free[index - 1].0 + self.free[index - 1].1 == offset {
            self.free[index - 1].1 += self.free.remove(index).1;
        }
    }

    /// True when nothing is reserved.
    pub fn is_empty(&self) -> bool {
        self.free == [(0, self.size)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_reservations_split_and_releases_merge_back_to_one_range() {
        let mut space = BlockSpace::new(1024);
        assert_eq!(space.reserve(100, 1), Some(0));
        assert_eq!(space.reserve(100, 256), Some(256));
        assert_eq!(space.free, vec![(100, 156), (356, 668)]);
        assert_eq!(space.reserve(150, 1), Some(100));
        space.release(256, 100);
        space.release(0, 100);
        space.release(100, 150);
        assert!(space.is_empty(), "{:?}", space.free);
    }

    #[test]
    fn a_request_larger_than_any_aligned_hole_is_refused() {
        let mut space = BlockSpace::new(512);
        assert_eq!(space.reserve(300, 1), Some(0));
        assert_eq!(space.reserve(256, 256), None);
        assert_eq!(space.reserve(200, 4), Some(300));
        assert_eq!(space.reserve(13, 1), None);
    }
}
