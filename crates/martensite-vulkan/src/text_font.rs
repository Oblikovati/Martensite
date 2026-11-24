//! Reuse Martensite's font adapter and formatter under one per-source font-cache lock.
use crate::{
    GlyphLayout, RenderError, TextLayoutSource, error::rejected, text_layout::TextLayoutSnapshot,
};
use martensite_tkernel::{
    standard::{Handle, SharedState},
    tcollection::AsciiString,
};
use martensite_tkservice::{font::*, graphic3d::*};
use std::collections::BTreeMap;
mod retained_layouts;
use retained_layouts::{LayoutKey, RetainedLayouts};
type FontKey = (Vec<u8>, i32, u32, u32, i32);
/// Native FreeType/layout provider; `FreeTypeTextSource::default()` can be shared across groups.
#[derive(Default)]
pub struct FreeTypeTextSource {
    state: SharedState<FontState>,
}

#[derive(Default)]
struct FontState {
    fonts: BTreeMap<FontKey, LoadedFont>,
    layouts: RetainedLayouts,
    atlases: crate::text_atlas::RetainedGlyphAtlases,
}
/// A loaded font and the texel columns of its glyph tiles.
struct LoadedFont {
    font: Handle<FontFTFont>,
    columns: crate::text_atlas::NativeTileColumns,
}
impl TextLayoutSource for FreeTypeTextSource {
    fn layout(
        &self,
        text: &Graphic3dText,
        aspects: &dyn Graphic3dAspectsSource,
        params: &Graphic3dRenderingParams,
    ) -> Result<Option<GlyphLayout>, RenderError> {
        self.layout_for_draw(text, aspects, params)
            .map(|layout| layout.map(TextLayoutSnapshot::into_public))
    }
}

impl FreeTypeTextSource {
    pub(crate) fn layout_for_draw(
        &self,
        text: &Graphic3dText,
        aspects: &dyn Graphic3dAspectsSource,
        params: &Graphic3dRenderingParams,
    ) -> Result<Option<TextLayoutSnapshot>, RenderError> {
        let contents = text.text();
        if contents.is_empty() || text.height() < 2. {
            return Ok(None);
        }
        let key = LayoutKey {
            font: font_key(text, aspects, params),
            text: contents,
            horizontal: text.horizontal_alignment(),
            vertical: text.vertical_alignment(),
        };
        let formatter = text.text_formatter();
        if formatter.is_null()
            && let Some(layout) = self.state.read().layouts.get(&key)
        {
            return Ok(Some(layout));
        }
        self.state.write().layout(key, formatter)
    }
}

impl FontState {
    fn layout(
        &mut self,
        key: LayoutKey,
        formatter: Handle<FontTextFormatter>,
    ) -> Result<Option<TextLayoutSnapshot>, RenderError> {
        let retain = formatter.is_null();
        if retain && let Some(layout) = self.layouts.get(&key) {
            return Ok(Some(layout));
        }
        let loaded = self
            .fonts
            .entry(key.font.clone())
            .or_insert_with(|| LoadedFont::load(&key.font));
        let layout = loaded.layout(&key, formatter, &mut self.atlases)?;
        Ok(layout.map(|layout| {
            if retain {
                self.layouts.record(key, layout)
            } else {
                TextLayoutSnapshot::Live(layout)
            }
        }))
    }
}

impl LoadedFont {
    fn load(key: &FontKey) -> Self {
        Self {
            font: load_font(key),
            columns: Default::default(),
        }
    }

