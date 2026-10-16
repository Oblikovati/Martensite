use super::{Array1, Storage};
use crate::ncollection::ArrayError;

#[allow(unsafe_code)]
impl<T: Copy> Array1<'_, T> {
    /// Copy-assigns directly on possibly overlapping raw buffers.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array1;
    /// let mut values = [1, 2, 3];
    /// let pointer = values.as_mut_ptr();
    /// unsafe { Array1::copy_assign_trivial_raw(pointer.add(1), pointer, 2); }
    /// assert_eq!(values, [1, 1, 2]);
    /// ```
    ///
    /// # Safety
    /// For nonzero `count`, `source` must be readable and `target` writable for
    /// `count` properly aligned `T` values with valid provenance; source values
    /// must be initialized. The regions may
    /// overlap. No references conflicting with either access may remain live.
    /// Zero count accepts null pointers and performs no accesses.
    pub unsafe fn copy_assign_trivial_raw(target: *mut T, source: *const T, count: usize) {
        if count == 0 {
            return;
        }
        // SAFETY: the caller guarantees both ranges are valid and aligned for `count` values and may overlap, which std::ptr::copy allows (memmove semantics for trivially-copyable elements).
        unsafe { std::ptr::copy(source, target, count) };
    }

    /// Copies potentially overlapping raw storage, preserving this array's bounds.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array1;
    /// let mut array = Array1::<i32>::with_size(2);
    /// let source = [3, 4];
    /// unsafe { array.copy_values_trivial_from_raw_parts(source.as_ptr(), 2).unwrap(); }
    /// assert_eq!(array.as_slice(), source);
    /// ```
    ///
    /// # Safety
    /// When lengths match and are nonzero, `source` must point to `source_len`
    /// initialized, readable `T` values in one allocation. It may overlap this
    /// array. No incompatible references may remain live during the copy.
    pub unsafe fn copy_values_trivial_from_raw_parts(
        &mut self,
        source: *const T,
        source_len: usize,
    ) -> Result<&mut Self, ArrayError> {
        if self.len() != source_len {
            return Err(ArrayError::DimensionMismatch {
                expected: self.len(),
                actual: source_len,
            });
        }
        // SAFETY: `self.data_mut_ptr()` addresses `source_len` live elements (the lengths
        // match), and the caller guarantees `source` is readable for `source_len` values.
        unsafe { Self::copy_assign_trivial_raw(self.data_mut_ptr(), source, source_len) };
        Ok(self)
    }

    /// Assigns raw storage and bounds, retaining equal-size borrowed storage.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array1;
    /// let mut array = Array1::<i32>::default();
    /// let source = [3, 4];
    /// unsafe { array.assign_trivial_from_raw_parts(source.as_ptr(), 2, -1); }
    /// assert_eq!(array.lower(), -1);
    /// ```
    ///
    /// # Safety
    /// For nonzero `source_len`, `source` must point to that many initialized,
    /// readable `T` values in one allocation. Overlap with this array is allowed;
    /// no incompatible references may remain live during the copy.
    #[allow(clippy::iter_cloned_collect)]
    pub unsafe fn assign_trivial_from_raw_parts(
        &mut self,
        source: *const T,
        source_len: usize,
        source_lower: i32,
    ) -> &mut Self {
        if self.len() == source_len {
            // SAFETY: the lengths match, and the caller guarantees `source` is readable for
            // `source_len` initialized values.
            unsafe { self.copy_values_trivial_from_raw_parts(source, source_len) }.unwrap();
        } else {
            // SAFETY: the caller guarantees `source` is readable for `source_len` initialized values. Read before releasing old storage: the source can be a slice of that allocation.
            self.storage = Storage::Owned(unsafe { copy_trivial_raw_to_vec(source, source_len) });
        }
        self.lower = source_lower;
        self
    }
}

#[allow(clippy::iter_cloned_collect, unsafe_code)]
unsafe fn copy_trivial_raw_to_vec<T: Copy>(source: *const T, count: usize) -> Vec<T> {
    if count == 0 {
        return Vec::new();
    }
    // SAFETY: The caller guarantees a readable initialized range; copied bypasses Clone hooks.
    let values = unsafe { std::slice::from_raw_parts(source, count) };
    values.iter().copied().collect()
}

#[allow(unsafe_code)]
impl<T> Array1<'_, T> {
    /// Applies nontrivial copy-assignment in ascending slot order.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array1;
    /// let mut values = [1, 2, 3];
    /// let pointer = values.as_mut_ptr();
    /// unsafe {
    ///     Array1::copy_assign_nontrivial_raw(pointer.add(1), pointer, 2,
    ///         |slot, donor| *slot = *donor, |_| {});
    /// }
    /// assert_eq!(values, [1, 1, 1]);
    /// ```
    ///
    /// # Safety
    /// For nonzero `count` and nonzero-sized `T`, both pointers must identify
    /// ranges of `count` live, initialized, aligned values with valid provenance.
    /// Each range must fit within one allocation and `isize::MAX` bytes.
    /// The target range must be writable and the source range readable. Ranges
    /// may overlap, but no conflicting references may remain live. Callbacks
    /// must not access either range through retained pointers or reenter it.
    /// Equal element pointers call `assign_self` with one mutable reference.
    /// Zero count permits null pointers and performs no accesses or callbacks.
    ///
    /// # Panics
    /// Nonzero `count` with zero-sized `T` is rejected before accessing pointers.
    pub unsafe fn copy_assign_nontrivial_raw(
        target: *mut T,
        source: *const T,
        count: usize,
        mut assign: impl FnMut(&mut T, &T),
        mut assign_self: impl FnMut(&mut T),
    ) {
        assert!(
            count == 0 || size_of::<T>() > 0,
            "element size 0; expected nonzero size for {count} slots"
        );
        for offset in 0..count {
            // SAFETY: the caller guarantees `target` addresses `count` slots, and `offset <
            // count`.
            let slot = unsafe { target.add(offset) };
            // SAFETY: the caller guarantees `source` addresses `count` slots, and `offset <
            // count`.
            let donor = unsafe { source.add(offset) };
            // SAFETY: `slot` and `donor` are in-bounds slots of the caller's ranges, which hold
            // initialized values with no conflicting references.
            unsafe { assign_raw_slot(slot, donor, &mut assign, &mut assign_self) };
        }
    }
}

#[allow(unsafe_code)]
unsafe fn assign_raw_slot<T>(
    slot: *mut T,
    donor: *const T,
    assign: &mut impl FnMut(&mut T, &T),
    assign_self: &mut impl FnMut(&mut T),
) {
    if std::ptr::eq(slot, donor) {
        // SAFETY: the caller guarantees `slot` is an initialized in-bounds slot with no other
        // live reference, so one exclusive borrow is sound.
        unsafe { assign_self(&mut *slot) };
    } else {
        // SAFETY: `slot` and `donor` are distinct initialized in-bounds slots (checked above), so a shared and an exclusive borrow do not alias. Per-element borrows permit the forward overlap cascade.
        unsafe { assign(&mut *slot, &*donor) };
    }
}
