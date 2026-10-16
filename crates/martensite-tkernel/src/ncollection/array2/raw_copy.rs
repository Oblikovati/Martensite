use super::{Array2, ArrayError, extent};

#[allow(unsafe_code)]
impl<T: Copy> Array2<'_, T> {
    /// Copies overlapping row-major raw storage without changing rectangle bounds.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array2;
    /// let mut array = Array2::<i32>::with_size(1, 2);
    /// let source = [3, 4];
    /// unsafe { array.copy_values_trivial_from_raw_parts(source.as_ptr(), 1, 2).unwrap(); }
    /// assert_eq!(array.as_slice(), source);
    /// ```
    ///
    /// # Safety
    /// If dimensions match and the rectangle is nonempty, `source` must point to
    /// `rows * cols` initialized, readable `T` values in one allocation. Overlap
    /// with this rectangle is allowed, without incompatible live references.
    pub unsafe fn copy_values_trivial_from_raw_parts(
        &mut self,
        source: *const T,
        rows: usize,
        cols: usize,
    ) -> Result<&mut Self, ArrayError> {
        self.check_dimensions(rows, cols)?;
        let length = raw_rectangle_len(rows, cols);
        // SAFETY: check_dimensions proved the rectangle has this array's length, and the caller
        // guarantees `source` is readable for `rows * cols` values.
        unsafe {
            self.elements
                .copy_values_trivial_from_raw_parts(source, length)?
        };
        Ok(self)
    }

    /// Assigns overlapping raw storage with inclusive rectangle bounds.
    ///
    /// ```
    /// use martensite_tkernel::ncollection::Array2;
    /// let mut array = Array2::<i32>::default();
    /// let source = [3, 4];
    /// unsafe { array.assign_trivial_from_raw_parts(source.as_ptr(), -1, -1, 3, 4).unwrap(); }
    /// assert_eq!(array.value(-1, 3).copied(), Ok(3));
    /// ```
    ///
    /// # Safety
    /// For a nonempty rectangle, `source` must point to its full initialized,
    /// readable row-major extent in one allocation. Overlap with this array is
    /// allowed, without incompatible live references during assignment.
    pub unsafe fn assign_trivial_from_raw_parts(
        &mut self,
        source: *const T,
        rl: i32,
        ru: i32,
        cl: i32,
        cu: i32,
    ) -> Result<&mut Self, ArrayError> {
        let (rows, cols) = (extent(rl, ru)?, extent(cl, cu)?);
        // SAFETY: the caller guarantees `source` is readable for `rows * cols` initialized
        // values, which assign_raw_rectangle requires.
        unsafe { self.assign_raw_rectangle(source, rows, cols, rl, cl) };
        Ok(self)
    }

    unsafe fn assign_raw_rectangle(
        &mut self,
        source: *const T,
        rows: usize,
        cols: usize,
        rl: i32,
        cl: i32,
    ) {
        let length = raw_rectangle_len(rows, cols);
        let lower = (i64::from(cl) + i64::from(rl) * cols as i64) as i32;
        // SAFETY: the caller guarantees `source` is readable for `rows * cols` initialized
        // values, as assign_trivial_from_raw_parts requires; it reads them before it releases
        // the old storage.
        unsafe {
            self.elements
                .assign_trivial_from_raw_parts(source, length, lower)
        };
        (self.rows, self.cols, self.row_lower, self.col_lower) = (rows, cols, rl, cl);
    }
}

fn raw_rectangle_len(rows: usize, cols: usize) -> usize {
    rows.checked_mul(cols).unwrap_or_else(|| {
        panic!("array dimensions ({rows}, {cols}) exceed the expected usize product")
    })
}
