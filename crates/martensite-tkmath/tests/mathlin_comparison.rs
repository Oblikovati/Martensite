//! Mathlin comparison tests.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::math::{Gauss, Matrix, Vector};
use martensite_tkmath::math_lin::*;
const THE_TOLERANCE: f64 = 1e-10;
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b}");
}
include!("mathlin_comparison/tests_0.rs");
include!("mathlin_comparison/tests_1.rs");
include!("mathlin_comparison/tests_2.rs");
include!("mathlin_comparison/tests_3.rs");
