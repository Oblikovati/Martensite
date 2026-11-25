// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::certify_conical_material_with_developed_boundaries_and_refinement as selected_material;
const HALF: ConicalPrimitiveRefinement = ConicalPrimitiveRefinement::HalfSplitV1;
fn with_selected(
    body: impl FnOnce(
        ConicalTrimPairQuery<'_>,
        ConicalBoundaryQuery<'_>,
        &ConicalBoundaryEvidence,
        &ConicalChartDomainEvidence,
        &ConicalDevelopedBoundaryEvidence,
    ),
) {
    with_fixture(|pair, source, _, domain| {
        let boundary = certify_conical_boundary_in_chart_with_refinement(
            source,
            domain,
            fixture::options(),
            HALF,
            || false,
        )
        .unwrap();
        let evidence = selected_witness(pair, source, &boundary, domain);
        body(pair, source, &boundary, domain, &evidence);
    });
}
#[test]
fn explicit_unsplit_material_preserves_exact_bounds_work_and_poll_order() {
    with_witness(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let (old, old_polls) = counted_material(query, &contexts, domain, false);
        let (new, new_polls) = counted_material(query, &contexts, domain, true);
        assert_eq!(old.spatial_error().to_bits(), new.spatial_error().to_bits());
        assert_eq!(old.operations(), new.operations());
        assert_eq!(old_polls, new_polls);
        assert!(old.matches_developed(&query, &contexts, domain));
        assert!(new.matches_developed_with_refinement(
            &query,
            &contexts,
            domain,
            ConicalPrimitiveRefinement::UnsplitV1
        ));
    });
}
#[test]
fn half_split_witness_requires_explicit_material_authority() {
    with_selected(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let proof = selected_material(query, &contexts, domain, HALF, || false).unwrap();
        assert!(proof.matches_developed_with_refinement(&query, &contexts, domain, HALF));
        assert!(!proof.matches_developed(&query, &contexts, domain));
        assert_wrong_mode(query, &contexts, domain);
        assert!(!evidence.matches(&pair, &source, boundary, domain, 80.));
        assert!(evidence.matches_with_refinement(&pair, &source, boundary, domain, 80., HALF));
    });
}
#[test]
fn selected_material_rejects_actual_uv_source_domain_and_grid_mutation() {
    with_selected(|pair, source, boundary, domain, evidence| {
        let spans = [span(source, pair.pcurves[0])];
        let query = material(source, pair, &spans);
        let contexts = contexts(source, boundary, evidence);
        let proof = selected_material(query, &contexts, domain, HALF, || false).unwrap();
        let changed = [span(source, &[[0., 1e-8]; 2])];
        assert_selected_refusal(
            ConicalMaterialQuery {
                spans: &changed,
                ..query
            },
            &contexts,
            domain,
        );
        assert_selected_refusal(query, &contexts, &fixture::domain(0.25));
        let mut changed = query;
        changed.options.fractional_bits = 64;
        assert_selected_refusal(changed, &contexts, domain);
        assert!(proof.matches_developed_with_refinement(&query, &contexts, domain, HALF));
    });
}
fn assert_selected_refusal(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
) {
    let error = selected_material(query, contexts, domain, HALF, || true).unwrap_err();
    assert_eq!(error.operations, 0);
    assert!(!error.is_cancelled());
}

fn selected_witness(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
) -> ConicalDevelopedBoundaryEvidence {
    certify_conical_trim_pair_with_developed_boundary_and_refinement(
        pair,
        source,
        boundary,
        domain,
        80.,
        HALF,
        || false,
    )
    .unwrap()
    .1
}
fn counted_material(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    explicit: bool,
) -> (ConicalMaterialEvidence, usize) {
    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        false
    };
    let proof = if explicit {
        selected_material(
            query,
            contexts,
            domain,
            ConicalPrimitiveRefinement::UnsplitV1,
            &mut cancel,
        )
    } else {
        certify_conical_material_with_developed_boundaries(query, contexts, domain, &mut cancel)
    }
    .unwrap();
    (proof, polls)
}

fn assert_wrong_mode(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
) {
    let mut polls = 0;
    let error = selected_material(
        query,
        contexts,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
        || {
            polls += 1;
            true
        },
    )
    .unwrap_err();
    assert_eq!(error.operations, 0);
    assert_eq!(polls, 0);
    assert!(!error.is_cancelled());
}
