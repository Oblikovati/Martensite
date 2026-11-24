//! Draw-table twins read per-draw transforms and stock surface materials at set 7.
use super::super::pipeline_variants::PipelineVariant;
use super::super::runtime::VulkanRuntime;
use super::super::storage_descriptor::StorageDescriptorLayout;
use super::DrawPipeline;
use crate::{RenderError, error::rejected};
use ash::vk;
use ash::vk::Handle;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// `lighting.vert` compiled with `-DMARTENSITE_DRAW_TABLE=1` (shaders/draw_table.glsl).
const HIZ_VERTEX: &[u8] = include_bytes!("../../shaders/placement_hiz_depth.vert.spv");
const TABLE_VERTEX: &[u8] = include_bytes!("../../shaders/lighting.draw-table.vert.spv");
const INDEXED_VERTEX: &[u8] = include_bytes!("../../shaders/lighting.indexed-material.vert.spv");
const INDEXED_FRAGMENT: &[u8] = include_bytes!("../../shaders/lighting.indexed-material.frag.spv");
const INDEXED_COORDINATES_FRAGMENT: &[u8] =
    include_bytes!("../../shaders/lighting.indexed-material.coordinates-v2.frag.spv");
const COORDINATES_FRAGMENT: &[u8] =
    include_bytes!("../../shaders/lighting.coordinates-v2.frag.spv");

/// Draw-table twins keyed by the raw handle of their source pipeline; `None` marks a source
/// that has no twin, so that it is classified once.
type DrawTableTwinKey = (u64, bool);
type DrawTableTwinCache = BTreeMap<DrawTableTwinKey, Option<Arc<DrawPipeline>>>;
#[derive(Default)]
pub(in crate::backend) struct DrawTablePipelines(Mutex<DrawTableTwinCache>);

impl DrawTablePipelines {
    /// The twin of `source`, or `None` when the device or the source has none;
    /// `twins.twin(&runtime, &pipeline)?`.
    pub fn twin(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<DrawPipeline>,
    ) -> Result<Option<Arc<DrawPipeline>>, RenderError> {
        self.twin_for(runtime, source, false)
    }

    pub(in crate::backend) fn hi_z(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<DrawPipeline>,
    ) -> Result<Option<Arc<DrawPipeline>>, RenderError> {
        self.twin_for(runtime, source, true)
    }

    fn twin_for(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<DrawPipeline>,
        hiz_depth_only: bool,
    ) -> Result<Option<Arc<DrawPipeline>>, RenderError> {
        let mut twins = self
            .0
            .lock()
            .map_err(|_| rejected("poisoned draw-table pipeline cache", "an available cache"))?;
        let key = (source.handle.as_raw(), hiz_depth_only);
        if let Some(twin) = twins.get(&key) {
            return Ok(twin.clone());
        }
        let twin = source
            .build_draw_table_twin(runtime, hiz_depth_only)?
            .map(Arc::new);
        twins.insert(key, twin.clone());
        Ok(twin)
    }
}

impl DrawPipeline {
    /// The twin of this pipeline, built with the inputs it was built with, or `None` when it
    /// is not a single-pass plain lit surface pipeline.
    fn build_draw_table_twin(
        &self,
        runtime: &Arc<VulkanRuntime>,
        hiz_depth_only: bool,
    ) -> Result<Option<DrawPipeline>, RenderError> {
        let Some((samples, alpha_coverage)) = self.table_inputs else {
            return Ok(None);
        };
        if !runtime.draw_tables || !self.has_plain_lit_stages() {
            return Ok(None);
        }
        let mut kind = self.kind;
        kind.draw_table = true;
        kind.hiz_depth_only = hiz_depth_only;
        Self::build(runtime, kind, None, samples, alpha_coverage, None, false).map(Some)
    }

    /// Whether the pipeline draws with exactly the plain `lighting.vert` and a fragment stage,
    /// with none of the emulations that read more than the matrices a table replaces; read
    /// once per pipeline, since the upload asks it for every draw.
    pub(in crate::backend) fn has_plain_lit_stages(&self) -> bool {
        *self
            .plain_lit_stages
            .get_or_init(|| self.classify_plain_lit_stages())
    }

