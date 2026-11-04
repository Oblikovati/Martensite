//! Ordinary point UVs sample the base image at the vertex, independently of marker sprites.
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, error::rejected};

pub(crate) fn uses_point_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Points
        && (draw.surface_texture.is_some() || draw.surface_texture_unavailable.is_some())
}
pub(crate) fn require_plain_points(draws: &[PrimitiveDraw]) -> Result<(), RenderError> {
    for draw in draws
        .iter()
        .filter(|draw| draw.topology() == PrimitiveTopology::Points)
    {
        let policy = draw.point_style.as_ref().map(|style| style.color_policy);
        if policy != Some(crate::point_style::PointColorPolicy::Plain) {
            return Err(rejected(
                policy,
                "ordinary UV-mapped plain points; surface/marker-sprite coexistence is unsupported",
            ));
        }
    }
    Ok(())
}
pub(crate) fn finish_point_mapping(draw: &mut PrimitiveDraw) -> Result<(), RenderError> {
    if !uses_point_surface(draw) {
        return Ok(());
    }
    if let Some(style) = draw.surface_texture.as_mut() {
        require_base_color_only(style.material_maps.is_some())?;
        // Native programmable marker lighting uses the sampled image as vertex color.
        style.modulate = true;
    }
    if let Some(style) = draw.surface_texture_unavailable.as_mut() {
        style.modulate = true;
    }
    Ok(())
}
fn resolve_unavailable_point_base(draw: &mut PrimitiveDraw) {
    if draw.surface_texture_unavailable.is_none() || draw.source_color_array {
        return;
    }
    // Fresh unit-zero failure samples incomplete black before unmodulated emission is added.
    for vertex in &mut draw.vertices {
        vertex.color = [0., 0., 0., 1.];
    }
}
fn require_base_color_only(has_material_maps: bool) -> Result<(), RenderError> {
    if has_material_maps {
        return Err(rejected(
            "mapped point material maps",
            "base-color mapped points; extra material maps are unsupported",
        ));
    }
    Ok(())
}
pub(crate) fn point_surface_flags(draw: &PrimitiveDraw) -> u32 {
    if !uses_point_surface(draw) {
        return 0;
    }
    // Fragment base is already resolved at the vertex; preserve source-color precedence.
    2 | (8192 * u32::from(draw.source_color_array))
}

/// Avoid applying the missing-image black a second time; `resolved_surface_flags(&draw, flags)`.
pub(crate) fn resolved_surface_flags(draw: &PrimitiveDraw, flags: u32) -> u32 {
    if !uses_point_surface(draw) {
        return flags;
    }
    (flags & !16) | point_surface_flags(draw)
}

/// Resolve point base inputs after real color-array provenance; `resolve_point_shading_base(draws, model)`.
pub(crate) fn resolve_point_shading_base(
    draws: &mut [PrimitiveDraw],
    model: martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel,
) {
    for draw in draws.iter_mut().filter(|draw| uses_point_surface(draw)) {
        resolve_unavailable_point_base(draw);
        // Only Native PointSimple Unlit discards <= 0.1; lit plain points do not discard.
        draw.surface_cutoff = (model
            == martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel::Unlit)
            .then_some(0.1_f32.next_up());
    }
}
