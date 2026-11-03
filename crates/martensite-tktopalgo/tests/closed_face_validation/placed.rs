//! Relative local frames require an exact world-vector relation after candidate inversion.
use super::{check, fixture};
use martensite_tkbrep::brep::BRepTEdge;
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3},
    toploc::Location,
};
use martensite_tktopalgo::face_validation::*;

fn displaced_net() -> [[f64; 2]; 9] {
    let mut points = fixture::ROUND_NET;
    points[1][0] += 1. / 1_073_741_824.;
    points
}
fn reflected_carrier() -> fixture::StripFixture {
    let points = displaced_net();
    let shape = fixture::strip_with_carrier(1e-7, points);
    let rotation = Transform3::mirror_axis(Axis1::new(Point3::default(), Direction3::X));
    for (index, level) in [(1, 2.), (2, -1.)] {
        fixture::replace_cap_points(
            &shape.edges[index],
            points.map(|p| Point3::new(p[0], -p[1], -level)),
            1e-7,
        );
        place_spatial_curve(&shape.edges[index], rotation);
    }
    shape
}
fn place_spatial_curve(edge: &martensite_tkbrep::topods::Edge, transform: Transform3) {
    let owner = edge.tshape().cast::<BRepTEdge>();
    for representation in owner.get().unwrap().curves() {
        if representation.is_curve3d() {
            representation.set_location(&Location::new(transform));
        }
    }
}
#[test]
fn exact_relative_frame_certifies_without_changing_previous_policy() {
    let shape = reflected_carrier();
    let previous = check(&shape, ClosedFacePolicy::piecewise_boundary());
    assert!(matches!(
        previous.completion(),
        ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::BoundaryRelation { .. })
    ));
    let report = check(&shape, ClosedFacePolicy::placed_piecewise_boundary());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(report.work().support_finished, 20);
    assert_eq!(report.history(), ClosedFaceHistory::NoGeometryChanges);
}
#[test]
fn placed_policy_retains_finite_global_limits() {
    let shape = reflected_carrier();
    let mut policy = ClosedFacePolicy::placed_piecewise_boundary();
    policy.support.max_obligations = 19;
    let report = check(&shape, policy);
    assert!(matches!(
        report.completion(),
        ClosedFaceCompletion::Incomplete(_)
    ));
    assert!(report.certificate().is_none());
}
#[test]
fn placed_policy_is_explicit_and_retains_piecewise_contract() {
    assert_eq!(ClosedFacePolicy::bounded().version, 1);
    assert_eq!(ClosedFacePolicy::piecewise_boundary().version, 2);
    let policy = ClosedFacePolicy::placed_piecewise_boundary();
    assert_eq!(policy.version, 3);
    assert_eq!(
        policy.check_mode(),
        Some(ClosedFaceCheckMode::PiecewiseRepresentations)
    );
}
