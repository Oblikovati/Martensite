use super::*;
fn triangle() -> [[f32; 4]; 3] {
    [[-1., -1., 0., 1.], [1., -1., 0., 1.], [0., 1., 0., 1.]]
}
fn plane(equation: [f32; 4]) -> ClipDepthHalfspace {
    ClipDepthHalfspace {
        equation,
        chain_length: 1,
    }
}
#[test]
fn unchanged_input_keeps_exact_bits_and_identity_weights() {
    let original = triangle();
    let result = clipped_depth_polygon(original, &[plane([0., 0., 0., 1.])]).unwrap();
    assert_eq!(result.iter().map(|v| v.clip).collect::<Vec<_>>(), original);
    assert_eq!(
        result.iter().map(|v| v.weights).collect::<Vec<_>>(),
        [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
    );
}
#[test]
fn exact_closedform_cut_keeps_source_order_and_interpolates_all_weights() {
    let result = clipped_depth_polygon(triangle(), &[plane([1., 0., 0., 0.])]).unwrap();
    assert_eq!(
        result.iter().map(|v| v.clip).collect::<Vec<_>>(),
        [
            [0., -1., 0., 1.],
            [1., -1., 0., 1.],
            [0., 1., 0., 1.],
            [0., 1., 0., 1.]
        ]
    );
    assert_eq!(result[0].weights, [0.5, 0.5, 0.]);
    assert_eq!(
        clipped_depth_last_fan(&result)[0],
        [result[1], result[2], result[0]]
    );
}
#[test]
fn intersection_chain_completes_empty_without_dropping_an_interior_triangle() {
    let empty = clipped_depth_polygon(triangle(), &[plane([1., 0., 0., -2.])]).unwrap();
    assert!(empty.is_empty());
    assert!(clipped_depth_last_fan(&empty).is_empty());
    let result = clipped_depth_polygon(
        triangle(),
        &[plane([1., 0., 0., 0.5]), plane([-1., 0., 0., 0.5])],
    )
    .unwrap();
    assert!(result.iter().all(|v| (-0.5..=0.5).contains(&v.clip[0])));
}
#[test]
fn unsupported_union_and_nan_planes_refuse_explicitly() {
    let union = ClipDepthHalfspace {
        equation: [1.; 4],
        chain_length: 2,
    };
    assert_eq!(
        clipped_depth_polygon(triangle(), &[union]),
        Err(ClipDepthFailure::UnionChain {
            index: 0,
            length: 2
        })
    );
    assert_eq!(
        clipped_depth_polygon(triangle(), &[plane([f32::NAN; 4])]),
        Err(ClipDepthFailure::InvalidPlane { index: 0 })
    );
}
#[test]
fn nonpositive_w_refuses_before_interpolation() {
    let mut invalid = triangle();
    invalid[1][3] = 0.;
    assert_eq!(
        clipped_depth_polygon(invalid, &[]),
        Err(ClipDepthFailure::InvalidVertex { index: 1 })
    );
}
#[test]
fn exact_boundary_and_generated_degenerate_fans_are_retained_for_raster_empty_validation() {
    let result = clipped_depth_polygon(triangle(), &[plane([0., 1., 0., -1.])]).unwrap();
    assert!(!result.is_empty());
    assert!(result.iter().all(|v| v.clip == [0., 1., 0., 1.]));
    assert!(!clipped_depth_last_fan(&result).is_empty());
}
#[test]
fn bounded_plane_and_arithmetic_exhaustion_never_return_partial_success() {
    assert_eq!(
        clipped_depth_polygon(triangle(), &[plane([0.; 4]); 15]),
        Err(ClipDepthFailure::PlaneLimit {
            actual: 15,
            maximum: 14
        })
    );
    let mut huge = triangle();
    huge[0][0] = f32::MAX;
    assert_eq!(
        clipped_depth_polygon(huge, &[plane([f32::MAX; 4])]),
        Err(ClipDepthFailure::Arithmetic)
    );
}
#[test]
fn perspective_intersection_interpolates_homogeneous_coordinates_before_division() {
    let clip = [[-2., 0., 0., 2.], [2., 0., 2., 4.], [0., 2., 0., 1.]];
    let result = clipped_depth_polygon(clip, &[plane([1., 0., 0., 0.])]).unwrap();
    assert_eq!(result[0].clip, [0., 0., 1., 3.]);
    assert_eq!(result[0].weights, [0.5, 0.5, 0.]);
    assert_eq!(result[0].clip[2] / result[0].clip[3], 1. / 3.);
    assert_ne!(result[0].clip[2] / result[0].clip[3], 0.25);
}
#[test]
fn ordered_two_plane_cut_matches_independent_quadrant_triangle() {
    let planes = [plane([1., 0., 0., 0.]), plane([0., 1., 0., 0.])];
    let result = clipped_depth_polygon(triangle(), &planes).unwrap();
    let points: Vec<_> = result.iter().map(|v| [v.clip[0], v.clip[1]]).collect();
    assert!(points.contains(&[0., 0.]));
    assert!(points.contains(&[0.5, 0.]));
    assert!(points.contains(&[0., 1.]));
    assert!(
        points
            .iter()
            .all(|p| p[0] >= 0. && p[1] >= 0. && 2. * p[0] + p[1] <= 1.)
    );
    assert_eq!(doubled_polygon_area(&points).abs(), 0.5);
}
#[test]
fn repeated_halfspace_preserves_exact_boundary_positions_and_source_weights() {
    let first = plane([1., 0., 0., 0.]);
    let once = clipped_depth_polygon(triangle(), &[first]).unwrap();
    let twice = clipped_depth_polygon(triangle(), &[first, first]).unwrap();
    assert_eq!(once, twice);
}

fn doubled_polygon_area(points: &[[f32; 2]]) -> f32 {
    points
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let b = points[(i + 1) % points.len()];
            a[0] * b[1] - a[1] * b[0]
        })
        .sum()
}
