//! Owned incidence fixtures are shared by feature-gap stage controls.
use super::Connections;
use crate::bop_algo::remove_features::helpers::{compound_of, explore};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedDataMapOfShapeListOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
pub(super) struct GapIncidenceFixture {
    pub(super) solid: Shape,
    pub(super) feature: Shape,
    pub(super) faces: Vec<Shape>,
    pub(super) edge_faces: IndexedDataMapOfShapeListOfShape,
    pub(super) face_solids: IndexedDataMapOfShapeListOfShape,
}
impl GapIncidenceFixture {
    pub(super) fn connections(&self) -> Connections<'_> {
        Connections {
            edge_faces: &self.edge_faces,
            face_solids: &self.face_solids,
        }
    }
}
fn incidence_map(
    shape: &Shape,
    from: ShapeEnum,
    to: ShapeEnum,
) -> IndexedDataMapOfShapeListOfShape {
    let mut result = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shape, from, to, &mut result).unwrap();
    result
}
pub(super) fn gap_incidence_fixture(count: usize) -> GapIncidenceFixture {
    let solid = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let faces = explore(&solid, ShapeEnum::Face).unwrap();
    let feature = compound_of(&faces[..count]).unwrap();
    let edge_faces = incidence_map(&solid, ShapeEnum::Edge, ShapeEnum::Face);
    let face_solids = incidence_map(&solid, ShapeEnum::Face, ShapeEnum::Solid);
    GapIncidenceFixture {
        solid,
        feature,
        faces,
        edge_faces,
        face_solids,
    }
}
