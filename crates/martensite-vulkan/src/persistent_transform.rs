//! Mirror GlStructure::applyPersistence, including local anchors.
use crate::{Graphic3dDrawGroup, RenderError, error::rejected};
use martensite_tkernel::{
    ncollection::{Mat4, Vec4},
    standard::Handle,
};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::*;

#[path = "persistent_transform_flipping.rs"]
mod flipping;
pub(crate) use flipping::flipping_model_state;

pub(crate) fn group_clip_transform(
    group: &Graphic3dDrawGroup,
    camera: &Graphic3dCamera,
    extent: Option<[u32; 2]>,
    structure: &Graphic3dCStructureFields,
) -> Result<([f32; 16], bool), RenderError> {
    let parent = &structure.trsf_pers;
    let local = group.transform_persistence();
    let model = model_transform(group, structure);
    if parent.is_null() && local.is_null() {
        return Ok((ordinary_clip(camera, model), false));
    }
    let transform = persistent_view(camera, model, parent, &local, extent)?;
    let clamp = !parent.is_null() && parent.mode() == Graphic3dTransModeFlags::CAMERA_PERS;
    Ok((transform.clip_matrix(), clamp))
}
/// The clip transform of drawing after the structure's model-world is popped while its
/// persistence still applies (GlStructure::Render's highlight box), with the native depth
/// clamp of camera persistence; `structure_persistence_clip(camera, &trsf_pers, extent)?`.
pub(crate) fn structure_persistence_clip(
    camera: &Graphic3dCamera,
    persistence: &Handle<dyn Graphic3dTransformPersSource>,
    extent: Option<[u32; 2]>,
) -> Result<([f32; 16], bool), RenderError> {
    let transform = persistent_view(
        camera,
        Mat4::identity(),
        persistence,
        &Handle::default(),
        extent,
    )?;
    let clamp = persistence.mode() == Graphic3dTransModeFlags::CAMERA_PERS;
    Ok((transform.clip_matrix(), clamp))
}
fn persistent_view(
    camera: &Graphic3dCamera,
    model: Mat4<f64>,
    parent: &Handle<dyn Graphic3dTransformPersSource>,
    local: &Handle<dyn Graphic3dTransformPersSource>,
    extent: Option<[u32; 2]>,
) -> Result<PersistentView, RenderError> {
    let mut transform = PersistentView::new(camera, model);
    let extent = viewport_size(extent)?;
    transform.apply(parent, extent, None)?;
    let anchor = local_anchor(local, &mut transform.model)?;
    transform.apply(local, extent, anchor.as_ref())?;
    Ok(transform)
}
/// Model-world, world-view and projection stacks of `group`'s elements (the structure pushes the
/// model-world matrix, persistence pushes the world view for structure and group persistence and
/// the model-world for a local zoom/rotate anchor); `extent` is the persistence viewport.
pub(crate) fn element_matrix_states(
    group: &Graphic3dDrawGroup,
    camera: &Graphic3dCamera,
    extent: Option<[u32; 2]>,
    structure: &Graphic3dCStructureFields,
) -> Result<[crate::RenderMatrixState<f32>; 3], RenderError> {
    let model = model_transform(group, structure);
    let mut states: [crate::RenderMatrixState<f32>; 3] = Default::default();
    states[0].push();
    states[0].set_current_converted(&model);
    states[1].set_current(&camera.orientation_matrix_f());
    states[2].set_current(&camera.projection_matrix_f());
    let (parent, local) = (&structure.trsf_pers, group.transform_persistence());
    if parent.is_null() && local.is_null() {
        return Ok(states);
    }
    let transform = persistent_view(camera, model, parent, &local, extent)?;
    if !local.is_null() && local.is_zoom_or_rotate() {
        states[0].push();
    }
    for _ in [parent.is_null(), local.is_null()]
        .iter()
        .filter(|null| !**null)
    {
        states[1].push();
    }
    states[0].set_current(&transform.model);
    states[1].set_current(&transform.world_view);
    Ok(states)
}
pub(crate) fn text_matrices(
    group: &Graphic3dDrawGroup,
    camera: &Graphic3dCamera,
    extent: [u32; 2],
    structure: &Graphic3dCStructureFields,
) -> Result<(Mat4<f64>, Mat4<f64>), RenderError> {
    let model = model_transform(group, structure);
    let parent = &structure.trsf_pers;
    let local = group.transform_persistence();
    if parent.is_null() && local.is_null() {
        // GlText promotes the float ProjectionState and WorldView * ModelWorld product: a double camera product moves a window anchor that projects to an exact
        // pixel edge across the floor that places non-zoomable text.
        let model_view = camera.orientation_matrix_f() * Mat4::<f32>::from_other(&model);
        return Ok((
            Mat4::from_other(&camera.projection_matrix_f()),
            Mat4::from_other(&model_view),
        ));
    }
    let transform = persistent_view(camera, model, parent, &local, Some(extent))?;
    Ok((
        Mat4::from_other(&transform.projection),
        Mat4::from_other(&(transform.world_view * transform.model)),
    ))
}
struct PersistentView {
    camera: Handle<Graphic3dCamera>,
    model: Mat4<f32>,
    projection: Mat4<f32>,
    world_view: Mat4<f32>,
}
impl PersistentView {
    fn new(camera: &Graphic3dCamera, model: Mat4<f64>) -> Self {
        Self {
            camera: Handle::new(camera.copy_members()),
            model: Mat4::from_other(&model),
            // GlStructure::applyPersistence starts from the float ProjectionState and
            // WorldViewState, i.e. ProjectionMatrixF and OrientationMatrixF.
            projection: camera.projection_matrix_f(),
            world_view: camera.orientation_matrix_f(),
        }
    }
    fn apply(
        &mut self,
        persistence: &Handle<dyn Graphic3dTransformPersSource>,
        extent: [i32; 2],
        anchor: Option<&Point3>,
    ) -> Result<(), RenderError> {
        if persistence.is_null() {
            return Ok(());
        }
        persistence
            .apply_world_view(
                &self.camera,
                &self.projection,
                &mut self.world_view,
                extent,
                anchor,
                true,
            )
            .map_err(|error| rejected(error, "a valid persistent world-view transform"))
    }
    fn clip_matrix(&self) -> [f32; 16] {
        (self.projection * self.world_view * self.model).data()
    }
}

