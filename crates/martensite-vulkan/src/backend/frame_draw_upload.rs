//! Draw upload stages preserve the original fallible allocation and cleanup sequence.
use super::super::surface_image_descriptor::SurfaceImageBinding;
use super::super::{
    geometry_residency::{GeometryKey, ResidentGeometry},
    geometry_transfer::GeometryTransfers,
    pipeline::DrawPipeline,
    pipeline_variants::RasterPipelines,
    runtime::VulkanRuntime,
    transform_constants::encode_transform,
};
use super::super::{retained_submission::RetirementStore, storage_descriptor::UploadedStorage};
use super::{
    DrawBindings, DrawLine, DrawPresentation, FrameDrawUploads, UploadedDraw, encode_vertex_patch,
    encode_vertex_sources, encode_vertices, geometry_key,
};
use crate::{PrimitiveDraw, RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;
struct DrawUploadContext<'a> {
    runtime: &'a Arc<VulkanRuntime>,
    draw: &'a PrimitiveDraw,
    semantic_draw: &'a PrimitiveDraw,
    wide: Option<&'a crate::wide_line::WideLineReplay>,
    pipelines: &'a RasterPipelines,
    extent: vk::Extent2D,
    window_depth: Option<super::super::window_depth_parameters::WindowDepthPipelineParameters>,
    retirements: &'a RetirementStore,
}
// Fields follow reverse preparation order, so later-stage errors retain the original cleanup.
struct EncodedGeometry {
    indices: Vec<u8>,
    vertices: Vec<u8>,
    #[cfg(feature = "command-state-diagnostics")]
    source_count: usize,
}
struct PreparedGeometry {
    pipeline: Arc<DrawPipeline>,
    resident: ResidentGeometry,
    count: u32,
    #[cfg(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ))]
    encoding: EncodedGeometry,
}
struct PreparedBindings {
    lighting: Option<UploadedStorage>,
    sphere_map: Option<UploadedStorage>,
    clipping: Option<UploadedStorage>,
    surface_image: Option<Arc<SurfaceImageBinding>>,
    sprite: Option<UploadedStorage>,
    pbr_environment: Option<Arc<super::super::pbr_environment::PbrEnvironment>>,
    material_maps: Option<Arc<super::super::material_map_descriptor::MaterialMapBinding>>,
    shadow_maps: Option<Arc<super::super::shadow_maps::ShadowMapSet>>,
}
/// Prepare one ordered draw and queue its vertex and index uploads with the frame.
pub(super) fn upload_draw(
    runtime: &Arc<VulkanRuntime>,
    draw: &PrimitiveDraw,
    pipelines: &RasterPipelines,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
    retirements: &RetirementStore,
    uploads: &mut FrameDrawUploads<'_>,
) -> Result<UploadedDraw, RenderError> {
    super::super::factorized_projection::admit(runtime.projection_behavior, draw)?;
    let window_depth = super::super::window_depth_upload::prepare(runtime, draw, extent, samples)?;
    super::super::factorized_projection::admit_composition(
        runtime.projection_behavior,
        draw,
        runtime.world_clipping,
        runtime.phong_interpolation,
        runtime.triangle_coverage,
    )?;
    let grid12_world_clip = runtime.admit_world_clip_interpolation(draw)?;
    let unclipped_grid12_v3 =
        super::super::world_clip_interpolation_upload::permits_unclipped_grid12_v3_coverage(
            draw,
            runtime.world_clip_interpolation,
            runtime.world_clipping,
            runtime.triangle_coverage,
            runtime.phong_interpolation,
        );
    let triangle_grid = super::super::triangle_grid_coverage_upload::prepare(
        runtime.triangle_coverage,
        runtime.phong_interpolation,
        draw,
        super::super::phong_interpolation_upload::viewport(runtime.attachment_coordinates, extent),
        samples,
        runtime.limits.sub_pixel_precision_bits,
        grid12_world_clip,
        unclipped_grid12_v3,
    )?;
    let selected_geometry = triangle_grid.as_ref().unwrap_or(draw);
    super::super::phong_interpolation_upload::admit_for_coverage(
        runtime.phong_interpolation,
        selected_geometry,
        super::super::phong_interpolation_upload::viewport(runtime.attachment_coordinates, extent),
        grid12_world_clip,
        matches!(
            runtime.triangle_coverage,
            crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
        ),
    )?;
    super::super::text_grid_interpolation_upload::admit(
        runtime.text_interpolation,
        draw,
        super::super::phong_interpolation_upload::viewport(runtime.attachment_coordinates, extent),
    )?;
    runtime.admit_world_clipping(draw)?;
    let semantic_draw = draw;
    let draw = selected_geometry;
    let restored = super::super::line_source_upload::restored_source(runtime, draw);
    let draw = restored.as_ref().unwrap_or(draw);
    let factorized_line =
        super::super::factorized_line_upload::lowered(runtime.projection_behavior, draw);
    let draw = factorized_line.as_ref().unwrap_or(draw);
    let clip_space = runtime.effective_world_clipping(draw)
        == crate::RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5;
    let fixed_function = super::super::factorized_line_upload::fixed_function(
        runtime.projection_behavior,
        clip_space,
        draw,
    );
    let draw = fixed_function.as_ref().unwrap_or(draw);
    let selected =
        super::super::thin_axis_upload::private_geometry(runtime, draw, extent, samples)?;
    let draw = selected.as_ref().unwrap_or(draw);
    let wide = wide_line_replay(runtime, draw, extent);
    // Emulated wide lines upload their segments reordered by major axis.
    let reordered = wide.as_ref().map(|replay| {
        draw.with_replacement_geometry(
            replay.vertices.clone().into(),
            replay.indices.clone().into(),
        )
    });
    let context = DrawUploadContext {
        runtime,
        semantic_draw,
        draw: reordered.as_ref().unwrap_or(draw),
        wide: wide.as_ref(),
        pipelines,
        extent,
        window_depth,
        retirements,
    };
    context.upload_prepared(uploads)
}
impl<'a> DrawUploadContext<'a> {
    fn upload_prepared(
        &self,
        uploads: &mut FrameDrawUploads<'_>,
    ) -> Result<UploadedDraw, RenderError> {
        super::super::window_depth_upload::require_physical_source(
            self.window_depth,
            self.semantic_draw,
            self.draw,
        )?;
        self.validate_styles()?;
        let geometry = self.upload_geometry(&mut uploads.geometry)?;
        super::super::factorized_projection::admit_pipeline(
            self.runtime.projection_behavior,
            self.semantic_draw,
            geometry.pipeline.kind,
        )?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.runtime.vertex_output {
            if self.draw.vertex_output_target == request.source_token {
                request.require_actual_indices(&geometry.encoding.indices)?;
            }
        }
        let bindings = self.upload_bindings(&geometry.pipeline, &mut uploads.storage)?;
        let user_program = self.upload_user_program(&geometry.pipeline)?;
        let mut uploaded = self.assemble(geometry, bindings);
        uploaded.window_depth_status =
            super::super::window_depth_status::WindowDepthStatus::prepare(
                self.runtime,
                self.semantic_draw,
                &uploaded.pipeline,
            )?;
        uploaded.user_program = user_program;
        Ok(uploaded)
    }

