use super::*;

fn point(value: f32) -> DepthShaderBound {
    DepthShaderBound::source(value).unwrap()
}
fn contains(bound: DepthShaderBound, value: f32) -> bool {
    bound.minimum <= f64::from(value) && f64::from(value) <= bound.maximum
}

#[test]
fn cancellation_encloses_separate_and_contracted_operations() {
    let (first, second, third) = (4097_f32, 4097_f32, -16_785_408_f32);
    let bound = point(first)
        .multiply(point(second))
        .unwrap()
        .add(point(third))
        .unwrap();
    assert!(contains(bound, first * second + third));
    assert!(contains(bound, first.mul_add(second, third)));
    assert!(contains(point(1.).subtract(point(1.)).unwrap(), 0.));
}

#[test]
fn normal_division_encloses_rounding_and_optional_flush_to_zero() {
    let bound = point(1.).divide(point(3.)).unwrap();
    assert!(contains(bound, 1_f32 / 3.));
    let underflow = point(f32::MIN_POSITIVE).multiply(point(0.25)).unwrap();
    assert!(contains(underflow, f32::MIN_POSITIVE * 0.25));
    assert!(contains(underflow, 0.));
    assert_eq!(point(f32::MIN_POSITIVE / 2.).endpoints()[0], 0.);
}

#[test]
fn snapped_hull_covers_both_sides_of_a_half_cell() {
    let half = 0.5 / 4096.;
    let interval = DepthShaderBound {
        minimum: half - 1e-10,
        maximum: half + 1e-10,
    };
    let snapped = interval
        .snap(RenderViewportGridPrecision::Fractional12)
        .unwrap();
    assert_eq!(snapped.minimum, 0.);
    assert_eq!(snapped.maximum, 1. / 4096.);
    assert!(!snapped.excludes_zero());
}

#[test]
fn unsupported_divisors_overflow_and_grid_ranges_refuse() {
    assert!(point(1.).divide(point(0.)).is_none());
    assert!(point(1.).divide(point(f32::MIN_POSITIVE / 2.)).is_none());
    assert!(point(f32::MAX).multiply(point(2.)).is_none());
    assert!(
        point(4096.)
            .snap(RenderViewportGridPrecision::Fractional12)
            .is_none()
    );
    assert!(DepthShaderBound::source(f32::NAN).is_none());
}

#[test]
fn finite_signed_intervals_keep_their_known_sign() {
    assert!(point(-2.).excludes_zero());
    assert!(point(2.).excludes_zero());
    assert!(!point(0.).excludes_zero());
}

#[test]
fn exact_identity_operations_preserve_normal_and_optional_ftz_hulls() {
    for value in [0., -0., 1., -7., f32::MAX, f32::MIN_POSITIVE / 2.] {
        let bound = point(value);
        assert_eq!(bound.add(point(0.)), Some(bound));
        assert_eq!(point(0.).add(bound), Some(bound));
        assert_eq!(bound.subtract(point(0.)), Some(bound));
        assert_eq!(bound.multiply(point(1.)), Some(bound));
        assert_eq!(point(1.).multiply(bound), Some(bound));
        assert_eq!(bound.multiply(point(0.)), Some(point(0.)));
        assert_eq!(point(0.).multiply(bound), Some(point(0.)));
    }
}

#[test]
fn uncertain_near_identity_intervals_do_not_take_exact_shortcuts() {
    let uncertain = DepthShaderBound {
        minimum: 0.,
        maximum: 1.,
    };
    let product = point(2.).multiply(uncertain).unwrap();
    assert!(contains(product, 0.));
    assert!(contains(product, 2.));
    assert_ne!(product, point(0.));
    assert_ne!(product, point(2.));
    assert_ne!(point(2.).add(uncertain).unwrap(), point(2.));
}

#[test]
fn clip_transform_interval_is_operation_local_near_the_far_plane() {
    let matrix = [
        0.85745096,
        -0.4950496,
        -0.66666555,
        0.0,
        0.85745096,
        0.4950496,
        0.66666555,
        0.0,
        0.0,
        0.9900992,
        -0.66666555,
        0.0,
        -0.85745096,
        -0.49504954,
        0.3333329,
        1.0,
    ];
    let vertex = crate::RenderVertex::homogeneous([0.0, 1.0, 0.0, 1.0], [1.0; 4]);

    let bounds = clip_transform_bounds(matrix, vertex).unwrap();
    let z = ((matrix[6] * vertex.position[1] + matrix[2] * vertex.position[0])
        + matrix[10] * vertex.position[2])
        + matrix[14] * vertex.weight;
    let w = ((matrix[7] * vertex.position[1] + matrix[3] * vertex.position[0])
        + matrix[11] * vertex.position[2])
        + matrix[15] * vertex.weight;

    assert!(contains(bounds[2], z));
    assert!(contains(bounds[3], w));
    assert!(bounds[2].maximum <= bounds[3].minimum);
}

#[test]
fn exact_identity_operations_keep_normal_and_ftz_bounds_without_new_uncertainty() {
    let zero = DepthShaderBound::source(0.).unwrap();
    let one = DepthShaderBound::source(1.).unwrap();
    for value in [f32::MAX, -3., -0., f32::MIN_POSITIVE / 2., 0.25] {
        let bound = DepthShaderBound::source(value).unwrap();
        assert_eq!(bound.add(zero), Some(bound));
        assert_eq!(zero.add(bound), Some(bound));
        assert_eq!(bound.multiply(one), Some(bound));
        assert_eq!(one.multiply(bound), Some(bound));
        assert_eq!(bound.multiply(zero).unwrap().endpoints(), [0., 0.]);
    }
    let near_one = DepthShaderBound::source(1_f32.next_up()).unwrap();
    assert_ne!(near_one.multiply(near_one), Some(near_one));
}
