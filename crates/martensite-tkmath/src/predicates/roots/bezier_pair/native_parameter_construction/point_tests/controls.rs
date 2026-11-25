// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn signed_zero_native_and_limit_fields_are_preserved_on_exact_zero_error() {
    for native in [-0., 0.] {
        for limit in [-0., 0.] {
            let value = measure_axis(
                &interval(0., 0.),
                [0., 1.],
                settings(MetricParameterMap::Identity, native, limit),
            )
            .unwrap();
            assert_eq!(value.native_parameter().to_bits(), native.to_bits());
            assert_eq!(value.requested_limit().to_bits(), limit.to_bits());
            assert_eq!(value.decision(), PairParameterDecision::Within);
            assert_eq!(value.error_enclosure.lower, ExactRatio::from_finite(0.));
        }
    }
}
#[test]
fn exact_limit_and_adjacent_representable_limits_keep_threshold_decisions() {
    let point = interval(0.5, 0.5);
    let native = 0.5 + 0.125;
    for (limit, expected) in threshold_cases() {
        let value = measure_axis(
            &point,
            [0., 1.],
            settings(MetricParameterMap::Identity, native, limit),
        )
        .unwrap();
        assert_eq!(value.decision(), expected);
        compare(
            &point,
            MetricParameterMap::Identity,
            native,
            limit,
            [0., 1.],
        );
    }
}
fn failure(result: Result<MeasuredPairParameter, ResidualError>) -> (&'static str, usize, u64) {
    match result {
        Err(ResidualError::Value { role, index, value }) => (role, index, value.to_bits()),
        _ => panic!("expected original typed scalar/map admission failure"),
    }
}
#[test]
fn an_invalid_unselected_map_endpoint_is_never_ignored() {
    for (parameter, map, index) in [(0., [2., f64::INFINITY], 1), (1., [f64::NAN, 5.], 0)] {
        let map = MetricParameterMap::Endpoints(map);
        let point = interval(parameter, parameter);
        let old = failure(original_axis(&point, [0., 1.], settings(map, 0., 0.)));
        let new = failure(measure_axis(&point, [0., 1.], settings(map, 0., 0.)));
        assert_eq!(old, new);
        assert_eq!(new.0, "parameter endpoint");
        assert_eq!(new.1, index);
    }
}
#[test]
fn native_then_limit_then_map_admission_order_is_unchanged() {
    let invalid = MetricParameterMap::Endpoints([f64::INFINITY, f64::NAN]);
    for (native, limit, role) in [
        (f64::NAN, -1., "native parameter construction"),
        (0., -1., "native parameter limit"),
        (0., 0., "parameter endpoint"),
    ] {
        let point = interval(0., 0.);
        let old = failure(original_axis(
            &point,
            [0., 1.],
            settings(invalid, native, limit),
        ));
        let new = failure(measure_axis(
            &point,
            [0., 1.],
            settings(invalid, native, limit),
        ));
        assert_eq!(old, new);
        assert_eq!(new.0, role);
    }
}
#[test]
fn reversed_extreme_map_keeps_both_exact_native_endpoints() {
    let map = MetricParameterMap::Endpoints([f64::MAX, -f64::MAX]);
    for (slot, expected) in [(0., f64::MAX), (1., -f64::MAX)] {
        let point = interval(slot, slot);
        let mapped = native_image(&point, map).unwrap();
        assert_eq!(mapped.lower, ExactRatio::from_finite(expected));
        assert_eq!(mapped.lower, mapped.upper);
    }
}
#[test]
fn both_axis_failures_still_select_the_first_canonical_axis() {
    let first = interval(0., 0.);
    let second = interval(1., 1.);

    let failure = measure_captured_axes(
        [&first, &second],
        captured_sources(),
        [MetricParameterMap::Endpoints([0., f64::INFINITY]); 2],
        [f64::NAN, f64::INFINITY],
        [-1.; 2],
    )
    .err()
    .unwrap();
    assert_eq!(failure.axis, 0);
    assert!(
        matches!(failure.cause,ResidualError::Value {role:"native parameter construction",index:0,value} if value.is_nan())
    );
}

fn threshold_cases() -> [(f64, PairParameterDecision); 3] {
    [
        (0.125f64.next_down(), PairParameterDecision::Exceeds),
        (0.125, PairParameterDecision::Within),
        (0.125f64.next_up(), PairParameterDecision::Within),
    ]
}
#[test]
fn a_nearby_observed_endpoint_is_not_snapped_to_the_certificate_endpoint() {
    let point = interval(1., 1.);
    let native = 1f64.next_down();
    let value = measure_axis(
        &point,
        [0., 1.],
        settings(MetricParameterMap::Identity, native, 0.),
    )
    .unwrap();
    assert_eq!(value.native_parameter().to_bits(), native.to_bits());
    assert_eq!(value.decision(), PairParameterDecision::Exceeds);
    assert_eq!(
        value.error_enclosure.lower,
        ExactRatio::from_finite(f64::EPSILON / 2.)
    );
    compare(&point, MetricParameterMap::Identity, native, 0., [0., 1.]);
}
#[test]
fn the_first_axis_invalid_unused_map_endpoint_precedes_second_axis_native_error() {
    let first = interval(0., 0.);
    let second = interval(1., 1.);

    let maps = [
        MetricParameterMap::Endpoints([2., f64::INFINITY]),
        MetricParameterMap::Identity,
    ];
    let error = measure_captured_axes(
        [&first, &second],
        captured_sources(),
        maps,
        [2., f64::NAN],
        [0.; 2],
    )
    .err()
    .unwrap();
    assert_eq!(error.axis, 0);
    assert!(
        matches!(error.cause,ResidualError::Value {role:"parameter endpoint",index:1,value} if value==f64::INFINITY)
    );
}
#[test]
fn nonfinite_native_values_and_limits_keep_the_original_typed_admission() {
    for (native, limit) in [
        (f64::INFINITY, 0.),
        (-f64::INFINITY, 0.),
        (0., f64::NAN),
        (0., f64::INFINITY),
        (0., -f64::INFINITY),
    ] {
        let point = interval(0., 0.);
        let map = MetricParameterMap::Identity;
        assert_eq!(
            failure(original_axis(
                &point,
                [0., 1.],
                settings(map, native, limit)
            )),
            failure(measure_axis(&point, [0., 1.], settings(map, native, limit)))
        );
    }
}

fn captured_sources() -> [super::super::super::BezierPairCurve<'static>; 2] {
    let curve = super::super::super::BezierPairCurve {
        controls: &[[0., 0.], [1., 1.]],
        weights: None,
        interval: [0., 1.],
    };
    [curve; 2]
}
