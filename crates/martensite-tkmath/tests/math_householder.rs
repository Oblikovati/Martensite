//! Math householder tests.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::math::{Householder, Matrix, Vector};
fn assert_near(left: f64, right: f64, tolerance: f64) {
    assert!((left - right).abs() <= tolerance, "{left} != {right}");
}
include!("math_householder/suites_0.rs");
include!("math_householder/suites_1.rs");
include!("math_householder/suites_2.rs");
include!("math_householder/suites_3.rs");
