//! Shared native DNaming image orientation and generated/modified/deleted history loading.
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::MapOfShape};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingShapeShapeMap};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Native result-shape orientation table; `orientation_map(&shape, &[ShapeEnum::Face])?`.
pub fn orientation_map(
    shape: &Shape,
    kinds: &[ShapeEnum],
) -> Result<TNamingShapeShapeMap, Failure> {
    let mut orientations = TNamingShapeShapeMap::default();
    for kind in kinds {
        for item in Explorer::new(shape, *kind, ShapeEnum::Shape)? {
            orientations.bind(item.clone(), item);
        }
    }
    Ok(orientations)
}
/// Use an actual result incidence's orientation; `oriented_image(&image, &map)`.
pub fn oriented_image(image: &Shape, orientations: &TNamingShapeShapeMap) -> Shape {
    let mut oriented = image.clone();
    if let Some(original) = orientations.seek(image) {
        oriented.set_orientation(original.orientation());
    }
    oriented
}

/// DNaming::LoadAndOrientGeneratedShapes; `load_generated(&mut maker, &source, ShapeEnum::Edge, &builder, &map)?`.
pub fn load_generated(
    maker: &mut impl BRepBuilderAPIMakeShape,
    source: &Shape,
    kind: ShapeEnum,
    builder: &TNamingBuilder,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    load_images(maker, source, kind, builder, orientations, true)
}
/// DNaming::LoadAndOrientModifiedShapes; `load_modified(&mut maker, &source, ShapeEnum::Face, &builder, &map)?`.
pub fn load_modified(
    maker: &mut impl BRepBuilderAPIMakeShape,
    source: &Shape,
    kind: ShapeEnum,
    builder: &TNamingBuilder,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    load_images(maker, source, kind, builder, orientations, false)
}
fn load_images(
    maker: &mut impl BRepBuilderAPIMakeShape,
    source: &Shape,
    kind: ShapeEnum,
    builder: &TNamingBuilder,
    orientations: &TNamingShapeShapeMap,
    generated: bool,
) -> Result<(), Failure> {
    let mut seen = MapOfShape::default();
    for old in Explorer::new(source, kind, ShapeEnum::Shape)? {
        if !seen.add(old.clone()) {
            continue;
        }
        load_root_images(maker, &old, builder, orientations, generated)?;
    }
    Ok(())
}

fn root_images(
    maker: &mut impl BRepBuilderAPIMakeShape,
    old: &Shape,
    generated: bool,
) -> Result<Vec<Shape>, Failure> {
    let images = if generated {
        maker.generated(old)?
    } else {
        maker.modified(old)?
    };
    Ok(images.iter().cloned().collect())
}
fn load_root_images(
    maker: &mut impl BRepBuilderAPIMakeShape,
    old: &Shape,
    builder: &TNamingBuilder,
    orientations: &TNamingShapeShapeMap,
    generated: bool,
) -> Result<(), Failure> {
    for image in root_images(maker, old, generated)? {
        let new = oriented_image(&image, orientations);
        if !old.is_same(&new) {
            record_pair(builder, old, &new, generated)?;
        }
    }
    Ok(())
}
fn record_pair(
    builder: &TNamingBuilder,
    old: &Shape,
    new: &Shape,
    generated: bool,
) -> Result<(), Failure> {
    if generated {
        return builder.generated_from(old, new);
    }
    builder.modify(old, new)
}

/// DNaming::LoadDeletedShapes; `load_deleted(&mut maker, &source, ShapeEnum::Face, &builder)?`.
pub fn load_deleted(
    maker: &mut impl BRepBuilderAPIMakeShape,
    source: &Shape,
    kind: ShapeEnum,
    builder: &TNamingBuilder,
) -> Result<(), Failure> {
    let mut seen = MapOfShape::default();
    for old in Explorer::new(source, kind, ShapeEnum::Shape)? {
        if seen.add(old.clone()) && maker.is_deleted(&old)? {
            builder.delete(&old)?;
        }
    }
    Ok(())
}
