//! Frozen representation admission precedes pair arithmetic without cancellation polling.
use super::*;
use martensite_tkbrep::{brep::BRepBuilder, topods};
#[test]
fn invalid_second_representation_precedes_first_zero_budget() {
    fixture(invalid_legacy_pair);
}
fn invalid_legacy_pair(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    invalidate_last_pcurve(source, piece);
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 0,
        ..Default::default()
    });
    let (cause, polls) = polling_failure(source, piece, curves, &mut work);
    assert!(matches!(cause.cause, ConeCause::Storage(_)));
    assert_eq!(cause.span, Some(piece.span));
    assert_eq!(polls, 0);
    assert_eq!(work.consumed, 0);
}
fn polling_failure(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
    work: &mut Work,
) -> (ConeFailure, usize) {
    let mut polls = 0;
    let cause = certify(
        source,
        piece,
        curves,
        &source.prepared.fits[&piece.span].boundary,
        work,
        &mut || {
            polls += 1;
            false
        },
    )
    .err()
    .unwrap();
    (cause, polls)
}
#[test]
fn complete_representations_preserve_first_pair_depth_failure_and_work() {
    fixture(exhausted_legacy_pair);
}
fn exhausted_legacy_pair(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 1,
        ..Default::default()
    });
    let cause = certify(
        source,
        piece,
        curves,
        &source.prepared.fits[&piece.span].boundary,
        &mut work,
        &mut || false,
    )
    .err()
    .unwrap();
    assert_pair_failure(cause, &work);
}
pub(super) fn invalidate_last_pcurve(source: &SourceCapture<'_>, piece: &AuthoredPiece) {
    let edge_key = retained::key(piece.span, HistoryRole::LastDepthEdge);
    let face_key = capture::region_key(source.original.region, HistoryRole::LastDepthFace);
    let edge = topods::edge(retained::lookup(edge_key, source.bindings).unwrap()).unwrap();
    let face = topods::face(retained::lookup(face_key, source.bindings).unwrap()).unwrap();
    let mut poles = source.prepared.fits[&piece.span].uv;
    poles[0][0] += 0.1;
    let curve = crate::surface_sketch::wrapped_profile::boundaries::curve2(&poles).unwrap();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&edge, &curve, &face, source.prepared.source.tolerance / 2.)
        .unwrap();
}
fn fixture(check: impl FnOnce(&SourceCapture<'_>, &AuthoredPiece, &[retained::OwnedCurve; 2])) {
    fixture_with_policy(WrappedConeDomainPolicy::LegacySectorV1, check);
}
pub(super) fn fixture_with_policy(
    policy: WrappedConeDomainPolicy,
    check: impl FnOnce(&SourceCapture<'_>, &AuthoredPiece, &[retained::OwnedCurve; 2]),
) {
    let request = crate::surface_sketch::wrapped_profile::operation::cone::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = Work::new(request.proof.arithmetic);
    let selection =
        prepare::select_with_policy(&request, &inputs.originals, policy, &mut work, &mut || {
            false
        })
        .unwrap();
    let prepared = &selection.prepared[0];
    check_native(&request, &inputs.originals[0], prepared, check);
}
fn check_native(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    check: impl FnOnce(&SourceCapture<'_>, &AuthoredPiece, &[retained::OwnedCurve; 2]),
) {
    let native = native::construct(prepared, request.support).unwrap();
    let bindings = retained::index(&native.bindings).unwrap();
    let carriers = capture::carriers(prepared, &bindings).unwrap();
    let occurrences = actual_occurrences(prepared, &native);
    let source = SourceCapture {
        original,
        prepared,
        bindings: &bindings,
        carriers,
        occurrences: &occurrences,
    };
    let piece = &prepared.source.loops[0][0];
    let curves = retained::captured_span(piece, &bindings).unwrap();
    check(&source, piece, &curves);
}
fn actual_occurrences(prepared: &Prepared, native: &Constructed) -> coverage::Occurrences {
    coverage::certify_developed_oriented(&prepared.source,&native.solid,&native.bindings,
        crate::surface_sketch::wrapped_profile::operation::topology::DevelopedMapHandedness::Reversing).unwrap().1
}
#[test]
fn original_pair_error_outranks_secondary_bookkeeping_failure() {
    fixture(|source, piece, _| {
        let fit = source.prepared.fits.get(&piece.span).unwrap();
        let cap = CapCapture {
            source,
            piece,
            expected: &fit.uv,
        };
        let original = refusal(source.carriers[1]);
        let mut work = Work::new(ConeClearanceOptions {
            max_operations: 1,
            ..Default::default()
        });
        let refusal = cap.charged(Err(original), &mut work).err().unwrap();
        let ConeCause::TrimPair(refusal) = refusal.cause else {
            panic!("original pair refusal");
        };
        check_original_refusal(&refusal);
    });
}
fn check_original_refusal(refusal: &ConicalTrimPairError) {
    assert_eq!(refusal.depth, 1);
    assert_eq!(refusal.operations, 2);
    assert!(matches!(
        refusal.cause.cause,
        CylindricalProfileFailure::Value {
            role: "original trim refusal",
            value: 13.,
            ..
        }
    ));
}
pub(super) fn refusal(carrier: ConicalTrimCarrier) -> ConicalTrimPairError {
    ConicalTrimPairError {
        depth: 1,
        operations: 2,
        cause: Box::new(ConicalTrimError {
            carrier,
            requested_spatial_limit: 0.,
            requested_options: Default::default(),
            cause: CylindricalProfileFailure::Value {
                role: "original trim refusal",
                index: 0,
                value: 13.,
            },
        }),
    }
}

fn assert_pair_failure(cause: ConeFailure, work: &Work) {
    let ConeCause::TrimPair(cause) = cause.cause else {
        panic!("paired trim cause");
    };
    assert_eq!(cause.depth, 0);
    assert_eq!(work.consumed, cause.operations);
    assert_eq!(work.consumed, 1);
}
