//! Mathroot trig tests.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::{math_root::*, math_utils::Status};
use std::f64::consts::{PI, TAU};
const THE_TOL: f64 = 1e-10;
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b}");
}
fn verify_roots(result: &TrigResult, c: [f64; 5], tolerance: f64) {
    for i in 0..result.nb_roots {
        let x: f64 = result.roots[i];
        let cosine: f64 = martensite_cmath::cos(x);
        let sine: f64 = martensite_cmath::sin(x);
        let [a, b, c, d, e] = c;
        let value: f64 = a * cosine * cosine + 2.0 * b * cosine * sine + c * cosine + d * sine + e;
        near(value, 0.0, tolerance);
    }
}
include!("mathroot_trig/tests_0.rs");
include!("mathroot_trig/tests_1.rs");
include!("mathroot_trig/tests_2.rs");
include!("mathroot_trig/tests_3.rs");
include!("mathroot_trig/tests_4.rs");
include!("mathroot_trig/tests_5.rs");
include!("mathroot_trig/tests_6.rs");
include!("mathroot_trig/tests_7.rs");
include!("mathroot_trig/tests_8.rs");
include!("mathroot_trig/tests_9.rs");
include!("mathroot_trig/tests_10.rs");
include!("mathroot_trig/tests_11.rs");
include!("mathroot_trig/tests_12.rs");
include!("mathroot_trig/tests_13.rs");
include!("mathroot_trig/tests_14.rs");
include!("mathroot_trig/tests_15.rs");
include!("mathroot_trig/tests_16.rs");
