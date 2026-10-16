//! Bvh triangulation tests.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::bvh::{
    BvhSet, BvhTriangulation, BvhVec2d, BvhVec3d, BvhVec3f, BvhVec4i, array,
};

fn assert_near(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "{left} != {right} within {tolerance}"
    );
}

/// Float arguments of the near-equality checks are compared as doubles.
fn assert_near_f32(left: f32, right: f32, tolerance: f32) {
    assert_near(f64::from(left), f64::from(right), f64::from(tolerance));
}

include!("bvh_triangulation/suites_0.rs");
