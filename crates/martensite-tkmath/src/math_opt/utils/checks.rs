//! Input validation predicates of `math_opt::utils`.
use crate::math::{Matrix, Vector};
use crate::math_utils::Config;

/// Accepts a solver configuration only with finite, strictly positive tolerances (the
/// relative tolerance may be zero) and a nonzero iteration budget; `math_opt::utils::IsValidConfig`.
///
/// ```
/// use martensite_tkmath::{math_opt::utils::is_valid_config, math_utils::Config};
/// assert!(is_valid_config(&Config::new(1e-8, 10)));
/// assert!(!is_valid_config(&Config::new(0.0, 10)));
/// ```
pub fn is_valid_config(config: &Config) -> bool {
    config.max_iterations > 0
        && config.tolerance.is_finite()
        && config.x_tolerance.is_finite()
        && config.f_tolerance.is_finite()
        && config.relative_tolerance.is_finite()
        && config.step_min.is_finite()
        && config.tolerance > 0.0
        && config.relative_tolerance >= 0.0
        && config.x_tolerance > 0.0
        && config.f_tolerance > 0.0
        && config.step_min > 0.0
}

/// True for a non-empty vector of finite components; `math_opt::utils::IsFinite(Vector)`.
///
/// ```
/// use martensite_tkmath::{math::Vector, math_opt::utils::is_finite_vector};
/// assert!(!is_finite_vector(&Vector::with_size(0)));
/// assert!(is_finite_vector(&Vector::with_size(2)));
/// ```
pub fn is_finite_vector(vector: &Vector<'_>) -> bool {
    vector.length() > 0 && vector.values().iter().all(|value| value.is_finite())
}

/// True for a non-empty matrix of finite cells; `math_opt::utils::IsFinite(Matrix)`.
///
/// ```
/// use martensite_tkmath::{math::Matrix, math_opt::utils::is_finite_matrix};
/// assert!(is_finite_matrix(&Matrix::filled(1, 2, 1, 2, 0.0).unwrap()));
/// ```
pub fn is_finite_matrix(matrix: &Matrix<'_>) -> bool {
    matrix.row_number() > 0
        && matrix.col_number() > 0
        && matrix.cells().iter().all(|v| v.is_finite())
}

/// Accepts equal-size, non-empty finite boxes with `lower < upper` and a finite width
/// per coordinate; `math_opt::utils::IsValidBounds`.
///
/// ```
/// use martensite_tkmath::{math::Vector, math_opt::utils::is_valid_bounds};
/// let lower = Vector::filled(1, 2, -1.0).unwrap();
/// let upper = Vector::filled(1, 2, 1.0).unwrap();
/// assert!(is_valid_bounds(&lower, &upper));
/// assert!(!is_valid_bounds(&upper, &lower));
/// ```
pub fn is_valid_bounds(lower: &Vector<'_>, upper: &Vector<'_>) -> bool {
    if lower.length() == 0 || upper.length() != lower.length() {
        return false;
    }
    lower.values().iter().zip(upper.values()).all(|(lo, hi)| {
        // `lo >= hi` is false for NaN, so the explicit finiteness checks carry the rejection.
        lo.is_finite() && hi.is_finite() && lo < hi && (hi - lo).is_finite()
    })
}

/// Overflow-safe Euclidean norm accumulated with `hypot` in index order;
/// `math_opt::utils::Norm`.
///
/// ```
/// use martensite_tkmath::{math::Vector, math_opt::utils::norm};
/// let mut v = Vector::with_size(2);
/// v[0] = 3.0;
/// v[1] = 4.0;
/// assert_eq!(norm(&v), 5.0);
/// ```
pub fn norm(vector: &Vector<'_>) -> f64 {
    let mut value: f64 = 0.0;
    for component in vector.values() {
        value = martensite_cmath::hypot(value, *component);
    }
    value
}