    fn validate_styles(&self) -> Result<(), RenderError> {
        self.validate_line_clipping_stage()?;
        self.validate_line()?;
        self.validate_point()?;
        Ok(())
    }
    fn validate_line_clipping_stage(&self) -> Result<(), RenderError> {
        let stages = self.semantic_draw.user_program.iter().flat_map(|program| {
            program
                .modules
                .stages
                .iter()
                .map(|(reflection, _)| reflection.stage)
        });
        if crate::backend::line_clip_stage::line_stage_requires_admission(
            self.semantic_draw.topology(),
            stages,
        ) {
            self.runtime
                .admit_line_clipping(self.semantic_draw.user_program.is_some())?;
        }
        Ok(())
    }
    fn validate_line(&self) -> Result<(), RenderError> {
        let Self {
            runtime,
            semantic_draw,
            ..
        } = self;
        if let Some(mut line) = semantic_draw
            .line_style
            .or(semantic_draw.polygon_edge_style)
        {
            if self.wide.is_some() {
                line.width = 1.;
            }
            runtime
                .lines
                .validate(line, runtime.limits.line_width_range)?;
        }
        Ok(())
    }
    fn validate_point(&self) -> Result<(), RenderError> {
        super::upload_point::validate_point(
            self.runtime,
            self.semantic_draw,
            self.draw,
            self.extent,
        )
    }
    fn upload_geometry(
        &self,
        geometry_transfers: &mut GeometryTransfers,
    ) -> Result<PreparedGeometry, RenderError> {
        let runtime = self.runtime;
        let draw = self.draw;
        let pipelines = self.pipelines;
        let (resident, _encoding) = self.prepare_resident_geometry(geometry_transfers)?;
        let count = u32::try_from(draw.indices.len())
            .map_err(|_| rejected(draw.indices.len(), "a 32-bit index count"))?;
        let factorized = super::super::factorized_projection::enabled(
            runtime.projection_behavior,
            self.semantic_draw,
        );
        let pipeline = pipelines.select(draw, factorized, self.window_depth)?;
        super::super::window_depth_upload::require_pipeline(self.window_depth, pipeline.kind)?;
        super::super::phong_interpolation_upload::require_interface(
            runtime.phong_interpolation,
            self.semantic_draw,
            pipeline.phong_interpolation_shader,
        )?;
        super::super::text_grid_interpolation_upload::require_interface(
            runtime.text_interpolation,
            self.semantic_draw,
            pipeline.text_grid_interpolation_shader,
        )?;
        Ok(PreparedGeometry {
            pipeline,
            resident,
            count: if draw.fixed_grid_point { 6 } else { count },
            #[cfg(any(
                feature = "command-state-diagnostics",
                feature = "vertex-output-diagnostics"
            ))]
            encoding: _encoding
                .ok_or_else(|| rejected("resident geometry", "encoded diagnostic bytes"))?,
        })
    }
    fn prepare_resident_geometry(
        &self,
        transfers: &mut GeometryTransfers,
    ) -> Result<(ResidentGeometry, Option<EncodedGeometry>), RenderError> {
        let key = geometry_key(self.runtime, self.draw);
        if let Some(resident) = key.and_then(|key| transfers.residency.find(key)) {
            return Ok((resident, None));
        }
        if let Some(key) = key
            && let Some(resident) = self.patch_resident_geometry(transfers, key)?
        {
            transfers.residency.queue(key, self.draw, resident.clone());
            return Ok((resident, None));
        }
        let encoding = geometry_encoding::encoded_geometry(self.runtime, self.draw);
        let resident =
            encoding.queue_transfers(self.runtime, &self.pipelines.geometry_staging, transfers)?;
        if let Some(key) = key {
            transfers.residency.queue(key, self.draw, resident.clone());
        }
        Ok((resident, Some(encoding)))
    }

    fn patch_resident_geometry(
        &self,
        transfers: &mut GeometryTransfers,
        key: GeometryKey,
    ) -> Result<Option<ResidentGeometry>, RenderError> {
        if let Some(update) = self.patch_resident_vertices(transfers, key)? {
            return Ok(Some(update));
        }
        self.patch_resident_indices(transfers, key)
    }

    fn patch_resident_vertices(
        &self,
        transfers: &mut GeometryTransfers,
        key: GeometryKey,
    ) -> Result<Option<ResidentGeometry>, RenderError> {
        let Some(revision) = self.draw.vertex_revision() else {
            return Ok(None);
        };
        let previous = key
            .previous_vertices(revision.previous_identity())
            .and_then(|key| transfers.residency.find_completed(key));
        let Some(previous) = previous else {
            return Ok(None);
        };
        let Some((offset, bytes)) =
            encode_vertex_patch(self.draw, revision.dirty_range(), previous.vertices.size())
        else {
            return Ok(None);
        };
        let vertices = transfers.patch_stream(
            self.runtime,
            &self.pipelines.geometry_staging,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            previous.vertices,
            offset,
            &bytes,
        )?;
        Ok(Some(ResidentGeometry {
            vertices,
            indices: previous.indices,
        }))
    }

    fn patch_resident_indices(
        &self,
        transfers: &mut GeometryTransfers,
        key: GeometryKey,
    ) -> Result<Option<ResidentGeometry>, RenderError> {
        let Some(revision) = self.draw.index_revision() else {
            return Ok(None);
        };
        let Some(previous) = key
            .previous_indices(revision.previous_identity())
            .and_then(|key| transfers.residency.find_completed(key))
        else {
            return Ok(None);
        };
        let Some((offset, bytes)) = geometry_encoding::encode_index_patch(
            self.draw,
            revision.dirty_range(),
            previous.indices.size(),
        ) else {
            return Ok(None);
        };
        let indices = transfers.patch_stream(
            self.runtime,
            &self.pipelines.geometry_staging,
            vk::BufferUsageFlags::INDEX_BUFFER,
            previous.indices,
            offset,
            &bytes,
        )?;
        Ok(Some(ResidentGeometry {
            vertices: previous.vertices,
            indices,
        }))
    }
    fn sample_cutoff(&self) -> bool {
        let draw = self.draw;
        draw.native_material_cutoff
            .is_some_and(|cutoff| cutoff <= 1.)
            || draw.surface_cutoff.is_some()
    }
    fn presentation(&self) -> DrawPresentation {
        super::super::frame_draw_presentation::from_semantic_draw(self.semantic_draw)
    }
    fn line_state(&self) -> DrawLine {
        let draw = self.draw;
        DrawLine {
            stipple: crate::fixed_line_stipple::fixed_line_stipple(draw),
            width: draw.line_style.or(draw.polygon_edge_style).map(|style| {
                if self.wide.is_some() {
                    1.
                } else {
                    style.raster_width()
                }
            }),
            wide: self.wide.map(|replay| super::WideLineDraw {
                x_major: replay.x_major as u32,
                offsets: replay.offsets().collect(),
            }),
        }
    }
}

/// Plan the GL wide-line replay for independent segments wider than the device range; strips,
/// polygon edges and devices without Bresenham lines keep the ordinary validation. Widths inside
/// the range stay device lines: moving replica endpoints re-rounds their depth, so a highlight
/// drawn over the same edge (bug26721 on RADV) would lose its LEQUAL tie.
fn wide_line_replay(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    extent: vk::Extent2D,
) -> Option<crate::wide_line::WideLineReplay> {
    let style = draw
        .line_style
        .filter(|_| draw.polygon_edge_style.is_none())?;
    let strip = draw.fixed_function_lighting && draw.source_line_strip;
    let beyond = style.raster_width() > runtime.limits.line_width_range[1];
    (beyond && !strip && runtime.lines.bresenham).then(|| {
        crate::wide_line::WideLineReplay::new(draw, [extent.width, extent.height], style.width)
    })
}

#[path = "frame_upload_geometry.rs"]
mod geometry_encoding;

#[path = "frame_upload_assembly.rs"]
mod draw_assembly;

#[path = "frame_upload_bindings.rs"]
mod bindings;

#[path = "frame_upload_bindings_result.rs"]
mod bindings_result;
