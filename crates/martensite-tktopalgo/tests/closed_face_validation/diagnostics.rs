//! Query failures retain actual graph owners and numerical causes (#378).
use super::fixture;
use martensite_tkbrep::{
    brep::{BRepTEdge, face_input_capture::*},
    topods::{Edge, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomLine;
use martensite_tkmath::{
    gp::{Direction3, Point3},
    predicates::roots::{RootIsolationError, metric_bounds::ResidualError},
};
use martensite_tktopalgo::face_validation::*;

fn snapshot(face: &Face) -> FaceInputSnapshot {
    capture_closed_extrusion_face(face, FaceCaptureLimits::bounded(), || false).unwrap()
}
fn perturbed_seam(epsilon: f64) -> FaceInputSnapshot {
    let shape = fixture::strip(1e-7, 0.);
    let line = seam_line(&shape.edges[0]);
    line.set_location(Point3::new(1. - epsilon, 0., -1.));
    line.set_direction(Direction3::new(epsilon, 0., 1.).unwrap());
    snapshot(&shape.face)
}
fn seam_line(edge: &Edge) -> Handle<GeomLine> {
    let owner = edge.tshape().cast::<BRepTEdge>();
    let curves = owner.get().unwrap().curves();
    let spatial = curves.iter().find(|curve| curve.is_curve3d()).unwrap();
    spatial.curve3d().unwrap().cast()
}
fn seam_owner(input: &FaceInputSnapshot) -> usize {
    let owners: Vec<_> = input
        .cells()
        .iter()
        .filter_map(|(owner, cell)| match cell {
            CapturedFaceCell::Edge {
                representations, ..
            } if has_seam(input, representations) => Some(*owner),
            _ => None,
        })
        .collect();
    assert_eq!(owners.len(), 1);
    owners[0]
}
fn has_seam(input: &FaceInputSnapshot, representations: &[usize]) -> bool {
    representations.iter().any(|reference| {
        input.cells().iter().any(|(owner, cell)| {
            owner == reference && matches!(cell, CapturedFaceCell::Seam { .. })
        })
    })
}
fn no_bisections(input: FaceInputSnapshot) -> ClosedFaceReport {
    let mut policy = ClosedFacePolicy::bounded();
    policy.max_bisections = 0;
    certify_closed_extrusion_face(input, policy, || false)
}
fn metric_context(report: &ClosedFaceReport, owner: usize) {
    let ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Metric { query, cause }) =
        report.completion()
    else {
        panic!("{:?}", report.completion())
    };
    assert_eq!(
        *query,
        ClosedFaceQueryContext::new(owner, ClosedFaceRelation::SeamConformance)
    );
    assert!(matches!(
        cause,
        ResidualError::Roots(RootIsolationError::SubdivisionLimit { limit: 0 })
    ));
}
#[test]
fn numerical_failure_keeps_actual_seam_owner_relation_and_original_cause() {
    let input = perturbed_seam(1. / 1_073_741_824.);
    let owner = seam_owner(&input);
    assert_ne!(
        owner, 0,
        "fixture edge occurrence zero must differ from its owner"
    );
    let report = no_bisections(input);
    metric_context(&report, owner);
    assert_eq!(report.work().bisections, 0);
    assert!(report.certificate().is_none());
}
#[test]
fn metric_query_limit_retains_the_same_known_relation() {
    let input = perturbed_seam(1. / 1_073_741_824.);
    let owner = seam_owner(&input);
    let count = no_bisections(input.clone()).work().queries;
    let mut policy = ClosedFacePolicy::bounded();
    policy.max_queries = count - 1;
    let report = certify_closed_extrusion_face(input, policy, || false);
    assert_limit(
        &report,
        count - 1,
        count,
        owner,
        ClosedFaceRelation::SeamConformance,
    );
}
fn assert_limit(
    report: &ClosedFaceReport,
    requested: u64,
    actual: u64,
    owner: usize,
    relation: ClosedFaceRelation,
) {
    let ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Limit {
        kind,
        requested: limit,
        actual: used,
        query,
    }) = report.completion()
    else {
        panic!("{:?}", report.completion())
    };
    assert_eq!((*kind, *limit, *used), ("queries", requested, actual));
    assert_eq!(*query, Some(ClosedFaceQueryContext::new(owner, relation)));
    assert!(report.certificate().is_none());
}
#[test]
fn trim_query_limit_identifies_its_actual_edge_owner() {
    let input = snapshot(&fixture::strip(1e-7, 0.).face);
    let owner = seam_owner(&input);
    let mut policy = ClosedFacePolicy::bounded();
    policy.max_queries = 0;
    let report = certify_closed_extrusion_face(input, policy, || false);
    assert_limit(&report, 0, 1, owner, ClosedFaceRelation::TrimEndpoint);
}
#[test]
fn spatial_violation_owner_is_not_the_boundary_occurrence_index() {
    let input = perturbed_seam(1. / 1024.);
    let owner = seam_owner(&input);
    assert_ne!(owner, 0);
    let report = certify_closed_extrusion_face(input, ClosedFacePolicy::bounded(), || false);
    let ClosedFaceCompletion::Invalid(violation) = report.completion() else {
        panic!("{:?}", report.completion())
    };
    assert_eq!(violation.owner, owner);
    assert_eq!(violation.invariant, "spatial consistency");
    assert!(violation.actual.starts_with("role=seam, limit="));
    assert!(violation.actual.contains("witness=Endpoint("));
}
