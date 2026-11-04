//! Ordered depth and raster state follows submitted topology through private geometry lowering.
use super::frame::DrawPresentation;
use crate::{PrimitiveDraw, PrimitiveTopology};

pub(super) fn from_semantic_draw(draw: &PrimitiveDraw) -> DrawPresentation {
    DrawPresentation {
        annotation: draw.annotation,
        stencil_rect: draw.stencil_rect,
        text_shadow_bias: draw.text_shadow_bias,
        text_logic_xor: draw.text_logic_xor,
        encoded_surface: super::pipeline_variants::encoded_surface_attachment(draw),
        polygon_offset: draw.polygon_offset,
        // Native font glyph triangles retain ordinary Fill slope/units; GlText.
        polygon: draw.topology() == PrimitiveTopology::Triangles,
        polygon_edge: draw.polygon_edge_style.is_some(),
        surface_blend: draw.surface_blend,
        capping: draw.capping_pass,
        face_culling: draw.face_culling,
        mirrored: draw.mirrored,
    }
}

#[cfg(test)]
#[path = "frame_draw_presentation_tests.rs"]
mod tests;
