use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Edge, Face, Wire},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkmath::gp::{Plane3, Point2, Point3};
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};

pub(super) const INITIAL: [[(f64, f64); 2]; 4] = [
    [(0.01, 0.), (2., 0.02)],
    [(2.03, 0.), (2., 3.04)],
    [(0.05, 3.), (2., 3.06)],
    [(0.07, 0.), (0., 3.08)],
];
fn rectangle_edge(first: (f64, f64), last: (f64, f64), reversed: bool) -> Edge {
    let points = [
        Point3::new(first.0, first.1, 0.),
        Point3::new(last.0, last.1, 0.),
    ];
    let poles = Array1::from_shared(&points, 1);
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomBezierCurve::new(&poles).unwrap()).cast();
    let mut edge = BRepBuilderAPIMakeEdge::from_curve(&curve)
        .unwrap()
        .edge()
        .unwrap();
    if reversed {
        edge.reverse();
    }
    edge
}
fn rectangle_wire() -> Wire {
    let edges = [
        rectangle_edge((0., 0.), (2., 0.), false),
        rectangle_edge((2., 0.), (2., 3.), false),
        rectangle_edge((0., 3.), (2., 3.), true),
        rectangle_edge((0., 0.), (0., 3.), true),
    ];
    BRepBuilderAPIMakeWire::from_four_edges(&edges[0], &edges[1], &edges[2], &edges[3])
        .unwrap()
        .wire()
        .unwrap()
}
pub(super) fn wire_edges(wire: &Wire) -> Vec<Edge> {
    let sequence = ShapeExtendWireData::new();
    sequence.init_wire(wire, true, true).unwrap();
    (1..=sequence.nb_edges())
        .map(|index| sequence.edge(index).unwrap())
        .collect()
}
pub(super) fn gapped_face() -> (Face, Vec<Edge>) {
    let wire = rectangle_wire();
    let face = BRepBuilderAPIMakeFace::from_plane_wire(Plane3::default(), &wire, true)
        .unwrap()
        .face()
        .unwrap();
    let edges = wire_edges(&wire);
    for (edge, points) in edges.iter().zip(INITIAL) {
        install_pcurve(edge, &face, points);
    }
    (face, edges)
}
fn parameter_curve(points: [(f64, f64); 2]) -> Handle<dyn Geom2dCurve> {
    let points = points.map(|point| Point2::new(point.0, point.1));
    let poles = Array1::from_shared(&points, 1);
    Handle::new(Geom2dBezierCurve::new(&poles).unwrap()).cast()
}

fn install_pcurve(edge: &Edge, face: &Face, points: [(f64, f64); 2]) {
    let curve = parameter_curve(points);
    let builder = BRepBuilder::default();
    builder
        .update_edge_pcurve_on_face(edge, &curve, face, 0.)
        .unwrap();
    builder.range_on_face(edge, face, 0., 1.).unwrap();
}