pub(crate) fn model_transform(
    group: &Graphic3dDrawGroup,
    structure: &Graphic3dCStructureFields,
) -> Mat4<f64> {
    let mut model = Mat4::<f64>::identity();
    let mut local = Mat4::<f64>::identity();
    if let Some(datum) = structure.trsf.get() {
        datum.transformation().get_mat4(&mut model);
    }
    group.transformation().get_mat4(&mut local);
    model * local
}
pub(crate) fn clipping_model(
    group: &Graphic3dDrawGroup,
    structure: &Graphic3dCStructureFields,
) -> Result<[f32; 16], RenderError> {
    let mut model = Mat4::<f32>::from_other(&model_transform(group, structure));
    // Local zoom/rotate persistence resets the native ModelWorld translation,
    // so fragment planes must see that same model, not the pre-persistence one.
    local_anchor(&group.transform_persistence(), &mut model)?;
    Ok(model.data())
}

fn viewport_size(extent: Option<[u32; 2]>) -> Result<[i32; 2], RenderError> {
    let Some([width, height]) = extent else {
        return Err(rejected(
            extent,
            "an actual viewport for transform persistence",
        ));
    };
    if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
        return Err(rejected(
            [width, height],
            "positive viewport dimensions fitting i32",
        ));
    }
    Ok([width as i32, height as i32])
}
fn local_anchor(
    persistence: &Handle<dyn Graphic3dTransformPersSource>,
    model: &mut Mat4<f32>,
) -> Result<Option<Point3>, RenderError> {
    if persistence.is_null() || !persistence.is_zoom_or_rotate() {
        return Ok(None);
    }
    let point = persistence
        .anchor_point()
        .map_err(|e| rejected(e, "a local persistence anchor"))?;
    let transformed = *model * Vec4::new(point.x() as f32, point.y() as f32, point.z() as f32, 1.);
    // OpenGL moves the local anchor through the full model, then clears translation exactly once.
    model.set_column(3, Vec4::new(0., 0., 0., 1.));
    Ok(Some(Point3::new(
        transformed.x() as f64,
        transformed.y() as f64,
        transformed.z() as f64,
    )))
}

