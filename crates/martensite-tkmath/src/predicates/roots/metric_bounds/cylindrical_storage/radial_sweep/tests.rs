// SPDX-License-Identifier: GPL-2.0-only
use super::super::MetricBezier;
use super::*;
use crate::predicates::roots::metric_bounds::BoundedArithmeticFailure;
use CylindricalRadialSense::{Inward, Outward};

const INNER: [[f64; 3]; 2] = [[3., 0., -1.], [3., 0., 4.]];
const OUTER: [[f64; 3]; 2] = [[5., 0., -1.], [5., 0., 4.]];
const WEIGHTS: [f64; 2] = [1., 0.25];

fn rail<'a>(poles: &'a [[f64; 3]], weights: Option<&'a [f64]>) -> MetricBezier<'a, 3> {
    MetricBezier { poles, weights }
}

fn linear() -> CylindricalRadialSweepQuery<'static> {
    CylindricalRadialSweepQuery {
        rails: [rail(&INNER, Some(&WEIGHTS)), rail(&OUTER, Some(&WEIGHTS))],
        axis_origin: [0., 0.],
        sense: Outward,
        minimum: 1.,
        options: Default::default(),
    }
}

fn refusal(query: CylindricalRadialSweepQuery<'_>) -> CylindricalRadialSweepFailure {
    certify_cylindrical_radial_sweep(query, || false)
        .unwrap_err()
        .cause
}

#[test]
fn finds_the_exact_depth_endpoint_minimum_for_both_senses() {
    let mut input = linear();
    for sense in [Outward, Inward] {
        input.sense = sense;
        let proof = certify_cylindrical_radial_sweep(input, || false).unwrap();
        // Radius is 3+2s for the boss and 5-2s for the pocket.
        assert_eq!(proof.signed_squared_radius_derivative_lower(), 12.);
        assert_eq!(proof.requested_minimum(), input.minimum);
        assert_eq!(proof.requested_options(), input.options);
        assert!((1..=input.options.max_operations).contains(&proof.operations()));
        assert!(proof.matches_source(input.rails, input.axis_origin, sense));
        input.rails.swap(0, 1);
    }
    input.sense = Outward;
    input.minimum = 12.;
    assert_eq!(refusal(input), Failure::Uncertified { lower: 12. });
}

#[test]
fn rational_quarter_circle_keeps_its_known_radial_margin() {
    // For q(t)=1+t² the homogeneous XY curve is R(1-t², 2t).
    let inner = [[3., 0., 0.], [3., 3., 0.], [0., 3., 0.]];
    let outer = [[6., 0., 0.], [6., 6., 0.], [0., 6., 0.]];
    let weights = [1., 1., 2.];
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&inner, Some(&weights)), rail(&outer, Some(&weights))],
        minimum: 17.,
        ..linear()
    };
    let proof = certify_cylindrical_radial_sweep(input, || false).unwrap();
    assert!(proof.signed_squared_radius_derivative_lower() > 17.);
    assert!(proof.signed_squared_radius_derivative_lower() <= 18.);
}

#[test]
fn admitted_axis_translation_uses_the_actual_center() {
    let inner = INNER.map(|[x, y, z]| [x + 32., y - 8., z]);
    let outer = OUTER.map(|[x, y, z]| [x + 32., y - 8., z]);
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&inner, None), rail(&outer, None)],
        axis_origin: [32., -8.],
        ..linear()
    };
    let proof = certify_cylindrical_radial_sweep(input, || false).unwrap();
    assert_eq!(proof.signed_squared_radius_derivative_lower(), 12.);
    let shifted = [40., -8.];
    assert!(!proof.matches_source(input.rails, shifted, Outward));
    let invalid = CylindricalRadialSweepQuery {
        axis_origin: shifted,
        ..input
    };
    assert_eq!(refusal(invalid), Failure::Uncertified { lower: -20. });
}

#[test]
fn axis_crossing_during_depth_cannot_pass_by_endpoint_radius_order() {
    let across = OUTER.map(|[x, y, z]| [-x, y, z]);
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&INNER, None), rail(&across, None)],
        ..linear()
    };
    assert_eq!(refusal(input), Failure::Uncertified { lower: -48. });
}

