//! Keep complete report counts while corrupting identities or source-bound certificates.
use super::super::tests::proofs;
use super::*;
fn fixture() -> (Request, Request, Vec<storage::StoredProof>, ConeEvidence) {
    let (original, prepared, stored) = proofs();
    let cones = certify(&original, &prepared, &stored).unwrap();
    (original, prepared, stored, cones)
}
#[test]
fn report_order_and_loop_start_do_not_supply_binding_identity() {
    let (original, mut prepared, stored, mut cones) = fixture();
    cones.pieces.reverse();
    cones.joins.reverse();
    for contour in &mut prepared.loops {
        contour.rotate_left(1);
    }
    bind(&original, &prepared, &stored, &cones).unwrap();
}
#[test]
fn a_valid_regular_certificate_for_another_span_is_not_coverage() {
    let (original, prepared, stored, mut cones) = fixture();
    cones.pieces[0].certificate = cones.pieces[1].certificate.clone();
    assert_binding(
        bind(&original, &prepared, &stored, &cones),
        "retained O2 source binding",
    );
}
#[test]
fn a_valid_join_certificate_for_another_pair_is_not_coverage() {
    let (original, prepared, stored, mut cones) = fixture();
    cones.joins[0].certificate = cones.joins[1].certificate.clone();
    assert_binding(
        bind(&original, &prepared, &stored, &cones),
        "retained O2 source binding",
    );
}
#[test]
fn duplicated_or_relabelled_report_keys_cannot_hide_a_missing_cone() {
    let (original, prepared, stored, mut cones) = fixture();
    let saved = cones.pieces[0].spans.clone();
    cones.pieces[0].spans = cones.pieces[1].spans.clone();
    assert_binding(
        bind(&original, &prepared, &stored, &cones),
        "cone multiplicity",
    );
    cones.pieces[0].spans = saved;
    cones.joins[0].spans.reverse();
    assert_binding(
        bind(&original, &prepared, &stored, &cones),
        "cone membership",
    );
}
#[test]
fn changed_original_sources_and_missing_reports_refuse() {
    let (mut original, prepared, stored, mut cones) = fixture();
    original.loops[0][0].poles[1][0] += 0.01;
    assert_binding(
        bind(&original, &prepared, &stored, &cones),
        "retained O2 source binding",
    );
    cones.joins.pop();
    assert_binding(bind(&original, &prepared, &stored, &cones), "cone reports");
}
fn assert_binding(result: Result<(), ConeFailure>, field: &'static str) {
    assert!(matches!(result, Err(ConeFailure::Binding { field: actual, .. }) if actual == field));
}
