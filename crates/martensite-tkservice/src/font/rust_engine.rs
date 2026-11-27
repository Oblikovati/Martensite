//! Pure-Rust font engine backed by `swash` and safe WOFF1 unpacking (#632).
//!
//! Used automatically on `wasm32-unknown-unknown` and on hosts without FreeType,
//! while keeping FreeType FFI as the native default so existing Linux Vulkan
//! reference images stay byte-for-byte unchanged.

use super::ft_font::FontFTFontParams;
use super::glyph_outline::{
    FontGlyphOutline, FontGlyphOutlineContour, FontGlyphOutlineFillRule, FontGlyphOutlineSegment,
};
use super::woff::decode_woff1_or_sfnt;
use martensite_tkmath::gp::Xy;
use std::sync::Arc;
use swash::scale::{Render, ScaleContext, Source};
use swash::zeno::{Format, Transform, Verb};
use swash::{FontRef, StringId, TableProvider};

const KERN_TAG: u32 = u32::from_be_bytes(*b"kern");

/// Cached pure-Rust font face loaded from WOFF1 or SFNT bytes.
#[derive(Clone)]
pub(crate) struct RustFontFace {
    sfnt: Arc<[u8]>,
    face_index: usize,
    ppem: f32,
    units_per_em: u16,
    glyph_count: u16,
    ascent_px: f32,
    descent_px: f32,
    line_spacing_px: f32,
    max_width_px: f32,
    underline_offset_px: f32,
    underline_thickness_px: f32,
    to_synthesize_italic: bool,
    family_name: String,
}

/// Snapshot of a loaded or rendered glyph in the pure-Rust engine.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LoadedRustGlyph {
    pub(crate) character: u32,
    pub(crate) glyph_id: u16,
    pub(crate) advance_x: f32,
    pub(crate) has_bitmap: bool,
    pub(crate) bitmap_left: i32,
    pub(crate) bitmap_top: i32,
    pub(crate) bitmap_width: u32,
    pub(crate) bitmap_height: u32,
}

/// Rendered 8-bit grayscale alpha bitmap produced by [`RustFontFace::render_glyph`].
pub(crate) struct RenderedRustBitmap {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<u8>,
}

/// Metadata extracted when registering raw font bytes in `FontFontMgr`.
pub(crate) struct InspectedFontBytes {
    pub(crate) sfnt: Arc<[u8]>,
    pub(crate) family_name: String,
}

impl RustFontFace {
    /// Validates and unpacks WOFF1/SFNT bytes without opening a sized face.
    pub(crate) fn inspect_bytes(raw: &[u8]) -> Result<InspectedFontBytes, String> {
        let unpacked = decode_woff1_or_sfnt(raw)?;
        let font = FontRef::from_index(&unpacked, 0)
            .ok_or_else(|| "font bytes do not contain a valid SFNT face at index 0".to_string())?;
        let family_name = read_family_name(&font).unwrap_or_default();
        Ok(InspectedFontBytes {
            sfnt: Arc::from(unpacked),
            family_name,
        })
    }

    /// Opens a pure-Rust face from WOFF1 or SFNT bytes at `params` and `face_id`.
    pub(crate) fn from_bytes(raw: &[u8], params: &FontFTFontParams, face_id: i32) -> Option<Self> {
        if !params.is_valid() || face_id < 0 {
            return None;
        }
        let unpacked = decode_woff1_or_sfnt(raw).ok()?;
        let sfnt: Arc<[u8]> = Arc::from(unpacked);
        Self::from_sfnt(sfnt, params, face_id as usize)
    }

    /// Opens a pure-Rust face from already-unpacked SFNT bytes.
    pub(crate) fn from_sfnt(
        sfnt: Arc<[u8]>,
        params: &FontFTFontParams,
        face_index: usize,
    ) -> Option<Self> {
        let font = FontRef::from_index(&sfnt, face_index)?;
        let ppem = (params.point_size as f32) * (params.resolution as f32) / 72.0;
        let metrics = font.metrics(&[]).scale(ppem);
        if metrics.units_per_em == 0 {
            return None;
        }
        let line_spacing_px = metrics.ascent + metrics.descent + metrics.leading;
        let family_name = read_family_name(&font).unwrap_or_default();
        Some(Self {
            sfnt,
            face_index,
            ppem,
            units_per_em: metrics.units_per_em,
            glyph_count: metrics.glyph_count,
            ascent_px: metrics.ascent,
            descent_px: -metrics.descent,
            line_spacing_px,
            max_width_px: metrics.max_width.max(ppem),
            underline_offset_px: metrics.underline_offset,
            underline_thickness_px: metrics.stroke_size,
            to_synthesize_italic: params.to_synthesize_italic,
            family_name,
        })
    }

