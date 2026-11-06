//! Core vertex attributes and flat varyings need resource limits, not barycentric features.
use crate::{RenderError, RenderTextInterpolationFailure as Failure};
use ash::vk;
const REQUIRED: [u32; 5] = [9, 112, 104, 40, 40];
const BINDINGS: [vk::VertexInputBindingDescription; 1] = [vk::VertexInputBindingDescription {
    binding: 0,
    stride: 112,
    input_rate: vk::VertexInputRate::VERTEX,
}];
const ATTRIBUTES: [vk::VertexInputAttributeDescription; 9] = [
    attribute(0, 0, 4),
    attribute(1, 16, 4),
    attribute(2, 32, 2),
    attribute(3, 40, 4),
    attribute(4, 56, 4),
    attribute(5, 72, 4),
    attribute(6, 88, 2),
    attribute(7, 96, 2),
    attribute(8, 104, 2),
];

pub(super) fn admit_limits(limits: &vk::PhysicalDeviceLimits) -> Result<(), RenderError> {
    let actual = observed_limits(limits);
    if actual
        .iter()
        .zip(REQUIRED)
        .any(|(&value, required)| value < required)
    {
        return Err(Failure::InsufficientLimits {
            actual,
            required: REQUIRED,
        }
        .into());
    }
    Ok(())
}

fn observed_limits(limits: &vk::PhysicalDeviceLimits) -> [u32; 5] {
    [
        limits.max_vertex_input_attributes,
        limits.max_vertex_input_binding_stride,
        limits.max_vertex_input_attribute_offset,
        limits.max_vertex_output_components,
        limits.max_fragment_input_components,
    ]
}

pub(super) fn vertex_state() -> vk::PipelineVertexInputStateCreateInfo<'static> {
    vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&BINDINGS)
        .vertex_attribute_descriptions(&ATTRIBUTES)
}

const fn attribute(location: u32, offset: u32, count: u32) -> vk::VertexInputAttributeDescription {
    vk::VertexInputAttributeDescription {
        location,
        binding: 0,
        offset,
        format: if count == 4 {
            vk::Format::R32G32B32A32_SFLOAT
        } else {
            vk::Format::R32G32_SFLOAT
        },
    }
}

pub(super) fn require_raster(
    behavior: crate::RenderTextInterpolationBehavior,
    selected: bool,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: u32,
) -> Result<(), RenderError> {
    if behavior != crate::RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4 || !selected
    {
        return Ok(());
    }
    if ordinary_fill(raster, stages) {
        return Ok(());
    }
    Err(unsupported_raster(raster, stages))
}

fn ordinary_fill(raster: &vk::PipelineRasterizationStateCreateInfo<'_>, stages: u32) -> bool {
    raster.polygon_mode == vk::PolygonMode::FILL
        && raster.p_next.is_null()
        && raster.rasterizer_discard_enable == vk::FALSE
        && stages == 2
}

fn unsupported_raster(
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: u32,
) -> RenderError {
    Failure::UnsupportedInterface { actual: format!(
        "flat text polygon={:?}, pNext={:?}, discard={}, stages={stages}; expected ordinary fill and two stages",
        raster.polygon_mode, raster.p_next, raster.rasterizer_discard_enable,
    ) }.into()
}

#[cfg(test)]
#[path = "text_flat_corners_layout_tests.rs"]
mod tests;
