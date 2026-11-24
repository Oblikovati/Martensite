//! Object persistence and entity flipping use the same model-view matrix as rendering.
use super::*;
use crate::{
    prs_mgr::PrsMgrPresentableObjectMembers,
    select3d::{Select3DSensitiveEntity, Select3DSensitiveEntityMembers},
};
use martensite_tkernel::{
    ncollection::Mat4,
    standard::{Failure, Handle},
};
use martensite_tkmath::gp::GeneralTransform3;
use martensite_tkservice::graphic3d::*;
pub(super) fn selector_object_inverse(
    object: &Handle<dyn SelectMgrSelectableObject>,
    view: &SelectMgrTraversalView,
) -> Result<Option<GeneralTransform3>, Failure> {
    let persistence = object.transform_persistence();
    if persistence.is_null() {
        return Ok(Some(if object.has_transformation() {
            object.inversed_transformation()
        } else {
            GeneralTransform3::default()
        }));
    }
    if view.camera().is_null() {
        return Ok(None);
    }
    persistent_object_inverse(&persistence, object, view).map(Some)
}
fn persistent_object_inverse(
    persistence: &Handle<dyn Graphic3dTransformPersSource>,
    object: &Handle<dyn SelectMgrSelectableObject>,
    view: &SelectMgrTraversalView,
) -> Result<GeneralTransform3, Failure> {
    let matrix = persistence.compute(
        &view.camera(),
        &view.projection,
        &view.world_view,
        view.size,
        false,
    )?;
    let mut transform = GeneralTransform3::default();
    transform.set_mat4(&matrix);
    (transform * GeneralTransform3::from(object.transformation())).inverted()
}
pub(super) fn selector_entity_inverse(
    entity: &Handle<dyn Select3DSensitiveEntity>,
    object: &Handle<dyn SelectMgrSelectableObject>,
    inverse: &GeneralTransform3,
    view: &SelectMgrTraversalView,
) -> Result<Option<GeneralTransform3>, Failure> {
    let persistence = entity.transform_persistence();
    let mut sensitive = *inverse;
    if !persistence.is_null() {
        if view.camera().is_null() {
            return Ok(None);
        }
        sensitive = persistent_object_inverse(&persistence, object, view)?;
    }
    let mut flipped = GeneralTransform3::default();
    // Only a flipped entity needs the model-view matrix (#594).
    if let Some(flipper) = entity.flipper().get() {
        flipped.set_mat4(&flipper.compute(&entity_model_view(object, view)));
    }
    Ok(Some(flipped * sensitive))
}
/// The view's world-view matrix times the object's transformation.
fn entity_model_view(
    object: &Handle<dyn SelectMgrSelectableObject>,
    view: &SelectMgrTraversalView,
) -> Mat4<f64> {
    if !object.has_transformation() {
        return view.world_view;
    }
    let mut model = Mat4::identity();
    object.transformation().get_mat4(&mut model);
    view.world_view * model
}
