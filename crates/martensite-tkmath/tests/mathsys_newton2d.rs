//! Mathsys newton2d tests.
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    clippy::field_reassign_with_default,
    clippy::needless_range_loop
)]
use martensite_tkmath::{math_sys::*, math_utils::Status};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b}");
}
include!("mathsys_newton2d/fixtures.rs");
include!("mathsys_newton2d/test_0.rs");
include!("mathsys_newton2d/test_1.rs");
include!("mathsys_newton2d/test_2.rs");
include!("mathsys_newton2d/test_3.rs");
include!("mathsys_newton2d/test_4.rs");
include!("mathsys_newton2d/test_5.rs");
include!("mathsys_newton2d/test_6.rs");
include!("mathsys_newton2d/test_7.rs");
include!("mathsys_newton2d/test_8.rs");
include!("mathsys_newton2d/test_9.rs");
include!("mathsys_newton2d/test_10.rs");
include!("mathsys_newton2d/test_11.rs");
