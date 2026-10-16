//! `math_opt::utils` helpers of the Newton, conjugate gradient and other minimizers.
//!
//! The helper namespace is public API; it maps to this public module. Overloads
//! that differ by argument type get distinct names (`is_finite_vector`/`is_finite_matrix`,
//! `scalar_value_status`/`value_status`).
use crate::math::{Matrix, Vector};

mod bounded;
mod checks;
mod evaluation;
mod hessian;
mod line;
mod stationary;
pub use bounded::BoundedFunction;
pub use checks::{is_finite_matrix, is_finite_vector, is_valid_bounds, is_valid_config, norm};
pub use evaluation::{
    CheckedFunction, checked_gradient, checked_hessian, checked_value, gradient_status,
    hessian_status, scalar_value_status, value_status,
};
pub use hessian::regularize_hessian;
pub(crate) use line::failure_after_fallback;
pub use line::{
    backtrack, is_line_search_evaluation_error, line_search_failure_status,
    line_search_fallback_failure_status,
};
pub use stationary::is_stationary;
pub(crate) use stationary::stationary_status;

/// Wolfe curvature constant of the FRPR line search; `math_opt::utils::THE_FRPR_WOLFE_CURVATURE`.
///
/// ```
/// assert_eq!(martensite_tkmath::math_opt::utils::FRPR_WOLFE_CURVATURE, 0.1);
/// ```
pub const FRPR_WOLFE_CURVATURE: f64 = 0.1;

/// Copies a vector into zero-based storage, as `Vector` copies keep only offsets.
pub(crate) fn zero_based(point: &Vector<'_>) -> Vector<'static> {
    let mut result: Vector<'static> = Vector::with_size(point.length() as usize);
    result.initialized(point).unwrap();
    result
}

/// Zero-based identity matrix of order `n`, the initial BFGS/Powell matrix.
pub(crate) fn identity(n: usize) -> Matrix<'static> {
    let mut matrix: Matrix<'static> =
        Matrix::filled(0, n as i32 - 1, 0, n as i32 - 1, 0.0).unwrap();
    for index in 0..n {
        *matrix.change_at(index, index).unwrap() = 1.0;
    }
    matrix
}

/// Largest absolute component difference, NaN-ignoring like the `<` update.
pub(crate) fn max_difference(a: &Vector<'_>, b: &Vector<'_>) -> f64 {
    let mut maximum: f64 = 0.0;
    for (left, right) in a.values().iter().zip(b.values()) {
        let diff: f64 = (left - right).abs();
        if maximum < diff {
            maximum = diff;
        }
    }
    maximum
}
