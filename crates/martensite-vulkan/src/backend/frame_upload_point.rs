//! Point admission stays separate from upload ordering and encoded geometry.
use super::super::{point_sprite, runtime::VulkanRuntime};
use crate::{PrimitiveDraw, RenderError};
use ash::vk;

pub(super) fn validate_point(
    runtime: &VulkanRuntime,
    semantic_draw: &PrimitiveDraw,
    encoded_draw: &PrimitiveDraw,
    extent: vk::Extent2D,
) -> Result<(), RenderError> {
    validate_viewport(runtime, semantic_draw, encoded_draw, extent)?;
    validate_size(runtime, semantic_draw)
}
fn validate_viewport(
    runtime: &VulkanRuntime,
    semantic_draw: &PrimitiveDraw,
    encoded_draw: &PrimitiveDraw,
    extent: vk::Extent2D,
) -> Result<(), RenderError> {
    let attachment = runtime.attachment_coordinates.attachment_extent(extent);
    crate::fixed_grid_point::validate_draw_viewport(
        semantic_draw.fixed_grid_point,
        encoded_draw.fixed_grid_point,
        encoded_draw.fixed_grid_point_unit12,
        [attachment.width, attachment.height],
    )
}
fn validate_size(
    runtime: &VulkanRuntime,
    semantic_draw: &PrimitiveDraw,
) -> Result<(), RenderError> {
    let Some(style) = &semantic_draw.point_style else {
        return Ok(());
    };
    point_sprite::validate_point_size(
        style.size,
        runtime.large_points,
        runtime.limits.point_size_range,
    )
}
