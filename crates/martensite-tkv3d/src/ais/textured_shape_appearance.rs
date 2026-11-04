//! Native textured appearance overrides update only already-existing mode3 presentations.
use super::textured_group_cursor::LiveNodeCursor;
use super::{
    AisTexturedShapeSource, ais_shape_set_color, ais_shape_set_material, ais_shape_unset_color,
    ais_shape_unset_material, ais_textured_shape_update_attributes,
};
use crate::prs_mgr::PrsMgrPresentation;
use martensite_tkernel::{
    quantity::Color,
    standard::{Failure, HandleObserver},
};
use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dStructureSource};

/// Apply inherited color then update existing textured modes; `ais_textured_shape_set_color(&object,color)?`.
pub fn ais_textured_shape_set_color<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
    color: Color,
) -> Result<(), Failure> {
    ais_shape_set_color(object, color)?;
    refresh_textured_mode_presentations(object)
}
/// Remove inherited color without refreshing the native copied texture aspect; `ais_textured_shape_unset_color(&object)?`.
pub fn ais_textured_shape_unset_color<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
) -> Result<(), Failure> {
    // Delegate directly, without the loop used by the other three overrides.
    ais_shape_unset_color(object)
}
/// Apply inherited material then refresh existing textured modes; `ais_textured_shape_set_material(&object,&material)?`.
pub fn ais_textured_shape_set_material<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
    material: &Graphic3dMaterialAspect,
) -> Result<(), Failure> {
    ais_shape_set_material(object, material)?;
    refresh_textured_mode_presentations(object)
}
/// Restore inherited material while preserving native parent ownership behavior; `ais_textured_shape_unset_material(&object)?`.
pub fn ais_textured_shape_unset_material<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
) -> Result<(), Failure> {
    ais_shape_unset_material(object)?;
    refresh_textured_mode_presentations(object)
}
fn refresh_textured_mode_presentations<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
) -> Result<(), Failure> {
    let mut cursor = LiveNodeCursor::default();
    // Iterate the live presentation Sequence node by node: a group callback
    // may remove earlier or later presentations or append new ones, and the walk goes on
    // from the same node. The guard ends before the callback; no owner is added.
    while let Some(presentation) = next_live_presentation(object, &mut cursor)? {
        if presentation.get().is_some_and(|p| p.mode() == 3) {
            let structure = presentation.observe_as::<dyn Graphic3dStructureSource>();
            ais_textured_shape_update_attributes(object, &structure)?;
        }
    }
    Ok(())
}
fn next_live_presentation<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
    cursor: &mut LiveNodeCursor,
) -> Result<Option<HandleObserver<PrsMgrPresentation>>, Failure> {
    let fields = object.presentable_base().fields.read();
    cursor.advance(&fields.presentations)
}
