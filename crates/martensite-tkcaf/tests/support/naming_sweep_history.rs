//! Shared source-identical prism/revolution root, lateral and primitive cap histories.
use super::naming_history::{load_generated, orientation_map, oriented_image};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingShapeShapeMap};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::{TdfLabel, TdfTagSource};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
fn root_names(
    result: &TdfLabel,
    shape: &Shape,
    basis: &Shape,
    context: &Shape,
) -> Result<(), Failure> {
    TdfTagSource::set(result)?.set_value(0)?;
    let builder = TNamingBuilder::new(result)?;
    if basis == context {
        builder.generated(shape)?;
    } else {
        builder.generated_from(context, shape)?;
    }
    Ok(())
}
fn lateral_names(
    result: &TdfLabel,
    maker: &mut impl BRepBuilderAPIMakeShape,
    basis: &Shape,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    let lateral = TNamingBuilder::new(&result.new_child()?)?;
    load_generated(maker, basis, ShapeEnum::Edge, &lateral, orientations)?;
    Ok(())
}
/// Load a cap or every compound cap child; `cap_names(&label,&cap,&orientations)?`.
pub(super) fn cap_names(
    result: &TdfLabel,
    shape: &Shape,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    if shape.is_null() {
        return Ok(());
    }
    if shape.shape_type()? != ShapeEnum::Compound {
        return TNamingBuilder::new(&result.new_child()?)?
            .generated(&oriented_image(shape, orientations));
    }
    for child in ShapeIterator::new(shape, true, true) {
        TNamingBuilder::new(&result.new_child()?)?.generated(&child)?;
    }
    Ok(())
}

/// Load the identical native root/tag/lateral contract; `sweep_history(&label,&mut maker,&basis,&context)?`.
pub(super) fn sweep_history(
    result: &TdfLabel,
    maker: &mut impl BRepBuilderAPIMakeShape,
    basis: &Shape,
    context: &Shape,
) -> Result<TNamingShapeShapeMap, Failure> {
    let shape = maker.shape()?.clone();
    let orientations = orientation_map(&shape, &[ShapeEnum::Face])?;
    root_names(result, &shape, basis, context)?;
    lateral_names(result, maker, basis, &orientations)?;
    Ok(orientations)
}
