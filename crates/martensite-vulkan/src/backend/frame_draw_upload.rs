//! Draw upload stages preserve the original fallible allocation and cleanup sequence.
use super::super::surface_image_descriptor::SurfaceImageBinding;
use super::super::{
    geometry_residency::{GeometryKey, ResidentGeometry},
    geometry_transfer::GeometryTransfers,
    pipeline::DrawPipeline,
    pipeline_variants::{LineTintLayout, RasterPipelines},
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
    line_tint: Option<LineTint<'a>>,
    retirements: &'a RetirementStore,
}
type LineTint<'a> = (&'a crate::draw::DrawStream<crate::RenderVertex>, [f32; 4]);
// Fields follow reverse preparation order, so later-stage errors retain the original cleanup.
struct EncodedGeometry {
    indices: Vec<u8>,
    vertices: Vec<u8>,
    vertex_alignment: Option<u64>,
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
type EncodedPatch = (u64, Vec<u8>);

struct GeometryPatch {
    previous: ResidentGeometry,
    vertices: Option<EncodedPatch>,
    indices: Option<EncodedPatch>,
}

struct PreparedBindings {
    lighting: Option<Arc<UploadedStorage>>,
    sphere_map: Option<UploadedStorage>,
    clipping: Option<UploadedStorage>,
    surface_image: Option<Arc<SurfaceImageBinding>>,
    sprite: Option<Arc<UploadedStorage>>,
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
    let draw = reordered.as_ref().unwrap_or(draw);
    let tinted = retained_line_color(runtime, semantic_draw, draw, &wide, window_depth);
    let context = DrawUploadContext {
        runtime,
        semantic_draw,
        draw,
        wide: wide.as_ref(),
        pipelines,
        extent,
        window_depth,
        line_tint: tinted,
        retirements,
    };
    context.upload_prepared(uploads)
}

fn retained_line_color<'a>(
    runtime: &VulkanRuntime,
    semantic: &PrimitiveDraw,
    selected: &'a PrimitiveDraw,
    wide: &Option<crate::wide_line::WideLineReplay>,
    window_depth: Option<super::super::window_depth_parameters::WindowDepthPipelineParameters>,
) -> Option<LineTint<'a>> {
    let source = crate::highlight_style::HighlightLineSource::resolve(selected)?;
    if runtime.projection_behavior != crate::RenderProjectionBehavior::PrecomposedV1
        || !std::ptr::eq(semantic, selected)
        || wide.is_some()
        || window_depth.is_some()
    {
        return None;
    }
    Some(source)
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
        let geometry = match uploads.geometry_batch {
            Some(batch) => self.upload_geometry_batch(batch, &mut uploads.geometry)?,
            None => self.upload_geometry(&mut uploads.geometry, uploads.dynamic_lighting)?,
        };
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
            )?
            .map(Box::new);
        uploaded.user_program = user_program.map(Box::new);
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
    #[cfg(not(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    )))]
    fn upload_geometry_batch(
        &self,
        batch: &Arc<super::GeometryBatch>,
        transfers: &mut GeometryTransfers,
    ) -> Result<PreparedGeometry, RenderError> {
        let resident = batch.resident(self.runtime, &self.pipelines.geometry_staging, transfers)?;
        let pipeline = self
            .pipelines
            .select_batch(self.draw, batch.vertex_count())?;
        Ok(PreparedGeometry {
            pipeline,
            resident,
            count: batch.index_count(),
        })
    }

    #[cfg(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ))]
    fn upload_geometry_batch(
        &self,
        _batch: &Arc<super::GeometryBatch>,
        _transfers: &mut GeometryTransfers,
    ) -> Result<PreparedGeometry, RenderError> {
        Err(rejected(
            "retained geometry batch",
            "individual draws for capture diagnostics",
        ))
    }

    fn upload_geometry(
        &self,
        geometry_transfers: &mut GeometryTransfers,
        dynamic_lighting: bool,
    ) -> Result<PreparedGeometry, RenderError> {
        let runtime = self.runtime;
        let draw = self.draw;
        let pipelines = self.pipelines;
        let tint_pipeline = self.optional_line_tint_pipeline()?;
        let (resident, _encoding) =
            self.prepare_resident_geometry(geometry_transfers, tint_pipeline.is_some())?;
        let count = u32::try_from(draw.indices.len())
            .map_err(|_| rejected(draw.indices.len(), "a 32-bit index count"))?;
        let factorized = super::super::factorized_projection::enabled(
            runtime.projection_behavior,
            self.semantic_draw,
        );
        let pipeline = match tint_pipeline {
            Some(tinted) => tinted,
            None => pipelines.select(draw, factorized, self.window_depth, dynamic_lighting)?,
        };
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

    fn optional_line_tint_pipeline(&self) -> Result<Option<Arc<DrawPipeline>>, RenderError> {
        if self.line_tint.is_none() {
            return Ok(None);
        }
        let layout = if crate::highlight_style::HighlightLineSource::compact(self.draw) {
            LineTintLayout::Compact
        } else {
            LineTintLayout::Full
        };
        match self.pipelines.select_draw_color_line(self.draw, layout) {
            Ok(pipeline) => Ok(Some(pipeline)),
            Err(error) if super::super::device_lifecycle::contains_device_loss(&error) => {
                Err(error)
            }
            // The semantic draw still has its validated, recolored vertices.
            Err(_) => Ok(None),
        }
    }

    fn prepare_resident_geometry(
        &self,
        transfers: &mut GeometryTransfers,
        use_line_tint: bool,
    ) -> Result<(ResidentGeometry, Option<EncodedGeometry>), RenderError> {
        let mut key = geometry_key(self.runtime, self.draw);
        let line_source = self
            .line_tint
            .filter(|_| use_line_tint)
            .map(|(source, _)| source);
        if let Some(source) = line_source {
            let compact = crate::highlight_style::HighlightLineSource::compact(self.draw);
            key = key.and_then(|key| {
                let identity = source.identity()?;
                if compact {
                    key.with_compact_line(identity)
                } else {
                    key.with_unlit_vertices(identity)
                }
            });
            if let Some(resident) = key.and_then(|key| transfers.residency.find(key)) {
                return Ok((resident, None));
            }
            let draw = self
                .draw
                .with_replacement_geometry(source.clone(), self.draw.indices.clone());
            return self.encode_resident_geometry(transfers, &draw, key, compact);
        }
        if let Some(resident) = key.and_then(|key| transfers.residency.find(key)) {
            return Ok((resident, None));
        }
        if let Some(key) = key
            && let Some(resident) = self.patch_resident_geometry(transfers, key)?
        {
            transfers.residency.queue(key, self.draw, resident.clone());
            return Ok((resident, None));
        }
        self.encode_resident_geometry(transfers, self.draw, key, false)
    }
    fn encode_resident_geometry(
        &self,
        transfers: &mut GeometryTransfers,
        draw: &PrimitiveDraw,
        key: Option<GeometryKey>,
        compact_line: bool,
    ) -> Result<(ResidentGeometry, Option<EncodedGeometry>), RenderError> {
        let encoding = if compact_line {
            geometry_encoding::compact_line_geometry(draw)
        } else {
            geometry_encoding::encoded_geometry(self.runtime, draw)
        };
        let resident = encoding.queue_transfers(
            self.runtime,
            &self.pipelines.geometry_staging,
            transfers,
            key.is_some() && !cfg!(feature = "lit-only-command-diagnostics"),
        )?;
        if let Some(key) = key {
            transfers.residency.queue(key, draw, resident.clone());
        }
        Ok((resident, Some(encoding)))
    }

    fn patch_resident_geometry(
        &self,
        transfers: &mut GeometryTransfers,
        key: GeometryKey,
    ) -> Result<Option<ResidentGeometry>, RenderError> {
        let Some(patch) = self.geometry_patch(transfers, key) else {
            return Ok(None);
        };
        let pool = &self.pipelines.geometry_staging;
        let mut previous = patch.previous;
        if let Some((offset, bytes)) = patch.vertices {
            previous.vertices = transfers.patch_stream(
                self.runtime,
                pool,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                previous.vertices,
                offset,
                &bytes,
            )?;
        }
        if let Some((offset, bytes)) = patch.indices {
            previous.indices = transfers.patch_stream(
                self.runtime,
                pool,
                vk::BufferUsageFlags::INDEX_BUFFER,
                previous.indices,
                offset,
                &bytes,
            )?;
        }
        Ok(Some(previous))
    }

    fn geometry_patch(
        &self,
        transfers: &GeometryTransfers,
        mut key: GeometryKey,
    ) -> Option<GeometryPatch> {
        let (vertices, indices) = self.draw.combined_revision().map_or_else(
            || (self.draw.vertex_revision(), self.draw.index_revision()),
            |(vertices, indices)| (Some(vertices), Some(indices)),
        );
        if vertices.is_none() && indices.is_none() {
            return None;
        }
        if let Some(revision) = vertices {
            key = key.with_unlit_vertices(revision.previous_identity())?;
        }
        if let Some(revision) = indices {
            key = key.previous_indices(revision.previous_identity())?;
        }
        let previous = transfers.residency.find_earlier(key)?;
        // Qualify both ranges before staging either, so a rejected pair can use the full upload.
        let vertices = match vertices {
            Some(revision) => Some(encode_vertex_patch(
                self.draw,
                revision.dirty_range(),
                previous.vertices.size(),
            )?),
            None => None,
        };
        let indices = match indices {
            Some(revision) => Some(geometry_encoding::encode_index_patch(
                self.draw,
                revision.dirty_range(),
                previous.indices.size(),
            )?),
            None => None,
        };
        Some(GeometryPatch {
            previous,
            vertices,
            indices,
        })
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
