// SPDX-License-Identifier: GPL-2.0-only
use super::tests as fixture;
use super::*;
fn counted(
    query: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    explicit: bool,
) -> (ConicalBoundaryEvidence, usize) {
    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        false
    };
    let proof = if explicit {
        certify_conical_boundary_in_chart_with_refinement(
            query,
            domain,
            fixture::options(),
            ConicalPrimitiveRefinement::UnsplitV1,
            &mut cancel,
        )
    } else {
        certify_conical_boundary_in_chart(query, domain, fixture::options(), &mut cancel)
    }
    .unwrap();
    (proof, polls)
}
#[test]
fn explicit_unsplit_is_byte_work_and_poll_identical_to_existing_chart_entry() {
    let poles = fixture::poles();
    let query = fixture::query(&poles);
    let domain = fixture::domain(0.125);
    let (old, old_polls) = counted(query, &domain, false);
    let (new, new_polls) = counted(query, &domain, true);
    assert_eq!(format!("{old:?}"), format!("{new:?}"));
    assert_eq!(old_polls, new_polls);
    assert_eq!(old.operations(), new.operations());
    assert_eq!(
        new.primitive_refinement(),
        Some(ConicalPrimitiveRefinement::UnsplitV1)
    );
}
#[test]
fn half_split_receipt_cannot_authorize_the_existing_unsplit_selector() {
    let poles = fixture::poles();
    let query = fixture::query(&poles);
    let domain = fixture::domain(0.125);
    let half = ConicalPrimitiveRefinement::HalfSplitV1;
    let proof = certify_conical_boundary_in_chart_with_refinement(
        query,
        &domain,
        fixture::options(),
        half,
        || false,
    )
    .unwrap();
    assert!(proof.matches_chart_with_refinement(&query, &domain, half));
    assert!(!proof.matches_chart(&query, &domain));
    assert!(proof.retained_in_chart(&query, &domain, 128).is_none());
    assert!(
        proof
            .retained_in_chart_with_refinement(&query, &domain, 128, half)
            .is_some()
    );
}
