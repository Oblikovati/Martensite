//! Stop actual retained proof work without returning partial certificates or changing geometry.
use super::tests::frozen;
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundedArithmeticFailure, CylindricalProfileFailure, CylindricalStorageFailure,
};
#[derive(Default)]
struct ProofPolls {
    storage: usize,
    profile: usize,
    trims: usize,
    cap_profile: usize,
    span: usize,
}
fn phase_polls(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
) -> ProofPolls {
    let indexed = index(bindings).unwrap();
    let piece = &prepared.loops[0][0];
    let curves = captured_span(piece, &indexed).unwrap();
    let authored = authored_index(original).unwrap();
    let mut polls = ProofPolls::default();
    polls.storage = count_storage(prepared, piece, [&curves[0], &curves[1]]);
    polls.profile =
        polls.storage + count_profile(prepared, piece, &curves[0], authored[&piece.span.curve]);
    polls.trims = polls.profile + count_trims(prepared, piece, [&curves[0], &curves[1]], &indexed);
    polls.cap_profile = polls.trims + count_cap_profile(prepared, piece, &curves[0]);
    polls.span = count_span(prepared, piece, &indexed, &authored);
    polls
}

fn count_poll(polls: &mut usize) -> bool {
    *polls += 1;
    false
}
fn count_storage(request: &Request, piece: &AuthoredPiece, curves: [&OwnedCurve; 2]) -> usize {
    let mut polls = 0;
    let source = key(piece.span, HistoryRole::FirstDepthEdge);
    prove_captured(request, source, curves, &mut || count_poll(&mut polls)).unwrap();
    polls
}
fn count_profile(
    request: &Request,
    piece: &AuthoredPiece,
    curve: &OwnedCurve,
    authored: &[[f64; 2]],
) -> usize {
    let mut polls = 0;
    prove_profile(request, piece.span, curve, authored, &mut || {
        count_poll(&mut polls)
    })
    .unwrap();
    polls
}
fn count_trims(
    request: &Request,
    piece: &AuthoredPiece,
    curves: [&OwnedCurve; 2],
    indexed: &Bindings<'_>,
) -> usize {
    let mut polls = 0;
    trims::certify(request, piece, curves, indexed, &mut || {
        count_poll(&mut polls)
    })
    .unwrap();
    polls
}

fn count_cap_profile(request: &Request, piece: &AuthoredPiece, curve: &OwnedCurve) -> usize {
    let mut polls = 0;
    cap_profile::certify(request, piece, curve, &mut || count_poll(&mut polls)).unwrap();
    polls
}

fn count_span(
    request: &Request,
    piece: &AuthoredPiece,
    indexed: &Bindings<'_>,
    authored: &BTreeMap<SurfaceCurveId, &[[f64; 2]]>,
) -> usize {
    let mut polls = 0;
    certify_span(request, piece, indexed, authored, &mut || {
        count_poll(&mut polls)
    })
    .unwrap();
    polls
}

pub(super) fn encoded(prepared: &Request, bindings: &[(BindingKey, Shape)]) -> Vec<u8> {
    let root = BindingKey {
        source: Source::Region(prepared.region),
        role: HistoryRole::Solid,
    };
    let solid = &bindings.iter().find(|(key, _)| *key == root).unwrap().1;
    crate::surface_sketch::encoding::encode(solid, bindings).unwrap()
}
fn cancelled_after(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    limit: usize,
) -> StorageFailure {
    let before = encoded(prepared, bindings);
    let mut polls = 0;
    let failure = certify_with_cancel(original, prepared, bindings, || {
        polls += 1;
        polls >= limit
    })
    .err()
    .unwrap();
    assert_eq!(polls, limit);
    assert_eq!(encoded(prepared, bindings), before);
    failure
}
#[test]
fn storage_arithmetic_cancels_with_its_original_key_and_policy() {
    let (original, prepared, bindings) = frozen();
    let error = cancelled_after(&original, &prepared, &bindings, 8);
    assert_eq!(
        error.key,
        key(prepared.loops[0][0].span, HistoryRole::FirstDepthEdge)
    );
    let StorageCause::Certificate(cause) = error.cause else {
        panic!("storage certificate")
    };
    assert!(matches!(cause.cause, CylindricalStorageFailure::Cancelled));
    assert_eq!(
        cause.requested_limits,
        CylindricalStorageLimits::uniform(storage_share(prepared.tolerance))
    );
}
#[test]
fn profile_arithmetic_cancels_after_storage_proof_completion() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.storage + 8);
    let StorageCause::Profile(cause) = error.cause else {
        panic!("profile certificate")
    };
    assert!(
        matches!(cause.cause, CylindricalProfileFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
    );
    assert_eq!(
        cause.requested_spatial_limit,
        (prepared.tolerance / 4.).next_down()
    );
}
fn assert_trim_cancelled(error: StorageFailure, role: HistoryRole) {
    let StorageCause::Trim { face, cause } = error.cause else {
        panic!("trim context")
    };
    assert_eq!(face.role, role);
    let StorageCause::CapResidual(cause) = cause.cause else {
        panic!("cap residual")
    };
    assert!(
        matches!(cause.cause, CylindricalProfileFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
    );
}
#[test]
fn first_cap_residual_can_cancel_after_profile_completion() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.profile + 8);
    assert_trim_cancelled(error, HistoryRole::FirstDepthFace);
}
#[test]
fn last_cap_residual_can_cancel_before_a_complete_span_escapes() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.trims);
    assert_trim_cancelled(error, HistoryRole::LastDepthFace);
}
#[test]
fn cancellation_in_a_later_span_does_not_return_earlier_span_proofs() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.span + 8);
    assert_eq!(
        error.key,
        key(prepared.loops[0][1].span, HistoryRole::FirstDepthEdge)
    );
    assert!(matches!(error.cause, StorageCause::Certificate(_)));
}

#[test]
fn cap_profile_can_cancel_after_both_trim_residuals_complete() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.cap_profile);
    let StorageCause::Profile(cause) = error.cause else {
        panic!("cap profile certificate")
    };
    assert!(
        matches!(cause.cause, CylindricalProfileFailure::Arithmetic(cause)
        if *cause == BoundedArithmeticFailure::Cancelled)
    );
    let expected = ((prepared.tolerance / 2.).next_down() / prepared.radius.max(1.)).next_down();
    assert_eq!(cause.requested_spatial_limit, expected);
}

#[test]
fn lift_alignment_can_cancel_after_cap_profile_completion() {
    let (original, prepared, bindings) = frozen();
    let polls = phase_polls(&original, &prepared, &bindings);
    let error = cancelled_after(&original, &prepared, &bindings, polls.span);
    let StorageCause::Lift(cause) = error.cause else {
        panic!("lift alignment certificate")
    };
    assert!(
        matches!(cause.cause, CylindricalProfileFailure::Arithmetic(cause)
        if *cause == BoundedArithmeticFailure::Cancelled)
    );
    assert_eq!(cause.requested_options, Default::default());
}
