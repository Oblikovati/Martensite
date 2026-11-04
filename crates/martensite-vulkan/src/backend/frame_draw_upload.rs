//! Draw upload stages preserve the original fallible allocation and cleanup sequence.
use super::super::surface_image_descriptor::SurfaceImageBinding;
use super::super::{
    buffer::DeviceBuffer, pipeline::DrawPipeline, pipeline_variants::RasterPipelines,
    runtime::VulkanRuntime, transform_constants::encode_transform,
};
use super::super::{retained_submission::RetirementStore, storage_descriptor::UploadedStorage};
use super::{
    DrawBindings, DrawLine, DrawPresentation, UploadedDraw, encode_vertex_sources, upload_bytes,
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
struct EncodedGeometry<'a> {
    indices: Vec<u8>,
    _expanded: Option<Vec<u32>>,
    vertices: Vec<u8>,
    _sources: Vec<(usize, usize)>,
    _original_indices: std::borrow::Cow<'a, [u32]>,
}
struct PreparedGeometry<'a> {
    pipeline: Arc<DrawPipeline>,
    indices: DeviceBuffer,
    vertices: DeviceBuffer,
    count: u32,
    _encoding: EncodedGeometry<'a>,
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
/// Upload one ordered draw; `upload_draw(runtime, draw, pipelines, extent, retirements)` preserves failed owners.
pub(super) fn upload_draw(
    runtime: &Arc<VulkanRuntime>,
    draw: &PrimitiveDraw,
    pipelines: &RasterPipelines,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
    retirements: &RetirementStore,
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
        draw.with_replacement_geometry(replay.vertices.clone(), replay.indices.clone())
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
    context.upload_prepared()
}
impl<'a> DrawUploadContext<'a> {
    fn upload_prepared(&self) -> Result<UploadedDraw, RenderError> {
        super::super::window_depth_upload::require_physical_source(
            self.window_depth,
            self.semantic_draw,
            self.draw,
        )?;
        self.validate_styles()?;
        let geometry = self.upload_geometry()?;
        super::super::factorized_projection::admit_pipeline(
            self.runtime.projection_behavior,
            self.semantic_draw,
            geometry.pipeline.kind,
        )?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.runtime.vertex_output {
            if self.draw.vertex_output_target == request.source_token {
                request.require_actual_indices(&geometry._encoding.indices)?;
            }
        }
        let bindings = self.upload_bindings(&geometry.pipeline)?;
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
    fn encoded_geometry(&self) -> EncodedGeometry<'a> {
        geometry_encoding::encoded_geometry(self.runtime, self.draw)
    }