    fn font_ref(&self) -> Option<FontRef<'_>> {
        FontRef::from_index(&self.sfnt, self.face_index)
    }

    pub(crate) fn family_name(&self) -> &str {
        &self.family_name
    }

    pub(crate) fn glyph_count(&self) -> u16 {
        self.glyph_count
    }

    pub(crate) fn char_index(&self, character: u32) -> u32 {
        let Some(ch) = char::from_u32(character) else {
            return 0;
        };
        self.font_ref()
            .map_or(0, |font| u32::from(font.charmap().map(ch)))
    }

    pub(crate) fn glyph_max_size_x(&self) -> u32 {
        (self.max_width_px + 0.5).max(1.0) as u32
    }

    pub(crate) fn glyph_max_size_y(&self) -> u32 {
        (self.line_spacing_px + 0.5).max(1.0) as u32
    }

    pub(crate) fn ascender(&self) -> f32 {
        self.ascent_px
    }

    pub(crate) fn descender(&self) -> f32 {
        self.descent_px
    }

    pub(crate) fn line_spacing(&self) -> f32 {
        self.line_spacing_px
    }

    pub(crate) fn underline_position(&self) -> f32 {
        self.underline_offset_px
    }

    pub(crate) fn underline_thickness(&self) -> f32 {
        self.underline_thickness_px
    }

    pub(crate) fn advance_x_for_glyph(&self, glyph_id: u16) -> f32 {
        let Some(font) = self.font_ref() else {
            return 0.0;
        };
        font.glyph_metrics(&[])
            .scale(self.ppem)
            .advance_width(glyph_id)
    }

    /// Evaluates horizontal format-0 TrueType `kern` table kerning in pixels.
    pub(crate) fn kerning_x(&self, left: u16, right: u16) -> f32 {
        if left == 0 || right == 0 {
            return 0.0;
        }
        let Some(font) = self.font_ref() else {
            return 0.0;
        };
        let Some(kern) = font.table_by_tag(KERN_TAG) else {
            return 0.0;
        };
        let units = lookup_kern_format0(kern, left, right);
        f32::from(units) * (self.ppem / f32::from(self.units_per_em))
    }

    /// Rasterizes `glyph_id` into an 8-bit grayscale alpha bitmap.
    pub(crate) fn render_glyph(&self, glyph_id: u16) -> Option<RenderedRustBitmap> {
        let font = self.font_ref()?;
        let mut ctx = ScaleContext::new();
        let mut scaler = ctx.builder(font).size(self.ppem).hint(false).build();
        let mut render = Render::new(&[Source::Outline]);
        render.format(Format::Alpha);
        if self.to_synthesize_italic {
            let angle = 10.0 * std::f64::consts::PI / 180.0;
            let shear = martensite_cmath::tan(angle) as f32;
            render.transform(Some(Transform::new(1.0, 0.0, shear, 1.0, 0.0, 0.0)));
        }
        let image = render.render(&mut scaler, glyph_id)?;
        let width = image.placement.width as usize;
        let height = image.placement.height as usize;
        if width == 0 || height == 0 || image.data.len() < width * height {
            return None;
        }
        Some(RenderedRustBitmap {
            left: image.placement.left,
            top: image.placement.top,
            width,
            height,
            data: image.data,
        })
    }

    /// Extracts unscaled font-unit vector contours for `FontBRepFont` 3D text.
    pub(crate) fn glyph_outline(
        &self,
        glyph_id: u16,
        width_scale: f64,
        is_single_stroke: bool,
    ) -> Option<FontGlyphOutline> {
        let font = self.font_ref()?;
        let mut ctx = ScaleContext::new();
        let mut scaler = ctx.builder(font).size(0.0).hint(false).build();
        let outline = scaler.scale_outline(glyph_id)?;
        let mut builder = OutlineBuilder::new(width_scale, is_single_stroke);
        builder.visit(outline.points(), outline.verbs());
        builder.finish(f64::from(self.units_per_em))
    }
}

