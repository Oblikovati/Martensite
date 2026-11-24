//! The frame of an immediate redraw that keeps its main pass (immediate_elision.rs): empty
//! layers in place of the main layers, then the immediate layers and the statistics overlay,
//! built and culled as `frame_for_view` builds them.
use super::{
    ViewFrameExtras, ViewLayerInputs, apply_view_culling, cull_layers, ordered_layers,
    scene_extent, source_layer_passes, validate_view, view_clear_color,
};
use crate::{RenderError, RenderFrame, RenderLayer, immediate_elision::MainElision};
use martensite_tkernel::standard::HandleReference;
use martensite_tkservice::graphic3d::*;

/// The immediate-only frame of `view` over the main pass that `elision` proves;
/// `immediate_frame_for_view(view, extent, clipping, extras, &elision)?`.
pub(crate) fn immediate_frame_for_view(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    clipping: &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>,
    extras: ViewFrameExtras<'_>,
    elision: &MainElision,
) -> Result<RenderFrame, RenderError> {
    validate_view(view)?;
    let (mut passes, rendering) = immediate_layer_passes(view, extent, (clipping, extras))?;
    let extent = scene_extent(view, extent)?;
    apply_view_culling(&mut passes, view.backfacing_model());
    crate::retained_lighting::apply_view_lights(&mut passes, view)?;
    let mut layers = elision.shell_layers();
    layers.append(&mut passes);
    if let (Some(stats), Some(overlay)) = (extras.stats, extras.overlay) {
        layers.extend(overlay.layer(view, stats, extent)?);
    }
    let clear = view_clear_color(view);
    let mut frame = RenderFrame::with_layers_and_grid(
        extent[0],
        extent[1],
        clear,
        layers,
        None,
        super::retained_mesh_lod(extras.mesh_lod),
    )?;
    frame.immediate_start = Some(elision.start());
    frame.raster = crate::retained_transparency::view_raster_settings(view, &rendering)?;
    // GlView::redraw clears alpha to 1 with buffersOpaqueAlpha, else to 0.
    frame.clear_color[3] = if frame.raster.alpha_writes { 0. } else { 1. };
    crate::compatibility_text_color::capture_text_color_target(&mut frame);
    Ok(frame)
}

/// The culled immediate layers of the view, each GlLayerList::Render pass loop expanded for
/// the depth prepass, and the frame's rendering parameters.
fn immediate_layer_passes(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    (clipping, extras): (
        &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>,
        ViewFrameExtras<'_>,
    ),
) -> Result<(Vec<RenderLayer>, Graphic3dRenderingParams), RenderError> {
    let mut layers = ordered_layers(view);
    layers.retain(|layer| layer.is_immediate());
    cull_layers(view, &layers, (extent, extras.stats), None)?;
    let camera = crate::view_render_camera::owned_render_camera(view);
    let limits = (clipping, extras.limits);
    let layer_inputs = ViewLayerInputs::new(view, extent, &camera, limits, extras.frame_steps)?;
    let passes = source_layer_passes(view, &layers, &layer_inputs, true)?;
    let count = passes.len();
    let (passes, _) = crate::depth_prepass::expand_view_layers(
        passes,
        count,
        layer_inputs.rendering.to_enable_depth_prepass,
    );
    Ok((passes, layer_inputs.rendering))
}

#[cfg(test)]
#[path = "view_scene_immediate_tests.rs"]
mod tests;
