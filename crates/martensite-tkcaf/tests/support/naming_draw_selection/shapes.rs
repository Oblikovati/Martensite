//! Exact command-host incidence/multiple-shape collection and comparison diagnostics.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Compound, Shape, ShapeIterator},
    toptools::MapOfShape,
};
use martensite_tkernel::{ncollection::Map, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

use super::super::e2_pipeline::collect_shapes;
/// Original CollectShapes then RemoveFirst, with no deduplication; `single_shapes(&shape)`.
pub(super) fn single_shapes(shape: &Shape) -> Vec<Shape> {
    let mut shapes = Vec::new();
    collect_shapes(shape, &mut shapes);
    shapes.remove(0);
    shapes
}
/// Original compound for every lower shape kind, including empty compounds; `multiple_shapes(&shape)?`.
pub(super) fn multiple_shapes(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut compounds = Vec::new();
    for kind in ShapeEnum::ALL
        .into_iter()
        .take(8)
        .skip(shape.shape_type()? as usize + 1)
    {
        let mut compound = Compound::default();
        let builder = BRepBuilder::default();
        builder.make_compound(&mut compound);
        for item in Explorer::new(shape, kind, ShapeEnum::Shape)? {
            builder.add(&mut compound, &item)?;
        }
        compounds.push((*compound).clone());
    }
    Ok(compounds)
}
/// Original null and degenerate-edge skipping only; `eligible(&shape)?`.
pub(super) fn eligible(shape: &Shape) -> Result<bool, Failure> {
    if shape.is_null() {
        return Ok(false);
    }
    if shape.shape_type()? != ShapeEnum::Edge {
        return Ok(true);
    }
    Ok(!BRepTool::degenerated(&topods::edge(shape)?)?)
}
/// Orientation-sensitive default shape map includes all incidences; `oriented_context(&shape)`.
pub(super) fn oriented_context(shape: &Shape) -> Map<Shape> {
    let mut shapes = Vec::new();
    collect_shapes(shape, &mut shapes);
    let mut context = Map::default();
    for item in shapes {
        context.add(item);
    }
    context
}
fn compound_contains(old: &Shape, new: &Shape, keep: bool) -> Result<bool, Failure> {
    if keep {
        for item in ShapeIterator::new(new, true, true) {
            if item.shape_type()? == old.shape_type()? && item == *old {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    let mut children = MapOfShape::default();
    for child in ShapeIterator::new(new, true, true) {
        if child.shape_type()? == old.shape_type()? {
            children.add(child);
        }
    }
    Ok(children.contains(old))
}
fn identity_difference(old: &Shape, new: &Shape) -> Result<String, Failure> {
    if old.shape_type()? != new.shape_type()? {
        return Ok(format!(
            " Type was changed: Initial type = {}; Resulting type = {}",
            old.shape_type()? as i32,
            new.shape_type()? as i32
        ));
    }
    if old.tshape() != new.tshape() {
        return Ok(" TShape was changed".into());
    }
    if old.location() != new.location() {
        return Ok(" Location was changed".into());
    }
    Ok(String::new())
}
fn orientation_difference(old: &Shape, new: &Shape) -> Result<bool, Failure> {
    if old.shape_type()? == ShapeEnum::Vertex || old.orientation() == new.orientation() {
        return Ok(false);
    }
    if new.shape_type()? != ShapeEnum::Compound {
        return Ok(true);
    }
    Ok(ShapeIterator::new(new, true, true)
        .any(|item| item.is_same(old) && item.orientation() != old.orientation()))
}
/// Exact compareShapes including accepted Warning type migration; `compare_single(&old,&new,true)?`.
pub(super) fn compare_single(old: &Shape, new: &Shape, keep: bool) -> Result<String, Failure> {
    if old.shape_type()? != ShapeEnum::Compound
        && new.shape_type()? == ShapeEnum::Compound
        && compound_contains(old, new, keep)?
    {
        return Ok(" Warning => Type migration to Compound".into());
    }
    let mut result = identity_difference(old, new)?;
    if keep && orientation_difference(old, new)? {
        if !result.is_empty() {
            result.push(',');
        }
        result.push_str(" Orientation was changed");
    }
    Ok(result)
}
/// Exact multiple comparison uses every immediate compound child; `compare_multiple(&shape,&context)?`.
pub(super) fn compare_multiple(shape: &Shape, context: &Map<Shape>) -> Result<String, Failure> {
    if shape.shape_type()? != ShapeEnum::Compound {
        return Ok("the specified shape is not COMPOUND".into());
    }
    let mut result = String::new();
    for child in ShapeIterator::new(shape, true, true) {
        if !context.contains(&child) {
            result.push_str(" Not in the context");
        }
    }
    Ok(result)
}

#[cfg(test)]
mod checks {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    fn cuboid() -> Shape {
        let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
        maker.shape().unwrap().clone()
    }
    fn wrapped(shape: &Shape) -> Shape {
        let mut compound = Compound::default();
        let builder = BRepBuilder::default();
        builder.make_compound(&mut compound);
        builder.add(&mut compound, shape).unwrap();
        (*compound).clone()
    }
    fn first_face(shape: &Shape) -> Shape {
        Iterator::next(&mut Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap())
            .unwrap()
    }
    #[test]
    fn single_comparison_respects_orientation_and_accepted_compound_migration() {
        let box_shape = cuboid();
        let face = first_face(&box_shape);
        let mut reversed = face.clone();
        reversed.reverse();
        assert_eq!(compare_single(&face, &reversed, false).unwrap(), "");
        assert_eq!(
            compare_single(&face, &reversed, true).unwrap(),
            " Orientation was changed"
        );
        assert_eq!(
            compare_single(&face, &wrapped(&face), true).unwrap(),
            " Warning => Type migration to Compound"
        );
        assert!(
            compare_single(&face, &wrapped(&reversed), true)
                .unwrap()
                .contains("Type was changed")
        );
    }
    fn moved_shape(shape: &Shape) -> Shape {
        let location = martensite_tkmath::toploc::Location::new(
            martensite_tkmath::gp::Transform3::translation([1.0, 2.0, 3.0]),
        );
        let mut moved = shape.clone();
        moved.set_location(&location, true).unwrap();
        moved
    }
    #[test]
    fn single_comparison_distinguishes_shape_identity_location_and_type() {
        let box_shape = cuboid();
        let face = first_face(&box_shape);
        let other_face = first_face(&cuboid());
        assert_eq!(
            compare_single(&face, &other_face, false).unwrap(),
            " TShape was changed"
        );
        let moved = moved_shape(&face);
        assert_eq!(
            compare_single(&face, &moved, false).unwrap(),
            " Location was changed"
        );
        assert!(
            compare_single(&face, &box_shape, false)
                .unwrap()
                .contains("Type was changed")
        );
    }
    #[test]
    fn multiple_collection_retains_empty_compounds_for_every_lower_kind() {
        let box_shape = cuboid();
        let compounds = multiple_shapes(&wrapped(&box_shape)).unwrap();
        assert_eq!(compounds.len(), 7);
        assert_eq!(compounds[0].nb_children(), 0);
    }
    #[test]
    fn multiple_comparison_preserves_oriented_membership_and_empty_compounds() {
        let box_shape = cuboid();
        let context = oriented_context(&box_shape);
        assert_eq!(
            compare_multiple(&wrapped(&box_shape), &context).unwrap(),
            ""
        );
        let mut reversed = box_shape.clone();
        reversed.reverse();
        assert_eq!(
            compare_multiple(&wrapped(&reversed), &context).unwrap(),
            " Not in the context"
        );
        assert_eq!(
            compare_multiple(&box_shape, &context).unwrap(),
            "the specified shape is not COMPOUND"
        );
    }
}