    fn classify_plain_lit_stages(&self) -> bool {
        let [plain, _] = super::lighting_shaders(false, false, false);
        self.shaders.len() == 2
            && self.vertex_bytes == plain
            && !self.phong_interpolation_shader
            && !self.triangle_grid_coverage_shader
            && !self.world_clip_grid12_shader
            && !self.text_grid_interpolation_shader
            && plain_lit_kind(self.kind)
    }

    /// Give a plain lit pipeline the stages and layout of its draw-table twin.
    pub(super) fn apply_draw_table(&mut self) -> Result<(), RenderError> {
        if !self.kind.draw_table {
            return Ok(());
        }
        // Not the cached answer: after the swap below, this pipeline's stages are the twin's.
        if !self.classify_plain_lit_stages() {
            return Err(rejected(
                self.kind,
                "a plain lit surface pipeline for a draw table",
            ));
        }
        let indexed_fragment = self.indexed_material_fragment();
        if self.kind.hiz_depth_only {
            self.replace_vertex(HIZ_VERTEX)?;
        } else if let Some(fragment) = indexed_fragment {
            self.replace_vertex(INDEXED_VERTEX)?;
            self.replace_fragment(fragment)?;
        } else {
            self.replace_vertex(TABLE_VERTEX)?;
        }
        let layout =
            StorageDescriptorLayout::draw_table(&self.runtime, indexed_fragment.is_some())?;
        self.draw_table_layout = Some(layout);
        Ok(())
    }

    pub(in crate::backend) fn uses_indexed_draw_materials(&self) -> bool {
        self.indexed_material_fragment().is_some()
    }

    fn indexed_material_fragment(&self) -> Option<&'static [u8]> {
        let dynamic_lighting = self
            .lighting_layout
            .as_ref()
            .is_some_and(StorageDescriptorLayout::uses_dynamic_offset);
        if !super::super::draw_material_table::INDEX_MATERIALS
            || !indexed_storage_limits(&self.runtime.limits, dynamic_lighting)
        {
            return None;
        }
        let [_, fragment] = super::lighting_shaders(false, false, false);
        if self.fragment_bytes == fragment {
            return Some(INDEXED_FRAGMENT);
        }
        if self.fragment_bytes == COORDINATES_FRAGMENT {
            return Some(INDEXED_COORDINATES_FRAGMENT);
        }
        None
    }

    fn replace_vertex(&mut self, bytes: &'static [u8]) -> Result<(), RenderError> {
        let replacement = super::load_shader(&self.runtime, bytes)?;
        // SAFETY: the module belongs to this device and no pipeline was created from it yet.
        unsafe {
            self.runtime
                .device()
                .destroy_shader_module(self.shaders[0], None);
        }
        self.shaders[0] = replacement;
        self.vertex_bytes = bytes;
        #[cfg(test)]
        if !self.executable_bytes.is_empty() {
            self.executable_bytes[0] = bytes;
        }
        Ok(())
    }
}

pub(in crate::backend) fn indexed_storage_limits(
    limits: &vk::PhysicalDeviceLimits,
    dynamic_lighting: bool,
) -> bool {
    // Set 6 contributes shadow storage and set 7 contributes both draw-table buffers.
    // Sets 0-5 include the lighting layout, even when a shader leaves the slots unused.
    const STAGE_STORAGE: u32 = 6 + 1 + 2;
    let static_storage = if dynamic_lighting { 3 } else { STAGE_STORAGE };
    limits.max_per_stage_descriptor_storage_buffers >= STAGE_STORAGE
        && limits.max_descriptor_set_storage_buffers >= static_storage
}

/// A lit surface variant with no stage, emulation or binding besides lighting, the PBR-free
/// fragment and shadow maps.
fn plain_lit_kind(kind: PipelineVariant) -> bool {
    kind.lighting
        && !kind.fixed_lighting
        && !kind.pbr
        && !kind.material_maps
        && !kind.sphere_map
        && !(kind.line || kind.point || kind.text || kind.polygon_points || kind.polygon_edges)
        && !(kind.surface_texture || kind.surface_alpha || kind.mesh_edges)
        && !(kind.fixed_grid_point || kind.triangle_grid_coverage || kind.factorized_projection)
        && kind.window_depth.is_none()
        && !(kind.clipped
            || kind.world_clip_v2
            || kind.world_clip_v3_line
            || kind.world_clip_grid12)
        && !kind.encoded_surface
        && kind.capping == crate::RenderCappingPass::None
}

#[cfg(test)]
#[path = "pipeline_draw_table_tests.rs"]
mod tests;
