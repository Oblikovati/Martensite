// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::boundary_chart::tests::{domain, options, poles, query};
use super::*;
pub(super) fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut never);
    body(&mut arithmetic)
}
pub(super) fn normalized<'a>(query: &'a ConicalBoundaryQuery<'a>) -> SpatialRequest<'a> {
    let mut request = super::super::super::boundary::combined_request(query).spatial;
    request.original = Some(ConicalOriginalSpan {
        poles: query.source,
        index: 0,
        level: 0,
    });
    request
}
#[test]
fn actual_retained_projection_requires_full_query_domain_and_grid() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let proof = certify_conical_boundary_in_chart(query, &domain, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &proof,
    };
    let request = normalized(&span.query);
    execute(|arithmetic| {
        prepare_projection_in_chart(&request, &span, &domain, arithmetic).unwrap();
    });
    assert_changed_anchor(&span, &domain);
}
#[test]
fn legacy_boundary_cannot_be_relabelled_as_a_retained_chart_projection() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let proof = certify_conical_boundary(query, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &proof,
    };
    let request = normalized(&span.query);
    execute(|arithmetic| {
        assert!(matches!(
            prepare_projection_in_chart(&request, &span, &domain, arithmetic),
            Err(CombinedFailure::JoinMismatch)
        ));
        assert_eq!(arithmetic.operations(), 0);
    });
}
#[test]
fn retained_capture_owns_one_shared_immutable_allocation() {
    let poles = poles();
    let query = query(&poles);
    let proof =
        certify_conical_boundary_in_chart(query, &domain(0.125), options(), || false).unwrap();
    let retained = proof.retained.as_ref().unwrap();
    let shared = std::sync::Arc::clone(retained);
    assert!(std::sync::Arc::ptr_eq(retained, &shared));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RetainedSpatial>();
}
#[test]
fn changed_grid_is_refused_before_using_retained_integer_units() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let proof = certify_conical_boundary_in_chart(query, &domain, options(), || false).unwrap();
    let span = ConicalJoinSpan {
        query,
        boundary: &proof,
    };
    let request = normalized(&span.query);
    let mut never = || false;
    let mut policy = sector_api::policy(options());
    policy.fractional_bits = 64;
    let mut arithmetic = Arithmetic::new(policy, &mut never);
    assert!(matches!(
        prepare_projection_in_chart(&request, &span, &domain, &mut arithmetic),
        Err(CombinedFailure::JoinMismatch)
    ));
    assert_eq!(arithmetic.operations(), 0);
}

fn assert_changed_anchor(span: &ConicalJoinSpan<'_>, domain: &ConicalChartDomainEvidence) {
    let mut changed = normalized(&span.query);
    changed.anchor.point[1] = 1.;
    execute(|arithmetic| {
        assert!(matches!(
            prepare_projection_in_chart(&changed, span, domain, arithmetic),
            Err(CombinedFailure::JoinMismatch)
        ));
    });
}
