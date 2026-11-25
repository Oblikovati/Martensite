//! Retained developed context binds the actual native pair and exact datum without a reproof.
use super::*;
#[test]
fn chart_developed_witness_keeps_actual_uv_reference_and_producing_work() {
    fixture_with_policy(
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        check_retained,
    );
}
fn check_retained(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
) {
    let cap = cap(source, piece);
    let caps = cap.captured().unwrap();
    let mut work = Work::new(ConeClearanceOptions::default());
    let pair = cap.query(curves, &caps.pcurves, work.profile());
    let boundary = boundary::query(source.original, source.prepared, piece, curves).unwrap();
    let proof = &source.prepared.fits[&piece.span].boundary;
    let (pair_proof, witness) = certify(
        &cap,
        pair,
        curves,
        proof,
        source.prepared.chart_domain.as_deref().unwrap(),
        &mut work,
        &mut || false,
    )
    .unwrap();
    assert_eq!(witness.operations(), pair_proof.operations());
    assert_eq!(work.consumed, pair_proof.operations());
    check_mutations(source, &pair, &boundary, proof, &witness);
}
fn check_mutations(
    source: &SourceCapture<'_>,
    pair: &ConicalTrimPairQuery<'_>,
    boundary: &ConicalBoundaryQuery<'_>,
    proof: &ConicalBoundaryEvidence,
    witness: &ConicalDevelopedBoundaryEvidence,
) {
    let domain = source.prepared.chart_domain.as_deref().unwrap();
    let reference = source.prepared.reference_slant;
    assert!(witness.matches(pair, boundary, proof, domain, reference));
    assert!(!witness.matches(pair, boundary, proof, domain, reference.next_up()));
    let mut controls = pair.pcurves[0].to_vec();
    controls[1][0] = controls[1][0].next_up();
    let mut changed = *pair;
    changed.pcurves[0] = &controls;
    assert!(!witness.matches(&changed, boundary, proof, domain, reference));
    let mut changed = *pair;
    changed.options.fractional_bits += 1;
    assert!(!witness.matches(&changed, boundary, proof, domain, reference));
}
