//! Stop actual retained proof work without returning partial certificates or changing geometry.
use super::tests::frozen;
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundedArithmeticFailure, CylindricalProfileFailure, CylindricalStorageFailure,
};
struct ProofPolls {
    storage: usize,
    profile: usize,
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
    let storage = count_storage(prepared, piece, [&curves[0], &curves[1]]);
    let profile = storage + count_profile(prepared, piece, &curves[0], authored[&piece.span.curve]);
    let span = count_span(prepared, piece, &indexed, &authored);
    ProofPolls {
        storage,
        profile,
        span,
    }
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
    let error = cancelled_after(&original, &prepared, &bindings, polls.span);
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
