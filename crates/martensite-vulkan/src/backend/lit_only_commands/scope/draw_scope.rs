//! The binary sprite exception has no fractional blending or filtered texture sampling.
use super::LitOnlyFailure;
use crate::{PrimitiveDraw, PrimitiveTopology};

pub(super) fn validate(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    validate_routes(ordinal, draw)?;
    validate_source_alpha(ordinal, draw)?;
    if draw.lighting.is_some() {
        return validate_lit(ordinal, draw);
    }
    match draw.topology() {
        PrimitiveTopology::Lines => reject(
            ordinal,
            draw.line_style.as_ref().is_some_and(|s| s.alpha_blend),
            "line blending",
        ),
        PrimitiveTopology::Points => validate_point(ordinal, draw),
        actual => Err(refusal(ordinal, "unlit interface", format!("{actual:?}"))),
    }
}
fn validate_source_alpha(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    if let Some(vertex) = draw
        .vertices
        .iter()
        .find(|v| v.color[3].to_bits() != 1f32.to_bits())
    {
        return Err(refusal(
            ordinal,
            "source alpha",
            format!("bits={:#x}", vertex.color[3].to_bits()),
        ));
    }
    Ok(())
}
fn validate_lit(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    let lighting = draw.lighting.as_ref().expect("selected lit draw");
    let interface = draw.topology() == PrimitiveTopology::Triangles
        && lighting.shading() == crate::RenderShading::Phong
        && !lighting.is_fixed_lighting()
        && !lighting.is_pbr();
    reject(ordinal, !interface, "lit interface")?;
    reject(ordinal, !opaque_material(lighting), "material opacity")
}
fn opaque_material(lighting: &crate::RenderLighting) -> bool {
    lighting
        .material
        .common
        .iter()
        .all(|m| m.diffuse[3].to_bits() == 1f32.to_bits())
}
fn validate_routes(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    reject(ordinal, draw.user_program.is_some(), "user program")?;
    reject(
        ordinal,
        draw.capping_pass != crate::RenderCappingPass::None || draw.stencil_rect,
        "capping/stencil",
    )?;
    reject(
        ordinal,
        draw.surface_blend || draw.surface_forced_blend || draw.text_background_blend,
        "blending",
    )?;
    validate_texture_routes(ordinal, draw)?;
    validate_fragment_routes(ordinal, draw)
}
fn validate_fragment_routes(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    reject(
        ordinal,
        draw.clipping.is_some() || draw.hatch_rows.is_some(),
        "clipping/hatch",
    )?;
    reject(
        ordinal,
        draw.annotation || draw.text_logic_xor || draw.text_shadow_bias,
        "annotation/logic",
    )?;
    reject(
        ordinal,
        draw.depth_pass != crate::depth_prepass::RenderDepthPass::Shaded,
        "depth prepass",
    )
}
fn validate_texture_routes(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    validate_alternate_shader_routes(ordinal, draw)?;
    reject(
        ordinal,
        draw.surface_texture.is_some() || draw.surface_texture_binding_only,
        "surface sampler",
    )?;
    reject(
        ordinal,
        draw.surface_texture_unavailable.is_some() || draw.textured_style.is_some(),
        "texture route",
    )?;
    reject(
        ordinal,
        draw.mesh_edges.is_some() || draw.polygon_face_lines || draw.polygon_points,
        "polygon edge/point raster",
    )
}
fn validate_alternate_shader_routes(
    ordinal: usize,
    draw: &PrimitiveDraw,
) -> Result<(), LitOnlyFailure> {
    reject(
        ordinal,
        draw.unlit_back_color.is_some() || draw.surface_cutoff.is_some(),
        "alternate surface alpha",
    )?;
    reject(
        ordinal,
        draw.compatibility_color_target || draw.fixed_function_lighting,
        "compatibility shader",
    )?;
    reject(
        ordinal,
        draw.fixed_grid_point || draw.fixed_grid_point_sprite || draw.fixed_grid_point_unit12,
        "alternate point raster",
    )?;
    Ok(())
}
fn validate_point(ordinal: usize, draw: &PrimitiveDraw) -> Result<(), LitOnlyFailure> {
    let Some(style) = &draw.point_style else {
        return Err(refusal(ordinal, "point style", "None".into()));
    };
    reject(
        ordinal,
        style.color_policy != crate::point_style::PointColorPolicy::AlphaSprite,
        "non-AlphaSprite policy",
    )?;
    reject(
        ordinal,
        style.fixed_material_cutoff.is_some(),
        "fixed cutoff",
    )?;
    validate_binary_sprite(ordinal, style)
}
fn validate_binary_sprite(ordinal: usize, style: &crate::PointStyle) -> Result<(), LitOnlyFailure> {
    // marker.glsl directly indexes the SSBO: nearest only; point.frag discards zero alpha.
    if let Some(pixel) = style
        .image
        .pixels()
        .iter()
        .find(|p| ![0, 1f32.to_bits()].contains(&p[3].to_bits()))
    {
        return Err(refusal(
            ordinal,
            "binary sprite mask",
            format!("alpha bits={:#x}", pixel[3].to_bits()),
        ));
    }
    Ok(())
}
pub(super) fn reject(
    ordinal: usize,
    active: bool,
    route: &'static str,
) -> Result<(), LitOnlyFailure> {
    if active {
        return Err(refusal(ordinal, route, "active=true".into()));
    }
    Ok(())
}
fn refusal(ordinal: usize, route: &'static str, actual: String) -> LitOnlyFailure {
    LitOnlyFailure::Draw {
        ordinal,
        route,
        actual,
    }
}
