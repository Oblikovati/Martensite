//! polynomial library: polynomial evaluation, interpolation and basis conversion.
//!
//! `double&` buffer arguments become slices. Where a raw access would read or
//! write outside such a buffer, or where a negative degree, dimension or
//! derivative count makes the pointer arithmetic undefined, the call returns a
//! out-of-range failure `Failure` instead. Exceptions keep their type.
//! Overloads on Point3/Point2 share one generic function over [`PolePoint`];
//! overloads on real arrays carry a `_scalar` or `_flat` suffix.
pub(crate) mod access;
mod binomial;
mod coefficients_poles;
mod constraint;
mod hermit_jacobi;
mod hermite;
mod horner;
mod interpolation;
mod jacobi_polynomial;
mod jacobi_tables;
mod length;
mod poles;
mod rational;
mod trimming;

pub use binomial::{MAX_BINOMIAL_DEGREE, bin};
pub use coefficients_poles::{
    coefficients_poles, coefficients_poles_flat, coefficients_poles_scalar,
};
pub use constraint::{constraint_order, jacobi_parameters, niv_constr};
pub use hermit_jacobi::HermitJacobi;
pub use hermite::{hermite_coefficients, hermite_interpolate};
pub use horner::{eval_poly_2var, eval_polynomial, no_derivative_eval_polynomial};
pub use interpolation::{eval_cubic_hermite, eval_lagrange};
pub use jacobi_polynomial::BasisRows;
pub use jacobi_polynomial::JacobiPolynomial;
pub use length::{eval_length, eval_length_with_tolerance};
pub use poles::{PolePoint, get_poles, get_rational_poles, set_poles, set_rational_poles};
pub use rational::{rational_derivative, rational_derivatives};
pub use trimming::{trimming, trimming_flat, trimming_scalar, u_trimming, v_trimming};

mod grid_poles;
pub use grid_poles::coefficients_poles_grid;

use martensite_tkernel::ncollection::{Array1, Array2};

/// The absent weight array of non-rational calls (`plib::no_weights`);
/// `plib::no_weights().is_none()`.
pub const fn no_weights<'r, 'a>() -> Option<&'r mut Array1<'a, f64>> {
    None
}

/// The absent weight grid of non-rational calls (`plib::no_weights2`);
/// `plib::no_weights2().is_none()`.
pub const fn no_weights2<'r, 'a>() -> Option<&'r mut Array2<'a, f64>> {
    None
}
