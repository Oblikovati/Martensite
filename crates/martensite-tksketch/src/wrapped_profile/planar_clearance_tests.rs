//! Margin failures are diagnosed before wrapped native construction.
use super::super::{subdivision, tests::request};
use super::*;

#[test]
fn mixed_profile_covers_every_nonadjacent_pair() {
    let original = request();
    let prepared = subdivision::prepare(&original).unwrap();
    let proof = certify(&original, &prepared).unwrap();
    assert_eq!(proof.certificate.pair_count(), 152);
    assert_eq!(proof.certificate.excluded_pairs().len(), 19);
    assert!(proof.certificate.clearance_lower_bound() >= 4. * original.tolerance);
    assert!(
        proof
            .periodic
            .separation_lower_bound()
            .compare_f64(4. * original.tolerance)
            .unwrap()
            .is_ge()
    );
}

#[test]
fn narrow_original_gap_refuses_before_wrapped_native_construction() {
    let mut original = request();
    for pole in original.loops[1]
        .iter_mut()
        .flat_map(|piece| &mut piece.poles)
    {
        if pole[0] == 0.5 {
            pole[0] = 1e-6;
        }
    }
    let prepared = subdivision::prepare(&original).unwrap();
    let failure = certify(&original, &prepared).err().unwrap();
    assert!(matches!(failure, ClearanceFailure::Certificate { .. }));
    let error = super::super::pipeline::wrap_extrude(&original)
        .err()
        .unwrap();
    assert_eq!(error.stage, Stage::Clearance);
}

#[test]
fn later_o2_sources_must_match_the_preconstruction_clearance_proof() {
    let (mut original, prepared, stored) = super::super::planar_cones::tests::proofs();
    let evidence = certify(&original, &prepared).unwrap();
    bind(&evidence, &original, &prepared, &stored).unwrap();
    original.loops[0][0].poles[1][0] += 0.01;
    assert!(matches!(
        bind(&evidence, &original, &prepared, &stored)
            .err()
            .unwrap(),
        ClearanceFailure::Binding { .. }
    ));
}

#[test]
fn global_boundary_arithmetic_observes_cancellation_without_a_partial_certificate() {
    let original = request();
    let prepared = subdivision::prepare(&original).unwrap();
    let mut polls = 0;
    let failure = certify_many_with_cancel(&[original], &[prepared], || {
        polls += 1;
        polls >= 8
    })
    .err()
    .unwrap();
    assert_eq!(polls, 8);
    assert_cancelled_certificate(failure);
}
fn assert_cancelled_certificate(failure: ClearanceFailure) {
    let ClearanceFailure::Certificate { spans, cause } = failure else {
        panic!("certificate failure")
    };
    assert_eq!(spans.len(), 19);
    assert_eq!(cause.source_count, 19);
    assert_eq!(cause.requested_clearance, 4e-5);
    assert_cancelled_arithmetic(&cause.cause);
}
fn assert_cancelled_arithmetic(
    cause: &martensite_tkmath::predicates::roots::metric_bounds::CylindricalClearanceFailure,
) {
    use martensite_tkmath::predicates::roots::metric_bounds::{
        BoundedArithmeticFailure, CylindricalClearanceFailure,
    };
    match cause {
        CylindricalClearanceFailure::Arithmetic(cause) => {
            assert_eq!(**cause, BoundedArithmeticFailure::Cancelled)
        }
        CylindricalClearanceFailure::PeriodicImage { cause, .. } => {
            assert_cancelled_arithmetic(&cause.cause)
        }
        other => panic!("expected cancellation, got {other:?}"),
    }
}
#[test]
fn the_same_callback_reaches_periodic_work_after_global_boundary_completion() {
    let original = request();
    let prepared = subdivision::prepare(&original).unwrap();
    let mut within_polls = 0;
    within_certificate(&original, &prepared, &mut || {
        within_polls += 1;
        false
    });
    let mut polls = 0;
    let failure = certify_many_with_cancel(&[original], &[prepared], || {
        polls += 1;
        polls > within_polls
    })
    .err()
    .unwrap();
    assert_eq!(polls, within_polls + 1);
    assert_cancelled_certificate(failure);
}

fn within_certificate(original: &Request, prepared: &Request, cancelled: &mut CancelPoll<'_>) {
    let authored = authored_collection(std::slice::from_ref(original)).unwrap();
    let spans = prepared_spans(std::slice::from_ref(prepared));
    let sources = sources(&spans, &authored).unwrap();
    let excluded = excluded_pairs(std::slice::from_ref(prepared));
    certify_input(
        clearance_query(original, &sources, &excluded),
        &spans,
        cancelled,
    )
    .unwrap();
}
#[test]
fn cancellation_inside_the_last_periodic_image_keeps_its_turn_and_arithmetic_cause() {
    let original = subdivision::tests::helical_request();
    let prepared = subdivision::prepare(&original).unwrap();
    let total = count_boundary_polls(&original, &prepared);
    let mut polls = 0;
    let failure = certify_many_with_cancel(&[original], &[prepared], || {
        polls += 1;
        polls >= total
    })
    .err()
    .unwrap();
    assert_image_cancelled(failure, 2);
    assert_eq!(polls, total);
}
fn count_boundary_polls(original: &Request, prepared: &Request) -> usize {
    let mut polls = 0;
    let result = certify_many_with_cancel(
        std::slice::from_ref(original),
        std::slice::from_ref(prepared),
        || {
            polls += 1;
            false
        },
    )
    .unwrap();
    assert_eq!(result.periodic.images().len(), 2);
    polls
}

fn assert_image_cancelled(failure: ClearanceFailure, expected: u32) {
    use martensite_tkmath::predicates::roots::metric_bounds::CylindricalClearanceFailure;
    let ClearanceFailure::Certificate { cause, .. } = failure else {
        panic!("certificate")
    };
    let CylindricalClearanceFailure::PeriodicImage { turn, .. } = &cause.cause else {
        panic!("periodic image")
    };
    assert_eq!(*turn, expected);
    assert_cancelled_arithmetic(&cause.cause);
}