/// The clip matrix the view's shaders form: the view uploads the float camera matrices
/// (`ProjectionMatrixF`, `OrientationMatrixF`) and a float model matrix, and the GLSL chain
/// `P * W * M * v` multiplies them in float, left to right, column sums in order. Composing
/// in double and rounding once moves vertices by a subpixel step against the recorded image.
pub(crate) fn ordinary_clip(camera: &Graphic3dCamera, model: Mat4<f64>) -> [f32; 16] {
    let model = Mat4::<f32>::from_other(&model);
    (camera.projection_matrix_f() * camera.orientation_matrix_f() * model).data()
}

// The section plane replaces ModelWorld, retaining only structure persistence.
pub(crate) fn plane_clip_transform(
    camera: &Graphic3dCamera,
    orientation: Mat4<f32>,
    structure: &Graphic3dCStructureFields,
    extent: [u32; 2],
) -> Result<([f32; 16], bool), RenderError> {
    let model = Mat4::<f64>::from_other(&orientation);
    if structure.trsf_pers.is_null() {
        return Ok((ordinary_clip(camera, model), false));
    }
    let mut transform = PersistentView::new(camera, model);
    transform.apply(&structure.trsf_pers, viewport_size(Some(extent))?, None)?;
    Ok((
        transform.clip_matrix(),
        structure.trsf_pers.mode() == Graphic3dTransModeFlags::CAMERA_PERS,
    ))
}

// GlCappingAlgo calls Group::Render directly, bypassing renderGeometry's
// group transform/persistence wrapper.
pub(crate) fn cap_mask_transform(
    camera: &Graphic3dCamera,
    structure: &Graphic3dCStructureFields,
    extent: [u32; 2],
) -> Result<([f32; 16], [f32; 16], bool), RenderError> {
    let mut model = Mat4::<f64>::identity();
    if let Some(datum) = structure.trsf.get() {
        datum.transformation().get_mat4(&mut model);
    }
    let model = Mat4::<f32>::from_other(&model);
    let (clip, clamp) = plane_clip_transform(camera, model, structure, extent)?;
    Ok((clip, model.data(), clamp))
}

pub(crate) fn lighting_matrices(
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<([f32; 16], [f32; 16]), RenderError> {
    let model = model_transform(context.group, context.structure);
    let parent = &context.structure.trsf_pers;
    let local = context.group.transform_persistence();
    if parent.is_null() && local.is_null() {
        return Ok((
            context
                .model_override
                .unwrap_or_else(|| Mat4::<f32>::from_other(&model))
                .data(),
            // Invert the float OrientationMatrixF for occWorldViewMatrixInverse.
            context
                .view_override
                .unwrap_or_else(|| context.camera.orientation_matrix_f())
                .data(),
        ));
    }
    let transform = persistent_view(
        context.camera,
        model,
        parent,
        &local,
        context.persistence_extent(),
    )?;
    Ok((
        context.model_override.unwrap_or(transform.model).data(),
        context.view_override.unwrap_or(transform.world_view).data(),
    ))
}

// Capping replaces ModelWorld with the plane orientation.
pub(crate) fn cap_lighting_matrices(
    camera: &Graphic3dCamera,
    orientation: Mat4<f32>,
    structure: &Graphic3dCStructureFields,
    extent: [u32; 2],
) -> Result<([f32; 16], [f32; 16]), RenderError> {
    let mut transform = PersistentView::new(camera, Mat4::from_other(&orientation));
    if !structure.trsf_pers.is_null() {
        transform.apply(&structure.trsf_pers, viewport_size(Some(extent))?, None)?;
    }
    Ok((transform.model.data(), transform.world_view.data()))
}
pub(crate) fn cap_projection_only(
    camera: &Graphic3dCamera,
    structure: &Graphic3dCStructureFields,
    extent: [u32; 2],
) -> Result<[f32; 16], RenderError> {
    let mut transform = PersistentView::new(camera, Mat4::identity());
    transform.apply(&structure.trsf_pers, viewport_size(Some(extent))?, None)?;
    Ok(transform.projection.data())
}
pub(crate) fn group_projection_only(
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<[f32; 16], RenderError> {
    if context.structure.trsf_pers.is_null() && context.group.transform_persistence().is_null() {
        return Ok(Mat4::<f32>::from_other(&context.camera.projection_matrix()).data());
    }
    let transform = persistent_view(
        context.camera,
        model_transform(context.group, context.structure),
        &context.structure.trsf_pers,
        &context.group.transform_persistence(),
        context.persistence_extent(),
    )?;
    Ok(transform.projection.data())
}

#[cfg(test)]
#[path = "ordinary_clip_tests.rs"]
mod ordinary_clip_tests;
