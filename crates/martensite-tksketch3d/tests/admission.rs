// SPDX-License-Identifier: GPL-2.0-only
use super::fixture::{connect, lid, pid, point};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{EditFailure, MinimumComparison, MinimumLength};

#[test]
fn independent_spatial_13_fixture_sets_the_threshold_boundary() {
    let start = [100., 200., 300.];
    let end = [103., 204., 312.];
    for (minimum, comparison) in [
        (13_f64.next_down(), MinimumComparison::Greater),
        (13., MinimumComparison::Equal),
    ] {
        let result = connect(start, end, minimum).unwrap().line(lid(3)).unwrap();
        assert_eq!(result.length_estimate.to_bits(), 13_f64.to_bits());
        assert_eq!(result.minimum_comparison, comparison);
        assert_eq!(result.carrier.location(), point(start));
        assert_eq!(result.points[1], point(end));
    }
    let above = 13_f64.next_up();
    assert_eq!(
        connect(start, end, above),
        Err(EditFailure::BelowMinimumLength {
            line: lid(3),
            estimate: 13.,
            minimum: MinimumLength::new(above).unwrap(),
        })
    );
}

#[test]
fn exact_minimum_distinguishes_distances_with_the_same_rounded_estimate() {
    let tiny = f64::from_bits(1);
    let end = [1., 0., 0.];
    assert_eq!(
        connect([tiny, 0., 0.], end, 1.),
        Err(EditFailure::BelowMinimumLength {
            line: lid(3),
            estimate: 1.,
            minimum: MinimumLength::new(1.).unwrap(),
        })
    );
    let above = connect([-tiny, 0., 0.], end, 1.)
        .unwrap()
        .line(lid(3))
        .unwrap();
    assert_eq!(above.length_estimate.to_bits(), 1_f64.to_bits());
    assert_eq!(above.minimum_comparison, MinimumComparison::Greater);
}

#[test]
fn nonfinite_inputs_and_invalid_minimum_are_typed_refusals() {
    for value in [0., -0., -1., f64::INFINITY, f64::NAN] {
        assert_eq!(
            MinimumLength::new(value).unwrap_err().value.to_bits(),
            value.to_bits()
        );
    }
    let result = connect([0., 0., 0.], [1., f64::INFINITY, 0.], 0.1);
    assert!(matches!(result, Err(EditFailure::NonFinitePoint {
        point, axis: 1, value: f64::INFINITY,
    }) if point == pid(2)));
}

#[test]
fn finite_extremes_keep_measurable_lengths_and_safe_directions() {
    let max = f64::MAX;
    let subnormal = f64::from_bits(1);
    for (start, end, minimum, expected) in [
        ([-max / 2., 0., 0.], [max / 2., 0., 0.], max, max),
        ([0., 0., 0.], [subnormal, 0., 0.], subnormal, subnormal),
    ] {
        let geometry = connect(start, end, minimum).unwrap().line(lid(3)).unwrap();
        assert_eq!(geometry.length_estimate.to_bits(), expected.to_bits());
        assert_eq!(geometry.carrier.direction(), Direction3::X);
        assert_eq!(geometry.minimum_comparison, MinimumComparison::Equal);
    }
    let diagonal = connect([0.; 3], [1e300, 1e300, 0.], 1.)
        .unwrap()
        .line(lid(3))
        .unwrap();
    assert!(diagonal.length_estimate.is_finite());
    let rounded_control = 2_f64.sqrt() * 1e300;
    assert!(
        (rounded_control.next_down()..=rounded_control.next_up())
            .contains(&diagonal.length_estimate)
    );
}

#[test]
fn finite_values_can_refuse_displacement_length_or_direction() {
    let max = f64::MAX;
    let subnormal = f64::from_bits(1);
    assert_eq!(
        connect([-max, 0., 0.], [max, 0., 0.], 1.),
        Err(EditFailure::UnrepresentableDisplacement {
            line: lid(3),
            axis: 0,
            endpoints: [-max, max],
        })
    );
    assert_eq!(
        connect([0.; 3], [max, max, 0.], 1.),
        Err(EditFailure::UnrepresentableLength {
            line: lid(3),
            displacement: [max, max, 0.],
        })
    );
    assert_eq!(
        connect([0.; 3], [1e300, subnormal, 0.], 1.),
        Err(EditFailure::UnrepresentableDirection {
            line: lid(3),
            axis: 1,
            displacement: [1e300, subnormal, 0.],
        })
    );
}
