use super::*;
use crate::clipped_depth_plane::{framebuffer_depth_plane, framebuffer_depth_sample};
#[test]
fn sole_affine_plane_is_defined_beyond_every_original_triangle_edge() {
    let triangle = [[0, 0], [4, 0], [0, 4]];
    for x in -8..=12 {
        for y in -8..=12 {
            assert_eq!(selected_plane(&triangle, [x, y]), Ok(0));
        }
    }
    assert_eq!(selected(&triangle, [8, 8]), Ok(None));
}
fn split_expected(kind: usize, point: Point, root: Point) -> usize {
    if point == root {
        return 0;
    }
    let first = match kind {
        0 => point[0] >= point[1],
        1 => point[0] + point[1] >= 4,
        2 => point[1] > point[0],
        _ => point[0] + point[1] < 4,
    };
    usize::from(!first)
}
#[test]
fn two_planes_have_one_global_separator_for_every_winding_and_viewport_sample() {
    let polygons = [
        [[0, 0], [4, 0], [4, 4], [0, 4]],
        [[0, 4], [4, 4], [4, 0], [0, 0]],
        [[4, 4], [0, 4], [0, 0], [4, 0]],
        [[4, 0], [0, 0], [0, 4], [4, 4]],
    ];
    for (kind, polygon) in polygons.iter().enumerate() {
        prove_global_separator(kind, polygon);
    }
}
fn prove_global_separator(kind: usize, polygon: &[Point; 4]) {
    for x in -8..=12 {
        for y in -8..=12 {
            assert_eq!(
                selected_plane(polygon, [x, y]),
                Ok(split_expected(kind, [x, y], polygon[0]))
            );
        }
    }
}

#[test]
fn folded_zero_area_and_more_than_two_fans_refuse_instead_of_choosing_an_owner() {
    for polygon in [
        vec![[0, 0], [1, 1], [2, 2]],
        vec![[0, 0], [4, 0], [4, 4], [4, 4]],
        vec![[0, 0], [4, 0], [4, 4], [0, 4], [0, 2]],
        vec![
            [50827, 148696],
            [55425, 152105],
            [55243, 151607],
            [50826, 148696],
        ],
    ] {
        assert_eq!(
            selected_plane(&polygon, polygon[0]),
            Err(PartitionFailure::PlaneDomain)
        );
    }
}
#[test]
fn finite_viewport_extrapolation_has_independent_closed_form_depth_and_corner_bounds() {
    let plane = framebuffer_depth_plane([[0., 0., 0.25], [4., 0., 0.75], [0., 4., 0.]]).unwrap();
    prove_affine_viewport(plane);
    let mut overflowing = plane;
    overflowing.gradient = [f32::MAX; 2];
    assert!(framebuffer_depth_sample(overflowing, [8., 8.]).is_err());
}
fn prove_affine_viewport(plane: crate::clipped_depth_plane::FramebufferDepthPlane) {
    for x in 0..=8 {
        for y in 0..=8 {
            let expected = 0.25 + x as f32 * 0.125 - y as f32 * 0.0625;
            assert_eq!(
                framebuffer_depth_sample(plane, [x as f32, y as f32]).unwrap(),
                expected
            );
        }
    }
}

#[test]
fn actual_covered_sample_outside_reference_grid_still_has_its_sole_finite_plane() {
    let physical = [[129103, 77226], [128369, 77147], [128391, 77572]];
    let reference = [[2065644, 1235620], [2053905, 1234356], [2054254, 1241147]];
    assert_eq!(selected(&physical, [128432, 77552]), Ok(Some(0)));
    assert_eq!(selected(&reference, [2054912, 1240832]), Ok(None));
    assert_eq!(selected_plane(&reference, [2054912, 1240832]), Ok(0));
}
