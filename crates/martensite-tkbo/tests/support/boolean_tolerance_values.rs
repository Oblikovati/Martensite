//! Entity tolerance snapshots in original unique-map order.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn tolerance(shape: &Shape, kind: ShapeEnum) -> f64 {
    match kind {
        ShapeEnum::Vertex => BRepTool::vertex_tolerance(&topods::vertex(shape).unwrap()).unwrap(),
        ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(shape).unwrap()).unwrap(),
        ShapeEnum::Face => BRepTool::face_tolerance(&topods::face(shape).unwrap()).unwrap(),
        _ => panic!("tolerance kind {kind:?}: expected Vertex, Edge or Face"),
    }
}

/// Capture original unique-map entity order; `values(&shape, ShapeEnum::Edge)`.
pub fn values(shape: &Shape, kind: ShapeEnum) -> Vec<f64> {
    let mut entities = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut entities).unwrap();
    (1..=entities.extent())
        .map(|index| tolerance(entities.find_key(index).unwrap(), kind))
        .collect()
}
