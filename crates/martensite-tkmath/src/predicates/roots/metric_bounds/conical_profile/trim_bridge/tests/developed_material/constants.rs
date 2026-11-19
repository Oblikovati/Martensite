// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::conical_profile::developed_boundary;
type BoundRows = Vec<([Interval; 2], Interval, Interval)>;
#[test]
fn one_half_pi_matches_per_span_intervals_bounds_and_charges_fewer_actual_operations() {
    with_actual_spans(|query, contexts, domain| {
        let (old, old_work, old_polls) = evaluate(query, contexts, domain, false);
        let (shared, work, polls) = evaluate(query, contexts, domain, true);
        assert_eq!(old, shared);
        assert_eq!(shared.len(), 3);
        assert!(work < old_work);
        assert!(polls < old_polls);
    });
}
#[test]
fn setup_work_and_cancellation_are_global_and_preserve_the_original_cause() {
    with_actual_spans(|mut query, contexts, domain| {
        query.options.max_operations = 1;
        let error =
            certify_conical_material_with_developed_boundaries(query, contexts, domain, || false)
                .unwrap_err();
        assert_eq!(error.span, None);
        assert_eq!(error.operations, 1);
        assert!(!error.is_cancelled());
        let error =
            certify_conical_material_with_developed_boundaries(query, contexts, domain, || true)
                .unwrap_err();
        assert_eq!(error.span, None);
        assert_eq!(error.operations, 0);
        assert!(error.is_cancelled());
    });
}
#[test]
fn actual_source_binding_still_precedes_any_global_constant_setup() {
    with_actual_spans(|mut query, contexts, domain| {
        query.reference_slant = 81.;
        query.options.max_operations = 0;
        let error =
            certify_conical_material_with_developed_boundaries(query, contexts, domain, || true)
                .unwrap_err();
        assert_eq!(error.operations, 0);
        assert_eq!(error.span, Some(0));
        assert!(!error.is_cancelled());
        assert!(
            matches!(error.cause.as_ref(), ConicalDevelopedMaterialFailure::Binding { actual, expected, .. } if actual != expected)
        );
    });
}
fn with_actual_spans(
    body: impl FnOnce(
        ConicalMaterialQuery<'_>,
        &[ConicalDevelopedBoundarySpan<'_>],
        &ConicalChartDomainEvidence,
    ),
) {
    with_fixture(|pair, source, boundary, domain| {
        let uv = [[[0., 0.]; 2], [[1e-7, 0.]; 2], [[0., 1e-7]; 2]];
        let evidence = witnesses(pair, source, boundary, domain, &uv);
        let spans: Vec<_> = uv.iter().map(|uv| span(source, uv)).collect();
        let contexts = context_rows(source, boundary, &evidence);
        body(material(source, pair, &spans), &contexts, domain);
    });
}
fn witnesses(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    uv: &[[[f64; 2]; 2]],
) -> Vec<ConicalDevelopedBoundaryEvidence> {
    uv.iter()
        .map(|uv| {
            let actual = actual_pair(pair, uv);
            certify_conical_trim_pair_with_developed_boundary(
                actual,
                source,
                boundary,
                domain,
                80.,
                || false,
            )
            .unwrap()
            .1
        })
        .collect()
}
fn evaluate(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    shared: bool,
) -> (BoundRows, u32, usize) {
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        false
    };
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: query.options.fractional_bits,
            max_operations: 100_000,
        },
        &mut cancelled,
    );
    let values = evaluate_rows(query, contexts, domain, shared, &mut arithmetic);
    let operations = arithmetic.operations();
    (values, operations, polls)
}
fn evaluate_rows(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    shared: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> BoundRows {
    let shared_half = if shared {
        Some(developed_boundary::half_pi(arithmetic).unwrap())
    } else {
        None
    };
    query
        .spans
        .iter()
        .zip(contexts)
        .map(|(span, context)| {
            developed_boundary::validate(&query, span, context, domain).unwrap();
            let half = shared_half
                .clone()
                .unwrap_or_else(|| developed_boundary::half_pi(arithmetic).unwrap());
            developed_boundary::span_bounds(&query, span, context, domain, &half, arithmetic)
                .unwrap()
        })
        .collect()
}

fn context_rows<'a>(
    source: ConicalBoundaryQuery<'a>,
    boundary: &'a ConicalBoundaryEvidence,
    evidence: &'a [ConicalDevelopedBoundaryEvidence],
) -> Vec<ConicalDevelopedBoundarySpan<'a>> {
    evidence
        .iter()
        .map(|evidence| ConicalDevelopedBoundarySpan {
            source,
            boundary,
            evidence,
        })
        .collect()
}

fn actual_pair<'a>(pair: ConicalTrimPairQuery<'a>, uv: &'a [[f64; 2]]) -> ConicalTrimPairQuery<'a> {
    ConicalTrimPairQuery {
        curves: pair.curves,
        pcurves: [uv; 2],
        carriers: pair.carriers,
        spatial_limits: pair.spatial_limits,
        options: pair.options,
    }
}