    fn layout(
        &mut self,
        key: &LayoutKey,
        formatter: Handle<FontTextFormatter>,
        atlases: &mut crate::text_atlas::RetainedGlyphAtlases,
    ) -> Result<Option<GlyphLayout>, RenderError> {
        let Some(font) = self.font.get() else {
            return Ok(None);
        };
        let atlases = formatter
            .is_null()
            .then_some((atlases, self.font.object_address().addr()));
        let formatter = prepare_formatter(key, formatter, font)?;
        // Glyph images borrow a mutable FreeType slot. Serialize formatting, copying and column
        // allocation, and keep caller-owned formatters on that live path.
        crate::text_atlas::build_glyph_layout(font, &formatter, &mut self.columns, atlases)
    }
}
fn font_key(
    text: &Graphic3dText,
    aspects: &dyn Graphic3dAspectsSource,
    params: &Graphic3dRenderingParams,
) -> FontKey {
    let name = aspects.text_font();
    let name = name
        .get()
        .map_or_else(|| b"Courier".to_vec(), |s| s.string().as_bytes().to_vec());
    let aspect = match aspects.text_font_aspect() {
        FontFontAspect::Undefined => FontFontAspect::Regular,
        other => other,
    };
    (
        name,
        aspect as i32,
        text.height() as u32,
        params.resolution,
        params.font_hinting.0,
    )
}
fn load_font(key: &FontKey) -> Handle<FontFTFont> {
    let aspect = match key.1 {
        1 => FontFontAspect::Bold,
        2 => FontFontAspect::Italic,
        3 => FontFontAspect::BoldItalic,
        _ => FontFontAspect::Regular,
    };
    let params = FontFTFontParams {
        point_size: key.2,
        resolution: key.3,
        font_hinting: FontHinting(key.4),
        ..Default::default()
    };
    FontFTFont::find_and_create(
        &AsciiString::from_view(&key.0),
        aspect,
        &params,
        FontStrictLevel::Any,
    )
}
fn prepare_formatter(
    key: &LayoutKey,
    formatter: Handle<FontTextFormatter>,
    font: &FontFTFont,
) -> Result<Handle<FontTextFormatter>, RenderError> {
    let formatter = if formatter.is_null() {
        Handle::new(FontTextFormatter::default())
    } else {
        formatter
    };
    formatter.setup_alignment(key.horizontal, key.vertical);
    FontTextFormatter::reset(&formatter);
    formatter
        .append(&key.text, font)
        .map_err(|e| rejected(e, "a formattable native text string"))?;
    formatter
        .format()
        .map_err(|e| rejected(e, "a valid native text layout"))?;
    Ok(formatter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn font_cache_key_distinguishes_resolution_height_aspect_and_hinting() {
        let text = Graphic3dText::new(16.9);
        let style = Graphic3dAspectText3d::default();
        let mut params = Graphic3dRenderingParams::new().unwrap();
        let initial = font_key(&text, &style, &params);
        assert_eq!(initial.2, 16);
        params.resolution = 144;
        assert_ne!(initial, font_key(&text, &style, &params));
        let updated = font_key(&text, &style, &params);
        text.set_height(24.);
        assert_ne!(updated, font_key(&text, &style, &params));
        let updated = font_key(&text, &style, &params);
        style.set_text_font_aspect(FontFontAspect::Bold);
        assert_ne!(updated, font_key(&text, &style, &params));
        let updated = font_key(&text, &style, &params);
        params.font_hinting = FontHinting::FORCE_AUTOHINT;
        assert_ne!(updated, font_key(&text, &style, &params));
        let source = FreeTypeTextSource::default();
        assert!(source.layout(&text, &style, &params).unwrap().is_none());
        assert!(source.state.read().fonts.is_empty());
    }

    #[test]
    fn warm_layout_reuses_the_atlas_and_preserves_caller_owned_corners() {
        let source = FreeTypeTextSource::default();
        let text = label("L019");
        let style = Graphic3dAspectText3d::default();
        let params = Graphic3dRenderingParams::new().unwrap();
        let mut cold = render(&source, &text, &style, &params);
        let first_corner = cold.positions[0];
        cold.positions[0] = [f32::NAN; 2];
        render(&source, &label("UVWXYZ"), &style, &params);
        let warm = render(&source, &text, &style, &params);
        assert_eq!(warm.positions[0], first_corner);
        assert_eq!(warm.uv, cold.uv);
        assert!(Arc::ptr_eq(&cold.atlas.pixels, &warm.atlas.pixels));
    }

    #[test]
    fn changed_alignment_and_resolution_select_their_own_layouts() {
        let source = FreeTypeTextSource::default();
        let text = label("L019");
        let style = Graphic3dAspectText3d::default();
        let mut params = Graphic3dRenderingParams::new().unwrap();
        let left = render(&source, &text, &style, &params);
        text.set_horizontal_alignment(Graphic3dHorizontalTextAlignment::Center);
        let centered = render(&source, &text, &style, &params);
        assert_ne!(left.positions, centered.positions);
        assert!(Arc::ptr_eq(&left.atlas.pixels, &centered.atlas.pixels));
        params.resolution *= 2;
        let scaled = render(&source, &text, &style, &params);
        assert_ne!(centered.positions, scaled.positions);
        assert!(!Arc::ptr_eq(&centered.atlas.pixels, &scaled.atlas.pixels));
    }

    #[test]
    fn equivalent_glyph_sets_match_the_uncached_native_atlas() {
        let source = FreeTypeTextSource::default();
        let style = Graphic3dAspectText3d::default();
        let params = Graphic3dRenderingParams::new().unwrap();
        let first = render(&source, &label("L019"), &style, &params);
        let reordered = label("L910");
        let shared = render(&source, &reordered, &style, &params);
        assert!(Arc::ptr_eq(&first.atlas.pixels, &shared.atlas.pixels));
        assert_ne!(first.uv, shared.uv);
        reordered.set_text_formatter(Handle::new(FontTextFormatter::default()));
        let fresh = render(&source, &reordered, &style, &params);
        assert!(!Arc::ptr_eq(&shared.atlas.pixels, &fresh.atlas.pixels));
        assert_eq!(shared.atlas, fresh.atlas);
        assert_eq!(shared.uv, fresh.uv);
        assert_eq!(shared.positions, fresh.positions);
        assert_eq!(shared.indices, fresh.indices);
    }

    #[test]
    fn a_caller_formatter_reflows_unchanged_text_when_its_wrapping_changes() {
        let source = FreeTypeTextSource::default();
        let text = label("kernel wrap test");
        let style = Graphic3dAspectText3d::default();
        let params = Graphic3dRenderingParams::new().unwrap();
        let unwrapped = render(&source, &text, &style, &params);
        let formatter = Handle::new(FontTextFormatter::default());
        text.set_text_formatter(formatter.clone());
        formatter.set_wrapping(60.);
        let wrapped = render(&source, &text, &style, &params);
        assert_ne!(unwrapped.positions, wrapped.positions);
        formatter.set_wrapping(0.);
        let restored = render(&source, &text, &style, &params);
        assert_eq!(unwrapped.positions, restored.positions);
        assert!(!Arc::ptr_eq(
            &unwrapped.atlas.pixels,
            &restored.atlas.pixels
        ));
    }

    #[test]
    fn moving_native_text_keeps_its_geometry_and_recolors_only_vertices() {
        let provider = crate::text_layout::TextProvider::default();
        let text = label("L019");
        let style = Graphic3dAspectText3d::default();
        let params = Graphic3dRenderingParams::new().unwrap();
        let layout = || provider.layout(&text, &style, &params).unwrap().unwrap();
        let clip = crate::identity_clip();
        let first = layout().view().draw([1.; 4], clip).unwrap();
        let mut moved = clip;
        moved[12] = 0.5;
        let moved_draw = layout().view().draw([1.; 4], moved).unwrap();
        assert_eq!(moved_draw.clip_transform, moved);
        assert_eq!(first.vertices.identity(), moved_draw.vertices.identity());
        let exposed = layout().into_public();
        let live = crate::text_layout::GlyphLayoutView::from(&exposed);
        assert_eq!(moved_draw, live.draw([1.; 4], moved).unwrap());
        let colored = layout().view().draw([0., 0., 1., 1.], moved).unwrap();
        assert_ne!(first.vertices.identity(), colored.vertices.identity());
        assert_eq!(first.indices.identity(), colored.indices.identity());
        assert_eq!(
            first.textured_style.clone().unwrap().uv.identity(),
            colored.textured_style.clone().unwrap().uv.identity()
        );
    }

    fn label(contents: &str) -> Graphic3dText {
        let text = Graphic3dText::new(16.);
        text.set_text_str(contents);
        text
    }

    fn render(
        source: &FreeTypeTextSource,
        text: &Graphic3dText,
        style: &Graphic3dAspectText3d,
        params: &Graphic3dRenderingParams,
    ) -> GlyphLayout {
        source.layout(text, style, params).unwrap().unwrap()
    }
}
