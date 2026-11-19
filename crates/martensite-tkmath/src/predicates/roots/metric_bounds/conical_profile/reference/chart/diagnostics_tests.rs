// SPDX-License-Identifier: GPL-2.0-only
//! Bug #551: numerical projection keeps the existing enum and honest directed bounds.
use super::super::super::chart_domain::diagnostics_tests::with_arithmetic;
use super::*;
fn exact(value: f64) -> DyadicBound {
    DyadicBound::from_f64(value).unwrap()
}
fn diagnostic(
    actual: [f64; 2],
    expected: ConicalChartDomainRequirement,
) -> ConicalChartDomainFailure {
    ConicalChartDomainFailure::Unresolved {
        bound: Box::new(ConicalChartDomainBound {
            field: "control chart",
            actual: actual.map(exact),
            expected,
        }),
    }
}
#[test]
fn numerical_projection_retains_width_and_negated_failed_lower_bounds() {
    let width = project(diagnostic(
        [-4., 4.],
        ConicalChartDomainRequirement::WidthStrictlyBelow(exact(6.)),
    ));
    assert_values(width, [8., 6.]);
    let positive = project(diagnostic(
        [-2., 1.],
        ConicalChartDomainRequirement::PositiveLower,
    ));
    assert!(assert_values(positive, [2., 0.]).contains("negated"));
}
#[test]
fn strict_inside_projection_keeps_the_failed_endpoint_and_threshold() {
    let limits = [-3., 3.].map(exact);
    let lower = project(diagnostic(
        [-4., 1.],
        ConicalChartDomainRequirement::StrictlyInside(limits.clone()),
    ));
    assert_values(lower, [4., 3.]);
    let upper = project(diagnostic(
        [-1., 3.],
        ConicalChartDomainRequirement::StrictlyInside(limits),
    ));
    assert_values(upper, [3., 3.]);
}
#[test]
fn directed_projection_handles_both_overflow_signs_without_nan_or_zero() {
    let positive = DyadicBound::new(1.into(), 2048);
    let negative = DyadicBound::new((-1).into(), 2048);
    assert_eq!(projected_upper(&positive), f64::INFINITY);
    assert_eq!(projected_lower(&positive), f64::MAX);
    assert_eq!(projected_upper(&negative), -f64::MAX);
    assert_eq!(projected_lower(&negative), f64::NEG_INFINITY);
}
#[test]
fn homotopy_axis_contact_is_numerical_and_keeps_the_existing_work_charge() {
    let domain = domain();
    with_arithmetic(|arithmetic| {
        let contact = arithmetic.scalar(80.).unwrap();
        let before = arithmetic.operations();
        let failure = homotopy(&domain, &contact, arithmetic).unwrap_err();
        assert_values(failure, [0., 0.]);
        assert_eq!(arithmetic.operations(), before + 1);
    });
}

fn assert_values(failure: ConicalReferenceFailure, expected: [f64; 2]) -> &'static str {
    let ConicalReferenceFailure::Bound {
        field,
        actual,
        limit,
    } = failure
    else {
        panic!("numerical bound")
    };
    assert_eq!([actual, limit], expected);
    field
}
fn domain() -> ConicalChartDomainEvidence {
    let poles = [[80., -18.], [84., 24.]];
    let spans = [poles.as_slice()];
    let query = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: 0.125,
        chart_origin: 0.,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 100_000,
        },
    };
    certify_conical_chart_domain(query, || false).unwrap()
}
