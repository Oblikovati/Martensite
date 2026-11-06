//! Rendering resolution scale (`Graphic3dRenderingParams::RenderResolutionScale`).
//!
//! A redraw renders the scene into a framebuffer of
//! `int(scale * size + 0.5f)` pixels, without MSAA when the width changes, while
//! `GlContext::SetResolution` scales the resolution (font sizes and line widths) and maps
//! the viewport back for transform persistence (`VirtualViewport`). `blitBuffers` then draws the
//! colour texture over the target with a nearest filter for equal sizes and a bilinear one
//! otherwise, clamped to the edge. The scaled scene here is an ordinary frame; its capture becomes
//! the sRGB texture of one full-target quad, so the GPU filters it in linear space as GL does.
use crate::{
    FrameCapture, PrimitiveDraw, RenderError, RenderFrame, RenderLayer, RenderVertex,
    error::rejected,
    surface_texture::{
        SurfaceImageEncoding, SurfaceSamplerPolicy, SurfaceTextureImage, SurfaceTextureStyle,
    },
};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// The view's scale when it differs from 1, the only case native renders off the target.
/// Example: `active_render_scale(&params) == Some(2.)`.
pub(crate) fn active_render_scale(params: &Graphic3dRenderingParams) -> Option<f32> {
    (params.render_resolution_scale != 1.).then_some(params.render_resolution_scale)
}

/// The scaled framebuffer extent, `int(scale * size + 0.5f)` in float as `GlView::Redraw`.
/// Example: `scaled_extent([64, 64], 0.75)? == [48, 48]`.
pub(crate) fn scaled_extent(extent: [u32; 2], scale: f32) -> Result<[u32; 2], RenderError> {
    let scaled = extent.map(|size| (scale * size as f32 + 0.5) as i32);
    if !scale.is_finite() || scaled.iter().any(|&size| size < 1) {
        return Err(rejected(
            (extent, scale),
            "a resolution scale leaving at least one pixel",
        ));
    }
    Ok(scaled.map(|size| size as u32))
}

/// The viewport transform persistence sees: `GlContext::VirtualViewport`, the raster
/// extent times `1.0f / scale`, truncated. Example: `persistence_extent([32, 32], &params)`.
pub(crate) fn persistence_extent(raster: [u32; 2], params: &Graphic3dRenderingParams) -> [u32; 2] {
    let Some(scale) = active_render_scale(params) else {
        return raster;
    };
    let inverse = 1. / scale;
    raster.map(|size| (size as f32 * inverse) as i32 as u32)
}

/// The parameters the scaled scene renders with: the resolution of
/// `SetResolution(Resolution, ResolutionRatio, scale)` and no MSAA when the width changes.
/// Example: `scaled_rendering(&params, [64, 64], [128, 128]).resolution == 144` at scale 2.
pub(crate) fn scaled_rendering(
    params: &Graphic3dRenderingParams,
    extent: [u32; 2],
    raster: [u32; 2],
) -> Graphic3dRenderingParams {
    let mut scaled = params.clone();
    let Some(scale) = active_render_scale(params) else {
        return scaled;
    };
    scaled.resolution = (scale * params.resolution as f32 + 0.5) as u32;
    if extent[0] != raster[0] {
        scaled.nb_msaa_samples = 0;
    }
    scaled
}

/// The zoomable-text scale `GlText::setupMatrix` applies under a render scale.
/// Example: `zoomable_text_scale(&params) == 0.5` at scale 2.
pub(crate) fn zoomable_text_scale(params: &Graphic3dRenderingParams) -> f64 {
    active_render_scale(params).map_or(1., |scale| f64::from(1. / scale))
}

/// The frame that draws a scaled capture over a target of `extent`, as `blitBuffers` does.
/// Example: `blit_frame(&capture, [64, 64])?`.
pub(crate) fn blit_frame(
    capture: &FrameCapture,
    extent: [u32; 2],
) -> Result<RenderFrame, RenderError> {
    let draw = blit_quad(capture, blit_filter(capture, extent))?;
    let layer = RenderLayer::new(vec![draw], false, false, true);
    RenderFrame::with_layers(extent[0], extent[1], [0., 0., 0., 1.], vec![layer])
}

fn blit_filter(capture: &FrameCapture, extent: [u32; 2]) -> Graphic3dTypeOfTextureFilter {
    if [capture.width, capture.height] == extent {
        return Graphic3dTypeOfTextureFilter::Nearest;
    }
    Graphic3dTypeOfTextureFilter::Bilinear
}

