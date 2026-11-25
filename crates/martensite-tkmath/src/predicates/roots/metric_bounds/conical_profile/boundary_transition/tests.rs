// SPDX-License-Identifier: GPL-2.0-only
//! Complete positive-weight row motion and strict immutable authority controls.
use super::super::boundary_chart::tests as fixture;
use super::*;
const SELECTED: ConicalPrimitiveRefinement = ConicalPrimitiveRefinement::HalfSplitV1;
fn with_transition(offset: f64, body: impl FnOnce(ConicalBoundaryZRowTransition<'_>)) {
    let poles = offset_poles(offset);
    let previous = fixture::query(&poles);
    let domain = fixture::domain(0.125);
    let parent = nearest(previous, &domain);
    let gamma = gamma(previous, &parent, &domain, &poles);
    let current_poles = moved(&poles, &gamma);
    let mut current = previous;
    for (edge, row) in current.edges.iter_mut().zip(&current_poles) {
        edge.stored_poles = row;
    }
    body(ConicalBoundaryZRowTransition {
        span: ConicalActualSideId(7),
        parent: &parent,
        previous,
        current,
        domain: &domain,
        refinement: SELECTED,
        gamma: &gamma,
    });
}
fn offset_poles(offset: f64) -> [[[f64; 3]; 2]; 2] {
    let mut poles = fixture::poles();
    for row in &mut poles {
        for pole in row {
            pole[2] += offset;
        }
    }
    poles
}
fn gamma(
    previous: ConicalBoundaryQuery<'_>,
    parent: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    poles: &[[[f64; 3]; 2]; 2],
) -> ConicalDirectedGammaEvidence {
    let caps = caps(poles);
    let spans = [ConicalDirectedGammaSpan {
        id: ConicalActualSideId(7),
        member: ConicalActualMemberId(1),
        previous,
        parent,
        domain,
        caps: [ConicalActualCapId(0), ConicalActualCapId(1)],
    }];
    certify_conical_directed_gamma(
        ConicalDirectedGammaQuery {
            spans: &spans,
            caps: &caps,
            refinement: SELECTED,
            options: fixture::options(),
        },
        || false,
    )
    .unwrap()
}
fn caps(poles: &[[[f64; 3]; 2]; 2]) -> [ConicalActualCapCarrier; 2] {
    std::array::from_fn(|i| {
        let z = poles[i][0][2];
        ConicalActualCapCarrier {
            id: ConicalActualCapId(i as u64),
            member: ConicalActualMemberId(1),
            frame: [[0., 0., z], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            radius: poles[i][0][0],
            axial_origin: z,
            semi_angle: 0.125,
        }
    })
}
fn moved(poles: &[[[f64; 3]; 2]; 2], gamma: &ConicalDirectedGammaEvidence) -> [[[f64; 3]; 2]; 2] {
    std::array::from_fn(|row| {
        poles[row].map(|p| [p[0], p[1], gamma.transform_z(row, p[2]).unwrap()])
    })
}
fn issue(input: ConicalBoundaryZRowTransition<'_>) -> ConicalBoundaryZRowEvidence {
    certify_conical_boundary_z_rows(input, fixture::options(), || false).unwrap()
}
#[test]
fn actual_nonzero_storage_is_composed_and_parent_remains_immutable() {
    with_transition(1e-7, |input| {
        let before = format!("{:?}", input.parent);
        let proof = issue(input);
        assert!(proof.boundary().matches_chart_with_refinement(
            &input.current,
            input.domain,
            SELECTED
        ));
        assert!(!input.parent.matches(&input.current));
        assert!(!proof.boundary().matches_chart(&input.current, input.domain));
        assert_eq!(before, format!("{:?}", input.parent));
        assert!(proof.boundary().storage()[0] > input.parent.storage()[0]);
        assert!(proof.capture.parent_storage[0].upper > 0.into());
        assert_eq!(
            proof.cumulative_operations(),
            u64::from(input.parent.operations()) + u64::from(proof.incremental_operations())
        );
        assert_eq!(proof.remaining_options(), fixture::options());
    });
}
#[test]
fn final_current_source_domain_refinement_and_grid_are_bound() {
    with_transition(0., |input| {
        let proof = issue(input);
        assert!(proof.matches_current(&input.current, input.domain, SELECTED, 128));
        assert!(!proof.matches_current(&input.previous, input.domain, SELECTED, 128));
        assert!(!proof.matches_current(
            &input.current,
            input.domain,
            ConicalPrimitiveRefinement::UnsplitV1,
            128
        ));
        assert!(!proof.matches_current(&input.current, input.domain, SELECTED, 64));
        assert!(!proof.matches_current(&input.current, &fixture::domain(0.25), SELECTED, 128));
        assert!(Arc::ptr_eq(
            &proof.boundary_shared(),
            &proof.boundary_shared()
        ));
        assert!(proof.capture.matches_gamma(input.gamma));
    });
}
fn assert_binding(input: ConicalBoundaryZRowTransition<'_>) {
    let mut polls = 0;
    let error = certify_conical_boundary_z_rows(input, fixture::options(), || {
        polls += 1;
        true
    })
    .unwrap_err();
    assert_eq!(error.operations, 0);
    assert_eq!(polls, 0);
    assert!(!error.is_cancelled());
}
#[test]
fn changed_source_weights_mode_and_actual_xy_refuse_before_cancel() {
    with_transition(0., |input| {
        let mut changed = input;
        changed.current.source = &[[81., 0.]; 2];
        assert_binding(changed);
        changed = input;
        changed.current.edges[0].stored_weights = &[1., 2.];
        assert_binding(changed);
        changed = input;
        changed.refinement = ConicalPrimitiveRefinement::UnsplitV1;
        assert_binding(changed);
        let mut poles = input.current.edges[0].stored_poles.to_vec();
        poles[0][0] += 1e-8;
        changed = input;
        changed.current.edges[0].stored_poles = &poles;
        assert_binding(changed);
    });
}
#[test]
fn wrong_issued_row_and_labels_cannot_be_relabelled() {
    with_transition(0., |input| {
        let mut changed = input;
        changed.current = input.previous;
        assert_binding(changed);
        changed = input;
        changed.span = ConicalActualSideId(8);
        assert_binding(changed);
    });
}
#[test]
fn cancellation_and_exhaustion_keep_actual_incremental_work() {
    with_transition(0., |input| {
        let cancelled =
            certify_conical_boundary_z_rows(input, fixture::options(), || true).unwrap_err();
        assert!(cancelled.is_cancelled());
        assert_eq!(cancelled.operations, 0);
        let mut limit = fixture::options();
        limit.max_operations = 1;
        let error = certify_conical_boundary_z_rows(input, limit, || false).unwrap_err();
        assert_eq!(error.operations, 1);
        assert!(!error.is_cancelled());
    });
}
#[test]
fn transition_public_receipts_are_send_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<ConicalBoundaryZRowEvidence>();
    shared::<ConicalBoundaryZRowError>();
    shared::<ConicalBoundaryZRowFailure>();
    shared::<ConicalBoundaryZRowTransition<'static>>();
}

fn nearest(
    query: ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
) -> ConicalBoundaryEvidence {
    certify_conical_boundary_in_chart_with_refinement(
        query,
        domain,
        fixture::options(),
        SELECTED,
        || false,
    )
    .unwrap()
}
#[test]
fn changed_original_parent_grant_cannot_borrow_gamma_authority() {
    with_transition(0., |input| {
        let mut options = fixture::options();
        options.max_operations -= 1;
        let parent = certify_conical_boundary_in_chart_with_refinement(
            input.previous,
            input.domain,
            options,
            SELECTED,
            || false,
        )
        .unwrap();
        let mut changed = input;
        changed.parent = &parent;
        assert_binding(changed);
    });
}
#[test]
fn composed_limit_refusal_preserves_actual_bound_and_work() {
    with_transition(2e-5, |input| {
        let error =
            certify_conical_boundary_z_rows(input, fixture::options(), || false).unwrap_err();
        assert!(error.operations > 0);
        assert!(!error.is_cancelled());
        let ConicalBoundaryZRowFailure::Boundary(cause) = error.cause.as_ref() else {
            panic!("{error}");
        };
        assert_eq!(cause.stage(), ConicalBoundaryStage::Budget);
        let ConicalBoundaryFailure::Proof(proof) = cause.cause() else {
            panic!("{cause}");
        };
        let (bound, limit) = proof.budget().unwrap();
        assert!(bound.binary64_bounds().unwrap()[0] > limit);
    });
}
#[test]
fn final_boundary_is_consumed_by_selected_trim_and_join_paths() {
    with_transition(0., |input| {
        let proof = issue(input);
        assert_selected_trim_capture(input, &proof);
        assert_selected_join_capture(input, &proof);
    });
}
fn actual_pair<'a>(
    current: ConicalBoundaryQuery<'a>,
    previous: ConicalBoundaryQuery<'a>,
) -> super::super::super::ConicalTrimPairQuery<'a> {
    use super::super::super::{ConicalTrimCarrier, ConicalTrimPairQuery, MetricBezier};
    ConicalTrimPairQuery {
        curves: current.edges.map(|edge| MetricBezier {
            poles: edge.stored_poles,
            weights: Some(edge.stored_weights),
        }),
        pcurves: [&[[0., 0.]; 2]; 2],
        carriers: previous.edges.map(|edge| ConicalTrimCarrier {
            radius: edge.stored_poles[0][0],
            axial_origin: edge.stored_poles[0][2],
            semi_angle: edge.semi_angle,
        }),
        spatial_limits: [1e-4; 2],
        options: trim_options(),
    }
}
fn assert_selected_join_capture(
    input: ConicalBoundaryZRowTransition<'_>,
    proof: &ConicalBoundaryZRowEvidence,
) {
    let spans = [ConicalJoinSpan {
        query: input.current,
        boundary: proof.boundary(),
    }];
    let query = zero_join_query(&spans);
    let error =
        certify_conical_joins_in_chart_with_refinement(query, input.domain, SELECTED, || false)
            .unwrap_err();
    assert!(!error.is_cancelled());
    assert_eq!(error.operations, 0);
    assert!(
        matches!(error.cause.as_ref(),ConicalJoinsFailure::Proof {cause,..} if matches!(cause.arithmetic_cause(),Some(BoundedArithmeticFailure::WorkLimit {completed:0,..})))
    );
    let mismatched =
        certify_conical_joins_in_chart(zero_join_query(&spans), input.domain, || true).unwrap_err();
    assert!(!mismatched.is_cancelled());
    assert_eq!(mismatched.operations, 0);
}

fn assert_selected_trim_capture(
    input: ConicalBoundaryZRowTransition<'_>,
    proof: &ConicalBoundaryZRowEvidence,
) {
    let pair = actual_pair(input.current, input.previous);
    let trace = certify_conical_trim_pair_in_chart_with_refinement(
        pair,
        input.current,
        proof.boundary(),
        input.domain,
        SELECTED,
        || false,
    )
    .unwrap();
    assert!(trace.matches_source(pair));
    let legacy = certify_conical_trim_pair_in_chart(
        pair,
        input.current,
        proof.boundary(),
        input.domain,
        || false,
    );
    assert!(legacy.is_err());
}

fn trim_options() -> super::super::super::CylindricalProfileOptions {
    super::super::super::CylindricalProfileOptions {
        fractional_bits: 128,
        max_operations: 100_000,
        ..super::super::super::CylindricalProfileOptions::default()
    }
}

fn zero_join_query<'a>(spans: &'a [ConicalJoinSpan<'a>]) -> ConicalJoinsQuery<'a> {
    ConicalJoinsQuery {
        spans,
        contour_lengths: &[1],
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 0,
        },
    }
}