fn read_family_name(font: &FontRef<'_>) -> Option<String> {
    let mut fallback = None;
    for name in font.localized_strings() {
        if name.id() == StringId::Family {
            let text = name.to_string();
            if name.is_unicode() {
                return Some(text);
            }
            if fallback.is_none() {
                fallback = Some(text);
            }
        }
    }
    fallback
}

fn lookup_kern_format0(table: &[u8], left: u16, right: u16) -> i16 {
    if table.len() < 4 {
        return 0;
    }
    let version = u16::from_be_bytes([table[0], table[1]]);
    if version != 0 {
        return 0;
    }
    let n_tables = usize::from(u16::from_be_bytes([table[2], table[3]]));
    let mut offset = 4usize;
    let target = (u32::from(left) << 16) | u32::from(right);
    for _ in 0..n_tables {
        if offset + 6 > table.len() {
            break;
        }
        let length = usize::from(u16::from_be_bytes([table[offset + 2], table[offset + 3]]));
        let coverage = u16::from_be_bytes([table[offset + 4], table[offset + 5]]);
        let format = coverage >> 8;
        let is_horizontal = (coverage & 1) != 0;
        let is_cross_stream = (coverage & 4) != 0;
        if format == 0
            && is_horizontal
            && !is_cross_stream
            && offset + 14 <= table.len()
            && let Some(delta) = search_kern_subtable(&table[offset + 6..], target)
        {
            return delta;
        }
        if length == 0 {
            break;
        }
        offset = offset.saturating_add(length);
    }
    0
}

fn search_kern_subtable(subtable: &[u8], target: u32) -> Option<i16> {
    if subtable.len() < 8 {
        return None;
    }
    let n_pairs = usize::from(u16::from_be_bytes([subtable[0], subtable[1]]));
    let pairs = subtable.get(8..8 + n_pairs.saturating_mul(6))?;
    let mut lo = 0usize;
    let mut hi = n_pairs;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let base = mid * 6;
        let key = u32::from_be_bytes([
            pairs[base],
            pairs[base + 1],
            pairs[base + 2],
            pairs[base + 3],
        ]);
        match key.cmp(&target) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => {
                return Some(i16::from_be_bytes([pairs[base + 4], pairs[base + 5]]));
            }
        }
    }
    None
}

struct OutlineBuilder {
    segments: Vec<FontGlyphOutlineSegment>,
    contours: Vec<FontGlyphOutlineContour>,
    current: Xy,
    contour_start: Xy,
    first_segment: u32,
    width_scale: f64,
    has_contour: bool,
    is_single_stroke: bool,
}

impl OutlineBuilder {
    fn new(width_scale: f64, is_single_stroke: bool) -> Self {
        Self {
            segments: Vec::new(),
            contours: Vec::new(),
            current: Xy::default(),
            contour_start: Xy::default(),
            first_segment: 0,
            width_scale,
            has_contour: false,
            is_single_stroke,
        }
    }

    fn point(&self, p: swash::zeno::Point) -> Xy {
        Xy::new(f64::from(p.x) * self.width_scale, f64::from(p.y))
    }

    fn visit(&mut self, points: &[swash::zeno::Point], verbs: &[Verb]) {
        let mut cursor = 0usize;
        for &verb in verbs {
            match verb {
                Verb::MoveTo => {
                    self.finish_contour();
                    if let Some(&p) = points.get(cursor) {
                        cursor += 1;
                        let pt = self.point(p);
                        self.current = pt;
                        self.contour_start = pt;
                        self.first_segment = self.segments.len() as u32;
                        self.has_contour = true;
                    }
                }
                Verb::LineTo => {
                    if let Some(&p) = points.get(cursor) {
                        cursor += 1;
                        let end = self.point(p);
                        self.segments
                            .push(FontGlyphOutlineSegment::line(self.current, end));
                        self.current = end;
                    }
                }
                Verb::QuadTo => {
                    if let (Some(&c), Some(&p)) = (points.get(cursor), points.get(cursor + 1)) {
                        cursor += 2;
                        let ctrl = self.point(c);
                        let end = self.point(p);
                        self.segments.push(FontGlyphOutlineSegment::quadratic(
                            self.current,
                            ctrl,
                            end,
                        ));
                        self.current = end;
                    }
                }
                Verb::CurveTo => {
                    if let (Some(&c1), Some(&c2), Some(&p)) = (
                        points.get(cursor),
                        points.get(cursor + 1),
                        points.get(cursor + 2),
                    ) {
                        cursor += 3;
                        let p1 = self.point(c1);
                        let p2 = self.point(c2);
                        let end = self.point(p);
                        self.segments.push(FontGlyphOutlineSegment::cubic(
                            self.current,
                            p1,
                            p2,
                            end,
                        ));
                        self.current = end;
                    }
                }
                Verb::Close => {
                    self.finish_contour();
                }
            }
        }
        self.finish_contour();
    }