/// The strip of `GlView::initBlitQuad` as two triangles, with the texture bottom-up as in the
/// native framebuffer: the same plane and the same texture coordinate give the same filter weights,
/// where a mirrored coordinate (1 - v) rounds them apart.
fn blit_quad(
    capture: &FrameCapture,
    filter: Graphic3dTypeOfTextureFilter,
) -> Result<PrimitiveDraw, RenderError> {
    let corners = [[1., -1.], [1., 1.], [-1., -1.], [-1., 1.]];
    let vertices = corners
        .iter()
        .map(|[x, y]| RenderVertex::new([*x, *y, 0.], [1.; 4]))
        .collect();
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 2, 1, 3], crate::identity_clip())?;
    draw.surface_texture = Some(SurfaceTextureStyle {
        sphere_map: None,
        image: Arc::new(SurfaceTextureImage {
            bytes: Arc::new(bottom_up_rows(capture)),
            top_down: false,
            ..capture_image(capture)
        }),
        pending_texgen: None,
        fixed_environment: false,
        sampler: blit_sampler(filter),
        uv: corners
            .map(|[x, y]| [(x + 1.) / 2., (y + 1.) / 2., 0., 1.])
            .to_vec(),
        back_vertex: false,
        vertex_override: false,
        modulate: true,
        native_gen_mode: Graphic3dTypeOfTextureMode::Manual,
        material_maps: None,
    });
    Ok(draw)
}

/// The capture rows in the native framebuffer's order, bottom row first.
fn bottom_up_rows(capture: &FrameCapture) -> Vec<u8> {
    capture
        .rgba
        .chunks_exact(4 * capture.width as usize)
        .rev()
        .flatten()
        .copied()
        .collect()
}

/// An uncached (empty id) single-level sRGB image of the capture rows.
pub(crate) fn capture_image(capture: &FrameCapture) -> SurfaceTextureImage {
    SurfaceTextureImage {
        lifetime: Arc::default(),
        id: vec![],
        revision: 0,
        extent: [capture.width, capture.height],
        depth: None,
        encoding: SurfaceImageEncoding::Srgb8,
        alpha_only: false,
        has_alpha: false,
        bytes: Arc::new(capture.rgba.clone()),
        top_down: true,
        mipmaps: false,
        supplied_levels: vec![],
    }
}

/// `GlFrameBuffer` colour textures: no repeat (clamp to edge), one level.
fn blit_sampler(filter: Graphic3dTypeOfTextureFilter) -> SurfaceSamplerPolicy {
    SurfaceSamplerPolicy {
        repeat: false,
        filter,
        anisotropy: Graphic3dLevelOfTextureAnisotropy::Off,
        first_level: 0,
        last_level: 0,
    }
}

/// Resample the scaled depth like the blit's `gl_FragDepth` read: nearest for equal sizes,
/// otherwise bilinear between texel centres, clamped to the edge (computed on the CPU in f32).
/// Example: `blit_depth(&scaled, [64, 64])`.
pub(crate) fn blit_depth(scaled: &FrameCapture, extent: [u32; 2]) -> Vec<f32> {
    let [width, height] = extent;
    (0..height)
        .flat_map(|row| (0..width).map(move |col| (col, row)))
        .map(|(col, row)| depth_sample(scaled, extent, [col, row]))
        .collect()
}

fn depth_sample(scaled: &FrameCapture, extent: [u32; 2], [col, row]: [u32; 2]) -> f32 {
    let (x0, x1, fx) = texel_pair(col, extent[0], scaled.width);
    let (y0, y1, fy) = texel_pair(row, extent[1], scaled.height);
    let at = |x: usize, y: usize| scaled.depth[y * scaled.width as usize + x];
    let top = at(x0, y0) * (1. - fx) + at(x1, y0) * fx;
    let bottom = at(x0, y1) * (1. - fx) + at(x1, y1) * fx;
    top * (1. - fy) + bottom * fy
}

/// The two clamped source texels and the weight of the second one for a target pixel centre.
fn texel_pair(target: u32, target_size: u32, source_size: u32) -> (usize, usize, f32) {
    if target_size == source_size {
        return (target as usize, target as usize, 0.);
    }
    let coordinate = (target as f32 + 0.5) / target_size as f32 * source_size as f32 - 0.5;
    let base = coordinate.floor();
    let last = source_size as i64 - 1;
    let clamp = |texel: i64| texel.clamp(0, last) as usize;
    (
        clamp(base as i64),
        clamp(base as i64 + 1),
        coordinate - base,
    )
}

#[cfg(test)]
#[path = "render_scale_tests.rs"]
mod tests;
