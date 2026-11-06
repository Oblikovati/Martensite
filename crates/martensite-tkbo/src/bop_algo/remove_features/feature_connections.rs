//! Operation-local incidence maps outlive every gap task and its borrowed connection view.
use super::super::fill_gap::Connections;
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedDataMapOfShapeListOfShape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
pub(super) struct FeatureIncidences {
    edge_faces: IndexedDataMapOfShapeListOfShape,
    face_solids: IndexedDataMapOfShapeListOfShape,
}
impl FeatureIncidences {
    pub(super) fn new(shape: &Shape) -> Result<Self, Failure> {
        Ok(Self {
            edge_faces: incidence_relations(shape, ShapeEnum::Edge, ShapeEnum::Face)?,
            face_solids: incidence_relations(shape, ShapeEnum::Face, ShapeEnum::Solid)?,
        })
    }
    pub(super) fn connections(&self) -> Connections<'_> {
        Connections {
            edge_faces: &self.edge_faces,
            face_solids: &self.face_solids,
        }
    }
}
fn incidence_relations(
    shape: &Shape,
    from: ShapeEnum,
    to: ShapeEnum,
) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut relations = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shape, from, to, &mut relations)?;
    Ok(relations)
}
