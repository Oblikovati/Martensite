//! A text render element. It keeps the Graphic3d
//! text, the 2D placement switch and the glyph-buffer residency (built by the first frame that
//! draws it, dropped by `release`, `reset` and a font size change), and measures strings and
//! names font resources like the native static helpers.
use crate::element_stats::GroupElementStats;
use crate::element_upload::ElementUploadSlot;
use crate::render_element::TextPlacement;
use crate::{RenderElement, RenderElementContext, RenderError};
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkservice::font::{FontFTFontParams, FontFontAspect, FontHinting};
use martensite_tkservice::graphic3d::*;

/// The font of a text aspect when it names none (native `THE_DEFAULT_FONT`).
const DEFAULT_FONT: &str = "Courier";

/// One text drawn as a group element; `group.add_element(Arc::new(RenderText::new(&text)))`.
///
/// ```ignore
/// let label = Arc::new(RenderText::new(&text));
/// group.add_element(label.clone());
/// label.set_2d(true); // place the glyphs in the 2D frame on the next redraw
/// ```
pub struct RenderText {
    text: SharedState<Handle<Graphic3dText>>,
    two_d: SharedState<bool>,
    upload: ElementUploadSlot,
}

impl Default for RenderText {
    /// An empty text of height 10 (`GlText`); `RenderText::default.text`.
    fn default() -> Self {
        Self::new(&Handle::new(Graphic3dText::new(10.)))
    }
}

impl RenderText {
    /// An element drawing `text` placed in 3D (`GlText(theTextParams)`).
    /// Example: `RenderText::new(&text).is_2d == false`.
    pub fn new(text: &Handle<Graphic3dText>) -> Self {
        Self {
            text: SharedState::new(text.clone()),
            two_d: SharedState::new(false),
            upload: ElementUploadSlot::default(),
        }
    }
    /// The text parameters (`Text`); `label.text().height()`.
    pub fn text(&self) -> Handle<Graphic3dText> {
        self.text.read().clone()
    }
    /// Draw other text parameters; the built glyphs stay until the next frame (`SetText`).
    /// Example: `label.set_text(&other)`.
    pub fn set_text(&self, text: &Handle<Graphic3dText>) {
        *self.text.write() = text.clone();
    }
    /// Whether glyphs are placed in the 2D frame (`Is2D`); `label.is_2d()`.
    pub fn is_2d(&self) -> bool {
        *self.two_d.read()
    }
    /// Place the glyphs in the 2D frame: the text model replaces the world-view and no depth
    /// test applies (`Set2D`). Example: `label.set_2d(true)`.
    pub fn set_2d(&self, enabled: bool) {
        *self.two_d.write() = enabled;
    }
    /// Set the text height; another height releases the glyphs first (`SetFontSize`).
    /// Example: `label.set_font_size(20)`.
    pub fn set_font_size(&self, size: i32) {
        let text = self.text();
        if text.height() != size as f32 {
            self.release();
        }
        text.set_height(size as f32);
    }
    /// Drop the glyph buffers; the next frame lays the text out again (`Reset`).
    /// Example: `label.reset()`.
    pub fn reset(&self) {
        self.upload.release();
    }
    /// Drop the glyph buffers and the font binding (`Release`); `label.release()`.
    pub fn release(&self) {
        self.upload.release();
    }
    /// The font resource key: font name, aspect, resolution, height and hinting (`FontKey`).
    /// Example: `RenderText::font_key(&style, 16, 72, FontHinting::OFF) == "Courier:0:72:16:0"`.
    pub fn font_key(
        aspect: &dyn Graphic3dAspectsSource,
        height: i32,
        resolution: u32,
        hinting: FontHinting,
    ) -> String {
        let (name, font_aspect) = font_of(aspect);
        format!(
            "{name}:{}:{resolution}:{height}:{}",
            font_aspect as i32, hinting.0
        )
    }
    /// Width, ascender and descender of `text` in the aspect's font; zeros below height 2 or
    /// without a font (`StringSize`). Example: `RenderText::string_size("Ag", &style, 16., 72, h)`.
    pub fn string_size(
        text: &str,
        aspect: &dyn Graphic3dAspectsSource,
        height: f32,
        resolution: u32,
        hinting: FontHinting,
    ) -> [f32; 3] {
        let point_size = height as i32;
        if point_size < 2 {
            return [0.; 3];
        }
        let (name, font_aspect) = font_of(aspect);
        let params = FontFTFontParams {
            point_size: point_size as u32,
            resolution,
            font_hinting: hinting,
            ..Default::default()
        };
        crate::driver_text::measure(text, &name, font_aspect, &params)
    }
    fn placement(&self) -> TextPlacement {
        if self.is_2d() {
            TextPlacement::Pixel
        } else {
            TextPlacement::Scene
        }
    }
}

/// The aspect's font name and aspect, with the native defaults.
fn font_of(aspect: &dyn Graphic3dAspectsSource) -> (String, FontFontAspect) {
    let font_aspect = match aspect.text_font_aspect() {
        FontFontAspect::Undefined => FontFontAspect::Regular,
        other => other,
    };
    let name = aspect.text_font().get().map_or_else(
        || DEFAULT_FONT.to_owned(),
        |font| {
            let name = font.string();
            let bytes: &[u8] = name.as_ref();
            String::from_utf8_lossy(bytes).into_owned()
        },
    );
    (name, font_aspect)
}

impl RenderElement for RenderText {
    /// Draw the text with the current aspects, in 3D or in the 2D frame.
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        context.add_retained_text(&self.text(), self.upload.clone(), self.placement());
        Ok(())
    }
    fn estimated_data_size(&self) -> usize {
        self.upload.current().estimated_bytes()
    }
    fn update_draw_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp, detailed: bool) {
        GroupElementStats::Text(self.upload.current())
            .draw_stats(detailed)
            .apply_to(stats);
    }
    /// `GlText::DumpJson`: the element, the glyph texture count and the formatter box of
    /// the last built layout; `label.dump_json(&mut stream, -1)?`.
    fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        let address = crate::driver_dump::address_of(self);
        crate::driver_dump::text(stream, depth, address, self.upload.current())
    }
}

#[cfg(test)]
#[path = "render_text_tests.rs"]
mod tests;
