//! Default location-normalized ShapeSet count predicates shared by Boolean cases.
use martensite_tkbrep::{brep_tools::BRepToolsShapeSet, topods::Shape, toptools::ShapeSet};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Observe default counts without adding predicates; `let observed = counts(&shape)`.
pub fn counts(result: &Shape) -> [usize; 9] {
    // Default nbshapes normalizes locations through ShapeSet; -t has different identity rules.
    let mut shapes = BRepToolsShapeSet::default();
    shapes.add(result).unwrap();
    let mut counts: [usize; 9] = [0; 9];
    for index in 1..=shapes.nb_shapes() {
        counts[shapes.shape(index).unwrap().shape_type().unwrap() as usize] += 1;
    }
    counts[ShapeEnum::Shape as usize] = shapes.nb_shapes() as usize;
    counts
}

/// Preserve Vertex-first reference arrays while using default ShapeSet semantics.
/// Example: `assert_vertex_first_counts(&shape, &[8, 12, 6, 6, 1, 1, 0, 0, 34]);`.
pub fn assert_vertex_first_counts(result: &Shape, expected: &[usize; 9]) {
    // Older source adapters store kinds in the reverse order to ShapeEnum's Compound-first order.
    let mut observed = counts(result);
    observed[..8].reverse();
    assert_eq!(&observed, expected, "default ShapeSet Vertex-first counts");
}
