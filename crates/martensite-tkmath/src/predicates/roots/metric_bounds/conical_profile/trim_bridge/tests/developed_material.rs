// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::{
    ConicalDevelopedBoundarySpan, ConicalDevelopedMaterialFailure, ConicalMaterialEvidence,
    ConicalMaterialQuery, ConicalMaterialSpan, certify_conical_material_with_developed_boundaries,
};
fn material<'a>(
    source: ConicalBoundaryQuery<'a>,
    pair: ConicalTrimPairQuery<'a>,
    spans: &'a [ConicalMaterialSpan<'a>],
) -> ConicalMaterialQuery<'a> {
    ConicalMaterialQuery {
        spans,
        carriers: pair.carriers,
        reference_slant: 80.,
        chart_origin: source.chart_origin,
        depths: source.depths,
        spatial_limit: 1e-4,
        options: fixture::options(),
    }
}
fn span<'a>(source: ConicalBoundaryQuery<'a>, uv: &'a [[f64; 2]]) -> ConicalMaterialSpan<'a> {
    let (original, index, level) = source.original.map_or((source.source, 0, 0), |span| {
        (span.poles, span.index, span.level)
    });
    ConicalMaterialSpan {
        original,
        interval: CylindricalProfileInterval::new(index, level).unwrap(),
        uv,
    }
}
fn with_witness(
    body: impl FnOnce(
        ConicalTrimPairQuery<'_>,
        ConicalBoundaryQuery<'_>,
        &ConicalBoundaryEvidence,
        &ConicalChartDomainEvidence,
        &ConicalDevelopedBoundaryEvidence,
    ),
) {
    with_fixture(|pair, source, boundary, domain| {
        let (_, evidence) = certify_conical_trim_pair_with_developed_boundary(
            pair,
            source,
            boundary,
            domain,
            80.,
            || false,
        )
        .unwrap();
        body(pair, source, boundary, domain, &evidence);
    });
}
fn certify_material(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
) -> ConicalMaterialEvidence {
    let (_, evidence) = certify_conical_trim_pair_with_developed_boundary(
        pair,
        source,
        boundary,
        domain,
        80.,
        || false,
    )
    .unwrap();
    let spans = [span(source, pair.pcurves[0])];
    let query = material(source, pair, &spans);
    let contexts = contexts(source, boundary, &evidence);
    certify_conical_material_with_developed_boundaries(query, &contexts, domain, || false).unwrap()
}
#[test]
fn retained_actual_operands_preserve_trim_work_and_legacy_callback_order() {
    with_fixture(|pair, source, boundary, domain| {
        let mut old_polls = 0;
        let old = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || {
            old_polls += 1;
            false
        })
        .unwrap();
        let (new, evidence, new_polls) = with_counted_capture(pair, source, boundary, domain);
        assert_legacy_work(&old, &new, old_polls, new_polls, &evidence);
        assert!(evidence.matches(&pair, &source, boundary, domain, 80.));
        assert!(!evidence.matches(&pair, &source, boundary, domain, 81.));
    });
}
fn assert_legacy_work(
    old: &CertifiedConicalTrimPair,
    new: &CertifiedConicalTrimPair,
    old_polls: usize,
    new_polls: usize,
    evidence: &ConicalDevelopedBoundaryEvidence,
) {
    assert_eq!(old.operations(), new.operations());
    assert_eq!(old_polls, new_polls);
    assert_eq!(format!("{old:?}"), format!("{new:?}"));
    assert_eq!(evidence.operations(), new.operations());
}
#[test]
fn exact_meridian_material_reuses_actual_operands_and_rejects_legacy_matching() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let proof =
            certify_conical_material_with_developed_boundaries(query, &contexts, domain, || false)
                .unwrap();
        assert!(proof.spatial_error() < 1e-10);
        assert!(proof.matches_developed(&query, &contexts, domain));
        assert!(!proof.matches(&query));
        assert!(proof.operations() < 1000);
    });
}
#[test]
fn actual_angular_error_bounds_independent_normal_circle_chord() {
    with_fixture(|pair, source, boundary, domain| {
        let uv = [[1e-7, 0.]; 2];
        let pair = ConicalTrimPairQuery {
            curves: pair.curves,
            pcurves: [&uv; 2],
            carriers: pair.carriers,
            spatial_limits: pair.spatial_limits,
            options: pair.options,
        };
        let proof = certify_material(pair, source, boundary, domain);
        let exact = 2. * pair.carriers[1].radius * martensite_cmath::sin(0.5e-7);
        assert!(proof.spatial_error() + 1e-12 >= exact);
        assert!(proof.spatial_error() < 2e-6);
    });
}
#[test]
fn actual_symmetric_slant_error_is_not_assumed_zero() {
    with_fixture(|pair, source, boundary, domain| {
        let uv = [[0., 1e-7]; 2];
        let pair = ConicalTrimPairQuery {
            curves: pair.curves,
            pcurves: [&uv; 2],
            carriers: pair.carriers,
            spatial_limits: pair.spatial_limits,
            options: pair.options,
        };
        let proof = certify_material(pair, source, boundary, domain);
        assert!(proof.spatial_error() >= 1e-7);
        assert!(proof.spatial_error() < 2e-7);
    });
}
#[test]
fn source_uv_reference_and_grid_mutations_refuse_before_cancellation() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let mut changed = query;
        changed.reference_slant = 81.;
        assert_binding(changed, &contexts, domain);
        changed = query;
        changed.options.fractional_bits = 64;
        assert_binding(changed, &contexts, domain);
        let changed_spans = [span(source, &[[0., 1e-8]; 2])];
        changed = query;
        changed.spans = &changed_spans;
        assert_binding(changed, &contexts, domain);
    });
}
fn assert_binding(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
) {
    let error =
        certify_conical_material_with_developed_boundaries(query, contexts, domain, || true)
            .unwrap_err();
    assert_eq!(error.operations, 0);
    assert!(!error.is_cancelled());
    assert!(
        matches!(error.cause.as_ref(), ConicalDevelopedMaterialFailure::Binding { actual, expected, .. } if actual != expected)
    );
}
#[test]
fn cancellation_and_exact_remaining_work_are_charged_without_recharging_witness() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let mut query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let error =
            certify_conical_material_with_developed_boundaries(query, &contexts, domain, || true)
                .unwrap_err();
        assert!(error.is_cancelled());
        query.options.max_operations = 1;
        let error =
            certify_conical_material_with_developed_boundaries(query, &contexts, domain, || false)
                .unwrap_err();
        assert_eq!(error.operations, 1);
        assert!(!error.is_cancelled());
    });
}
#[test]
fn developed_witness_public_types_are_immutable_send_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalDevelopedBoundaryEvidence>();
    shared::<ConicalDevelopedBoundarySpan<'static>>();
    shared::<ConicalDevelopedMaterialError>();
    shared::<ConicalDevelopedMaterialFailure>();
}
#[test]
fn all_negative_depths_keep_the_developed_angular_radius_bound() {
    let domain = fixture::domain(0.125);
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 100_000,
        },
        &mut never,
    );
    let radius = negative_depth_radius(&domain, &mut arithmetic);
    let upper = arithmetic.upper_f64(radius.upper).unwrap();
    let developed = 80. * martensite_cmath::sin(0.125);
    assert!(upper >= developed);
    assert!(80. * martensite_cmath::sin(0.125) - 0.2 * martensite_cmath::cos(0.125) < developed);
}

