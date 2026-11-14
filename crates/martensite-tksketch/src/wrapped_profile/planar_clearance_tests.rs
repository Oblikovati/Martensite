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
