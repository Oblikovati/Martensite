// SPDX-License-Identifier: GPL-2.0-only
//! Enumerate mapped context requirements explicitly, including missing entries.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::ncollection::IndexedMap;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn checked_occurrences(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut found = IndexedMap::default();
    append_occurrences(shape, &mut found)?;
    Ok(found.iter().cloned().collect())
}

fn append_occurrences(shape: &Shape, found: &mut IndexedMap<Shape>) -> Result<(), Failure> {
    // The default shape equality includes orientation. Keep one owner per
    // occurrence while visiting the live graph, in first-visit order.
    if found.contains(shape) {
        return Ok(());
    }
    found.add(shape.clone());
    let mut children = ShapeIterator::new(shape, true, true);
    while children.more() {
        append_occurrences(children.value()?, found)?;
        children.next();
    }
    Ok(())
}

pub(super) fn required_contexts(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    // Native contextual loops check one IsSame entity, not both orientations.
    let mut found = Vec::new();
    for kind in required_kinds(shape.shape_type()?) {
        append_kind_contexts(shape, *kind, &mut found)?;
    }
    Ok(found)
}

fn append_kind_contexts(
    shape: &Shape,
    kind: ShapeEnum,
    found: &mut Vec<Shape>,
) -> Result<(), Failure> {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape)?;
    while explorer.more() {
        let subshape = explorer.current();
        if !found.iter().any(|entry| entry.is_same(subshape)) {
            found.push(subshape.clone());
        }
        explorer.next()?;
    }
    Ok(())
}

fn required_kinds(kind: ShapeEnum) -> &'static [ShapeEnum] {
    match kind {
        ShapeEnum::Edge => &[ShapeEnum::Vertex],
        ShapeEnum::Face => &[ShapeEnum::Wire, ShapeEnum::Edge, ShapeEnum::Vertex],
        ShapeEnum::Solid => &[ShapeEnum::Shell],
        _ => &[],
    }
}
