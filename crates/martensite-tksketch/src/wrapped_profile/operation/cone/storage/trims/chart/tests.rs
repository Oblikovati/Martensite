//! The native bridge retains actual representation admission and failure precedence.
use super::super::tests::{fixture_with_policy, invalidate_last_pcurve, refusal};
use super::*;
#[test]
fn chart_bridge_certifies_actual_native_pair_and_retained_boundary() {
    fixture_with_policy(WrappedConeDomainPolicy::PositiveXCommonChartV2, actual_pair);
}
fn actual_pair(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    let mut work = Work::new(ConeClearanceOptions::default());
    let boundary = &source.prepared.fits[&piece.span].boundary;
    let traces =
        super::super::certify(source, piece, curves, boundary, &mut work, &mut || false).unwrap();
    assert_eq!(traces.count, 2);
    assert!(traces.developed_boundary.is_some());
    assert!(work.consumed > 0);
    assert!(work.consumed <= work.requested.max_operations);
}
#[test]
fn chart_bridge_keeps_invalid_native_representation_before_zero_work_and_cancel() {
    fixture_with_policy(
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        invalid_representation,
    );
}
fn invalid_representation(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    invalidate_last_pcurve(source, piece);
    let boundary = &source.prepared.fits[&piece.span].boundary;
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 0,
        ..Default::default()
    });
    let mut polls = 0;
    let cause = super::super::certify(source, piece, curves, boundary, &mut work, &mut || {
        polls += 1;
        true
    })
    .err()
    .unwrap();
    assert!(matches!(cause.cause, ConeCause::Storage(_)));
    assert_eq!(cause.span, Some(piece.span));
    assert_eq!(polls, 0);
    assert_eq!(work.consumed, 0);
}
#[test]
fn chart_bridge_rejects_changed_decoded_boundary_before_late_cancel() {
    fixture_with_policy(
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        changed_boundary,
    );
}
fn changed_boundary(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    let cap = cap(source, piece);
    let caps = cap.captured().unwrap();
    let mut changed = curves.each_ref().map(|curve| retained::OwnedCurve {
        poles: curve.poles.clone(),
        weights: curve.weights.clone(),
    });
    changed[1].weights[0] = changed[1].weights[0].next_up();
    let proof = &source.prepared.fits[&piece.span].boundary;
    let mut work = Work::new(ConeClearanceOptions::default());
    let cause = cap
        .certify(&changed, &caps.pcurves, proof, &mut work, &mut || true)
        .err()
        .unwrap();
    assert_eq!(cause.span, Some(piece.span));
    check_binding_error(cause, work.consumed);
}
fn check_binding_error(cause: ConeFailure, operations: u32) {
    let ConeCause::ChartTrimPair(cause) = cause.cause else {
        panic!("chart bridge binding cause")
    };
    assert!(matches!(
        cause.cause.as_ref(),
        ConicalChartTrimPairFailure::Binding { .. }
    ));
    assert!(!cause.is_cancelled());
    assert_eq!(operations, cause.operations);
}
#[test]
fn chart_bridge_trace_error_outranks_secondary_bookkeeping_failure() {
    fixture_with_policy(
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        original_error,
    );
}
fn original_error(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    _: &[retained::OwnedCurve; 2],
) {
    let cap = cap(source, piece);
    let original = refusal(source.carriers[1]);
    let error = ConicalChartTrimPairError {
        depth: Some(1),
        operations: original.operations,
        cause: Box::new(ConicalChartTrimPairFailure::Trace(*original.cause)),
    };
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 1,
        ..Default::default()
    });
    let failure = charged(&cap, Err(error), &mut work).err().unwrap();
    assert_eq!(failure.region, Some(source.original.region));
    assert_eq!(failure.span, Some(piece.span));
    let ConeCause::ChartTrimPair(cause) = failure.cause else {
        panic!("original chart trace error")
    };
    assert_eq!(cause.depth, Some(1));
    assert_eq!(cause.operations, 2);
}
fn cap<'a, 'b>(source: &'a SourceCapture<'b>, piece: &'a AuthoredPiece) -> CapCapture<'a, 'b> {
    CapCapture {
        source,
        piece,
        expected: &source.prepared.fits[&piece.span].uv,
    }
}

mod developed;
