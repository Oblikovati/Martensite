//! Keep label geometry and its GPU stream identities independent of view transforms.
use crate::{
    GlyphLayout, PrimitiveDraw, RasterImage, RenderError, RenderVertex, TexturedStyle,
    draw::DrawStream,
};
use martensite_tkernel::standard::SharedState;
use martensite_tkservice::font::FontRect;
use std::collections::VecDeque;

const COLOR_VARIANTS: usize = 2;
// Graphic3dShaderManager::getStdProgramFont hard-codes this cutoff.
const FONT_ALPHA_CUTOFF: f32 = 0.285;

pub(crate) struct RetainedGlyphLayout {
    positions: Vec<[f32; 2]>,
    uv: DrawStream<[f32; 2]>,
    indices: DrawStream<u32>,
    pub(super) atlas: RasterImage,
    pub(super) formatter_bounds: FontRect,
    pub(super) point_size: u32,
    geometry: SharedState<VecDeque<GlyphVariant>>,
}

struct GlyphVariant {
    color: [u32; 4],
    vertices: DrawStream<RenderVertex>,
}

impl RetainedGlyphLayout {
    pub(crate) fn new(layout: GlyphLayout) -> Self {
        Self {
            positions: layout.positions,
            uv: layout.uv.into(),
            indices: layout.indices.into(),
            atlas: layout.atlas,
            formatter_bounds: layout.formatter_bounds,
            point_size: layout.point_size,
            geometry: Default::default(),
        }
    }

    pub(super) fn to_public_layout(&self) -> GlyphLayout {
        GlyphLayout {
            positions: self.positions.clone(),
            uv: self.uv.to_vec(),
            indices: self.indices.to_vec(),
            atlas: self.atlas.clone(),
            formatter_bounds: self.formatter_bounds,
            point_size: self.point_size,
        }
    }

    pub(crate) fn reserved_geometry_bytes(vertex_count: usize) -> usize {
        COLOR_VARIANTS * (size_of::<GlyphVariant>() + size_of::<RenderVertex>() * vertex_count)
    }

    pub(super) fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub(super) fn draw(
        &self,
        rgba: [f32; 4],
        clip: [f32; 16],
    ) -> Result<PrimitiveDraw, RenderError> {
        let color = rgba.map(f32::to_bits);
        if let Some(vertices) = lookup(&self.geometry.read(), color) {
            return self.placed_glyphs(vertices, clip);
        }
        let mut variants = self.geometry.write();
        if let Some(vertices) = lookup(&variants, color) {
            return self.placed_glyphs(vertices, clip);
        }
        let mut draw = glyph_draw(
            &self.positions,
            self.indices.clone(),
            self.atlas.clone(),
            self.uv.clone(),
            rgba,
            clip,
        )?;
        if let Some(style) = &mut draw.textured_style {
            style.retained_geometry = true;
        }
        if variants.len() == COLOR_VARIANTS {
            variants.pop_front();
        } else if variants.capacity() == 0 {
            variants.reserve_exact(COLOR_VARIANTS);
        }
        variants.push_back(GlyphVariant {
            color,
            vertices: draw.vertices.clone(),
        });
        Ok(draw)
    }

    fn placed_glyphs(
        &self,
        vertices: DrawStream<RenderVertex>,
        clip: [f32; 16],
    ) -> Result<PrimitiveDraw, RenderError> {
        if clip.iter().any(|value| !value.is_finite()) {
            crate::scene::validate_components(&vertices, &clip)?;
        }
        let mut draw = PrimitiveDraw::from_validated_streams(
            vertices,
            self.indices.clone(),
            clip,
            None,
            false,
        );
        let mut style = TexturedStyle::from_validated_uv(
            self.atlas.clone(),
            self.uv.clone(),
            FONT_ALPHA_CUTOFF,
        );
        style.retained_geometry = true;
        draw.textured_style = Some(style);
        Ok(draw)
    }
}

fn lookup(variants: &VecDeque<GlyphVariant>, color: [u32; 4]) -> Option<DrawStream<RenderVertex>> {
    variants
        .iter()
        .find(|variant| variant.color == color)
        .map(|variant| variant.vertices.clone())
}

pub(super) fn glyph_draw(
    corners: &[[f32; 2]],
    indices: DrawStream<u32>,
    atlas: RasterImage,
    uv: DrawStream<[f32; 2]>,
    rgba: [f32; 4],
    clip: [f32; 16],
) -> Result<PrimitiveDraw, RenderError> {
    let vertices = corners
        .iter()
        .map(|p| RenderVertex::new([p[0], p[1], 0.], rgba))
        .collect();
    let style = TexturedStyle::with_uv_stream(atlas, uv, FONT_ALPHA_CUTOFF)?;
    PrimitiveDraw::textured_with_indices(vertices, indices, clip, style)
}
