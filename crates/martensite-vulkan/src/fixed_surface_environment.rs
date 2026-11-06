//! Native unlit FFP textures retain GlSampler component and encoded-target semantics.
use crate::PrimitiveDraw;
pub(crate) fn uses_fixed_surface_environment(draw: &PrimitiveDraw) -> bool {
    (draw.lighting.is_none()
        || draw
            .lighting
            .as_ref()
            .is_some_and(|state| state.is_fixed_lighting()))
        && (draw
            .surface_texture
            .as_ref()
            .is_some_and(|style| style.fixed_environment)
            || draw
                .surface_texture_unavailable
                .as_ref()
                .is_some_and(|style| style.fixed_environment))
}
pub(crate) fn fixed_surface_environment_flags(draw: &PrimitiveDraw) -> u32 {
    if draw.surface_texture_binding_only || !uses_fixed_surface_environment(draw) {
        return 0;
    }
    // A texture that failed to load binds nothing: the texture environment does not apply.
    let flags = draw
        .surface_texture
        .as_ref()
        .map_or(0, fixed_loaded_texture_flags);
    flags + 2048 * u32::from(fixed_uniform_alpha(draw).is_some())
}
pub(crate) fn fixed_uniform_alpha(draw: &PrimitiveDraw) -> Option<f32> {
    if draw.lighting.is_some() {
        return None;
    }
    let vertex_override = fixed_vertex_override(draw)?;
    if !uses_fixed_surface_environment(draw) || (vertex_override && !draw.is_polygon_edge_pass()) {
        return None;
    }
    // Native FFP glColor4fv supplies current alpha directly when no color array is enabled.
    draw.vertices.first().map(|vertex| vertex.color[3])
}
fn fixed_vertex_override(draw: &PrimitiveDraw) -> Option<bool> {
    draw.surface_texture
        .as_ref()
        .map(|style| style.vertex_override)
        .or_else(|| {
            draw.surface_texture_unavailable
                .as_ref()
                .map(|style| style.vertex_override)
        })
}
/// GL_REPLACE of an unlit Sprite-mode texture (GlSampler::applyGlobalTextureParams forces
/// it for Graphic3dTOTM_SPRITE): the texel colour, and the texel alpha when the texture format
/// has one, else the current alpha. The draw modulates by an opaque white (or white with its own
/// alpha) colour instead, which is exact; alpha-only textures keep the alpha-replace flag.
/// Example: `apply_sprite_replace(&mut draw)`.
pub(crate) fn apply_sprite_replace(draw: &mut PrimitiveDraw) {
    let Some(style) = draw.surface_texture.as_mut() else {
        return;
    };
    let sprite = style.native_gen_mode
        == martensite_tkservice::graphic3d::Graphic3dTypeOfTextureMode::Sprite;
    if !sprite || !style.fixed_environment || style.image.alpha_only || draw.lighting.is_some() {
        return;
    }
    style.modulate = true;
    let texel_alpha = style.image.has_alpha;
    for vertex in &mut draw.vertices {
        let alpha = if texel_alpha { 1. } else { vertex.color[3] };
        vertex.color = [1., 1., 1., alpha];
    }
}
fn fixed_loaded_texture_flags(style: &crate::surface_texture::SurfaceTextureStyle) -> u32 {
    fixed_environment_flag(style.modulate, style.image.alpha_only)
        + 1024 * u32::from(style.image.alpha_only)
}
fn fixed_environment_flag(modulate: bool, alpha_only: bool) -> u32 {
    // Global texture parameters choose MODULATE, ALPHA REPLACE, or RGBA DECAL.
    if modulate {
        return 32;
    }
    if alpha_only {
        return 128;
    }
    64
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface_texture::{SurfaceSamplerPolicy, SurfaceTextureImage, SurfaceTextureStyle};
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::{
        graphic3d::*,
        image::{ImageFormat, ImagePixMap},
    };
    fn environment_snapshot(
        format: ImageFormat,
        pixels: Vec<u8>,
    ) -> std::sync::Arc<SurfaceTextureImage> {
        let image = Handle::new(ImagePixMap::default());
        assert!(image.init_wrapper(format, Some(pixels.clone()), 1, 1, pixels.len()));
        let texture = Handle::new(Graphic3dTexture2D::from_pix_map(image));
        crate::retained_texture_images::RetainedTextureImages::default()
            .snapshot(&texture.cast())
            .unwrap()
            .unwrap()
    }
    fn environment_image(format: ImageFormat, pixels: Vec<u8>) -> SurfaceTextureStyle {
        SurfaceTextureStyle {
            sphere_map: None,
            image: environment_snapshot(format, pixels),
            pending_texgen: None,
            fixed_environment: true,
            sampler: SurfaceSamplerPolicy::read(&Graphic3dTextureParams::default()),
            uv: vec![],
            back_vertex: false,
            vertex_override: false,
            modulate: true,
            native_gen_mode: Graphic3dTypeOfTextureMode::Manual,
            material_maps: None,
        }
    }
    #[test]
    fn completed_texgen_keeps_fixed_transfer_and_original_component_flags() {
        let mut draw = crate::PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        draw.surface_texture = Some(environment_image(ImageFormat::Rgba, vec![255; 4]));
        assert_eq!(fixed_surface_environment_flags(&draw), 32);
        assert!(uses_fixed_surface_environment(&draw));
        draw.surface_texture = Some(environment_image(ImageFormat::Alpha, vec![128]));
        assert_eq!(fixed_surface_environment_flags(&draw), 1056);
        draw.surface_texture.as_mut().unwrap().modulate = false;
        assert_eq!(fixed_surface_environment_flags(&draw), 1152);
        draw.surface_texture = Some(environment_image(ImageFormat::Gray, vec![128]));
        assert_eq!(fixed_surface_environment_flags(&draw), 32);
        draw.surface_texture.as_mut().unwrap().fixed_environment = false;
        assert_eq!(fixed_surface_environment_flags(&draw), 0);
        assert!(!uses_fixed_surface_environment(&draw));
    }
    #[test]
    fn fixed_current_alpha_stays_uniform_until_a_real_color_array_is_enabled() {
        let vertices = vec![crate::RenderVertex::new([0.; 3], [1., 1., 1., 0.5]); 3];
        let mut draw =
            crate::PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
        draw.surface_texture = Some(environment_image(ImageFormat::Rgba, vec![255; 4]));
        assert_eq!(fixed_uniform_alpha(&draw), Some(0.5));
        assert_eq!(fixed_surface_environment_flags(&draw), 2080);
        draw.surface_texture.as_mut().unwrap().vertex_override = true;
        assert_eq!(fixed_uniform_alpha(&draw), None);
        draw.polygon_edge_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
        assert_eq!(fixed_uniform_alpha(&draw), Some(0.5));
        draw.surface_texture.as_mut().unwrap().fixed_environment = false;
        assert_eq!(fixed_uniform_alpha(&draw), None);
    }
    fn sprite_draw(format: ImageFormat, pixels: Vec<u8>) -> crate::PrimitiveDraw {
        let vertices = vec![crate::RenderVertex::new([0.; 3], [0.2, 0.4, 0.6, 0.5]); 3];
        let mut draw =
            crate::PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
        let mut style = environment_image(format, pixels);
        (style.modulate, style.native_gen_mode) = (false, Graphic3dTypeOfTextureMode::Sprite);
        draw.surface_texture = Some(style);
        draw
    }
    #[test]
    fn unlit_sprites_replace_with_the_texel_and_its_alpha_when_the_format_has_one() {
        let mut rgba = sprite_draw(ImageFormat::Rgba, vec![200, 100, 50, 128]);
        apply_sprite_replace(&mut rgba);
        assert!(rgba.surface_texture.as_ref().unwrap().modulate);
        assert!(rgba.vertices.iter().all(|v| v.color == [1., 1., 1., 1.]));
        let mut rgb = sprite_draw(ImageFormat::Rgb, vec![200, 100, 50]);
        apply_sprite_replace(&mut rgb);
        assert!(rgb.vertices.iter().all(|v| v.color == [1., 1., 1., 0.5]));
        let mut alpha = sprite_draw(ImageFormat::Alpha, vec![128]);
        apply_sprite_replace(&mut alpha);
        assert_eq!(fixed_surface_environment_flags(&alpha), 1152 + 2048);
        assert_eq!(alpha.vertices[0].color, [0.2, 0.4, 0.6, 0.5]);
    }
    #[test]
    fn alpha_replacement_and_rgba_decal_remain_distinct_from_modulation() {
        assert_eq!(fixed_environment_flag(true, false), 32);
        assert_eq!(fixed_environment_flag(true, true), 32);
        assert_eq!(fixed_environment_flag(false, true), 128);
        assert_eq!(fixed_environment_flag(false, false), 64);
    }
    #[test]
    fn incomplete_fixed_polygon_texture_leaves_texturing_off_in_both_pipelines() {
        // GL 2.1 section 3.8.10: an incomplete texture disables fixed-function texturing, and
        // BindTextures binds nothing for a texture whose load failed (witnesses
        // martensite_cubemap_incomplete: white; martensite_unknown_texture_format: white).
        let mut draw = crate::PrimitiveDraw::new(vec![], vec![], crate::identity_clip()).unwrap();
        draw.surface_texture_unavailable =
            Some(crate::surface_texture::SurfaceTextureUnavailable {
                resource_id: b"Martensite-incomplete-fixed-polygon".to_vec(),
                fixed_environment: false,
                modulate: true,
                vertex_override: false,
                back_vertex: false,
            });
        assert_eq!(fixed_surface_environment_flags(&draw), 0);
        draw.surface_texture_unavailable
            .as_mut()
            .unwrap()
            .fixed_environment = true;
        draw.polygon_edge_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
        assert_eq!(fixed_surface_environment_flags(&draw), 0);
        draw.surface_texture_binding_only = true;
        assert_eq!(fixed_surface_environment_flags(&draw), 0);
    }
}
