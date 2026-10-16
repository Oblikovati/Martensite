//! Math uzawa tests.
#![allow(non_snake_case)]
use martensite_tkmath::math::{Matrix, Uzawa, Vector};
const TOLERANCE: f64 = 1e-6;
fn assert_near(left: f64, right: f64, tolerance: f64) {
    assert!((left - right).abs() <= tolerance, "{left} != {right}");
}
include!("math_uzawa/suites_0.rs");
include!("math_uzawa/suites_1.rs");
include!("math_uzawa/suites_2.rs");
include!("math_uzawa/suites_3.rs");
include!("math_uzawa/suites_4.rs");
