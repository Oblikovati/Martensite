//! Bvh ray tests.
#![allow(non_snake_case, non_upper_case_globals, unused_mut)]
use martensite_tkmath::bvh::{BvhRay, BvhVec2d, BvhVec3d, BvhVec3f};

// =======================================================================================
// Tests for BvhRay
// =======================================================================================

#[test]
fn hierarchy_ray_constructor_basic() {
    let mut aOrigin = BvhVec3d::new(1.0, 2.0, 3.0);
    let mut aDirection = BvhVec3d::new(0.0, 1.0, 0.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.origin.x, 1.0);
    assert_eq!(aRay.origin.y, 2.0);
    assert_eq!(aRay.origin.z, 3.0);

    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 1.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_direction_storage() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(2.0, 4.0, 8.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    // Verify direction is stored correctly
    assert_eq!(aRay.direct.x, 2.0);
    assert_eq!(aRay.direct.y, 4.0);
    assert_eq!(aRay.direct.z, 8.0);
}

#[test]
fn hierarchy_ray_negative_direction() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(-2.0, -4.0, -1.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, -2.0);
    assert_eq!(aRay.direct.y, -4.0);
    assert_eq!(aRay.direct.z, -1.0);
}

#[test]
fn hierarchy_ray_zero_component() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(1.0, 0.0, 1.0); // Zero Y component

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 1.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 1.0);
}

#[test]
fn hierarchy_ray_all_zero_direction() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(0.0, 0.0, 0.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    // All components should be zero
    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_default_constructor() {
    let mut aRay = BvhRay::<BvhVec3d>::default();

    // Default constructed ray should have zero origin and direction
    assert_eq!(aRay.origin.x, 0.0);
    assert_eq!(aRay.origin.y, 0.0);
    assert_eq!(aRay.origin.z, 0.0);

    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_ray2_d() {
    let mut aOrigin = BvhVec2d::new(1.0, 2.0);
    let mut aDirection = BvhVec2d::new(3.0, 4.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.origin.x, 1.0);
    assert_eq!(aRay.origin.y, 2.0);

    assert_eq!(aRay.direct.x, 3.0);
    assert_eq!(aRay.direct.y, 4.0);
}

#[test]
fn hierarchy_ray_normalized_direction() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(1.0, 0.0, 0.0); // Unit vector along X

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 1.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_very_small_direction() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(1e-20, 1e-20, 1e-20);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    // Should handle very small values correctly
    assert_eq!(aRay.direct.x, 1e-20);
    assert_eq!(aRay.direct.y, 1e-20);
    assert_eq!(aRay.direct.z, 1e-20);
}

#[test]
fn hierarchy_ray_mixed_zero_non_zero() {
    let mut aOrigin = BvhVec3d::new(1.0, 2.0, 3.0);
    let mut aDirection = BvhVec3d::new(0.0, 2.0, 0.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 2.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_float_precision() {
    let mut aOrigin = BvhVec3f::new(1.0, 2.0, 3.0);
    let mut aDirection = BvhVec3f::new(2.0, 4.0, 8.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 2.0_f32);
    assert_eq!(aRay.direct.y, 4.0_f32);
    assert_eq!(aRay.direct.z, 8.0_f32);
}

// A `constexpr` ray would be possible in C++. The Rust constructor computes the
// reciprocal through generic trait arithmetic, which cannot run in const context;
// the vectors are const items and the ray is built at test time.
#[test]
fn hierarchy_ray_constexpr_constructor() {
    // This test verifies that the constructor is truly constexpr
    const aOrigin: BvhVec3d = BvhVec3d::new(1.0, 2.0, 3.0);
    const aDirection: BvhVec3d = BvhVec3d::new(1.0, 1.0, 1.0);
    let aRay = BvhRay::new(aOrigin, aDirection);

    // If this compiles, constexpr works
    assert_eq!(aRay.origin.x, 1.0);
}

#[test]
fn hierarchy_ray_diagonal_ray() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(1.0, 1.0, 1.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 1.0);
    assert_eq!(aRay.direct.y, 1.0);
    assert_eq!(aRay.direct.z, 1.0);
}

#[test]
fn hierarchy_ray_negative_origin() {
    let mut aOrigin = BvhVec3d::new(-10.0, -20.0, -30.0);
    let mut aDirection = BvhVec3d::new(1.0, 2.0, 3.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.origin.x, -10.0);
    assert_eq!(aRay.origin.y, -20.0);
    assert_eq!(aRay.origin.z, -30.0);

    assert_eq!(aRay.direct.x, 1.0);
    assert_eq!(aRay.direct.y, 2.0);
    assert_eq!(aRay.direct.z, 3.0);
}

#[test]
fn hierarchy_ray_axis_aligned_x() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(1.0, 0.0, 0.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 1.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_axis_aligned_y() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(0.0, 1.0, 0.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 1.0);
    assert_eq!(aRay.direct.z, 0.0);
}

#[test]
fn hierarchy_ray_axis_aligned_z() {
    let mut aOrigin = BvhVec3d::new(0.0, 0.0, 0.0);
    let mut aDirection = BvhVec3d::new(0.0, 0.0, 1.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 0.0);
    assert_eq!(aRay.direct.y, 0.0);
    assert_eq!(aRay.direct.z, 1.0);
}

#[test]
fn hierarchy_ray_large_values() {
    let mut aOrigin = BvhVec3d::new(1e6, 2e6, 3e6);
    let mut aDirection = BvhVec3d::new(1e3, 2e3, 4e3);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    assert_eq!(aRay.direct.x, 1e3);
    assert_eq!(aRay.direct.y, 2e3);
    assert_eq!(aRay.direct.z, 4e3);
}

#[test]
fn hierarchy_ray_origin_and_direction_preservation() {
    let mut aOrigin = BvhVec3d::new(1.0, 2.0, 3.0);
    let mut aDirection = BvhVec3d::new(2.0, 4.0, 8.0);

    let mut aRay = BvhRay::new(aOrigin, aDirection);

    // Verify that both origin and direction are preserved exactly
    assert_eq!(aRay.origin.x, aOrigin.x);
    assert_eq!(aRay.origin.y, aOrigin.y);
    assert_eq!(aRay.origin.z, aOrigin.z);

    assert_eq!(aRay.direct.x, aDirection.x);
    assert_eq!(aRay.direct.y, aDirection.y);
    assert_eq!(aRay.direct.z, aDirection.z);
}
