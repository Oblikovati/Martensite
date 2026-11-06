//! Section incidence assembly is shared by the public and ranged entries.
use super::face_sections::{add_face_common_edges, add_face_sections_checked};
use super::*;
impl BOPAlgoSection {
    /// Preserve the original construction order; `section.build_section_unbound(ds)?`.
    pub(super) fn build_section_unbound(
        &mut self,
        structure: &DataStructure,
    ) -> Result<(), Failure> {
        self.build_section_checked(structure, |_| false)
    }
    /// Keep real checkpoints before publication; `section.build_section_checked(ds, cancelled)?`.
    pub(super) fn build_section_checked(
        &mut self,
        structure: &DataStructure,
        mut cancelled: impl FnMut(&Self) -> bool,
    ) -> Result<(), Failure> {
        self.builder.options().clear();
        let mut sections = Shape::default();
        make_container(ShapeEnum::Compound, &mut sections);
        if !add_face_sections_checked(structure, &mut sections, || cancelled(self))? {
            return Ok(());
        }
        add_face_common_edges(structure, &mut sections)?;
        let Some(counts) = self.boundary_counts_checked(&mut cancelled)? else {
            return Ok(());
        };
        let vertex_edges = section_vertex_incidence(&sections, &counts)?;
        let Some(result) = section_vertex_result(&vertex_edges, || cancelled(self))? else {
            return Ok(());
        };
        self.builder.shape = result;
        Ok(())
    }
}
fn section_vertex_incidence(
    sections: &Shape,
    counts: &IndexedDataMap<Shape, i32, ShapeMapHasher>,
) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut vertex_edges = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(
        sections,
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        &mut vertex_edges,
    )?;
    for position in 1..=counts.extent() {
        if *counts.find_from_index(position)? > 1 {
            topexp::map_shapes_and_ancestors(
                counts.find_key(position)?,
                ShapeEnum::Vertex,
                ShapeEnum::Edge,
                &mut vertex_edges,
            )?;
        }
    }
    Ok(vertex_edges)
}
fn section_vertex_result(
    vertex_edges: &IndexedDataMapOfShapeListOfShape,
    mut cancelled: impl FnMut() -> bool,
) -> Result<Option<Shape>, Failure> {
    let mut result = Shape::default();
    make_container(ShapeEnum::Compound, &mut result);
    let mut fence = MapOfShape::default();
    for position in 1..=vertex_edges.extent() {
        if cancelled() {
            return Ok(None);
        }
        append_section_vertex_row(vertex_edges, position, &mut fence, &mut result)?;
    }
    Ok(Some(result))
}
fn append_section_vertex_row(
    vertex_edges: &IndexedDataMapOfShapeListOfShape,
    position: usize,
    fence: &mut MapOfShape,
    result: &mut Shape,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let edges = vertex_edges.find_from_index(position)?;
    if edges.is_empty() {
        // Alone vertex.
        let vertex = vertex_edges.find_key(position)?;
        if fence.add(vertex.clone()) {
            builder.add(result, vertex)?;
        }
        return Ok(());
    }
    for edge in edges.iter() {
        if fence.add(edge.clone()) {
            builder.add(result, edge)?;
        }
    }
    Ok(())
}
