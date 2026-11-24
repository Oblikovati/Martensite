//! Textured triangles keep UVs separate so ordinary geometry retains its compact vertex layout.
use crate::{
    PrimitiveDraw, RasterImage, RenderError, RenderVertex, draw::DrawStream, error::rejected,
};
/// Bilinear alpha atlas and one UV per vertex; `TexturedStyle::new(image, uv, 0.1)?`.
#[derive(Clone)]
pub struct TexturedStyle {
    pub(crate) image: RasterImage,
    pub(crate) uv: DrawStream<[f32; 2]>,
    pub(crate) alpha_cutoff: f32,
    pub(crate) blend: bool,
    pub(crate) retained_geometry: bool,
}
impl PartialEq for TexturedStyle {
    fn eq(&self, other: &Self) -> bool {
        self.image == other.image
            && self.uv == other.uv
            && self.alpha_cutoff == other.alpha_cutoff
            && self.blend == other.blend
    }
}

impl std::fmt::Debug for TexturedStyle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TexturedStyle")
            .field("image", &self.image)
            .field("uv", &self.uv)
            .field("alpha_cutoff", &self.alpha_cutoff)
            .field("blend", &self.blend)
            .finish()
    }
}
impl TexturedStyle {
    /// Validate UVs and the native alpha-mask threshold; `TexturedStyle::new(image, uv, 0.1)?`.
    pub fn new(
        image: RasterImage,
        uv: Vec<[f32; 2]>,
        alpha_cutoff: f32,
    ) -> Result<Self, RenderError> {
        Self::with_uv_stream(image, uv.into(), alpha_cutoff)
    }

    pub(crate) fn with_uv_stream(
        image: RasterImage,
        uv: DrawStream<[f32; 2]>,
        alpha_cutoff: f32,
    ) -> Result<Self, RenderError> {
        if !alpha_cutoff.is_finite() || !(0. ..=1.).contains(&alpha_cutoff) {
            return Err(rejected(alpha_cutoff, "an alpha cutoff in 0..=1"));
        }
        if uv.iter().flatten().any(|v| !v.is_finite()) {
            return Err(rejected(uv, "finite texture coordinates"));
        }
        Ok(Self::from_validated_uv(image, uv, alpha_cutoff))
    }

    pub(crate) fn from_validated_uv(
        image: RasterImage,
        uv: DrawStream<[f32; 2]>,
        alpha_cutoff: f32,
    ) -> Self {
        Self {
            image,
            uv,
            alpha_cutoff,
            blend: true,
            retained_geometry: false,
        }
    }
    /// Select native coverage blending; `style.with_alpha_blending(false)` writes unblended glyphs.
    pub fn with_alpha_blending(mut self, enabled: bool) -> Self {
        self.blend = enabled;
        self
    }
    /// Inspect the atlas; `style.image()`.
    pub fn image(&self) -> &RasterImage {
        &self.image
    }
    /// Inspect vertex UV coordinates; `style.uv()`.
    pub fn uv(&self) -> &[[f32; 2]] {
        &self.uv
    }
}
impl PrimitiveDraw {
    /// Submit alpha-blended textured triangles; `PrimitiveDraw::textured(vertices, indices, clip, style)?`.
    pub fn textured(
        vertices: Vec<RenderVertex>,
        indices: Vec<u32>,
        clip: [f32; 16],
        style: TexturedStyle,
    ) -> Result<Self, RenderError> {
        Self::textured_with_indices(vertices, indices.into(), clip, style)
    }

    pub(crate) fn textured_with_indices(
        vertices: Vec<RenderVertex>,
        indices: DrawStream<u32>,
        clip: [f32; 16],
        style: TexturedStyle,
    ) -> Result<Self, RenderError> {
        if vertices.len() != style.uv.len() {
            return Err(rejected(
                (vertices.len(), style.uv.len()),
                "one UV coordinate per vertex",
            ));
        }
        let mut draw = Self::validated(vertices, indices, clip, None)?;
        draw.textured_style = Some(style);
        // GlText preserves depth writes/order even when color or glyph coverage blends.
        draw.surface_blend = false;
        Ok(draw)
    }
    /// Inspect a textured submission; `draw.textured_style()`.
    pub fn textured_style(&self) -> Option<&TexturedStyle> {
        self.textured_style.as_ref()
    }
}
