//! Ordinary frames retain their existing pipelines; stencil variants are created on demand.
use super::{pipeline::DrawPipeline, runtime::VulkanRuntime};
use crate::{PrimitiveDraw, RenderCappingPass, RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;
mod index_flags;
use index_flags::{capping_variant, ordinary_clipping};
#[derive(Clone, Copy, Debug)]
pub(super) struct PipelineVariant {
    pub lighting: bool,
    pub fixed_lighting: bool,
    pub pbr: bool,           // PBR fragment; the environment binds at set 4
    pub material_maps: bool, // texture-set maps at set 5
    /// The environment-texture vertex stage (sphere_map.glsl); its matrices bind at set 2.
    pub sphere_map: bool,

    pub fixed_line_stipple: bool,
    pub fixed_line_strip: bool,
    pub surface_texture: bool,
    pub surface_alpha: bool,
    pub mesh_edges: bool,
    pub polygon_edges: bool,
    pub polygon_points: bool,
    pub fixed_grid_point: bool,
    pub fixed_grid_point_unit12: bool,
    pub(crate) fixed_grid_point_sprite: bool,
    pub triangle_grid_coverage: bool,
    pub factorized_projection: bool,
    pub(super) window_depth: Option<super::window_depth_parameters::WindowDepthPipelineParameters>,
    pub surface_blend: bool,
    pub line_blend: bool,
    pub point_blend: bool,
    pub clamp: bool,
    pub line: bool,
    pub point: bool,
    pub text: bool,
    pub text_blend: bool,
    pub text_logic_xor: bool,
    pub encoded_surface: bool,
    pub clipped: bool,
    pub world_clip_v2: bool,
    pub world_clip_v3_line: bool,
    pub world_clip_grid12: bool,
    pub stencil: bool,
    pub capping: RenderCappingPass,
    pub depth_pass: crate::depth_prepass::RenderDepthPass,
    /// Write the attachment alpha (native `buffersOpaqueAlpha` off).
    pub alpha_writes: bool,
}
impl PipelineVariant {
    pub(super) fn uses_marker_sampler(self) -> bool {
        self.point && !(self.surface_texture || self.surface_alpha)
    }
    pub(super) fn from_index(index: usize, stencil: bool) -> Self {
        if (40..44).contains(&index) {
            return text_xor_variant(index, stencil);
        }
        if index >= 28 {
            return blended_variant(index, stencil);
        }
        let clipped = ordinary_clipping(index);
        let mut kind = empty_variant(index & 1 != 0, clipped, stencil, capping_variant(index));
        if index < 20 {
            let base = (index % 10) / 2;
            kind.line = base == 1;
            kind.point = base == 2;
            kind.point_blend = kind.point;
            kind.text = base >= 3;
            kind.text_blend = base == 3;
        }
        kind
    }
}

fn blended_variant(index: usize, stencil: bool) -> PipelineVariant {
    let mut kind = empty_variant(
        index & 1 != 0,
        index & 2 != 0,
        stencil,
        RenderCappingPass::None,
    );
    kind.line_blend = (32..36).contains(&index);
    kind.surface_blend = index < 32;
    kind.line = kind.line_blend;
    kind.point = index >= 36;
    kind
}

fn empty_variant(
    clamp: bool,
    clipped: bool,
    stencil: bool,
    capping: RenderCappingPass,
) -> PipelineVariant {
    PipelineVariant {
        lighting: false,
        fixed_lighting: false,
        pbr: false,
        material_maps: false,
        sphere_map: false,
        fixed_line_stipple: false,
        fixed_line_strip: false,
        surface_texture: false,
        surface_alpha: false,
        mesh_edges: false,
        polygon_edges: false,
        polygon_points: false,
        fixed_grid_point: false,
        fixed_grid_point_unit12: false,
        fixed_grid_point_sprite: false,
        triangle_grid_coverage: false,
        factorized_projection: false,
        window_depth: None,
        surface_blend: false,
        line_blend: false,
        point_blend: false,
        clamp,
        line: false,
        point: false,
        text: false,
        text_blend: false,
        text_logic_xor: false,
        encoded_surface: false,
        clipped,
        world_clip_v2: false,
        world_clip_v3_line: false,
        world_clip_grid12: false,
        stencil,
        capping,
        depth_pass: Default::default(),
        alpha_writes: false,
    }
}
/// Base variant, depth-prepass flag and optional specialization words of a special pipeline.
type SpecialVariantKey = ((usize, u8), bool, Option<[u32; 10]>);

pub(super) struct RasterPipelines {
    pub oit: super::oit_pipeline_cache::OitPipelineCache,
    variants: Vec<Option<Arc<DrawPipeline>>>,
    pub surface_images: super::surface_image_cache::SurfaceImageCache,
    pub stencil: bool,
    runtime: Arc<VulkanRuntime>,
    special_variants:
        std::sync::Mutex<std::collections::BTreeMap<SpecialVariantKey, Arc<DrawPipeline>>>,
    alpha_writes: super::alpha_write_pipelines::AlphaWritePipelines,
    pub(super) pbr_environment:
        std::sync::Mutex<Option<Arc<super::pbr_environment::PbrEnvironment>>>,
    /// The PBR environment of the view being rendered, if the frame carries one.
    pub(super) frame_pbr_environment:
        std::sync::Mutex<Option<Arc<super::pbr_environment::PbrEnvironment>>>,
    pub(super) material_map_mocks:
        std::sync::Mutex<Option<Arc<super::material_map_descriptor::MaterialMapMocks>>>,
    /// The empty shadow-map set of lit draws in frames without shadow maps.
    pub(super) empty_shadow_maps: std::sync::Mutex<Option<Arc<super::shadow_maps::ShadowMapSet>>>,
    /// The shadow maps of the view being rendered, if the frame carries them.
    pub(super) frame_shadow_maps: std::sync::Mutex<Option<Arc<super::shadow_maps::ShadowMapSet>>>,
    /// Linked user programs (`user_program_pipeline`) and the layout of their mock sampler.
    pub(super) user_programs: super::user_program_pipeline::UserProgramPipelineCache,
    pub(super) user_program_mock_layout:
        std::sync::Mutex<Option<Arc<super::surface_image_descriptor::SurfaceImageLayout>>>,
}
impl RasterPipelines {
    pub fn new(runtime: &Arc<VulkanRuntime>, stencil: bool) -> Result<Self, RenderError> {
        if stencil {
            super::image::check_format(runtime, runtime.depth.formats.stencil, true)?;
        }
        let mut variants = Vec::new();
        for index in 0..if stencil { 28 } else { 20 } {
            let kind = PipelineVariant::from_index(index, stencil);
            let supported =
                (!kind.clamp || runtime.depth_clamp) && (!kind.line || runtime.lines.bresenham);
            variants.push(if supported {
                Some(Arc::new(DrawPipeline::new(runtime, kind)?))
            } else {
                None
            });
        }
        Ok(Self {
            oit: Default::default(),
            variants,
            surface_images: Default::default(),
            stencil,
            runtime: runtime.clone(),
            special_variants: Default::default(),
            alpha_writes: Default::default(),
            pbr_environment: Default::default(),
            frame_pbr_environment: Default::default(),
            material_map_mocks: Default::default(),
            empty_shadow_maps: Default::default(),
            frame_shadow_maps: Default::default(),
            user_programs: Default::default(),
            user_program_mock_layout: Default::default(),
        })
    }
    /// Release renderer-owned caches after actual completion; `pipelines.release_resources()` preserves child leases.
    pub fn release_resources(&mut self) {
        self.variants.clear();
        self.special_variants
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
        self.oit = Default::default();
        self.alpha_writes = Default::default();
        self.surface_images = Default::default();
        self.release_pbr_environment();
        self.release_shadow_maps();
        *self
            .material_map_mocks
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    }
    /// The alpha-writing twin of a selected pipeline; `pipelines.alpha_write_twin(&p)?`.
    pub fn alpha_write_twin(
        &self,
        source: &Arc<DrawPipeline>,
    ) -> Result<Arc<DrawPipeline>, RenderError> {
        self.alpha_writes.twin(&self.runtime, source)
    }
    pub fn select(
        &self,
        draw: &PrimitiveDraw,
        factorized: bool,
        window_depth: Option<super::window_depth_parameters::WindowDepthPipelineParameters>,
    ) -> Result<Arc<DrawPipeline>, RenderError> {
        // The lines of V3 and the unlit stencil masks of V4 have no ordinary raster-clip variant.
        let raster_clip = self.runtime.uses_raster_world_clipping(draw);
        if factorized || window_depth.is_some() || requires_special_pipeline(draw, raster_clip) {
            return self.select_special(draw, factorized, window_depth);
        }
        let index = variant_index(draw);
        self.variants
            .get(index)
            .and_then(Clone::clone)
            .ok_or_else(|| {
                rejected((draw.depth_clamp, draw.line_style.is_some(), draw.capping_pass),
                "device support for requested stencil, depth-clamp and line rasterization features")
            })
    }
    fn select_special(
        &self,
        draw: &PrimitiveDraw,
        factorized: bool,
        window_depth: Option<super::window_depth_parameters::WindowDepthPipelineParameters>,
    ) -> Result<Arc<DrawPipeline>, RenderError> {
        crate::polygon_point_raster::require_polygon_points(
            draw.polygon_points,
            self.runtime.polygon_lines,
            self.runtime.subset_point_polygons,
        )?;
        crate::polygon_point_raster::require_polygon_point_program(draw, draw.polygon_points)?;
        crate::text_logic::require_text_xor(draw.text_logic_xor, self.runtime.logic_op)?;
        crate::fixed_line_stipple::require_fixed_line_stipple(
            crate::fixed_line_stipple::fixed_line_stipple(draw).is_some(),
            self.runtime.lines.stippled,
        )?;
        crate::mesh_edges::require_polygon_lines(
            draw.polygon_edge_style.is_some(),
            self.runtime.polygon_lines,
        )?;
        crate::mesh_edges::require_mesh_geometry(
            draw.mesh_edges.is_some(),
            self.runtime.geometry_shader,
        )?;
        super::surface_texture_pipeline::require_sphere_map_without_mesh_edges(draw)?;
        let world_clip_v3_line = self.runtime.uses_world_clip_line_v3(draw)
            && self.runtime.uses_raster_world_clipping(draw);
        let world_clip_v2 = self.runtime.uses_raster_world_clipping(draw) && !world_clip_v3_line;
        let world_clip_grid12 = self.runtime.uses_grid12_world_clipping(draw);
        let index = projection_variant_key(
            draw,
            world_clip_v2,
            world_clip_v3_line,
            world_clip_grid12,
            factorized,
            window_depth.map(|value| value.cache_words()),
        );
        let mut variants = self
            .special_variants
            .lock()
            .map_err(|e| rejected(e.to_string(), "an unpoisoned pipeline cache"))?;
        if let Some(pipeline) = variants.get(&index) {
            return Ok(pipeline.clone());
        }
        let mut kind = special_variant(draw, self.stencil);
        kind.factorized_projection = factorized;
        kind.window_depth = window_depth;
        kind.world_clip_v2 = world_clip_v2;
        kind.world_clip_v3_line = world_clip_v3_line;
        apply_grid12_world_clip(&mut kind, world_clip_grid12);
        kind.triangle_grid_coverage = self.runtime.triangle_coverage
            != crate::RenderTriangleCoverageBehavior::HardwareV1
            && super::triangle_grid_coverage_upload::selected_phong(draw);
        validate_special_features(kind, self.runtime.depth_clamp, self.runtime.lines.bresenham)?;
        let pipeline = Arc::new(DrawPipeline::new(&self.runtime, kind)?);
        variants.insert(index, pipeline.clone());
        Ok(pipeline)
    }
}
fn requires_special_pipeline(draw: &PrimitiveDraw, raster_clip: bool) -> bool {
    raster_clip
        || draw.fixed_grid_point
        || draw.polygon_points
        || draw.polygon_edge_style.is_some()
        || draw.mesh_edges.is_some()
        || draw.surface_texture_unavailable.is_some()
        || draw.unlit_back_color.is_some()
        || draw.surface_texture.is_some()
        || draw.surface_cutoff.is_some()
        || draw.surface_forced_blend
        || draw.lighting.is_some()
        || draw.fixed_function_lighting
        || draw.compatibility_color_target
        || draw.surface_blend
        || draw.text_background_blend
        || draw.text_logic_xor
        || draw.line_style.is_some_and(|s| s.alpha_blend)
        || draw.point_style.as_ref().is_some_and(|s| !s.alpha_blend)
        || draw.depth_pass != Default::default()
}
fn projection_variant_key(
    draw: &PrimitiveDraw,
    phong: bool,
    line: bool,
    grid12: bool,
    factorized: bool,
    window_depth: Option<[u32; 10]>,
) -> ((usize, u8), bool, Option<[u32; 10]>) {
    (
        world_clip_cache_key(draw, phong, line, grid12),
        factorized,
        window_depth,
    )
}
fn apply_grid12_world_clip(kind: &mut PipelineVariant, enabled: bool) {
    kind.world_clip_grid12 = enabled;
    kind.clipped |= enabled;
}
fn world_clip_cache_key(
    draw: &PrimitiveDraw,
    phong: bool,
    line: bool,
    grid12: bool,
) -> (usize, u8) {
    (
        special_variant_key(draw),
        u8::from(phong) + 2 * u8::from(line) + 4 * u8::from(grid12),
    )
}
fn special_variant_key(draw: &PrimitiveDraw) -> usize {
    variant_index(draw)
        + 64 * usize::from(draw.lighting.is_some())
        + 128 * usize::from(draw.surface_texture.is_some())
        + 256 * usize::from(draw.surface_cutoff.is_some())
        + 512 * usize::from(draw.surface_texture_unavailable.is_some())
        + 1024 * (draw.capping_pass as usize)
        + 8192 * usize::from(draw.mesh_edges.is_some())
        + 16384 * usize::from(draw.polygon_edge_style.is_some())
        + 32768 * usize::from(draw.surface_texture_binding_only)
        + 65536 * usize::from(encoded_surface_attachment(draw))
        + 131072
            * usize::from(
                draw.lighting
                    .as_ref()
                    .is_some_and(|state| state.is_fixed_lighting()),
            )
        + 262144 * usize::from(crate::fixed_line_stipple::fixed_line_stipple(draw).is_some())
        + 524288 * usize::from(draw.fixed_function_lighting && draw.source_line_strip)
        + 1048576 * usize::from(draw.unlit_back_color.is_some())
        + 2097152 * (draw.depth_pass as usize)
        + 16777216 * usize::from(super::pbr_pipeline::draw_uses_pbr(draw))
        + 33554432 * usize::from(super::material_map_pipeline::draw_uses_material_maps(draw))
        + 67108864 * usize::from(super::surface_texture_pipeline::draw_uses_sphere_map(draw))
        + 134217728 * usize::from(draw.polygon_points)
        + 268435456 * usize::from(draw.fixed_grid_point)
        + 536870912 * usize::from(draw.fixed_grid_point_unit12)
        + 1073741824 * usize::from(draw.fixed_grid_point_sprite)
}
pub(super) fn special_variant(draw: &PrimitiveDraw, stencil: bool) -> PipelineVariant {
    let mut kind = PipelineVariant::from_index(variant_index(draw), stencil);
    kind.lighting = draw.lighting.is_some();
    kind.fixed_line_stipple = crate::fixed_line_stipple::fixed_line_stipple(draw).is_some();
    kind.fixed_line_strip = draw.fixed_function_lighting && draw.source_line_strip;
    kind.fixed_lighting = draw
        .lighting
        .as_ref()
        .is_some_and(|state| state.is_fixed_lighting());
    kind.pbr = super::pbr_pipeline::draw_uses_pbr(draw);
    kind.material_maps = super::material_map_pipeline::draw_uses_material_maps(draw);
    kind.sphere_map = super::surface_texture_pipeline::draw_uses_sphere_map(draw);
    kind.mesh_edges = draw.mesh_edges.is_some();
    kind.polygon_edges = draw.polygon_edge_style.is_some();
    kind.polygon_points = draw.polygon_points;
    kind.fixed_grid_point = draw.fixed_grid_point;
    kind.fixed_grid_point_unit12 = draw.fixed_grid_point_unit12;
    kind.fixed_grid_point_sprite = draw.fixed_grid_point_sprite;
    kind.line |= kind.polygon_edges;
    kind.surface_texture = draw.surface_texture.is_some() && !draw.surface_texture_binding_only;
    // The auxiliary surface family carries either UVs or V2 back RGBA, never both.
    kind.surface_alpha = draw.unlit_back_color.is_some()
        || draw.surface_cutoff.is_some()
        || (draw.surface_texture_unavailable.is_some() && !draw.surface_texture_binding_only);
    kind.capping = draw.capping_pass;
    kind.encoded_surface = encoded_surface_attachment(draw);
    kind.depth_pass = draw.depth_pass;
    kind
}
pub(super) fn encoded_surface_attachment(draw: &PrimitiveDraw) -> bool {
    !draw.surface_texture_binding_only
        && (draw.compatibility_color_target
            || draw.fixed_function_lighting
            || crate::fixed_surface_environment::uses_fixed_surface_environment(draw))
}
fn validate_special_features(
    kind: PipelineVariant,
    depth_clamp: bool,
    bresenham: bool,
) -> Result<(), RenderError> {
    if kind.clamp && !depth_clamp {
        return Err(rejected(kind.clamp, "device depth-clamp support"));
    }
    if kind.line && !bresenham {
        return Err(rejected(
            kind.line,
            "device Bresenham line-rasterization support",
        ));
    }
    Ok(())
}
fn variant_index(draw: &PrimitiveDraw) -> usize {
    if let Some(index) = alpha_variant_index(draw) {
        return index;
    }
    if draw.capping_pass != RenderCappingPass::None {
        return (if draw.capping_pass.is_mask() { 20 } else { 24 })
            + usize::from(draw.depth_clamp)
            + 2 * usize::from(crate::retained_hatch::needs_fragment_mask(draw));
    }
    10 * usize::from(crate::retained_hatch::needs_fragment_mask(draw))
        + usize::from(draw.depth_clamp)
        + 2 * usize::from(draw.line_style.is_some())
        + 4 * usize::from(draw.point_style.is_some())
        + draw
            .textured_style
            .as_ref()
            .map_or(0, |style| if style.blend { 6 } else { 8 })
}

fn alpha_variant_index(draw: &PrimitiveDraw) -> Option<usize> {
    if draw.text_logic_xor {
        return Some(
            40 + usize::from(draw.depth_clamp)
                + 2 * usize::from(crate::retained_hatch::needs_fragment_mask(draw)),
        );
    }
    let flags = usize::from(draw.depth_clamp)
        + 2 * usize::from(crate::retained_hatch::needs_fragment_mask(draw));
    if draw.line_style.is_some_and(|s| s.alpha_blend) {
        return Some(32 + flags);
    }
    if draw.point_style.as_ref().is_some_and(|s| !s.alpha_blend) {
        return Some(36 + flags);
    }
    (draw.surface_blend || draw.surface_forced_blend || draw.text_background_blend)
        .then_some(28 + flags)
}

#[cfg(test)]
#[path = "pipeline_variant_surface_tests.rs"]
mod surface_variant_tests;

pub(super) fn text_xor_variant(index: usize, stencil: bool) -> PipelineVariant {
    let mut kind = empty_variant(
        index & 1 != 0,
        index & 2 != 0,
        stencil,
        RenderCappingPass::None,
    );
    kind.text = true;
    kind.text_logic_xor = true;
    kind
}

#[cfg(test)]
#[path = "pipeline_variant_mesh_tests.rs"]
mod mesh_tests;

#[cfg(test)]
#[path = "polygon_point_pipeline_tests.rs"]
mod polygon_point_tests;

#[cfg(test)]
include!("pipeline_variant_inline_tests.rs");