#[test]
fn positive_rail_endpoints_do_not_hide_an_interior_axis_contact() {
    let inner = [[3., 0., 0.], [-3., 0., 1.], [3., 0., 2.]];
    let outer = [[5., 0., 0.], [-5., 0., 1.], [5., 0., 2.]];
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&inner, None), rail(&outer, None)],
        minimum: 0.,
        ..linear()
    };
    assert!(matches!(refusal(input), Failure::Uncertified { lower } if lower <= 0.));
}

#[test]
fn original_rail_bits_and_direction_are_required_for_rebinding() {
    let input = linear();
    let proof = certify_cylindrical_radial_sweep(input, || false).unwrap();
    let mut changed = INNER;
    changed[1][2] = changed[1][2].next_up();
    let new_first = [rail(&changed, Some(&WEIGHTS)), input.rails[1]];
    assert!(!proof.matches_source(new_first, input.axis_origin, Outward));
    let changed_weights = [1., WEIGHTS[1].next_up()];
    let changed = input.rails.map(|rail| MetricBezier {
        weights: Some(&changed_weights),
        ..rail
    });
    assert!(!proof.matches_source(changed, input.axis_origin, Outward));
    assert!(!proof.matches_source([input.rails[1], input.rails[0]], input.axis_origin, Outward));
    assert!(!proof.matches_source(input.rails, input.axis_origin, Inward));
}

#[test]
fn omitted_unit_weights_bind_the_literal_representation() {
    let ones = [1., 1.];
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&INNER, None), rail(&OUTER, Some(&ones))],
        ..linear()
    };
    let proof = certify_cylindrical_radial_sweep(input, || false).unwrap();
    assert_eq!(proof.signed_squared_radius_derivative_lower(), 12.);
    let both_unit = [input.rails[0], rail(&OUTER, None)];
    assert!(proof.matches_source(input.rails, input.axis_origin, Outward));
    assert!(!proof.matches_source(both_unit, input.axis_origin, Outward));
}

#[test]
fn refuses_different_positive_weights_and_invalid_weights() {
    let mut input = linear();
    input.rails[1].weights = Some(&[1., 0.5]);
    assert_eq!(
        refusal(input),
        Failure::CommonWeights {
            index: 1,
            actual: [0.25, 0.5]
        }
    );
    input.rails[1].weights = Some(&[1., 0.]);
    assert!(matches!(
        refusal(input),
        Failure::Admission(super::super::CylindricalStorageFailure::Value {
            depth: Some(1),
            field: "weight",
            index: 1,
            value: 0.
        })
    ));
}

#[test]
fn refuses_mismatched_degrees_and_nonfinite_axis() {
    let three = [[5., 0., 0.], [5., 0., 1.], [5., 0., 2.]];
    let mut input = linear();
    input.rails[1] = rail(&three, None);
    assert_eq!(refusal(input), Failure::PoleCounts([2, 3]));
    input = linear();
    input.axis_origin = [0., f64::INFINITY];
    assert_eq!(refusal(input), Failure::AxisOrigin([0., f64::INFINITY]));
}

#[test]
fn refuses_nonfinite_and_negative_strict_thresholds() {
    let mut input = linear();
    input.minimum = -1.;
    assert_eq!(refusal(input), Failure::Minimum(-1.));
    input.minimum = f64::NAN;
    assert!(matches!(refusal(input), Failure::Minimum(value) if value.is_nan()));
}

#[test]
fn original_work_limit_and_cancellation_are_typed() {
    let mut input = linear();
    input.options.max_operations = 0;
    let error = certify_cylindrical_radial_sweep(input, || false).unwrap_err();
    assert_eq!(error.requested_options, input.options);
    assert_eq!(error.requested_minimum, input.minimum);
    assert_eq!(error.sense, Outward);
    assert_eq!(
        error.cause,
        Failure::Arithmetic(BoundedArithmeticFailure::WorkLimit {
            completed: 0,
            limit: 0
        })
    );
    let error = certify_cylindrical_radial_sweep(linear(), || true).unwrap_err();
    assert_eq!(
        error.cause,
        Failure::Arithmetic(BoundedArithmeticFailure::Cancelled)
    );
}

#[test]
fn unresolved_positive_weight_denominator_refuses_at_the_requested_precision() {
    let weights = [f64::MIN_POSITIVE; 2];
    let input = CylindricalRadialSweepQuery {
        rails: [rail(&INNER, Some(&weights)), rail(&OUTER, Some(&weights))],
        ..linear()
    };
    assert!(matches!(
        refusal(input),
        Failure::Arithmetic(BoundedArithmeticFailure::DenominatorUnresolved { .. })
    ));
}
