//! Coverage choices compose only on a private upload copy.
use super::runtime::VulkanRuntime;
use crate::{PrimitiveDraw, RenderError};
use ash::vk;
pub(super) fn private_geometry(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if let Some(selected) = crate::thin_line::selected_draw(
        runtime.thin_line_coverage,
        runtime.thin_axis_coverage,
        draw,
        extent,
        samples == vk::SampleCountFlags::TYPE_1,
    )? {
        return Ok(Some(selected));
    }
    if let Some(selected) = crate::thin_axis::selected_draw(
        runtime.thin_axis_coverage,
        draw,
        [extent.width, extent.height],
        samples == vk::SampleCountFlags::TYPE_1,
    )? {
        return Ok(Some(selected));
    }
    super::fixed_grid_upload::selected_point_draw(runtime.point_raster, draw, samples)
}
