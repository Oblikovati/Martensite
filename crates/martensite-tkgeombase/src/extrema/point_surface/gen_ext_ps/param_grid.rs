// GenExtPS at .

//! The grids of points on surface of GenExtPS, with the storage rules of
//! Array2: a resize without copy keeps the old cells when the flat size
//! does not change (only the bounds move), and an access outside the bounds raises
//! index-range failure.
use crate::extrema::pon::POnSurfParams;
use crate::native_failure::failure;
use martensite_tkernel::standard::Failure;

/// A row-major 2D array of points on surface with retained bounds.
#[derive(Default)]
pub(super) struct ParamGrid {
    lower_row: i32,
    lower_col: i32,
    rows: usize,
    cols: usize,
    cells: Vec<POnSurfParams>,
}

impl ParamGrid {
    /// `Resize(theRowLower, theRowUpper, theColLower, theColUpper, false)`.
    pub fn resize(
        &mut self,
        (row_lower, row_upper): (i32, i32),
        (col_lower, col_upper): (i32, i32),
    ) -> Result<(), Failure> {
        if row_upper < row_lower || col_upper < col_lower {
            return Err(failure(
                "Standard_RangeError",
                &format!(
                    "NCollection_Array2::Resize - bounds ({row_lower}..{row_upper}, {col_lower}..{col_upper}); expected lower <= upper"
                ),
            ));
        }
        let rows: usize = (row_upper - row_lower + 1) as usize;
        let cols: usize = (col_upper - col_lower + 1) as usize;
        if rows * cols != self.cells.len() {
            self.cells = vec![POnSurfParams::default(); rows * cols];
        }
        (self.lower_row, self.lower_col, self.rows, self.cols) = (row_lower, col_lower, rows, cols);
        Ok(())
    }
    // The flat offset of `(row, col)`, index-range failure outside the bounds.
    fn offset(&self, row: i32, col: i32) -> Result<usize, Failure> {
        let (dr, dc) = (
            i64::from(row) - i64::from(self.lower_row),
            i64::from(col) - i64::from(self.lower_col),
        );
        if dr < 0 || dc < 0 || dr as usize >= self.rows || dc as usize >= self.cols {
            return Err(failure(
                "Standard_OutOfRange",
                &format!(
                    "NCollection_Array2::Value - index ({row}, {col}) is outside ({}..{}, {}..{})",
                    self.lower_row,
                    self.lower_row + self.rows as i32 - 1,
                    self.lower_col,
                    self.lower_col + self.cols as i32 - 1
                ),
            ));
        }
        Ok(dr as usize * self.cols + dc as usize)
    }
    /// Borrows all columns of one initialized row in storage order.
    pub(super) fn row(&self, row: i32) -> Result<&[POnSurfParams], Failure> {
        let start = self.offset(row, self.lower_col)?;
        Ok(&self.cells[start..start + self.cols])
    }

    /// Borrows all columns of one initialized row in storage order.
    pub(super) fn row_mut(&mut self, row: i32) -> Result<&mut [POnSurfParams], Failure> {
        let start = self.offset(row, self.lower_col)?;
        Ok(&mut self.cells[start..start + self.cols])
    }

    /// `Value(row, col)`.
    pub fn value(&self, row: i32, col: i32) -> Result<&POnSurfParams, Failure> {
        let offset: usize = self.offset(row, col)?;
        Ok(&self.cells[offset])
    }
    /// `ChangeValue(row, col)`.
    pub fn change_value(&mut self, row: i32, col: i32) -> Result<&mut POnSurfParams, Failure> {
        let offset: usize = self.offset(row, col)?;
        Ok(&mut self.cells[offset])
    }
    /// `SetValue(row, col, item)`.
    pub fn set_value(&mut self, row: i32, col: i32, item: POnSurfParams) -> Result<(), Failure> {
        *self.change_value(row, col)? = item;
        Ok(())
    }
    /// The square distance at `(row, col)`.
    pub fn sqr_distance(&self, row: i32, col: i32) -> Result<f64, Failure> {
        Ok(self.value(row, col)?.get_sqr_distance())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_rows_match_shifted_scalar_indexing_and_share_cells() {
        let mut grid = ParamGrid::default();
        grid.resize((-2, -1), (5, 7)).unwrap();
        for (col, param) in grid.row_mut(-1).unwrap().iter_mut().enumerate() {
            param.set_indices(-1, col as i32 + 5);
        }
        assert_eq!(grid.row(-1).unwrap().len(), 3);
        for col in 5..=7 {
            assert_eq!(grid.value(-1, col).unwrap().get_indices(), (-1, col));
        }
        assert_eq!(grid.row(-2).unwrap()[0], POnSurfParams::default());
        assert_eq!(
            grid.row(0).unwrap_err().exception_type(),
            grid.value(0, 5).unwrap_err().exception_type()
        );
    }
}
