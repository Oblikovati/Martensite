//! Official seam deletion removes the pair rather than retaining a null reverse member.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Cylinder3};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn cylindrical_seam() -> (Face, Edge) {
    let face = BRepBuilderAPIMakeFace::from_cylinder_bounds(
        Cylinder3::new(Axis3::default(), 2.).unwrap(),
        (0., std::f64::consts::TAU, 0., 1.),
    )
    .unwrap()
    .face()
    .unwrap();
    let edge = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::edge(&shape).unwrap())
        .find(|edge| BRepTool::is_closed_on_face(edge, &face).unwrap())
        .unwrap();
    (face, edge)
}
fn oriented_pcurves(
    edge: &Edge,
    face: &Face,
) -> (Handle<dyn Geom2dCurve>, Handle<dyn Geom2dCurve>) {
    let analysis = ShapeAnalysisEdge::new();
    let forward = analysis.pcurve_on_face(edge, face, false).unwrap().0;
    let reverse = analysis
        .pcurve_on_face(&topods::edge(&edge.reversed()).unwrap(), face, false)
        .unwrap()
        .0;
    (forward, reverse)
}
#[test]
fn official_seam_pair_deletion_cannot_preserve_a_closed_null_reverse_representation() {
    let (face, edge) = cylindrical_seam();
    let (current, reverse) = oriented_pcurves(&edge, &face);
    assert!(!current.is_null() && !reverse.is_null());
    let empty = Handle::default();
    BRepBuilder::default()
        .update_edge_pcurves_on_face(&edge, (&current, &empty), &face, 0.)
        .unwrap();
    assert!(!BRepTool::is_closed_on_face(&edge, &face).unwrap());
    let removed = oriented_pcurves(&edge, &face);
    assert!(removed.0.is_null() && removed.1.is_null());
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&edge, &current, &face, 0.)
        .unwrap();
    assert!(!BRepTool::is_closed_on_face(&edge, &face).unwrap());
    let restored = oriented_pcurves(&edge, &face);
    assert!(restored.0 == current && restored.1 == current);
}
