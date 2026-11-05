//! Live working state for external builder composition.
use super::{BuilderExtensionState, BuilderExtensionStateMut};
use martensite_tkbrep::topods::{Edge, Face, Shape, Solid};
use martensite_tkernel::ncollection::List;

impl BuilderExtensionState<'_> {
    /// Borrows the retained avoided edges; `let slot = state.avoided_edges();`.
    pub fn avoided_edges(&self) -> &List<Shape> {
        &self.builder.my_edge_avoid
    }

    /// Borrows the retained reference edge; `let slot = state.reference_edge();`.
    pub fn reference_edge(&self) -> &Edge {
        &self.builder.my_edge_reference
    }

    /// Borrows the retained destination edge; `let slot = state.destination_edge();`.
    pub fn destination_edge(&self) -> &Edge {
        &self.builder.my_edge_to_fill
    }

    /// Borrows the retained avoided faces; `let slot = state.avoided_faces();`.
    pub fn avoided_faces(&self) -> &List<Shape> {
        &self.builder.my_face_avoid
    }

    /// Borrows the retained reference face; `let slot = state.reference_face();`.
    pub fn reference_face(&self) -> &Face {
        &self.builder.my_face_reference
    }

    /// Borrows the retained destination face; `let slot = state.destination_face();`.
    pub fn destination_face(&self) -> &Face {
        &self.builder.my_face_to_fill
    }

    /// Borrows the retained working edges; `let slot = state.working_edges();`.
    pub fn working_edges(&self) -> &List<Shape> {
        &self.builder.my_list_of_edge
    }

    /// Borrows the retained working faces; `let slot = state.working_faces();`.
    pub fn working_faces(&self) -> &List<Shape> {
        &self.builder.my_list_of_face
    }

    /// Borrows the retained working solids; `let slot = state.working_solids();`.
    pub fn working_solids(&self) -> &List<Shape> {
        &self.builder.my_list_of_solid
    }

    /// Borrows the retained first operand; `let slot = state.first_operand();`.
    pub fn first_operand(&self) -> &Shape {
        &self.builder.my_shape1
    }

    /// Borrows the retained second operand; `let slot = state.second_operand();`.
    pub fn second_operand(&self) -> &Shape {
        &self.builder.my_shape2
    }

    /// Borrows the retained reference solid; `let slot = state.reference_solid();`.
    pub fn reference_solid(&self) -> &Solid {
        &self.builder.my_solid_reference
    }

    /// Borrows the retained destination solid; `let slot = state.destination_solid();`.
    pub fn destination_solid(&self) -> &Solid {
        &self.builder.my_solid_to_fill
    }

    /// Borrows the retained avoided vertices; `let slot = state.avoided_vertices();`.
    pub fn avoided_vertices(&self) -> &List<Shape> {
        &self.builder.my_vertex_avoid
    }
}
impl BuilderExtensionStateMut<'_> {
    /// Edits the retained avoided edges; `let slot = state.avoided_edges_mut();`.
    pub fn avoided_edges_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_edge_avoid
    }

    /// Edits the retained reference edge; `let slot = state.reference_edge_mut();`.
    pub fn reference_edge_mut(&mut self) -> &mut Edge {
        &mut self.builder.my_edge_reference
    }

    /// Edits the retained destination edge; `let slot = state.destination_edge_mut();`.
    pub fn destination_edge_mut(&mut self) -> &mut Edge {
        &mut self.builder.my_edge_to_fill
    }

    /// Edits the retained avoided faces; `let slot = state.avoided_faces_mut();`.
    pub fn avoided_faces_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_face_avoid
    }

    /// Edits the retained reference face; `let slot = state.reference_face_mut();`.
    pub fn reference_face_mut(&mut self) -> &mut Face {
        &mut self.builder.my_face_reference
    }

    /// Edits the retained destination face; `let slot = state.destination_face_mut();`.
    pub fn destination_face_mut(&mut self) -> &mut Face {
        &mut self.builder.my_face_to_fill
    }

    /// Edits the retained working edges; `let slot = state.working_edges_mut();`.
    pub fn working_edges_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_list_of_edge
    }

    /// Edits the retained working faces; `let slot = state.working_faces_mut();`.
    pub fn working_faces_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_list_of_face
    }

    /// Edits the retained working solids; `let slot = state.working_solids_mut();`.
    pub fn working_solids_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_list_of_solid
    }

    /// Edits the retained first operand; `let slot = state.first_operand_mut();`.
    pub fn first_operand_mut(&mut self) -> &mut Shape {
        &mut self.builder.my_shape1
    }

    /// Edits the retained second operand; `let slot = state.second_operand_mut();`.
    pub fn second_operand_mut(&mut self) -> &mut Shape {
        &mut self.builder.my_shape2
    }

    /// Edits the retained reference solid; `let slot = state.reference_solid_mut();`.
    pub fn reference_solid_mut(&mut self) -> &mut Solid {
        &mut self.builder.my_solid_reference
    }

    /// Edits the retained destination solid; `let slot = state.destination_solid_mut();`.
    pub fn destination_solid_mut(&mut self) -> &mut Solid {
        &mut self.builder.my_solid_to_fill
    }

    /// Edits the retained avoided vertices; `let slot = state.avoided_vertices_mut();`.
    pub fn avoided_vertices_mut(&mut self) -> &mut List<Shape> {
        &mut self.builder.my_vertex_avoid
    }
}
