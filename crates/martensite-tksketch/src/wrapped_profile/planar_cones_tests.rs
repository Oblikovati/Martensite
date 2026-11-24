//! Cone composition refuses missing or mismatched O2 source identity.
use super::super::{native, subdivision, tests::request};
use super::*;

pub(in super::super) fn proofs() -> (Request, Request, Vec<storage::StoredProof>) {
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

fn assert_cancelled_cone(error: ConeFailure, expected: &[CurveSpan]) {
    use martensite_tkmath::predicates::roots::metric_bounds::{
        BoundedArithmeticFailure, CylindricalConeFailure,
    };
    let ConeFailure::Certificate { spans, cause } = error else {
        panic!("cone certificate")
    };
    assert_eq!(spans, expected);
    assert!(
        matches!(cause.cause, CylindricalConeFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
    );
}
#[test]
fn regularity_cancellation_keeps_the_authored_span() {
    let (original, prepared, stored) = proofs();
    let mut polls = 0;
    let error = certify_with_cancel(&original, &prepared, &stored, || {
        polls += 1;
        polls >= 8
    })
    .err()
    .unwrap();
    assert_eq!(polls, 8);
    assert_cancelled_cone(error, &[prepared.loops[0][0].span]);
}
fn regularity_polls(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> usize {
    let authored = storage::authored_index(original).unwrap();
    let indexed = proof_index(stored).unwrap();
    let mut polls = 0;
    let mut count = || {
        polls += 1;
        false
    };
    let spans = [prepared.loops[0][0].span];
    prove(&spans, original.radius, &authored, &indexed, &mut count).unwrap();
    polls
}
#[test]
fn join_cancellation_after_regularity_does_not_return_a_partial_contour() {
    let (original, prepared, stored) = proofs();
    let limit = regularity_polls(&original, &prepared, &stored) + 8;
    let mut polls = 0;
    let error = certify_with_cancel(&original, &prepared, &stored, || {
        polls += 1;
        polls >= limit
    })
    .err()
    .unwrap();
    assert_cancelled_cone(
        error,
        &[prepared.loops[0][0].span, prepared.loops[0][1].span],
    );
    assert_eq!(polls, limit);
}
