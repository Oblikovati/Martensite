//! Original -t counts use independent typed exploration, including its matching-container stop.
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;

const KINDS: [ShapeEnum; 8] = [
    ShapeEnum::Vertex,
    ShapeEnum::Edge,
    ShapeEnum::Wire,
    ShapeEnum::Face,
    ShapeEnum::Shell,
    ShapeEnum::Solid,
    ShapeEnum::CompSolid,
    ShapeEnum::Compound,
];

/// Observe all -t categories without adding predicates; `let typed = counts(&shape)`.
pub fn counts(result: &Shape) -> [usize; 9] {
    let mut observed = [0_usize; 9];
    for (index, kind) in KINDS.into_iter().enumerate() {
        let mut unique = IndexedMapOfShape::default();
        // DBRep::dump_extent maps each type separately; an all-shapes map descends nested Compounds.
        topexp::map_shapes_of_type(result, kind, &mut unique).unwrap_or_else(|error| {
            panic!("typed count {kind:?}: expected successful topology exploration, got {error}")
        });
        observed[index] = unique.extent();
        observed[8] += observed[index];
    }
    observed
}

/// Check all eight topology kinds and total; `assert_counts(shape, &counts)`.
pub(super) fn assert_counts(result: &Shape, expected: &[usize; 9]) {
    let observed = counts(result);
    assert_eq!(observed[8], expected[8], "checknbshapes: total");
    for (index, kind) in KINDS.into_iter().enumerate() {
        assert_eq!(observed[index], expected[index], "checknbshapes: {kind:?}");
    }
}
