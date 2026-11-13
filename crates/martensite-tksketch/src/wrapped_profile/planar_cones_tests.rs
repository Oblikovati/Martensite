//! Cone composition refuses missing or mismatched O2 source identity.
use super::super::{native, subdivision, tests::request};
use super::*;

fn proofs() -> (Request, Request, Vec<storage::StoredProof>) {
    let original = request();
    let prepared = subdivision::prepare(&original).unwrap();
    let built = native::construct(&prepared).unwrap();
    let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
    let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
    let (_, bindings) = super::super::pipeline::decode(&bytes, &keys).unwrap();
    let stored = storage::certify(&original, &prepared, &bindings).unwrap();
    (original, prepared, stored)
}

#[test]
fn missing_o2_proof_refuses_instead_of_using_traversal_rank() {
    let (original, prepared, mut stored) = proofs();
    stored.remove(0);
    assert!(matches!(
        certify(&original, &prepared, &stored).err().unwrap(),
        ConeFailure::Membership { count: 0, .. }
    ));
}

#[test]
fn relabelled_o2_proofs_cannot_cover_different_authored_spans() {
    let (original, prepared, mut stored) = proofs();
    let (a, b) = (stored[0].span, stored[1].span);
    stored[0].span = b;
    stored[1].span = a;
    assert!(matches!(
        certify(&original, &prepared, &stored).err().unwrap(),
        ConeFailure::Certificate { .. }
    ));
}

#[test]
fn a_changed_original_polynomial_requires_a_new_derivative_certificate() {
    let (mut original, prepared, stored) = proofs();
    original.loops[0][0].poles[1][0] += 0.01;
    assert!(matches!(
        certify(&original, &prepared, &stored).err().unwrap(),
        ConeFailure::Certificate { .. }
    ));
}
