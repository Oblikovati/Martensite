//! Gaussian elimination (LU) with scaled partial pivoting.
use crate::math::{Matrix, Vector};
use crate::math_utils::{InverseResult, LinearMultipleResult, LinearResult, Status};
use martensite_tkernel::ncollection::Array1;
/// LU storage and pivot diagnostics; `LuResult::default()`.
pub struct LuResult {
    /// Completion status; `result.status`.
    pub status: Status,
    /// Combined factors including partial factors on failure; `result.lu`.
    pub lu: Option<Matrix<'static>>,
    /// Zero-based row permutation sequence; `result.pivot`.
    pub pivot: Option<Array1<'static, i32>>,
    /// Absent on overflow or underflow; `result.determinant`.
    pub determinant: Option<f64>,
    /// Permutation parity; `result.sign`.
    pub sign: i32,
    /// Working lower row index; `result.row_lower`.
    pub row_lower: usize,
    /// Working lower column index; `result.col_lower`.
    pub col_lower: usize,
    /// Validated square dimension; `result.dimension`.
    pub dimension: usize,
    /// Smallest completed pivot magnitude; `result.min_abs_pivot`.
    pub min_abs_pivot: f64,
    /// Largest completed pivot magnitude; `result.max_abs_pivot`.
    pub max_abs_pivot: f64,
    /// Finite pivot ratio; `result.condition_estimate`.
    pub condition_estimate: Option<f64>,
}
impl Default for LuResult {
    fn default() -> Self {
        Self {
            status: Status::NotConverged,
            lu: None,
            pivot: None,
            determinant: None,
            sign: 1,
            row_lower: 0,
            col_lower: 0,
            dimension: 0,
            min_abs_pivot: 0.0,
            max_abs_pivot: 0.0,
            condition_estimate: None,
        }
    }
}
impl LuResult {
    /// Reports success; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
    fn initialize(&mut self, a: &Matrix<'_>) -> Result<Vector<'static>, Status> {
        let n: usize = a.row_number() as usize;
        self.dimension = n;
        let mut lu = Matrix::with_bounds(0, n as i32 - 1, 0, n as i32 - 1).unwrap();
        self.pivot = Some(Array1::with_size(n));
        let mut scales: Vector<'static> = Vector::with_size(n);
        let src = a.cells();
        let dst = lu.cells_mut();
        dst.copy_from_slice(src);
        let scale_slice = scales.values_mut();
        for (i, row) in dst.chunks_exact(n).enumerate() {
            let mut max_val = 0.0_f64;
            for &v in row {
                max_val = max_val.max(v.abs());
            }
            if max_val == 0.0 {
                return Err(Status::NumericalError);
            }
            scale_slice[i] = max_val;
        }
        self.lu = Some(lu);
        Ok(scales)
    }
    fn pivot_row(&self, k: i32, scales: &Vector<'_>) -> i32 {
        let n = self.dimension;
        let ku = k as usize;
        let cells = self.lu.as_ref().unwrap().cells();
        let scale_slice = scales.values();
        let mut maximum: f64 = 0.0;
        let mut row: usize = ku;
        for i in ku..n {
            let scaled: f64 = cells[i * n + ku].abs() / scale_slice[i];
            if scaled > maximum {
                maximum = scaled;
                row = i;
            }
        }
        row as i32
    }
    fn pivot_step(&mut self, k: i32, scales: &mut Vector<'_>, tolerance: f64) -> bool {
        let row: i32 = self.pivot_row(k, scales);
        let matrix: &mut Matrix<'_> = self.lu.as_mut().unwrap();
        if row != k {
            matrix.swap_row(row, k).unwrap();
            self.sign = -self.sign;
            let s: f64 = scales[row];
            scales[row] = scales[k];
            scales[k] = s;
        }
        self.pivot.as_mut().unwrap().as_mut_slice()[k as usize] = row;
        let pivot: f64 = matrix[(k, k)].abs();
        let scaled: f64 = pivot / scales[k];
        if !pivot.is_finite() || !scaled.is_finite() || scaled <= tolerance {
            return false;
        }
        self.max_abs_pivot = self.max_abs_pivot.max(pivot);
        self.min_abs_pivot = if k == 0 {
            pivot
        } else {
            self.min_abs_pivot.min(pivot)
        };
        eliminate(matrix, k);
        true
    }
    fn diagnostics(&mut self) {
        let matrix: &Matrix<'_> = self.lu.as_ref().unwrap();
        let mut determinant: f64 = f64::from(self.sign);
        self.determinant = Some(determinant);
        for i in 0..self.dimension as i32 {
            let pivot: f64 = matrix[(i, i)];
            determinant *= pivot;
            if !determinant.is_finite() || (determinant == 0.0 && pivot != 0.0) {
                self.determinant = None;
                break;
            }
            self.determinant = Some(determinant);
        }
        let estimate: f64 = self.max_abs_pivot / self.min_abs_pivot;
        if estimate.is_finite() {
            self.condition_estimate = Some(estimate);
        }
    }
    fn solve_column(&self, x: &mut Vector<'_>) {
        let cells = self.lu.as_ref().unwrap().cells();
        let pivots: &[i32] = self.pivot.as_ref().unwrap().as_slice();
        let n: usize = self.dimension;
        let xv = x.values_mut();
        for i in 0..n {
            let pivot: usize = pivots[i] as usize;
            let mut sum: f64 = xv[pivot];
            xv[pivot] = xv[i];
            let row = &cells[i * n..i * n + i];
            for j in 0..i {
                sum -= row[j] * xv[j];
            }
            xv[i] = sum;
        }
        for i in (0..n).rev() {
            let mut sum: f64 = xv[i];
            let row = &cells[i * n..i * n + n];
            for j in i + 1..n {
                sum -= row[j] * xv[j];
            }
            xv[i] = sum / row[i];
        }
    }
}
impl From<&LuResult> for bool {
    fn from(result: &LuResult) -> bool {
        result.is_done()
    }
}
fn eliminate(matrix: &mut Matrix<'_>, k: i32) {
    let n = matrix.col_number() as usize;
    let ku = k as usize;
    let cells = matrix.cells_mut();
    let (top, bottom) = cells.split_at_mut((ku + 1) * n);
    let pivot_row = &top[ku * n..ku * n + n];
    let inv_pivot = 1.0 / pivot_row[ku];
    let pivot_tail = &pivot_row[ku + 1..];
    for row in bottom.chunks_exact_mut(n) {
        let factor = row[ku] * inv_pivot;
        row[ku] = factor;
        let row_tail = &mut row[ku + 1..];
        for (dst, &src) in row_tail.iter_mut().zip(pivot_tail.iter()) {
            *dst -= factor * src;
        }
    }
}
/// Computes scaled partial pivoting; `lu(&a,1e-15)`.
pub fn lu(a: &Matrix<'_>, tolerance: f64) -> LuResult {
    let n: i32 = a.row_number();
    let relative: f64 = super::utils::relative_tolerance(tolerance, n as usize);
    let mut result: LuResult = LuResult::default();
    if n != a.col_number() || n == 0 || relative < 0.0 || !super::jacobi::finite_matrix(a) {
        result.status = Status::InvalidInput;
        return result;
    }
    let mut scales: Vector<'static> = match result.initialize(a) {
        Ok(scales) => scales,
        Err(status) => {
            result.status = status;
            return result;
        }
    };
    for k in 0..n {
        if !result.pivot_step(k, &mut scales, relative) {
            result.status = Status::NumericalError;
            return result;
        }
    }
    result.diagnostics();
    result.status = Status::Ok;
    result
}
/// Solves with permutation and partial-result semantics; `solve(&a,&b,1e-15)`.
pub fn solve(a: &Matrix<'_>, b: &Vector<'_>, tolerance: f64) -> LinearResult {
    let factors: LuResult = lu(a, tolerance);
    if !factors.is_done() {
        return LinearResult {
            status: factors.status,
            ..Default::default()
        };
    }
    if b.length() != a.row_number() || !b.values().iter().all(|x| x.is_finite()) {
        return LinearResult {
            status: Status::InvalidInput,
            ..Default::default()
        };
    }
    let mut x: Vector<'static> = Vector::with_size(factors.dimension);
    x.initialized(b).unwrap();
    factors.solve_column(&mut x);
    let status: Status = if x.values().iter().all(|v| v.is_finite()) {
        Status::Ok
    } else {
        Status::NumericalError
    };
    LinearResult {
        status,
        solution: Some(x),
        determinant: factors.determinant,
    }
}
/// Retains determinant overflow as an absent diagnostic; `solve_multiple(&a,&b,1e-15)`.
pub fn solve_multiple(a: &Matrix<'_>, b: &Matrix<'_>, tolerance: f64) -> LinearMultipleResult {
    let factors: LuResult = lu(a, tolerance);
    if !factors.is_done() {
        return LinearMultipleResult {
            status: factors.status,
            ..Default::default()
        };
    }
    if b.row_number() != a.row_number() || !super::jacobi::finite_matrix(b) {
        return LinearMultipleResult {
            status: Status::InvalidInput,
            ..Default::default()
        };
    }
    let mut x: Matrix<'static> =
        Matrix::with_bounds(0, a.col_number() - 1, 0, b.col_number() - 1).unwrap();
    for col in 0..b.col_number() {
        let mut work: Vector<'static> = Vector::with_size(factors.dimension);
        for i in 0..factors.dimension {
            *work.change_at(i).unwrap() = *b.at(i, col as usize).unwrap();
        }
        factors.solve_column(&mut work);
        for i in 0..a.row_number() {
            x[(i, col)] = work[i];
        }
    }
    let status: Status = if super::jacobi::finite_matrix(&x) {
        Status::Ok
    } else {
        Status::NumericalError
    };
    LinearMultipleResult {
        status,
        solutions: Some(x),
        determinant: factors.determinant,
    }
}
/// Rejects unrepresentable determinants independently of solvability; `determinant(&a,1e-15)`.
pub fn determinant(a: &Matrix<'_>, tolerance: f64) -> LinearResult {
    let factors: LuResult = lu(a, tolerance);
    if !factors.is_done() {
        return LinearResult {
            status: factors.status,
            ..Default::default()
        };
    }
    let status: Status = if factors.determinant.is_some() {
        Status::Ok
    } else {
        Status::NumericalError
    };
    LinearResult {
        status,
        determinant: factors.determinant,
        ..Default::default()
    }
}
/// Inverts column by column with accumulation order; `invert(&a,1e-15)`.
pub fn invert(a: &Matrix<'_>, tolerance: f64) -> InverseResult {
    let factors: LuResult = lu(a, tolerance);
    if !factors.is_done() {
        return InverseResult {
            status: factors.status,
            ..Default::default()
        };
    }
    let n: i32 = a.row_number();
    let mut inverse: Matrix<'static> = Matrix::filled(0, n - 1, 0, n - 1, 0.0).unwrap();
    let mut column: Vector<'static> = Vector::with_size(n as usize);
    for col in 0..n {
        column.init(0.0);
        column[col] = 1.0;
        factors.solve_column(&mut column);
        for i in 0..n {
            inverse[(i, col)] = column[i];
        }
    }
    let status: Status = if super::jacobi::finite_matrix(&inverse) {
        Status::Ok
    } else {
        Status::NumericalError
    };
    InverseResult {
        status,
        inverse: Some(inverse),
        determinant: factors.determinant,
    }
}
