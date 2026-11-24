//! The lighting descriptor layout and the base-vertex stride of the stock pipelines.
use super::super::runtime::VulkanRuntime;
use super::lighting_shaders;
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

pub(super) fn lighting_layout(
    runtime: &Arc<VulkanRuntime>,
    kind: super::super::pipeline_variants::PipelineVariant,
) -> Result<Option<super::super::storage_descriptor::StorageDescriptorLayout>, RenderError> {
    use super::super::storage_descriptor::StorageDescriptorLayout;
    if !kind.lighting {
        return Ok(None);
    }
    let stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
    // Dynamic ranges are qualified for the stock opaque surface path.
    if !kind.dynamic_lighting
        || kind.surface_blend
        || kind.line
        || kind.point
        || kind.polygon_points
        || kind.pbr
        || kind.fixed_lighting
        || kind.clipped
        || kind.mesh_edges
        || kind.surface_texture
        || kind.surface_alpha
    {
        return StorageDescriptorLayout::new(runtime, stages).map(Some);
    }
    StorageDescriptorLayout::lighting(runtime, stages).map(Some)
}

// The packaged plain lighting vertex stage reads neither VertexIndex nor BaseVertex.
pub(super) fn stock_base_vertex_stride(
    kind: super::super::pipeline_variants::PipelineVariant,
    actual: &[u8],
) -> Option<u64> {
    if kind.compact_vertices {
        return Some(u64::from(super::super::mesh_vertex_codec::PACKED_STRIDE));
    }
    if !kind.lighting
        || kind.line
        || kind.point
        || kind.text
        || kind.surface_texture
        || kind.surface_alpha
        || kind.fixed_lighting
        || kind.mesh_edges
    {
        return None;
    }
    let [plain, _] = lighting_shaders(false, false, false);
    (actual == plain || actual == super::super::directional_phong::SHADERS[0])
        .then_some(u64::from(super::super::pipeline_state::LIT_VERTEX_STRIDE))
}
