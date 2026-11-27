//! Font I/O is injectable; draw extraction consumes a validated immutable glyph layout.
use crate::{FreeTypeTextSource, PrimitiveDraw, RasterImage, RenderError};
use martensite_tkservice::font::FontRect;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsSource, Graphic3dRenderingParams, Graphic3dText,
};
use std::sync::Arc;
/// Laid-out glyph triangles in native pixel coordinates; returned by `TextLayoutSource::layout`.
#[derive(Clone, Debug)]
pub struct GlyphLayout {
    /// Y-up local glyph corners, after native floor rounding.
    pub positions: Vec<[f32; 2]>,
    /// Top-down atlas UVs corresponding to the corners.
    pub uv: Vec<[f32; 2]>,
    /// Independent glyph triangles.
    pub indices: Vec<u32>,
    /// White RGBA texels with glyph coverage in alpha.
    pub atlas: RasterImage,
    /// Native formatter bounds, including alignment and whitespace; `layout.formatter_bounds`.
    pub formatter_bounds: martensite_tkservice::font::FontRect,
    /// Native font point size, before view-resolution scaling.
    pub point_size: u32,
}
/// Project-owned seam for font lookup, FreeType and formatting; inject a named fake for CPU tests.
pub trait TextLayoutSource: Send + Sync {
    /// Build a layout, or no geometry for empty text/unavailable font; `source.layout(text, aspects, params)?`.
    fn layout(
        &self,
        text: &Graphic3dText,
        aspects: &dyn Graphic3dAspectsSource,
        params: &Graphic3dRenderingParams,
    ) -> Result<Option<GlyphLayout>, RenderError>;
}

mod retained_geometry;
pub(crate) use retained_geometry::RetainedGlyphLayout;

#[derive(Clone)]
pub(crate) enum TextProvider {
    Native(Arc<FreeTypeTextSource>),
    Injected(Arc<dyn TextLayoutSource>),
}

impl Default for TextProvider {
    fn default() -> Self {
        Self::Native(Arc::default())
    }
}

impl TextProvider {
    pub(crate) fn layout(
        &self,
        text: &Graphic3dText,
        aspects: &dyn Graphic3dAspectsSource,
        params: &Graphic3dRenderingParams,
    ) -> Result<Option<TextLayoutSnapshot>, RenderError> {
        match self {
            Self::Native(fonts) => fonts.layout_for_draw(text, aspects, params),
            Self::Injected(fonts) => fonts
                .layout(text, aspects, params)
                .map(|layout| layout.map(TextLayoutSnapshot::Live)),
        }
    }
}

pub(crate) enum TextLayoutSnapshot {
    Retained(Arc<RetainedGlyphLayout>),
    Live(GlyphLayout),
}

impl TextLayoutSnapshot {
    pub(crate) fn view(&self) -> GlyphLayoutView<'_> {
        match self {
            Self::Retained(layout) => GlyphLayoutView::Retained(layout),
            Self::Live(layout) => GlyphLayoutView::Live(layout),
        }
    }

    pub(crate) fn into_public(self) -> GlyphLayout {
        match self {
            Self::Retained(layout) => layout.to_public_layout(),
            Self::Live(layout) => layout,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum GlyphLayoutView<'a> {
    Retained(&'a RetainedGlyphLayout),
    Live(&'a GlyphLayout),
}

impl<'a> From<&'a GlyphLayout> for GlyphLayoutView<'a> {
    fn from(layout: &'a GlyphLayout) -> Self {
        Self::Live(layout)
    }
}

impl GlyphLayoutView<'_> {
    pub(crate) fn formatter_bounds(self) -> FontRect {
        match self {
            Self::Retained(layout) => layout.formatter_bounds,
            Self::Live(layout) => layout.formatter_bounds,
        }
    }

    pub(crate) fn point_size(self) -> u32 {
        match self {
            Self::Retained(layout) => layout.point_size,
            Self::Live(layout) => layout.point_size,
        }
    }

    pub(crate) fn index_count(self) -> usize {
        match self {
            Self::Retained(layout) => layout.index_count(),
            Self::Live(layout) => layout.indices.len(),
        }
    }

    pub(crate) fn draw(
        self,
        rgba: [f32; 4],
        clip: [f32; 16],
    ) -> Result<PrimitiveDraw, RenderError> {
        match self {
            Self::Retained(layout) => layout.draw(rgba, clip),
            Self::Live(layout) => retained_geometry::glyph_draw(
                &layout.positions,
                layout.indices.clone().into(),
                layout.atlas.clone(),
                layout.uv.clone().into(),
                rgba,
                clip,
            ),
        }
    }
}
