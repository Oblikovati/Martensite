//! Checked access shared by the polynomial library functions.
//!
//! Array accesses through `Array1/2::operator()` raise out-of-range failure in
//! checked builds, and raw `double&` buffers are read without checks.
//! These helpers turn both into `Failure` values that name the offending index.
use martensite_tkernel::ncollection::{Array1, Array2, ArrayError};
use martensite_tkernel::standard::Failure;

/// Builds a typed exception; `failure("construction failure", "bad")`.
pub(crate) fn failure(exception_type: &'static str, message: &str) -> Failure {
    Failure::with_type(exception_type, message, "")
}

fn index_failure(error: ArrayError) -> Failure {
    Failure::out_of_range(&format!("NCollection_Array: {error}"))
}

/// Reads a checked Array1 element; `array_value(&array, 1)?`.
pub(crate) fn array_value<T: Copy>(array: &Array1<'_, T>, index: i32) -> Result<T, Failure> {
    array.value(index).copied().map_err(index_failure)
}

/// Borrows a checked Array1 element mutably; `*array_cell(&mut array, 1)? = 2.0`.
pub(crate) fn array_cell<'b, T>(
    array: &'b mut Array1<'_, T>,
    index: i32,
) -> Result<&'b mut T, Failure> {
    array.change_value(index).map_err(index_failure)
}

/// Reads a checked Array2 element; `grid_value(&grid, 1, 1)?`.
pub(crate) fn grid_value<T: Copy>(grid: &Array2<'_, T>, row: i32, col: i32) -> Result<T, Failure> {
    grid.value(row, col).copied().map_err(index_failure)
}

/// Borrows a checked Array2 element mutably; `*grid_cell(&mut grid, 1, 1)? = 2.0`.
pub(crate) fn grid_cell<'b, T>(
    grid: &'b mut Array2<'_, T>,
    row: i32,
    col: i32,
) -> Result<&'b mut T, Failure> {
    grid.change_value(row, col).map_err(index_failure)
}

/// Rejects a negative count that would make pointer arithmetic undefined;
/// `non_negative("plib::eval_polynomial", "Degree", degree)?`.
pub(crate) fn non_negative(function: &str, name: &str, value: i32) -> Result<usize, Failure> {
    usize::try_from(value)
        .map_err(|_| Failure::out_of_range(&format!("{function}: {name} = {value}; expected >= 0")))
}

/// Rejects a raw buffer shorter than the access pattern reads or writes;
/// `require_length("plib::eval_polynomial", "Results", results.len(), 3)?`.
pub(crate) fn require_length(
    function: &str,
    name: &str,
    length: usize,
    needed: usize,
) -> Result<(), Failure> {
    if length >= needed {
        return Ok(());
    }
    Err(Failure::out_of_range(&format!(
        "{function}: {name} holds {length} values; expected at least {needed}"
    )))
}

/// Rejects derivative counts outside the D0–D3 protected contract.
pub(crate) fn validate_derivative_order(function: &str, order: i32) -> Result<(), Failure> {
    if (0..=3).contains(&order) {
        return Ok(());
    }
    Err(Failure::out_of_range(&format!(
        "{function}: NDeriv = {order}; expected 0..=3"
    )))
}

/// Reads a raw-buffer entry addressed with signed arithmetic;
/// `buffer_value("JacobiPolynomial::max_error", coefficients, 7)?`.
pub(crate) fn buffer_value(function: &str, values: &[f64], index: i64) -> Result<f64, Failure> {
    usize::try_from(index)
        .ok()
        .and_then(|offset| values.get(offset).copied())
        .ok_or_else(|| {
            Failure::out_of_range(&format!(
                "{function}: index {index} outside a buffer of {} values",
                values.len()
            ))
        })
}
