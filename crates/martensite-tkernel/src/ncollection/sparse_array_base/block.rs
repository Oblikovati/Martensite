//! The block layout has no alignment padding between count, objects and bitmap.
use std::ffi::c_void;

/// Bitmap cell alias; `let mask: SparseArrayBlockCell = 0x80;`.
pub type SparseArrayBlockCell = u8;

/// Non-owning block projection; `unsafe { SparseArrayBlock::new(address, 8, 4) }`.
#[derive(Clone, Copy)]
pub struct SparseArrayBlock {
    /// Address of the block occupancy counter.
    pub count: *mut usize,
    /// Address of the first raw object slot, immediately after the counter.
    pub array: *mut c_void,
    /// Address of the occupancy bitmap after the object span.
    pub bits: *mut SparseArrayBlockCell,
}

impl SparseArrayBlock {
    /// Returns the cell width; `SparseArrayBlock::bits_per_cell() == 8`.
    pub const fn bits_per_cell() -> usize {
        SparseArrayBlockCell::BITS as usize
    }
    /// Computes the wrapping size; `const BYTES: usize = SparseArrayBlock::size(9, 12);`.
    /// This is unchecked arithmetic, not validation that the dimensions can be allocated.
    pub const fn size(items: usize, stride: usize) -> usize {
        // Unsigned arithmetic, including overflow before bitmap division.
        let cells = items.wrapping_add(Self::bits_per_cell() - 1) / Self::bits_per_cell();
        size_of::<usize>()
            .wrapping_add(size_of::<SparseArrayBlockCell>().wrapping_mul(cells))
            .wrapping_add(items.wrapping_mul(stride))
    }
    // Actual typed and erased allocations must never use wrapped metadata sizes.
    pub(crate) fn checked_storage_size(items: usize, stride: usize) -> Option<usize> {
        items
            .checked_mul(stride)
            .and_then(|bytes| bytes.checked_add(items.div_ceil(Self::bits_per_cell())))
            .and_then(|bytes| bytes.checked_add(size_of::<usize>()))
            .filter(|bytes| *bytes <= isize::MAX as usize)
    }
    /// Projects a block without accessing its unconstructed items; `unsafe { SparseArrayBlock::new(address, 8, 4) }`.
    /// # Safety
    /// Address must own the full non-wrapping header/items/bitmap span, addressable by isize,
    /// aligned for usize and all item addresses. The wrapping `size` helper cannot validate this.
    pub unsafe fn new(address: *mut c_void, items: usize, stride: usize) -> Self {
        // SAFETY: the caller guarantees `address` owns the whole header, item and bitmap span.
        let array = unsafe { Self::to_array(address) };
        Self {
            count: address.cast(),
            array,
            // SAFETY: the caller guarantees the item span of `items * stride` bytes lies inside
            // the block, so the bitmap start is in bounds.
            bits: unsafe { array.cast::<u8>().add(items * stride) },
        }
    }
    /// Returns the object-array start; `unsafe { SparseArrayBlock::to_array(address) }`.
    /// # Safety
    /// Address must identify a live block including its usize header.
    pub unsafe fn to_array(address: *mut c_void) -> *mut c_void {
        // SAFETY: the caller guarantees `address` is a live block including its usize header.
        unsafe { Self::native_to_array(address, 0, 0).cast() }
    }
    /// Ignores the size arguments; `unsafe { SparseArrayBlock::native_to_array(address, 8, 4) }`.
    /// # Safety
    /// Address must identify live storage including its usize header; dimensions are not evaluated.
    pub unsafe fn native_to_array(
        address: *mut c_void,
        _items: usize,
        _stride: usize,
    ) -> *mut std::ffi::c_char {
        // SAFETY: the caller guarantees `address` is a live block, which starts with a usize
        // header.
        unsafe { address.cast::<u8>().add(size_of::<usize>()).cast() }
    }
    /// Sets an occupancy bit and returns one if newly set; `unsafe { block.set(3) }`.
    /// # Safety
    /// Index must be in the block and its bitmap must permit exclusive mutation.
    /// Restore object-lifetime/count/occupancy agreement before the owner next uses or drops the block.
    pub unsafe fn set(self, index: usize) -> SparseArrayBlockCell {
        // SAFETY: the caller guarantees `index` is in the block, so its bitmap byte is in
        // bounds.
        let cell = unsafe { self.bits.add(index / 8) };
        let mask = 1u8 << (index % 8);
        // SAFETY: `cell` is an in-bounds bitmap byte, and the caller guarantees exclusive
        // access to the bitmap.
        let old = unsafe { *cell & mask };
        // SAFETY: `cell` is an in-bounds bitmap byte, and the caller guarantees exclusive
        // access to the bitmap.
        unsafe { *cell |= mask };
        u8::from(old == 0)
    }
    /// Returns the bit mask of an occupied slot; `unsafe { block.is_set(3) }`.
    /// # Safety
    /// Index must be within a readable initialized bitmap.
    pub unsafe fn is_set(self, index: usize) -> SparseArrayBlockCell {
        // SAFETY: the caller guarantees `index` is within the block's readable, initialized
        // bitmap.
        unsafe { *self.bits.add(index / 8) & (1u8 << (index % 8)) }
    }
    /// Clears and returns the old occupancy mask; `unsafe { block.unset(3) }`.
    /// # Safety
    /// Index must be in the block and its bitmap must permit exclusive mutation.
    /// Restore object-lifetime/count/occupancy agreement before the owner next uses or drops the block.
    pub unsafe fn unset(self, index: usize) -> SparseArrayBlockCell {
        // SAFETY: the caller guarantees `index` is in the block, so its bitmap byte is in
        // bounds.
        let cell = unsafe { self.bits.add(index / 8) };
        let mask = 1u8 << (index % 8);
        // SAFETY: `cell` is an in-bounds bitmap byte, and the caller guarantees exclusive
        // access to the bitmap.
        let old = unsafe { *cell & mask };
        // SAFETY: `cell` is an in-bounds bitmap byte, and the caller guarantees exclusive
        // access to the bitmap.
        unsafe { *cell &= !mask };
        old
    }
}

#[cfg(test)]
mod tests {
    use super::SparseArrayBlock;

    #[test]
    fn checked_storage_rejects_native_wrapping_and_pointer_span_overflow() {
        assert_eq!(
            SparseArrayBlock::checked_storage_size(9, 12),
            Some(size_of::<usize>() + 110)
        );
        assert_eq!(SparseArrayBlock::checked_storage_size(usize::MAX, 1), None);
        assert_eq!(
            SparseArrayBlock::checked_storage_size(1, isize::MAX as usize),
            None
        );
    }
}
