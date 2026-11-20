// SPDX-License-Identifier: GPL-2.0-only
//! Literal original interval formulas pin exact values and admission order for #569.
use super::*;
mod controls;
fn interval(lower: f64, upper: f64) -> RatioInterval {
    RatioInterval {
        lower: ExactRatio::from_finite(lower),
        upper: ExactRatio::from_finite(upper),
    }
}
fn original_image(
    bounds: &RatioInterval,
    map: MetricParameterMap,
) -> Result<RatioInterval, ResidualError> {
    let [origin, span] = map.exact_coefficients()?;
    let scale: BigInt = BigInt::from(1) << 1074_usize;
    let origin = RatioInterval::point(ExactRatio::new(origin, scale.clone()));
    let span = RatioInterval::point(ExactRatio::new(span, scale));
    Ok(origin.add(&bounds.multiply(&span)))
}
fn original_distance(bounds: &RatioInterval, native: &ExactRatio) -> RatioInterval {
    let left = absolute(bounds.lower.subtract(native));
    let right = absolute(bounds.upper.subtract(native));
    let minimum = if native < &bounds.lower {
        left.clone()
    } else if native > &bounds.upper {
        right.clone()
    } else {
        ExactRatio::from_finite(0.)
    };
    RatioInterval {
        lower: minimum,
        upper: left.max(right),
    }
}
fn original_axis(
    bounds: &RatioInterval,
    trim: [f64; 2],
    settings: AxisConstructionSettings,
) -> Result<MeasuredPairParameter, ResidualError> {
    let [supplied, threshold] = admitted_axis(&settings)?;
    let native = original_image(&inside_source_domain(bounds, trim), settings.map)?;
    let error = original_distance(&native, &supplied);
    let decision = threshold_decision(&error, &threshold);
    Ok(owned_measurement(settings, native, error, decision))
}
fn settings(map: MetricParameterMap, native: f64, limit: f64) -> AxisConstructionSettings {
    AxisConstructionSettings {
        map,
        native,
        limit,
        axis: 0,
    }
}
fn assert_interval(a: &RatioInterval, b: &RatioInterval) {
    assert_eq!(a.lower, b.lower);
    assert_eq!(a.upper, b.upper);
}
fn assert_measurement(a: &MeasuredPairParameter, b: &MeasuredPairParameter) {
    assert_eq!(
        a.native_parameter().to_bits(),
        b.native_parameter().to_bits()
    );
    assert_eq!(a.requested_limit().to_bits(), b.requested_limit().to_bits());
    assert_eq!(a.decision(), b.decision());
    assert_interval(&a.native_enclosure, &b.native_enclosure);
    assert_interval(&a.error_enclosure, &b.error_enclosure);
}
fn compare(
    bounds: &RatioInterval,
    map: MetricParameterMap,
    native: f64,
    limit: f64,
    trim: [f64; 2],
) {
    let old = original_axis(bounds, trim, settings(map, native, limit)).unwrap();
    let new = measure_axis(bounds, trim, settings(map, native, limit)).unwrap();
    assert_measurement(&old, &new);
}
fn maps() -> [MetricParameterMap; 7] {
    [
        MetricParameterMap::Identity,
        MetricParameterMap::Endpoints([2., 5.]),
        MetricParameterMap::Endpoints([5., 2.]),
        MetricParameterMap::Endpoints([3., 3.]),
        MetricParameterMap::Endpoints([-0., 0.]),
        MetricParameterMap::Endpoints([f64::MAX, -f64::MAX]),
        MetricParameterMap::Endpoints([f64::from_bits(1), -f64::from_bits(1)]),
    ]
}
#[test]
fn exact_endpoints_and_interior_points_equal_the_original_rational_construction() {
    let points = [
        ExactRatio::from_finite(0.),
        ExactRatio::from_finite(1.),
        ExactRatio::from_finite(0.5),
        ExactRatio::new(1.into(), 3.into()),
        ExactRatio::from_finite(f64::from_bits(1)),
        ExactRatio::from_finite(1f64.next_down()),
    ];
    for point in points {
        compare_point(&RatioInterval::point(point));
    }
}
fn compare_point(bounds: &RatioInterval) {
    for map in maps() {
        for native in [-0., 0., 0.5, f64::from_bits(1), f64::MAX, -f64::MAX] {
            compare(bounds, map, native, 0., [0., 1.]);
        }
    }
}
#[test]
fn nonpoint_and_clipped_source_intervals_retain_the_original_enclosures() {
    for bounds in [interval(0., 0.25), interval(0.25, 0.75), interval(0., 1.)] {
        for map in maps() {
            compare(&bounds, map, 0.5, 1., [0.125, 0.875]);
        }
    }
}
#[test]
fn point_error_is_the_exact_absolute_difference_without_binary64_rounding() {
    let point = RatioInterval::point(ExactRatio::new(1.into(), 3.into()));
    let native = ExactRatio::from_finite(1. / 3.);
    let actual = distance_enclosure(&point, &native);
    assert!(actual.lower > ExactRatio::from_finite(0.));
    assert_eq!(actual.lower, absolute(point.lower.subtract(&native)));
    assert_eq!(actual.lower, actual.upper);
}

#[test]
fn an_enclosure_clipped_to_a_source_trim_point_keeps_the_original_result() {
    for map in maps() {
        compare(&interval(0., 0.25), map, 0., 0., [0.25, 0.75]);
    }
}
