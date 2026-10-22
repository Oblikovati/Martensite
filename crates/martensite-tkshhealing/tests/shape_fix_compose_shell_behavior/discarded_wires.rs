//! Closed-mode dispatch drops unusable loops before repairing shared geometry.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::{
    ncollection::{HArray2, Sequence},
    standard::Handle,
};
use martensite_tkg3d::{geom::GeomSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    gp::{Axis3, Sphere3},
    toploc::Location,
};
use martensite_tkshhealing::{
    shape_build::ShapeBuildReShape,
    shape_extend::{ShapeExtendCompositeSurface, ShapeExtendParametrisation},
    shape_fix::{FixRoot, ShapeFixComposeShell, ShapeFixWireSegment},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn sphere_and_pole() -> (Face, Edge) {
    let face = BRepBuilderAPIMakeFace::from_sphere(Sphere3::new(Axis3::default(), 2.0).unwrap())
        .unwrap()
        .face()
        .unwrap();
    let edge = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::edge(&shape).unwrap())
        .find(|edge| BRepTool::degenerated(edge).unwrap())
        .unwrap();
    (face, edge)
}

fn grid_on(face: &Face) -> Handle<ShapeExtendCompositeSurface> {
    let surface = BRepTool::surface(face).unwrap();
    let cells = HArray2::with_value(1, 1, 1, 1, Handle::<dyn GeomSurface>::default()).unwrap();
    cells.change_array2().set_value(1, 1, surface).unwrap();
    Handle::new(
        ShapeExtendCompositeSurface::with_grid(
            &Handle::new(cells),
            ShapeExtendParametrisation::Natural,
        )
        .unwrap(),
    )
}

fn closed_composer(face: &Face) -> ShapeFixComposeShell {
    let compose = ShapeFixComposeShell::new();
    compose.set_context(&Handle::new(ShapeBuildReShape::new()));
    compose
        .init(&grid_on(face), &Location::default(), face, 1.0e-7)
        .unwrap();
    compose.set_closed_mode(true);
    compose
}

#[test]
fn a_discarded_natural_pole_preserves_its_shared_pcurve() {
    let (face, edge) = sphere_and_pole();
    let before = BRepTool::curve_on_face(&edge, &face).unwrap();
    assert!(!before.pcurve.is_null());
    let endpoints = pcurve_endpoints(&before.pcurve, (before.first, before.last));
    let segment = dispatch_pole(&face, &edge);
    let after = BRepTool::curve_on_face(&edge, &face).unwrap();
    assert!(
        before.pcurve == after.pcurve,
        "discard must not repair the pole"
    );
    assert_eq!((before.first, before.last), (after.first, after.last));
    assert_eq!(
        endpoints,
        pcurve_endpoints(&after.pcurve, (after.first, after.last))
    );
    assert_eq!(segment.nb_edges(), 1);
    assert!(segment.edge(1).unwrap().is_same(&edge));
}

fn dispatch_pole(face: &Face, edge: &Edge) -> ShapeFixWireSegment {
    let segment = ShapeFixWireSegment::new();
    segment.add_edge(0, edge).unwrap();
    dispatch_and_assert_discarded(&closed_composer(face), segment.clone());
    segment
}

fn pcurve_endpoints(
    curve: &Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
    range: (f64, f64),
) -> [martensite_tkmath::gp::Point2; 2] {
    let curve = curve.get().unwrap();
    [curve.value(range.0).unwrap(), curve.value(range.1).unwrap()]
}

#[test]
fn an_empty_wire_is_discarded_before_wire_repairs() {
    let (face, _) = sphere_and_pole();
    let segment = ShapeFixWireSegment::new();
    dispatch_and_assert_discarded(&closed_composer(&face), segment.clone());
    assert_eq!(segment.nb_edges(), 0);
}

fn dispatch_and_assert_discarded(compose: &ShapeFixComposeShell, segment: ShapeFixWireSegment) {
    let mut wires = Sequence::default();
    wires.append(segment);
    let mut faces: Sequence<Shape> = Sequence::default();
    compose.dispatch_wires(&mut faces, &mut wires).unwrap();
    assert!(wires.is_empty());
    assert!(faces.is_empty());
    assert!(compose.result().is_null());
}
