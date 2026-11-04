//! V4 preserves V3 plain-point replay and selects only the closed unlit sprite operation.
use crate::point_style::PointColorPolicy;
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, RenderPointRasterBehavior};
use ash::vk;
pub(super) fn selected_draw(
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if draw
        .point_style
        .as_ref()
        .is_some_and(|s| s.color_policy == PointColorPolicy::Plain)
    {
        return super::fixed_grid_upload::select_plain(
            RenderPointRasterBehavior::FixedGridUnit12V3,
            draw,
            samples,
        );
    }
    if samples != vk::SampleCountFlags::TYPE_1
        || draw.topology() != PrimitiveTopology::Points
        || draw.polygon_points
    {
        return Ok(None);
    }
    validate_sprite(draw)?;
    let mut selected = draw.clone();
    selected.fixed_grid_point = true;
    selected.fixed_grid_point_unit12 = true;
    selected.fixed_grid_point_sprite = true;
    Ok(Some(selected))
}
fn validate_sprite(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if draw.user_program.is_some()
        || draw.lighting.is_some()
        || draw.fixed_function_lighting
        || draw.textured_style.is_some()
        || draw.surface_texture.is_some()
        || draw.unlit_back_color.is_some()
        || draw.surface_cutoff.is_some()
        || draw.surface_texture_unavailable.is_some()
    {
        return Err(RenderError::rejected(
            draw,
            "V4 built-in unlit point sprites without user/lighting/surface programs",
        ));
    }
    crate::fixed_grid_point::fixed_grid_point_extent(draw.point_style.as_ref().unwrap().size)?;
    Ok(())
}
#[cfg(test)]
#[path = "fixed_grid_sprite_tests.rs"]
mod tests;
