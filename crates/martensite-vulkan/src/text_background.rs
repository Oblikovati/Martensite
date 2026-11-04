//! Native GlText::drawRect keeps formatter winding, order and depth writes.
use crate::{GlyphLayout, PrimitiveDraw, RenderError, RenderVertex};
use martensite_tkservice::{aspect::AspectTypeOfStyleText, graphic3d::*};

/// Preserve native formatter winding and ordered depth; `subtitle_rectangle(aspect, layout, clip)` returns the decoration pass.
pub(crate) fn subtitle_rectangle(
    aspect: &dyn Graphic3dAspectsSource,
    layout: &GlyphLayout,
    clip: [f32; 16],
) -> Result<PrimitiveDraw, RenderError> {
    let vertices = subtitle_vertices(layout, aspect);
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 1, 3, 2], clip)?;
    draw.surface_blend = false;
    draw.text_background_blend = aspect.alpha_mode() == Graphic3dAlphaMode::MaskBlend;
    draw.annotation = aspect.text_style() == AspectTypeOfStyleText::Annotation;
    draw.text_shadow_bias = !draw.annotation;
    let (mode, factor, units) = aspect.polygon_offsets();
    draw = draw.with_polygon_offset(crate::RenderPolygonOffset::new(mode, factor, units)?);
    Ok(draw)
}
/// GlText::render's dimension pass: `drawRect` with colour writes and the depth test off,
/// only writing the label rectangle into the stencil buffer (the stencil program brackets it).
/// Example: `dimension_rectangle(aspect, layout, clip)?.stencil_rect`.
pub(crate) fn dimension_rectangle(
    aspect: &dyn Graphic3dAspectsSource,
    layout: &GlyphLayout,
    clip: [f32; 16],
) -> Result<PrimitiveDraw, RenderError> {
    let vertices = subtitle_vertices(layout, aspect);
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 1, 3, 2], clip)?;
    draw.surface_blend = false;
    draw.stencil_rect = true;
    draw.depth_pass = crate::depth_prepass::RenderDepthPass::DepthOnly;
    Ok(draw)
}
fn subtitle_vertices(
    layout: &GlyphLayout,
    aspect: &dyn Graphic3dAspectsSource,
) -> Vec<RenderVertex> {
    let rectangle = layout.formatter_bounds;
    let rgba = crate::text_drawing::text_vertex_rgba(aspect.edge_color_rgba());
    // HTA_RIGHT has Right < Left reference; normalizing would reverse native culling behavior.
    [
        [rectangle.right, rectangle.bottom],
        [rectangle.right, rectangle.top],
        [rectangle.left, rectangle.bottom],
        [rectangle.left, rectangle.top],
    ]
    .map(|point| RenderVertex::new([point[0], point[1], 0.], rgba))
    .to_vec()
}
#[cfg(test)]
#[path = "text_background_tests.rs"]
mod tests;