    fn upload_geometry(&self) -> Result<PreparedGeometry<'a>, RenderError> {
        let runtime = self.runtime;
        let draw = self.draw;
        let pipelines = self.pipelines;
        let encoding = self.encoded_geometry();
        let vertex_buffer = encoding.upload_vertices(runtime)?;
        let index_buffer = encoding.upload_indices(runtime)?;
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
            indices: index_buffer,
            vertices: vertex_buffer,
            count: if draw.fixed_grid_point { 6 } else { count },
            _encoding: encoding,
        })
    }
    fn upload_bindings(&self, pipeline: &DrawPipeline) -> Result<PreparedBindings, RenderError> {
        let sprite = self.upload_sprite(pipeline)?;
        self.prepare_surface_cache()?;
        let surface_image = self.upload_surface(pipeline)?;
        let clipping = self.upload_clipping(pipeline)?;
        let lighting = self.upload_lighting(pipeline)?;
        let sphere_map = self.upload_sphere_map(pipeline)?;
        let pbr_environment = pipeline
            .kind
            .pbr
            .then(|| self.pipelines.pbr_environment(self.runtime))
            .transpose()?;
        let material_maps = self.upload_material_maps(pipeline)?;
        let shadow_maps = pipeline
            .shadow_layout
            .is_some()
            .then(|| self.pipelines.shadow_maps(self.runtime))
            .transpose()?;
        Ok(PreparedBindings {
            shadow_maps,
            lighting,
            sphere_map,
            clipping,
            surface_image,
            sprite,
            pbr_environment,
            material_maps,
        })
    }
    fn upload_sprite(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        let draw = self.draw;
        if pipeline.sprite_layout.is_none() {
            return Ok(None);
        }
        let image = draw.point_style.as_ref().map(|s| &s.image);
        image
            .map(|image| self.selected_sprite(pipeline, image))
            .transpose()
    }
    fn selected_sprite(
        &self,
        pipeline: &DrawPipeline,
        image: &crate::RasterImage,
    ) -> Result<UploadedStorage, RenderError> {
        let layout = pipeline
            .sprite_layout
            .as_ref()
            .expect("sprite pipeline layout");
        super::super::point_sprite::upload_sprite(
            self.runtime,
            layout,
            image,
            self.draw.point_style.is_some(),
        )
    }

    fn prepare_surface_cache(&self) -> Result<(), RenderError> {
        let draw = self.draw;
        if let Some(unavailable) = &draw.surface_texture_unavailable {
            self.pipelines
                .surface_images
                .forget(&unavailable.resource_id)?;
        }
        if draw.surface_texture_binding_only
            && let Some(style) = &draw.surface_texture
        {
            self.touch_surface(style)?;
        }
        Ok(())
    }
    fn touch_surface(
        &self,
        style: &crate::surface_texture::SurfaceTextureStyle,
    ) -> Result<(), RenderError> {
        self.pipelines.surface_images.touch(
            self.runtime,
            &style.image,
            crate::fixed_surface_environment::uses_fixed_surface_environment(self.draw),
            self.retirements,
        )
    }
    fn upload_surface(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Arc<SurfaceImageBinding>>, RenderError> {
        if let Some(style) = &self.draw.textured_style {
            return self.glyph_atlas(pipeline, &style.image).map(Some);
        }
        self.draw
            .surface_texture
            .as_ref()
            .filter(|_| !self.draw.surface_texture_binding_only)
            .map(|style| self.selected_surface(pipeline, style))
            .transpose()
    }
    fn glyph_atlas(
        &self,
        pipeline: &DrawPipeline,
        atlas: &crate::RasterImage,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let layout = pipeline
            .surface_layout
            .as_ref()
            .expect("glyph atlas image layout");
        super::super::glyph_atlas_image::upload_glyph_atlas(
            self.runtime,
            layout,
            atlas,
            self.retirements,
        )
    }

    fn selected_surface(
        &self,
        pipeline: &DrawPipeline,
        style: &crate::surface_texture::SurfaceTextureStyle,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let layout = pipeline
            .surface_layout
            .as_ref()
            .expect("surface image layout");
        self.pipelines.surface_images.binding(
            self.runtime,
            layout,
            style,
            crate::fixed_surface_environment::uses_fixed_surface_environment(self.draw),
            self.retirements,
        )
    }
    fn upload_material_maps(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Arc<super::super::material_map_descriptor::MaterialMapBinding>>, RenderError>
    {
        let (Some(layout), Some(image_layout)) =
            (&pipeline.material_map_layout, &pipeline.surface_layout)
        else {
            return Ok(None);
        };
        let style = self.draw.surface_texture.as_ref();
        let Some(maps) = style.and_then(|style| style.material_maps.as_ref()) else {
            return Ok(None);
        };
        let pipelines = self.pipelines;
        let mocks = pipelines.material_map_mocks(self.runtime, image_layout, self.retirements)?;
        let request = super::super::material_map_descriptor::MaterialMapRequest {
            runtime: self.runtime,
            layout,
            image_layout,
            cache: &pipelines.surface_images,
            mocks: &mocks,
            retirements: self.retirements,
        };
        request.bind(maps).map(Some)
    }
    fn upload_clipping(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        let bytes = if pipeline.kind.world_clip_v2 || pipeline.kind.world_clip_v3_line {
            crate::retained_hatch::raster_world_clip_fragment_mask_bytes(
                self.draw,
                self.extent.height,
            )
        } else {
            crate::retained_hatch::fragment_mask_bytes(self.draw, self.extent.height)
        };
        bytes
            .as_ref()
            .map(|bytes| {
                let layout = pipeline
                    .clipping_layout
                    .as_ref()
                    .expect("clipping pipeline layout");
                UploadedStorage::new(self.runtime, layout, bytes)
            })
            .transpose()
    }

    fn upload_lighting(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        self.draw
            .lighting
            .as_ref()
            .map(|lighting| {
                let layout = pipeline
                    .lighting_layout
                    .as_ref()
                    .expect("lighting pipeline layout");
                UploadedStorage::new(self.runtime, layout, &lighting.encode())
            })
            .transpose()
    }

    fn upload_sphere_map(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        let Some(layout) = &pipeline.sphere_layout else {
            return Ok(None);
        };
        let style = self.draw.surface_texture.as_ref();
        let transform = style
            .and_then(|style| style.sphere_map.as_ref())
            .ok_or_else(|| rejected("no sphere map", "an environment-mapped draw"))?;
        UploadedStorage::new(self.runtime, layout, &transform.encode()).map(Some)
    }
    /// The draw's registered user program with the selected variant's fixed state.
    fn upload_user_program(
        &self,
        pipeline: &Arc<DrawPipeline>,
    ) -> Result<Option<super::super::user_program_draw::UploadedUserProgram>, RenderError> {
        let Some(program) = &self.draw.user_program else {
            return Ok(None);
        };
        let variant = Arc::as_ptr(pipeline) as usize;
        let fixed =
            super::super::user_program_pipeline::UserPipelineState::single(pipeline.kind, variant);
        let linked =
            self.pipelines
                .user_programs
                .pipeline(self.runtime, &program.modules, fixed)?;
        let textures =
            self.pipelines
                .user_program_textures(self.runtime, self.draw, self.retirements)?;
        super::super::user_program_draw::UploadedUserProgram::new(
            self.runtime,
            (linked, fixed),
            self.draw,
            program,
            textures,
        )
        .map(Some)
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

#[path = "frame_upload_bindings_result.rs"]
mod bindings_result;
