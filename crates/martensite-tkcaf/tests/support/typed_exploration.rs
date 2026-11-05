//! Source typed explode chooses the first new identity after seeding the source shape.
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Return the source's first distinct explored kind; `first_distinct(&compound, ShapeEnum::Solid)`.
#[allow(dead_code)] // Some hosts retain the full typed binding list.
pub fn first_distinct(shape: &Shape, kind: ShapeEnum) -> Shape {
    let mut seen = IndexedMapOfShape::default();
    seen.add(shape.clone());
    for entity in topexp::Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        if seen.contains(&entity) {
            continue;
        }
        return entity;
    }
    panic!("explode: shape {shape:?}, expected a distinct {kind:?}");
}

/// Preserve every distinct typed binding in source order; `all_distinct(&shape, ShapeEnum::Face)`.
#[allow(dead_code)] // First-binding-only hosts need not enumerate every identity.
pub fn all_distinct(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut seen = IndexedMapOfShape::default();
    seen.add(shape.clone());
    topexp::Explorer::new(shape, kind, ShapeEnum::Shape)
        .unwrap()
        .filter(|entity| {
            if seen.contains(entity) {
                return false;
            }
            seen.add(entity.clone());
            true
        })
        .collect()
}
