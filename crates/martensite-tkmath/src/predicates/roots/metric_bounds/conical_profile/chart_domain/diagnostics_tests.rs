// SPDX-License-Identifier: GPL-2.0-only
//! Bug #551: failed predicates retain their original exact operands and requirements.
use super::*;
pub(in super::super) fn with_arithmetic<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let policy = ArithmeticPolicy {
        fractional_bits: 128,
        max_operations: 100_000,
    };
    body(&mut Arithmetic::new(policy, &mut never))
}
fn bound(value: f64) -> DyadicBound {
    DyadicBound::from_f64(value).unwrap()
}
#[test]
fn positive_lower_retains_exact_interval_without_charging_diagnostic_work() {
    with_arithmetic(|arithmetic| {
        let actual = interval([-1., 2.], arithmetic);
        let before = arithmetic.operations();
        let error = bounds::positive_lower(&actual, "control sine", arithmetic).unwrap_err();
        let ConicalChartDomainFailure::Unresolved { bound: retained } = error else {
            panic!("numeric refusal")
        };
        assert_eq!(retained.actual, [bound(-1.), bound(2.)]);
        assert_eq!(
            retained.expected,
            ConicalChartDomainRequirement::PositiveLower
        );
        assert_eq!(arithmetic.operations(), before);
        assert_positive_message(retained, before);
    });
}
#[test]
fn width_and_strict_chart_contact_keep_the_certified_thresholds() {
    with_arithmetic(|arithmetic| {
        let actual = interval([-4., 4.], arithmetic);
        let error = bounds::width(&actual, arithmetic).unwrap_err();
        let ConicalChartDomainFailure::Unresolved { bound: retained } = error else {
            panic!("width refusal")
        };
        assert_eq!(retained.actual, [bound(-4.), bound(4.)]);
        let ConicalChartDomainRequirement::WidthStrictlyBelow(limit) = retained.expected else {
            panic!("strict width")
        };
        assert!(limit > bound(6.) && limit < bound(7.));
        assert_chart_boundary(arithmetic);
    });
}
fn assert_chart_boundary(arithmetic: &mut Arithmetic<'_>) {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)
        .unwrap();
    let actual = Interval {
        lower: -pi.lower.clone(),
        upper: pi.lower,
    };
    let error = bounds::chart_membership(&actual, arithmetic).unwrap_err();
    let ConicalChartDomainFailure::Unresolved { bound } = error else {
        panic!("chart refusal")
    };
    assert_eq!(
        bound.expected,
        ConicalChartDomainRequirement::StrictlyInside(bound.actual.clone())
    );
}
#[test]
fn structural_failures_keep_actual_wedge_and_principal_point() {
    let error = bounds::wedge(4).unwrap_err();
    assert!(matches!(
        error,
        ConicalChartDomainFailure::Wedge {
            actual_mask: 4,
            expected: ConicalSectorWedge::PositiveX
        }
    ));
    let actual = [0., 0.];
    let error = bounds::angle_failure(angle::InitialAngleFailure::InvalidPoint(actual), actual);
    assert!(matches!(
        error,
        ConicalChartDomainFailure::Point {
            actual: [0., 0.],
            ..
        }
    ));
}
#[test]
fn diagnostic_payloads_are_owned_send_and_sync_and_legacy_debug_is_unchanged() {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalChartDomainBound>();
    shared::<ConicalChartDomainRequirement>();
    assert_eq!(
        format!("{:?}", ConicalSectorFailure::UnresolvedSine),
        "UnresolvedSine"
    );
}

fn interval(values: [f64; 2], arithmetic: &mut Arithmetic<'_>) -> Interval {
    let lower = arithmetic.scalar(values[0]).unwrap().lower;
    let upper = arithmetic.scalar(values[1]).unwrap().upper;
    Interval { lower, upper }
}

fn assert_positive_message(bound: Box<ConicalChartDomainBound>, operations: u32) {
    let message =
        domain_error(ConicalChartDomainFailure::Unresolved { bound }, operations).to_string();
    assert!(message.contains("actual=[-1 * 2^0, 1 * 2^1]"));
    assert!(message.contains("expected strictly positive lower bound"));
}
