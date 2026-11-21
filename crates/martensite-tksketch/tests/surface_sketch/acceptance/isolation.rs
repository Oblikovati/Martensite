//! Mutates the decoded owners themselves, so a detached temporary cannot fake isolation.
use super::{construct, geometry::*, rectangle};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};
use martensite_tksketch::surface_sketch::*;

#[derive(Debug, PartialEq)]
struct GeometrySnapshot {
    radius: f64,
    curve_point: Point3,
    pcurve_point: Point2,
}

fn bound_pair(body: &DetachedNormalBody) -> (Face, Edge) {
    let region = rectangle();
    let face = HistoryKey {
        source: HistorySource::Region(region.region),
        role: HistoryRole::FirstDepthFace,
    };
    let edge = HistoryKey {
        source: HistorySource::Curve(region.boundaries[0]),
        role: HistoryRole::FirstDepthEdge,
    };
    (
        topods::face(entity(body, face)).unwrap(),
        topods::edge(entity(body, edge)).unwrap(),
    )
}

fn snapshot(body: &DetachedNormalBody) -> GeometrySnapshot {
    let (face, edge) = bound_pair(body);
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let cylinder = surface.cast::<GeomCylindricalSurface>();
    let curve = BRepTool::curve_with_location(&edge).unwrap();
    let pcurve = BRepTool::curve_on_face(&edge, &face).unwrap();
    GeometrySnapshot {
        radius: cylinder.get().unwrap().radius(),
        curve_point: curve.curve.get().unwrap().value(curve.first).unwrap(),
        pcurve_point: pcurve.pcurve.get().unwrap().value(pcurve.first).unwrap(),
    }
}

fn mutate_reachable_geometry(body: &DetachedNormalBody) {
    let (face, edge) = bound_pair(body);
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let cylinder = surface.cast::<GeomCylindricalSurface>();
    cylinder.get().unwrap().set_radius(37.).unwrap();
    let curve = BRepTool::curve_with_location(&edge).unwrap();
    curve
        .curve
        .get()
        .unwrap()
        .translate(Vector3::new(11., 13., 17.))
        .unwrap();
    let pcurve = BRepTool::curve_on_face(&edge, &face).unwrap();
    pcurve
        .pcurve
        .get()
        .unwrap()
        .translate(Vector2::new(5., 7.))
        .unwrap();
}

#[test]
fn decoded_surface_curve_and_pcurve_owners_are_private_to_each_export() {
    let region = rectangle();
    let result = construct(&region, [-1., 2.]);
    let first = result.export_brep().unwrap();
    let second = result.export_brep().unwrap();
    let original = snapshot(&second);
    assert_eq!(snapshot(&first), original);
    mutate_reachable_geometry(&first);
    assert_mutations_visible(&snapshot(&first), &original);
    assert_eq!(snapshot(&second), original);
    assert_eq!(snapshot(&result.export_brep().unwrap()), original);
    assert_body(&result, &region, [-1., 2.]);
}

fn assert_mutations_visible(changed: &GeometrySnapshot, original: &GeometrySnapshot) {
    assert_eq!(changed.radius, 37.);
    assert_point(
        changed.curve_point,
        Point3::new(
            original.curve_point.x() + 11.,
            original.curve_point.y() + 13.,
            original.curve_point.z() + 17.,
        ),
    );
    assert!((changed.pcurve_point.x() - original.pcurve_point.x() - 5.).abs() < 1e-12);
    assert!((changed.pcurve_point.y() - original.pcurve_point.y() - 7.).abs() < 1e-12);
}