    fn finish_contour(&mut self) {
        if !self.has_contour {
            return;
        }
        let gap = self.current - self.contour_start;
        if !self.is_single_stroke && gap.square_modulus() > 0.0 {
            self.segments.push(FontGlyphOutlineSegment::line(
                self.current,
                self.contour_start,
            ));
            self.current = self.contour_start;
        }
        let first = self.first_segment as usize;
        let count = self.segments.len().saturating_sub(first);
        if count > 0 {
            self.contours.push(FontGlyphOutlineContour::new(
                self.first_segment,
                count as u32,
            ));
        }
        self.has_contour = false;
    }

    fn finish(self, units_per_em: f64) -> Option<FontGlyphOutline> {
        Some(FontGlyphOutline::create(
            units_per_em,
            FontGlyphOutlineFillRule::NonZero,
            false,
            self.is_single_stroke,
            self.segments,
            self.contours,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::RustFontFace;
    use crate::font::{
        FontFTFont, FontFTFontParams, FontFontAspect, FontFontMgr, FontRect, FontStrictLevel,
    };
    use crate::image::ImageFormat;
    use martensite_tkernel::tcollection::AsciiString;
    use std::sync::Arc;

    #[test]
    fn embedded_woff1_unpacks_rasterizes_and_extracts_outlines() {
        let buffer = FontFontMgr::embed_fallback_font();
        let raw = buffer
            .get()
            .expect("embedded fallback WOFF1 buffer")
            .as_slice();
        let params = FontFTFontParams {
            point_size: 24,
            resolution: 72,
            ..Default::default()
        };
        let font = FontFTFont::default();
        assert!(font.init_pure_rust_bytes(
            raw,
            &AsciiString::from("Embed Fallback Font"),
            &params,
            0
        ));
        assert!(font.is_valid());
        assert!(font.ascender() > 10.0);
        assert!(font.descender() < 0.0);
        assert!(font.line_spacing() > font.ascender());
        assert!(font.glyphs_number(false) > 50);
        assert!(font.has_symbol('A' as u32));
        assert!(font.advance_x_of('A' as u32, 'V' as u32) > 5.0);

        assert!(font.render_glyph('A' as u32));
        let img = font.glyph_image();
        assert_eq!(img.format(), ImageFormat::Alpha);
        assert!(img.width() > 4 && img.height() > 8);
        let mut rect = FontRect::default();
        font.glyph_rect(&mut rect);
        assert!(rect.right > rect.left);
        assert!(rect.top > rect.bottom);

        let outline = font
            .load_glyph_outline_char('A' as u32)
            .expect("pure-Rust outline for 'A'");
        assert!(outline.units_per_em() > 0.0);
        assert!(outline.contours().len() >= 2);
        assert!(outline.segments().len() >= 6);
    }

    #[test]
    fn register_font_bytes_loads_memory_font_through_font_manager() {
        let buffer = FontFontMgr::embed_fallback_font();
        let raw: Arc<[u8]> = Arc::from(buffer.get().unwrap().as_slice());
        let inspected = RustFontFace::inspect_bytes(&raw).expect("valid WOFF1");
        assert!(!inspected.sfnt.is_empty());

        let mgr = FontFontMgr::get_instance();
        let family = AsciiString::from("MartensiteTestMemorySans");
        mgr.register_font_bytes(&family, FontFontAspect::Regular, raw)
            .expect("register in-memory font");

        let params = FontFTFontParams {
            point_size: 18,
            resolution: 72,
            ..Default::default()
        };
        let font = FontFTFont::find_and_create(
            &family,
            FontFontAspect::Regular,
            &params,
            FontStrictLevel::Strict,
        );
        let font = font.get().expect("registered memory font must resolve");
        assert!(font.is_valid());
        assert!(font.render_glyph('M' as u32));
        assert!(font.load_glyph_outline_char('M' as u32).is_some());
    }
}
