//! Kept uploads of plain surfaces (#594, retained render list step 2). A draw body that
//! uploaded with the same inputs before replays that upload: it skips the admission checks,
//! the pipeline selection, the geometry lookup and the binding preparation, and binds only
//! its lighting block and push constants, which follow the camera.
use super::{DrawBindings, DrawUploadContext, FrameDrawUploads, UploadedDraw};
use crate::backend::pipeline_memo::SelectionInputs;
use crate::backend::pipeline_variants::RasterPipelines;
use crate::backend::retained_submission::RetirementStore;
use crate::backend::runtime::VulkanRuntime;
use crate::backend::upload_memo::{KeptUpload, UploadInputs};
use crate::{PrimitiveDraw, RenderError};
use ash::vk;
use std::sync::Arc;

/// The extent, sample count and retirement store a frame uploads with.
type FrameUpload<'a> = (vk::Extent2D, vk::SampleCountFlags, &'a RetirementStore);

/// The body's kept upload with this frame's lighting block and push constants, or `None`
/// when it has none for these inputs; `retained::replayed(runtime, draw, pipelines, frame, uploads)?`.
pub(super) fn replayed(
    runtime: &Arc<VulkanRuntime>,
    draw: &PrimitiveDraw,
    pipelines: &RasterPipelines,
    (extent, samples, retirements): FrameUpload<'_>,
    uploads: &mut FrameDrawUploads<'_>,
) -> Result<Option<UploadedDraw>, RenderError> {
    if uploads.geometry_batch.is_some() || !plain_body(draw) || !plain_runtime(runtime) {
        return Ok(None);
    }
    let context = DrawUploadContext {
        runtime,
        semantic_draw: draw,
        draw,
        wide: None,
        pipelines,
        extent,
        window_depth: None,
        line_tint: None,
        retirements,
    };
    let inputs = upload_inputs(&context, samples, uploads.dynamic_lighting);
    let Some(kept) = draw.upload_memo.get(&inputs) else {
        return Ok(None);
    };
    let (Some(vertices), Some(indices)) = (kept.vertices.upgrade(), kept.indices.upgrade()) else {
        return Ok(None);
    };
    let bindings = context.view_bindings(&kept.pipeline, &mut uploads.storage)?;
    Ok(Some(context.kept_draw(kept, (vertices, indices), bindings)))
}

/// Keep a fresh upload when it is a plain surface's; `retained::keep(&context, samples, uploads, &uploaded)`.
pub(super) fn keep(
    context: &DrawUploadContext<'_>,
    samples: vk::SampleCountFlags,
    uploads: &FrameDrawUploads<'_>,
    uploaded: &UploadedDraw,
) {
    if uploads.geometry_batch.is_some()
        || !plain_body(context.semantic_draw)
        || !plain_upload(context, uploaded)
    {
        return;
    }
    let inputs = upload_inputs(context, samples, uploads.dynamic_lighting);
    let kept = KeptUpload {
        pipeline: Arc::clone(&uploaded.pipeline),
        vertices: Arc::downgrade(&uploaded.vertices),
        indices: Arc::downgrade(&uploaded.indices),
        count: uploaded.count,
        sample_alpha_cutoff: uploaded.sample_alpha_cutoff,
    };
    context.semantic_draw.upload_memo.keep(inputs, kept);
}

/// Whether the draw's own fields leave it a plain surface: a glyph, sprite, line, texture,
/// clipped or user-program draw is told apart before any upload input is computed.
fn plain_body(draw: &PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && draw.textured_style.is_none()
        && draw.surface_texture.is_none()
        && draw.point_style.is_none()
        && draw.line_style.is_none()
        && draw.polygon_edge_style.is_none()
        && draw.clipping.is_none()
        && draw.user_program.is_none()
}

/// Whether the runtime uploads surfaces without emulation, so that the admission checks of a
/// kept upload hold on every frame.
fn plain_runtime(runtime: &VulkanRuntime) -> bool {
    crate::backend::directional_phong::supported_behavior(runtime)
}

