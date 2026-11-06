//! Original checksection toggles each visited vertex to report odd incidence counts.
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::MapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Count the original section report; `alone_vertices(&section)`.
pub(super) fn alone_vertices(result: &Shape) -> usize {
    odd_incidence_vertices(result).len()
}

/// Retain original odd-incidence vertex identities and iteration order; `odd_incidence_vertices(&section)`.
pub(super) fn odd_incidence_vertices(result: &Shape) -> Vec<Shape> {
    let mut alone = MapOfShape::default();
    let mut explorer = Explorer::new(result, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let vertex = explorer.current().clone();
        if !alone.add(vertex.clone()) {
            alone.remove(&vertex);
        }
        explorer.next().unwrap();
    }
    alone.iter().cloned().collect()
}
