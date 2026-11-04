//! The real group-local pushes surround flipper commands on one shared model-world stack.
use super::*;
use crate::RenderMatrixState;
use martensite_tkmath::gp::TransformForm;

pub(crate) struct FlippingModelState {
    pub model: RenderMatrixState<f32>,
    pub view: Mat4<f32>,
    pub local_pops: usize,
}

const _: fn(&FlippingModelState) -> &(dyn Send + Sync) = |value| value;

/// Apply the group's local transform and persistence to the incoming native model stack.
/// Example: `flipping_model_state(context, previous_model_state)?`.
pub(crate) fn flipping_model_state(
    context: &crate::text_submission::TextSceneContext<'_>,
    incoming: Option<RenderMatrixState<f32>>,
) -> Result<FlippingModelState, RenderError> {
    let mut model = incoming.unwrap_or_else(|| structure_model_state(context.structure));
    let mut transform = PersistentView::new(context.camera, Mat4::from_other(model.current()));
    let local = context.group.transform_persistence();
    let extent = persistence_extent(context, &local)?;
    transform.apply(&context.structure.trsf_pers, extent, None)?;
    let mut local_pops = push_group_transform(context.group, &mut model);
    let anchor = push_local_anchor(&local, &mut model, &mut local_pops)?;
    transform.apply(&local, extent, anchor.as_ref())?;
    Ok(FlippingModelState {
        model,
        view: transform.world_view,
        local_pops,
    })
}

fn structure_model_state(structure: &Graphic3dCStructureFields) -> RenderMatrixState<f32> {
    let mut state = RenderMatrixState::default();
    state.push();
    if let Some(datum) = structure.trsf.get() {
        datum.transformation().get_mat4(state.change_current());
    }
    state
}

fn push_group_transform(group: &Graphic3dDrawGroup, state: &mut RenderMatrixState<f32>) -> usize {
    let transform = group.transformation();
    if transform.form() == TransformForm::Identity {
        return 0;
    }
    let mut local = Mat4::<f32>::identity();
    transform.get_mat4(&mut local);
    state.push();
    *state.change_current() *= local;
    1
}

fn push_local_anchor(
    local: &Handle<dyn Graphic3dTransformPersSource>,
    model: &mut RenderMatrixState<f32>,
    pops: &mut usize,
) -> Result<Option<Point3>, RenderError> {
    if local.is_null() || !local.is_zoom_or_rotate() {
        return Ok(None);
    }
    model.push();
    *pops += 1;
    local_anchor(local, model.change_current())
}

fn persistence_extent(
    context: &crate::text_submission::TextSceneContext<'_>,
    local: &Handle<dyn Graphic3dTransformPersSource>,
) -> Result<[i32; 2], RenderError> {
    if context.structure.trsf_pers.is_null() && local.is_null() {
        return Ok([1, 1]);
    }
    viewport_size(context.persistence_extent())
}