/// Whether an upload replaced no geometry and bound nothing but lighting, PBR environment and
/// shadow maps.
fn plain_upload(context: &DrawUploadContext<'_>, uploaded: &UploadedDraw) -> bool {
    let bindings = &uploaded.bindings;
    plain_runtime(context.runtime)
        && std::ptr::eq(context.semantic_draw, context.draw)
        && context.wide.is_none()
        && context.window_depth.is_none()
        && context.line_tint.is_none()
        && context.draw.user_program.is_none()
        && uploaded.user_program.is_none()
        && uploaded.window_depth_status.is_none()
        && !uploaded.window_depth_expected
        && uploaded.point_viewports.is_none()
        && uploaded.instances == 1
        && (uploaded.line.width, uploaded.line.stipple) == (None, None)
        && uploaded.line.wide.is_none()
        && bindings.clipping.is_none()
        && bindings.sprite.is_none()
        && bindings.surface_image.is_none()
        && bindings.material_maps.is_none()
        && bindings.sphere_map.is_none()
}

fn upload_inputs(
    context: &DrawUploadContext<'_>,
    samples: vk::SampleCountFlags,
    dynamic_lighting: bool,
) -> UploadInputs {
    let factorized = crate::backend::factorized_projection::enabled(
        context.runtime.projection_behavior,
        context.draw,
    );
    let frame = (factorized, None, dynamic_lighting);
    UploadInputs {
        runtime: Arc::as_ptr(context.runtime).addr(),
        selection: SelectionInputs::of(context.pipelines.selection_serial(), context.draw, frame),
        extent: context.extent,
        samples,
    }
}

impl DrawUploadContext<'_> {
    /// The bindings that follow the view, as the fresh upload binds them: lighting, PBR
    /// environment and shadow maps.
    fn view_bindings(
        &self,
        pipeline: &crate::backend::pipeline::DrawPipeline,
        storage: &mut crate::backend::host_upload_pages::FrameUploadPages<'_>,
    ) -> Result<DrawBindings, RenderError> {
        let lighting = self.upload_lighting(pipeline, storage)?;
        let pbr_environment = self.frame_pbr_environment(pipeline, storage)?;
        let shadow_maps = self.frame_shadow_maps(pipeline, storage)?;
        Ok(DrawBindings {
            clipping: None,
            lighting,
            sprite: None,
            surface_image: None,
            pbr_environment,
            material_maps: None,
            sphere_map: None,
            shadow_maps,
        })
    }

    /// The kept upload with this frame's push constants and view bindings.
    fn kept_draw(
        &self,
        kept: KeptUpload,
        (vertices, indices): (
            Arc<crate::backend::geometry_buffer::GeometryBuffer>,
            Arc<crate::backend::geometry_buffer::GeometryBuffer>,
        ),
        bindings: DrawBindings,
    ) -> UploadedDraw {
        UploadedDraw {
            window_depth_expected: false,
            window_depth_status: None,
            native_alpha: None,
            oit_alpha: vec![],
            sample_alpha_cutoff: kept.sample_alpha_cutoff,
            oit: None,
            bindings,
            presentation: self.presentation(),
            transform: self.push_constants(&kept.pipeline),
            pipeline: kept.pipeline,
            vertices,
            indices,
            count: kept.count,
            instances: 1,
            line: self.line_state(),
            point_viewports: None,
            user_program: None,
        }
    }
}

/// Require a kept upload to equal the fresh one, under the `draw-record-verification` feature.
#[cfg(feature = "draw-record-verification")]
pub(super) fn verify(kept: &UploadedDraw, fresh: &UploadedDraw) {
    let same_bindings = |a: &DrawBindings, b: &DrawBindings| {
        a.lighting.is_some() == b.lighting.is_some()
            && a.pbr_environment.as_ref().map(Arc::as_ptr)
                == b.pbr_environment.as_ref().map(Arc::as_ptr)
            && a.shadow_maps.as_ref().map(Arc::as_ptr) == b.shadow_maps.as_ref().map(Arc::as_ptr)
            && b.clipping.is_none()
            && b.sprite.is_none()
            && b.surface_image.is_none()
            && b.material_maps.is_none()
            && b.sphere_map.is_none()
    };
    assert!(
        Arc::ptr_eq(&kept.pipeline, &fresh.pipeline)
            && Arc::ptr_eq(&kept.vertices, &fresh.vertices)
            && Arc::ptr_eq(&kept.indices, &fresh.indices)
            && (kept.count, kept.instances) == (fresh.count, fresh.instances)
            && kept.transform == fresh.transform
            && kept.sample_alpha_cutoff == fresh.sample_alpha_cutoff
            && kept.presentation == fresh.presentation
            && fresh.user_program.is_none()
            && fresh.window_depth_status.is_none()
            && same_bindings(&kept.bindings, &fresh.bindings),
        "a kept upload differs from uploading the draw again"
    );
}

#[cfg(test)]
#[path = "frame_retained_upload_tests.rs"]
mod tests;
