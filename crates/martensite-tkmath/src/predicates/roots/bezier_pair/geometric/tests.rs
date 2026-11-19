// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::RootIsolationOptions;
use super::*;
mod binding;
mod controls;
mod range;
fn policy() -> BezierPairPolicy {
    BezierPairPolicy {
        version: 2,
        roots: RootIsolationOptions::new(0.001),
        signs: BezierPairSignOptions::derived(),
        requested_world_error: 1e-8,
    }
}
fn pair(a: &[Point], b: &[Point]) -> CapturedBezierPair {
    let curve = |controls| BezierPairCurve {
        controls,
        weights: None,
        interval: [0., 1.],
    };
    CapturedBezierPair::capture_pair(curve(a), curve(b)).unwrap()
}
fn hairpin(scale: f64) -> CapturedBezierPair {
    let a = [[-2., 0.], [-1., 1.], [-0.5, 0.5], [0., 0.]].map(|p| p.map(|v| v * scale));
    let b = [[0., 0.], [0.5, -0.5], [1., -1.], [2., 0.]].map(|p| p.map(|v| v * scale));
    pair(&a, &b)
}
#[test]
fn exact_endpoint_sector_retains_parameters_and_source_without_solver_roots() {
    let query = hairpin(1.);
    let report = query.certify_geometric(policy(), || false);
    assert_eq!(report.completion(), BezierPairGeometricCompletion::Complete);
    assert!(report.matches_source(&query, policy()));
    assert_eq!(report.work().completed, 0);
    let proof = report.complete().unwrap();
    assert_eq!(proof.kind(), BezierPairGeometricKind::EndpointSector);
    assert_eq!(proof.contact_count(), 1);
    let contact = proof.contact().unwrap();
    assert_eq!(contact.parameters(), [1, 0]);
    assert_eq!(contact.point(), [0., 0.]);
}
#[test]
fn box_and_exact_hull_exclusions_are_completed_empty() {
    let box_pair = pair(&[[0., 0.]], &[[2., 2.]]);
    let report = box_pair.certify_geometric(policy(), || false);
    assert_eq!(
        report.complete().unwrap().kind(),
        BezierPairGeometricKind::BoxExclusion
    );
    assert_eq!(report.complete().unwrap().contact_count(), 0);
    let hull_pair = pair(
        &[[0., 2.], [1., 3.], [2., 4.], [0., 2.]],
        &[[2., 0.], [3., 1.], [4., 2.], [2., 0.]],
    );
    let report = hull_pair.certify_geometric(policy(), || false);
    assert_eq!(
        report.complete().unwrap().kind(),
        BezierPairGeometricKind::HullExclusion
    );
    assert!(report.complete().unwrap().contact().is_none());
}
#[test]
fn immutable_public_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<BezierPairGeometricReport>();
    shared::<BezierPairGeometricCompletion>();
    shared::<BezierPairGeometricCertificate<'static>>();
    shared::<CertifiedBezierEndpointContact<'static>>();
    shared::<BezierPairGeometricFailure>();
    shared::<BezierPairGeometricKind>();
}
