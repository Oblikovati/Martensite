//! Math gauss tests.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::math::{Gauss, Matrix, Vector};
fn assert_near(left: f64, right: f64, tolerance: f64) {
    assert!((left - right).abs() <= tolerance, "{left} != {right}");
}
include!("math_gauss/suites_0.rs");
include!("math_gauss/suites_1.rs");
include!("math_gauss/suites_2.rs");
include!("math_gauss/suites_3.rs");
include!("math_gauss/suites_4.rs");
