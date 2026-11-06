//! The view's 2D image background: `GlView::SetBackgroundImage` keeps a
//! textured, unlit, double-sided solid aspect and `GlView::drawBackground` draws
//! `GlBackgroundArray`'s texture quad (`createTextureArray`) with blending off, after the
//! gradient. The aspect outlives a later null image or gradient, so the image is drawn whenever
//! the background is a gradient or a texture and an image was ever set (native behaviour).
use crate::background_chart::{BackgroundChart, BackgroundGeometry};
use crate::surface_texture::{SurfaceSamplerPolicy, SurfaceTextureImage, SurfaceTextureStyle};
use crate::surface_texture_uv::transform_surface_uv;
use crate::{PrimitiveDraw, RenderError, RenderLayer, RenderVertex};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::AspectFillMethod, graphic3d::*};
use std::sync::Arc;

/// `THE_FS_QUAD_TRIS` of createTextureArray.
const QUAD_TRIANGLES: [u32; 6] = [0, 1, 2, 1, 3, 2];
/// createTextureArray's `aCoef`: -1 tiles from the top-left corner of the view.
const TILING_SIGN: f32 = -1.;

/// One corner of the background quad: a position in view pixels about the view centre, and its
/// texture coordinate.
type QuadCorner = ([f32; 2], [f32; 2]);

/// drawBackground's gradient condition: no image is mapped, or it is centered or not drawn.
/// Example: `if gradient_under_image(view) { /* draw the gradient */ }`.
pub(crate) fn gradient_under_image(view: &dyn Graphic3dCView) -> bool {
    view.background_image().is_null()
        || matches!(
            view.background_image_style(),
            AspectFillMethod::Centered | AspectFillMethod::None
        )
}

/// The image background layer of a `extent` viewport, or none without an image, with
/// `AspectFM_NONE` (`IsDefined`) or when the image has no pixels. Example:
/// `image_background_layer(view, [203, 151])?`.
pub(crate) fn image_background_layer(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
) -> Result<Option<RenderLayer>, RenderError> {
    let texture = view.background_image();
    let method = view.background_image_style();
    if texture.is_null() || method == AspectFillMethod::None {
        return Ok(None);
    }
    let images = crate::retained_texture_images::view_texture_images(view);
    let Some(image) = images.snapshot(&texture)? else {
        return Ok(None);
    };
    let parameters = texture.get_params();
    let sampler = images.samplers.policy(&image, &parameters)?;
    let camera = crate::view_render_camera::owned_render_camera(view);
    let chart = BackgroundChart::new(&camera, extent, BackgroundGeometry::Texture)?;
    let texture_size = image.extent.map(|size| size as f32);
    let corners = texture_quad(method, chart.view_size, texture_size);
    let draw = textured_quad(corners, chart.world_view(), (image, sampler), &parameters)?;
    // drawBackground disables depth testing and writes for every background.
    Ok(Some(RenderLayer::new(vec![draw], false, false, false)))
}

/// createTextureArray: the quad spans the view, or the texture when centered; a tiled texture
/// repeats once per texture size, from the top-left corner.
fn texture_quad(method: AspectFillMethod, view: [f32; 2], texture: [f32; 2]) -> [QuadCorner; 4] {
    let mut offset = [0.5 * view[0], 0.5 * view[1]];
    let mut range = [1_f32, 1.];
    match method {
        AspectFillMethod::Centered => offset = [0.5 * texture[0], 0.5 * texture[1]],
        AspectFillMethod::Tiled => range = [view[0] / texture[0], view[1] / texture[1]],
        _ => {}
    }
    let [x, y] = offset;
    [
        ([x, -TILING_SIGN * y], [range[0], 0.]),
        ([x, TILING_SIGN * y], [range[0], TILING_SIGN * range[1]]),
        ([-x, -TILING_SIGN * y], [0., 0.]),
        ([-x, TILING_SIGN * y], [0., TILING_SIGN * range[1]]),
    ]
}

/// The unlit, double-sided textured draw of the quad; the white vertex colour leaves the texel
/// unchanged (the unlit program samples the base colour alone).
fn textured_quad(
    corners: [QuadCorner; 4],
    world_view: [f32; 16],
    texture: (Arc<SurfaceTextureImage>, SurfaceSamplerPolicy),
    parameters: &Handle<Graphic3dTextureParams>,
) -> Result<PrimitiveDraw, RenderError> {
    let vertices = corners
        .iter()
        .map(|([x, y], _)| RenderVertex::new([*x, *y, 0.], [1.; 4]))
        .collect();
    let mut draw = PrimitiveDraw::new(vertices, QUAD_TRIANGLES.to_vec(), world_view)?;
    draw.surface_texture = Some(quad_texture_style(&corners, texture, parameters));
    draw.face_culling = crate::RenderFaceCulling::None;
    Ok(draw)
}
/// The texture style of the quad: its coordinates through the native texture transform.
fn quad_texture_style(
    corners: &[QuadCorner; 4],
    (image, sampler): (Arc<SurfaceTextureImage>, SurfaceSamplerPolicy),
    parameters: &Handle<Graphic3dTextureParams>,
) -> SurfaceTextureStyle {
    let uv = corners
        .iter()
        .map(|(_, [s, t])| transform_surface_uv([*s, *t, 0., 1.], parameters, image.top_down))
        .collect();
    SurfaceTextureStyle {
        image,
        pending_texgen: None,
        fixed_environment: false,
        sampler,
        uv,
        back_vertex: false,
        vertex_override: false,
        modulate: parameters.is_modulate(),
        native_gen_mode: parameters.gen_mode(),
        material_maps: None,
        sphere_map: None,
    }
}

#[cfg(test)]
#[path = "image_background_tests.rs"]
mod tests;
