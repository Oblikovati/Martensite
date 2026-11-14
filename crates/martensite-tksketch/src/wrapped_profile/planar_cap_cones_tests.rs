//! Cap cones must keep angular source identity independently of their report census.
use super::*;
fn fixture() -> (Request, Vec<storage::StoredProof>, CapConeEvidence) {
    let (_, request, stored) = super::super::tests::proofs();
    let evidence = certify(&request, &stored, &mut || false).unwrap();
    (request, stored, evidence)
}
#[test]
fn cap_coverage_survives_report_reordering_and_rotated_loop_starts() {
    let (mut request, stored, mut evidence) = fixture();
    let count = request.loops.iter().map(Vec::len).sum::<usize>();
    assert_eq!(
        (evidence.pieces.len(), evidence.joins.len()),
        (count, count)
    );
    evidence.pieces.reverse();
    evidence.joins.reverse();
    for contour in &mut request.loops {
        contour.rotate_left(1);
    }
    bind(&request, &stored, &evidence).unwrap();
}
#[test]
fn foreign_cap_regular_and_join_certificates_cannot_cover_matching_counts() {
    let (request, stored, mut evidence) = fixture();
    let regular = evidence.pieces[0].certificate.clone();
    evidence.pieces[0].certificate = evidence.pieces[1].certificate.clone();
    assert!(bind(&request, &stored, &evidence).is_err());
    evidence.pieces[0].certificate = regular;
    evidence.joins[0].certificate = evidence.joins[1].certificate.clone();
    assert!(bind(&request, &stored, &evidence).is_err());
}
#[test]
fn changed_cap_controls_or_missing_reports_refuse_binding() {
    let (mut request, stored, mut evidence) = fixture();
    let saved = request.loops[0][0].poles[0][1];
    request.loops[0][0].poles[0][1] = request.loops[0][0].poles[0][1].next_up();
    assert!(bind(&request, &stored, &evidence).is_err());
    request.loops[0][0].poles[0][1] = saved;
    bind(&request, &stored, &evidence).unwrap();
    evidence.joins.pop();
    assert!(bind(&request, &stored, &evidence).is_err());
}
#[test]
fn cancelled_cap_construction_returns_no_evidence() {
    let (request, stored, _) = fixture();
    assert!(matches!(
        certify(&request, &stored, &mut || true),
        Err(ConeFailure::Certificate { .. })
    ));
}
