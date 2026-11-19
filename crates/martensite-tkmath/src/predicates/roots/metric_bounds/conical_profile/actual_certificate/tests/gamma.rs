// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn issued_gamma_produces_actual_directed_movement_and_owns_complete_sources() {
    fixture(|query| {
        let proof = certify_conical_directed_gamma(query, || false).unwrap();
        assert!(proof.matches(&query));
        assert!(proof.operations() > 0);
        let old = query.spans[0].previous.edges[0].stored_poles[0][2];
        assert!(proof.transform_z(0, old).unwrap() < old);
        assert!(proof.transform_z(1, old).unwrap() > old);
        let cloned = proof.clone();
        assert!(cloned.same_authority(&proof));
        let mut changed = query;
        changed.options.max_operations -= 1;
        assert!(!proof.matches(&changed));
        let mut caps = query.caps.to_vec();
        caps[0].frame[1][1] = -0.;
        let changed = ConicalDirectedGammaQuery {
            caps: &caps,
            ..query
        };
        assert!(!proof.matches(&changed));
    });
}
#[test]
fn wrong_selected_parent_refuses_actual_side_before_later_cancellation() {
    fixture(|mut query| {
        query.refinement = ConicalPrimitiveRefinement::HalfSplitV1;
        let mut polls = 0;
        let failure = certify_conical_directed_gamma(query, || {
            polls += 1;
            true
        })
        .unwrap_err();
        assert_eq!(polls, 0);
        wrong_parent_failure(&failure);
    });
}
#[test]
fn actual_nonfinite_cap_keeps_original_payload_and_cap_identity() {
    fixture(|query| {
        let mut caps = query.caps.to_vec();
        let invalid = f64::from_bits(0x7ff8_0000_0000_0498);
        caps[0].axial_origin = invalid;
        caps[0].frame[0][2] = invalid;
        let query = ConicalDirectedGammaQuery {
            caps: &caps,
            ..query
        };
        let failure = certify_conical_directed_gamma(query, || true).unwrap_err();
        assert_eq!(failure.cap, Some(ConicalActualCapId(10)));
        assert_eq!(failure.operations, 0);
        if let ConicalActualPatchFailure::Value { actual, .. } = failure.cause.as_ref() {
            assert_eq!(actual.to_bits(), invalid.to_bits());
        } else {
            panic!("original nonfinite cap payload missing: {failure:?}")
        }
    });
}
#[test]
fn actual_wrong_query_poles_refuse_the_named_side_with_source_operands() {
    fixture(|query| {
        let source = [[81., 0.], [80., 0.]];
        let previous = ConicalBoundaryQuery {
            source: &source,
            ..query.spans[0].previous
        };
        let span = ConicalDirectedGammaSpan {
            previous,
            ..query.spans[0]
        };
        let spans = [span];
        let query = ConicalDirectedGammaQuery {
            spans: &spans,
            ..query
        };
        let failure = certify_conical_directed_gamma(query, || true).unwrap_err();
        wrong_source_failure(&failure);
    });
}
#[test]
fn zero_remaining_grant_reports_real_worklimit_and_no_issued_policy() {
    fixture(|mut query| {
        query.options.max_operations = 0;
        let failure = certify_conical_directed_gamma(query, || false).unwrap_err();
        assert_eq!(failure.operations, 0);
        assert_eq!(failure.side, Some(ConicalActualSideId(9)));
        assert!(matches!(
            failure.cause.as_ref(),
            ConicalActualPatchFailure::Arithmetic(BoundedArithmeticFailure::WorkLimit {
                completed: 0,
                limit: 0
            })
        ));
    });
}
#[test]
fn cancellation_at_every_observed_checkpoint_returns_original_cancelled() {
    fixture(|query| {
        let mut count = 0;
        certify_conical_directed_gamma(query, || {
            count += 1;
            false
        })
        .unwrap();
        for stop in 1..=count {
            let mut polls = 0;
            let error = certify_conical_directed_gamma(query, || {
                polls += 1;
                polls == stop
            })
            .unwrap_err();
            assert!(error.is_cancelled(), "checkpoint={stop}: {error:?}");
            assert_eq!(polls, stop);
        }
    });
}
#[test]
fn gamma_transform_rejects_unsupported_row_and_nonfinite_source() {
    fixture(|query| {
        let proof = certify_conical_directed_gamma(query, || false).unwrap();
        assert!(matches!(
            proof.transform_z(2, 80.).unwrap_err().cause.as_ref(),
            ConicalActualPatchFailure::Shape { actual: 2, .. }
        ));
        assert!(
            matches!(proof.transform_z(0,f64::INFINITY).unwrap_err().cause.as_ref(),ConicalActualPatchFailure::Value {actual,..} if actual.is_infinite())
        );
    });
}

fn wrong_parent_failure(failure: &ConicalActualPatchError) {
    assert_eq!(failure.operations, 0);
    assert_eq!(failure.side, Some(ConicalActualSideId(9)));
    assert!(matches!(
        failure.cause.as_ref(),
        ConicalActualPatchFailure::Binding {
            field: "nearest boundary authority",
            ..
        }
    ));
    if let ConicalActualPatchFailure::Binding {
        actual, expected, ..
    } = failure.cause.as_ref()
    {
        assert_ne!(actual, expected);
    }
}

fn wrong_source_failure(failure: &ConicalActualPatchError) {
    assert_eq!(failure.side, Some(ConicalActualSideId(9)));
    assert_eq!(failure.operations, 0);
    assert!(matches!(
        failure.cause.as_ref(),
        ConicalActualPatchFailure::Binding { .. }
    ));
}
