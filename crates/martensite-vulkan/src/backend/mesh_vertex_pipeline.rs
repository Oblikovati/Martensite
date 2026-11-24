//! Packed inputs for the stock programmable-lighting stages.
use super::{mesh_vertex_codec::PACKED_STRIDE, runtime::VulkanRuntime};
use crate::{PrimitiveDraw, RenderMeshVertexBehavior, RenderWindowDepthBehavior};
use ash::vk;

pub(super) const STOCK_VERTEX: &[u8] =
    include_bytes!("../../shaders/lighting.quantized-mesh.vert.spv");
pub(super) const DIRECTIONAL_VERTEX: &[u8] =
    include_bytes!("../../shaders/lighting.directional-phong.quantized-mesh.vert.spv");

pub(super) fn admits(runtime: &VulkanRuntime, draw: &PrimitiveDraw) -> bool {
    runtime.mesh_vertex_behavior == RenderMeshVertexBehavior::Quantized16OctV2
        && super::directional_phong::supported_behavior(runtime)
        && runtime.window_depth_behavior == RenderWindowDepthBehavior::HardwareV1
        && super::directional_phong::ordinary_surface(draw)
        && draw.native_material_cutoff.is_none_or(|cutoff| cutoff > 1.)
        && draw.lighting.as_ref().is_some_and(|lighting| {
            lighting.fixed.is_none()
                && !lighting.pbr
                && !lighting
                    .lights
                    .sources
                    .iter()
                    .any(|light| light.cast_shadows)
        })
}

pub(super) fn vertex_input() -> vk::PipelineVertexInputStateCreateInfo<'static> {
    const BINDING: [vk::VertexInputBindingDescription; 1] = [vk::VertexInputBindingDescription {
        binding: 0,
        stride: PACKED_STRIDE,
        input_rate: vk::VertexInputRate::VERTEX,
    }];
    const ATTRIBUTES: [vk::VertexInputAttributeDescription; 3] = [
        vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32_UINT,
            offset: 0,
        },
        vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: 12,
        },
        vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32_UINT,
            offset: 8,
        },
    ];
    vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&BINDING)
        .vertex_attribute_descriptions(&ATTRIBUTES)
}

#[cfg(all(
    test,
    not(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "lit-only-command-diagnostics"
    ))
))]
#[path = "mesh_vertex_pipeline_tests.rs"]
mod tests;
