// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const LINE: [[f64; 2]; 2] = [[80., -18.], [84., 24.]];
fn query<'a>(spans: &'a [&'a [[f64; 2]]]) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans,
        semi_angle: 0.125,
        chart_origin: 0.,
        fit_center: 0.,
        developed_turn: 0,
        physical_turn: 0,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 1_000_000,
        },
    }
}
#[test]
fn narrow_cone_has_a_quantitative_chart_despite_legacy_sine_refusal() {
    let spans = [LINE.as_slice()];
    let input = query(&spans);
    let proof = certify_conical_chart_domain(input, || false).unwrap();
    assert!(proof.matches(&input));
    assert_eq!(proof.wedge(), ConicalSectorWedge::PositiveX);
    assert!(proof.operations() > 0);
    let legacy = ConicalSectorQuery {
        spans: &spans,
        semi_angle: input.semi_angle,
        options: input.options,
    };
    assert!(matches!(
        certify_conical_sector(legacy, || false),
        Err(ConicalSectorFailure::UnresolvedSine)
    ));
}
#[test]
fn receipt_encloses_independent_endpoint_angles_and_positive_sine() {
    let spans = [LINE.as_slice()];
    let proof = certify_conical_chart_domain(query(&spans), || false).unwrap();
    let bounds = proof.relative_physical_bounds();
    let lower = bounds[0].binary64_bounds().unwrap()[0];
    let upper = bounds[1].binary64_bounds().unwrap()[1];
    let sine = martensite_cmath::sin(0.125);
    assert!(lower <= martensite_cmath::atan2(-18., 80.) / sine);
    assert!(upper >= martensite_cmath::atan2(24., 84.) / sine);
    assert!(proof.sine_bounds()[0].binary64_bounds().unwrap()[0] > 0.);
}
#[test]
fn source_and_policy_mutations_do_not_match_the_owned_receipt() {
    let spans = [LINE.as_slice()];
    let input = query(&spans);
    let proof = certify_conical_chart_domain(input, || false).unwrap();
    let mut changed = input;
    changed.options.max_operations -= 1;
    assert!(!proof.matches(&changed));
    changed = input;
    changed.fit_center = -0.;
    assert!(!proof.matches(&changed));
    let altered = [[80., -18.], [84., 24.000000000000004]];
    let altered_spans = [altered.as_slice()];
    assert!(!proof.matches(&query(&altered_spans)));
}
#[test]
fn unsupported_lifts_and_centers_are_typed_admission_failures() {
    let spans = [LINE.as_slice()];
    let mut input = query(&spans);
    input.developed_turn = 1;
    let error = certify_conical_chart_domain(input, || false).err().unwrap();
    assert_eq!(error.operations, 0);
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Chart { .. }
    ));
    input.developed_turn = 0;
    input.fit_center = 1.;
    assert!(certify_conical_chart_domain(input, || false).is_err());
}
#[test]
fn period_and_chart_pole_are_refused_without_widening_limits() {
    let wide = [[80., -70.], [80., 70.]];
    let spans = [wide.as_slice()];
    assert!(certify_conical_chart_domain(query(&spans), || false).is_err());
    let wrong = [[-84., -18.], [-80., 24.]];
    let spans = [wrong.as_slice()];
    assert!(certify_conical_chart_domain(query(&spans), || false).is_err());
}
#[test]
fn work_and_initial_cancellation_never_publish_a_receipt() {
    let spans = [LINE.as_slice()];
    let input = query(&spans);
    let error = certify_conical_chart_domain(input, || true).err().unwrap();
    assert!(error.is_cancelled());
    let mut exhausted = input;
    exhausted.options.max_operations = 0;
    let error = certify_conical_chart_domain(exhausted, || false)
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        ConicalChartDomainFailure::Sector(ConicalSectorFailure::Arithmetic(_))
    ));
}
#[test]
fn early_middle_and_final_cancellation_checkpoints_refuse_without_partial_success() {
    let spans = [LINE.as_slice()];
    let input = query(&spans);
    let mut polls = 0;
    certify_conical_chart_domain(input, || {
        polls += 1;
        false
    })
    .unwrap();
    for stop in [1, polls / 2, polls] {
        let error = cancel_at(input, stop);
        assert!(
            error.is_cancelled(),
            "checkpoint {stop} of {polls}: {error}"
        );
    }
}
#[test]
fn source_snapshots_and_numeric_receipts_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalChartDomainEvidence>();
    shared::<ConicalChartDomainVersion>();
    shared::<ConicalChartDomainError>();
    shared::<ConicalChartDomainFailure>();
    shared::<ConicalChartDomainQuery<'static>>();
}

fn cancel_at(input: ConicalChartDomainQuery<'_>, stop: usize) -> ConicalChartDomainError {
    let mut observed = 0;
    certify_conical_chart_domain(input, || {
        observed += 1;
        observed == stop
    })
    .err()
    .unwrap()
}
