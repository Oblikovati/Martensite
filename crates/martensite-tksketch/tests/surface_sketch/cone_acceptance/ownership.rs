//! Exports must own their conical carriers, curves and pcurves independently.
use super::{
    construct,
    geometry::{Expected, assert_cone},
    sector,
};
use crate::acceptance::geometry::entity;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::geom::GeomConicalSurface;
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};
use martensite_tksketch::surface_sketch::*;

fn bound_pair(body: &DetachedNormalBody) -> (topods::Face, topods::Edge) {
    let r = sector();
    let face = HistoryKey {
        source: HistorySource::Region(r.region),
        role: HistoryRole::FirstDepthFace,
    };
    let edge = HistoryKey {
        source: HistorySource::Curve(r.boundaries[0]),
        role: HistoryRole::FirstDepthEdge,
    };
    (
        topods::face(entity(body, face)).unwrap(),
        topods::edge(entity(body, edge)).unwrap(),
    )
}
#[derive(Debug, PartialEq)]
struct Snapshot {
    angle: f64,
    curve: Point3,
    pcurve: Point2,
}
fn snapshot(body: &DetachedNormalBody) -> Snapshot {
    let (face, edge) = bound_pair(body);
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let cone = surface.cast::<GeomConicalSurface>();
    let curve = BRepTool::curve_with_location(&edge).unwrap();
    let pcurve = BRepTool::curve_on_face(&edge, &face).unwrap();
    Snapshot {
        angle: cone.get().unwrap().semi_angle(),
        curve: curve.curve.get().unwrap().value(curve.first).unwrap(),
        pcurve: pcurve.pcurve.get().unwrap().value(pcurve.first).unwrap(),
    }
}
fn mutate(body: &DetachedNormalBody) {
    let (face, edge) = bound_pair(body);
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    surface
        .cast::<GeomConicalSurface>()
        .get()
        .unwrap()
        .set_semi_angle(0.4)
        .unwrap();
    BRepTool::curve_with_location(&edge)
        .unwrap()
        .curve
        .get()
        .unwrap()
        .translate(Vector3::new(11., 13., 17.))
        .unwrap();
    BRepTool::curve_on_face(&edge, &face)
        .unwrap()
        .pcurve
        .get()
        .unwrap()
        .translate(Vector2::new(5., 7.))
        .unwrap();
}
#[test]
fn cone_export_surface_curve_and_pcurve_mutations_do_not_escape() {
    let request = sector();
    let original = request;
    let result = construct(&request, [-1., 2.]);
    assert_eq!(request, original);
    let first = result.export_brep().unwrap();
    let second = result.export_brep().unwrap();
    let before = snapshot(&second);
    assert_eq!(snapshot(&first), before);
    mutate(&first);
    let changed = snapshot(&first);
    assert_eq!(changed.angle, 0.4);
    crate::acceptance::geometry::assert_point_with_limit(
        changed.curve,
        Point3::new(
            before.curve.x() + 11.,
            before.curve.y() + 13.,
            before.curve.z() + 17.,
        ),
        1e-7,
    );
    assert!((changed.pcurve.x() - before.pcurve.x() - 5.).abs() < 1e-12);
    assert!((changed.pcurve.y() - before.pcurve.y() - 7.).abs() < 1e-12);
    assert_eq!(snapshot(&second), before);
    assert_eq!(snapshot(&result.export_brep().unwrap()), before);
    assert_cone(&result, &request, [-1., 2.], Expected::outward());
}
#[test]
fn later_request_edits_do_not_change_published_cone_or_authored_identity() {
    let mut request = sector();
    let original = request;
    let result = construct(&request, [-1., 2.]);
    request.support.apex = [5., 6., 7.];
    request.slant = [9., 13.];
    let edited = construct(&request, [-1., 2.]);
    assert_eq!(edited.history(), result.history());
    assert_cone(&result, &original, [-1., 2.], Expected::outward());
}
