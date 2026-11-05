//! Shape-set deduplication keeps separately oriented keys on each addition path.
use super::shape_neighbour_extension::connected_edges;
use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

fn oriented_additions(edge: &Shape) -> [Shape; 4] {
    [edge.clone(), edge.clone(), edge.reversed(), edge.reversed()]
}

fn collected_shapes(set: &mut impl TopOpeBRepBuildShapeSet) -> Result<Vec<Shape>, Failure> {
    let mut shapes = Vec::new();
    set.init_shapes();
    while set.more_shapes() {
        shapes.push(set.shape()?.clone());
        set.next_shape();
    }
    Ok(shapes)
}

fn assert_oriented_pair(shapes: &[Shape], edge: &Shape) {
    assert_eq!(shapes.len(), 2);
    assert!(shapes.iter().all(|shape| shape.is_same(edge)));
    assert_eq!(shapes[0].orientation(), Orientation::Forward);
    assert_eq!(shapes[1].orientation(), Orientation::Reversed);
}

#[test]
fn repeated_oriented_start_additions_create_two_starts_and_two_incidents() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    for variant in oriented_additions(&edge) {
        set.add_start_element(&variant)?;
    }
    let starts: Vec<Shape> = set.start_elements().iter().cloned().collect();
    assert_oriented_pair(&starts, &edge);
    assert_eq!(set.max_number_sub_shape(&edge)?, 2);
    assert!(collected_shapes(&mut set)?.is_empty());
    Ok(())
}

#[test]
fn repeated_oriented_element_additions_leave_start_and_shape_lists_empty() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    for variant in oriented_additions(&edge) {
        set.add_element(&variant)?;
    }
    assert_eq!(set.max_number_sub_shape(&edge)?, 2);
    assert!(set.start_elements().is_empty());
    assert!(collected_shapes(&mut set)?.is_empty());
    Ok(())
}

#[test]
fn repeated_oriented_shape_additions_leave_element_and_start_state_empty() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    for variant in oriented_additions(&edge) {
        set.add_shape(&variant)?;
    }
    assert_oriented_pair(&collected_shapes(&mut set)?, &edge);
    assert!(set.start_elements().is_empty());
    assert_eq!(set.max_number_sub_shape(&edge)?, 0);
    Ok(())
}

#[path = "shape_set_dedup_indexes/extension.rs"]
mod extension;
