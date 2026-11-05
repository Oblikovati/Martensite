//! Source default nbshapes counts identity once, even when an edge repeats the same vertex.
#[path = "default_shape_counts.rs"]
mod case_shape_set;
use martensite_tkbrep::topods::Edge;
use martensite_tkg3d::top_abs::ShapeEnum;

/// Keep only the original unique Vertex/Edge predicates; `source_edge_counts_29573(&edge)`.
pub(super) fn source_edge_counts_29573(edge: &Edge) -> [usize; 2] {
    let counts = case_shape_set::counts(edge);
    [
        counts[ShapeEnum::Vertex as usize],
        counts[ShapeEnum::Edge as usize],
    ]
}
