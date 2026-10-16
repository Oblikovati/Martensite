//! Bvh tools tests.
//!
//! Uninitialized `aTimeEnter`/`aTimeLeave` locals start as NaN: they are only
//! writes them on a hit, and NaN keeps every comparison on an unwritten value false.
#![allow(non_snake_case, unused_mut)]
use martensite_tkernel::precision;
use martensite_tkmath::bvh::tools::{
    box_box_square_distance, point_box_projection, point_box_square_distance,
    point_triangle_projection, ray_box_intersection_by_direction,
};
use martensite_tkmath::bvh::{BvhBox, BvhVec2d, BvhVec3d, BvhVec3f, BvhVector};

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

/// `RayBoxIntersection(origin, direction, box, enter, leave)`: the outputs are
/// written only on a hit.
fn RayBoxIntersection<V: BvhVector>(
    origin: V,
    direction: V,
    bounding: &BvhBox<V>,
    enter: &mut V::Scalar,
    leave: &mut V::Scalar,
) -> bool {
    let Some((time_enter, time_leave)) =
        ray_box_intersection_by_direction(&origin, &direction, bounding)
    else {
        return false;
    };
    *enter = time_enter;
    *leave = time_leave;
    true
}

/// `PointTriangleProjection` without the optional state outputs.
fn PointTriangleProjection<V: BvhVector>(point: V, node0: V, node1: V, node2: V) -> V {
    point_triangle_projection(&point, &node0, &node1, &node2, None)
}

include!("bvh_tools/suites_0.rs");
include!("bvh_tools/suites_1.rs");
