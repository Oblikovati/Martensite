//! Live results state for external builder composition.
use super::super::{BuilderResultArray, DataMapOfShapeListOfShape};
use super::{BuilderExtensionState, BuilderExtensionStateMut};
use crate::top_ope_brep_ds::TopOpeBRepDSHDataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{DataMap, IndexedDataMap, IndexedMap, List, Map};
use martensite_tkernel::standard::Handle;

impl BuilderExtensionState<'_> {
    /// Borrows the retained consumed faces; `let slot = state.consumed_faces();`.
    pub fn consumed_faces(&self) -> &DataMap<Shape, bool, ShapeMapHasher> {
        &self.builder.my_consumed_faces
    }

    /// Borrows the retained intersection structure; `let slot = state.intersection_structure();`.
    pub fn intersection_structure(&self) -> &Handle<TopOpeBRepDSHDataStructure> {
        &self.builder.my_data_structure
    }

    /// Borrows the retained edge splits; `let slot = state.edge_splits();`.
    pub fn edge_splits(&self) -> &DataMapOfShapeListOfShape {
        &self.builder.my_e_splits
    }

    /// Borrows the retained face splits; `let slot = state.face_splits();`.
    pub fn face_splits(&self) -> &DataMapOfShapeListOfShape {
        &self.builder.my_f_splits
    }

    /// Borrows the retained paired faces; `let slot = state.paired_faces();`.
    pub fn paired_faces(&self) -> &DataMapOfShapeListOfShape {
        &self.builder.my_kp_map_f1f2
    }

    /// Borrows the retained split memo; `let slot = state.split_memo();`.
    pub fn split_memo(&self) -> &Map<Shape, ShapeMapHasher> {
        &self.builder.my_memo_split
    }

    /// Borrows the retained created faces; `let slot = state.created_faces();`.
    pub fn created_faces(&self) -> &BuilderResultArray<List<Shape>> {
        &self.builder.my_new_faces
    }

    /// Borrows the retained created vertices; `let slot = state.created_vertices();`.
    pub fn created_vertices(&self) -> &BuilderResultArray<Shape> {
        &self.builder.my_new_vertices
    }

    /// Borrows the retained on elements; `let slot = state.on_elements();`.
    pub fn on_elements(&self) -> &IndexedMap<Shape> {
        &self.builder.my_on_elem_map
    }

    /// Borrows the retained on faces; `let slot = state.on_faces();`.
    pub fn on_faces(&self) -> &IndexedDataMap<Shape, Shape, ShapeMapHasher> {
        &self.builder.my_on_faces_map
    }

    /// Borrows the retained section shapes; `let slot = state.section_shapes();`.
    pub fn section_shapes(&self) -> &List<Shape> {
        &self.builder.my_section
    }
}
impl BuilderExtensionStateMut<'_> {
    /// Edits the retained consumed faces; `let slot = state.consumed_faces_mut();`.
    pub fn consumed_faces_mut(&mut self) -> &mut DataMap<Shape, bool, ShapeMapHasher> {
        &mut self.builder.my_consumed_faces
    }

    /// Edits the retained intersection structure; `let slot = state.intersection_structure_mut();`.
    pub fn intersection_structure_mut(&mut self) -> &mut Handle<TopOpeBRepDSHDataStructure> {
        &mut self.builder.my_data_structure
    }

    /// Edits the retained edge splits; `let slot = state.edge_splits_mut();`.
    pub fn edge_splits_mut(&mut self) -> &mut DataMapOfShapeListOfShape {
        &mut self.builder.my_e_splits
    }

    /// Edits the retained face splits; `let slot = state.face_splits_mut();`.
    pub fn face_splits_mut(&mut self) -> &mut DataMapOfShapeListOfShape {
        &mut self.builder.my_f_splits
    }

    /// Edits the retained paired faces; `let slot = state.paired_faces_mut();`.
    pub fn paired_faces_mut(&mut self) -> &mut DataMapOfShapeListOfShape {
        &mut self.builder.my_kp_map_f1f2
    }

    /// Edits the retained split memo; `let slot = state.split_memo_mut();`.
    pub fn split_memo_mut(&mut self) -> &mut Map<Shape, ShapeMapHasher> {
        &mut self.builder.my_memo_split
    }

    /// Edits the retained created faces; `let slot = state.created_faces_mut();`.
    pub fn created_faces_mut(&mut self) -> &mut BuilderResultArray<List<Shape>> {
        &mut self.builder.my_new_faces
    }

    /// Edits the retained created vertices; `let slot = state.created_vertices_mut();`.
    pub fn created_vertices_mut(&mut self) -> &mut BuilderResultArray<Shape> {
        &mut self.builder.my_new_vertices
    }

    /// Edits the retained on elements; `let slot = state.on_elements_mut();`.
    pub fn on_elements_mut(&mut self) -> &mut IndexedMap<Shape> {
        &mut self.builder.my_on_elem_map
    }

    /// Edits the retained on faces; `let slot = state.on_faces_mut();`.
    pub fn on_faces_mut(&mut self) -> &mut IndexedDataMap<Shape, Shape, ShapeMapHasher> {
        &mut self.builder.my_on_faces_map
    }

    /// Edits the retained section shapes; `let slot = state.section_shapes_mut();`.
    pub fn section_shapes_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_section
    }
}
