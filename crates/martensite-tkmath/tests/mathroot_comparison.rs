//! Mathroot comparison tests.
#![allow(
    non_snake_case,
    unused_mut,
    clippy::field_reassign_with_default,
    clippy::default_constructed_unit_structs
)]
use martensite_tkmath::math::{
    BissecNewton, BracketedRoot, Function, FunctionRoot, FunctionWithDerivative, NewtonFunctionRoot,
};
use martensite_tkmath::math_root::*;
use martensite_tkmath::math_utils::{Config, ScalarResult};
const THE_TOLERANCE: f64 = 1e-9;
fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}
include!("mathroot/fixture_0.rs");
include!("mathroot/fixture_1.rs");
include!("mathroot/fixture_2.rs");
include!("mathroot/fixture_3.rs");
include!("mathroot/fixture_4.rs");
include!("mathroot_comparison/tests_0.rs");
include!("mathroot_comparison/tests_1.rs");
include!("mathroot_comparison/tests_2.rs");