fn contexts<'a>(
    source: ConicalBoundaryQuery<'a>,
    boundary: &'a ConicalBoundaryEvidence,
    evidence: &'a ConicalDevelopedBoundaryEvidence,
) -> [ConicalDevelopedBoundarySpan<'a>; 1] {
    [ConicalDevelopedBoundarySpan {
        source,
        boundary,
        evidence,
    }]
}

fn with_counted_capture(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
) -> (
    CertifiedConicalTrimPair,
    ConicalDevelopedBoundaryEvidence,
    usize,
) {
    let mut new_polls = 0;
    let (new, evidence) = certify_conical_trim_pair_with_developed_boundary(
        pair,
        source,
        boundary,
        domain,
        80.,
        || {
            new_polls += 1;
            false
        },
    )
    .unwrap();
    (new, evidence, new_polls)
}
#[test]
fn exact_original_dyadic_and_actual_domain_mutations_cannot_reuse_the_witness() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let mut spans = [span(source, pair.pcurves[0])];
        spans[0].interval = CylindricalProfileInterval::new(1, 1).unwrap();
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        assert_binding(query, &contexts, domain);
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        assert_binding(query, &contexts, &fixture::domain(0.25));
    });
}
#[test]
fn actual_optional_weights_and_second_cap_uv_are_in_the_issued_source_binding() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let mut changed = pair;
        changed.curves[0].weights = None;
        assert!(!evidence.matches(&changed, &source, boundary, domain, 80.));
        changed = pair;
        changed.pcurves[1] = &[[0., 1e-8]; 2];
        assert!(!evidence.matches(&changed, &source, boundary, domain, 80.));
    });
}

fn negative_depth_radius(
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Interval {
    let numerator = vec![arithmetic.scalar(80.).unwrap(); 2];
    let weights = vec![arithmetic.scalar(1.).unwrap(); 2];
    super::super::super::spatial::chart_radius(
        &numerator,
        &weights,
        [-0.4, -0.2],
        domain,
        arithmetic,
    )
    .unwrap()
}

mod constants;

mod selected;
