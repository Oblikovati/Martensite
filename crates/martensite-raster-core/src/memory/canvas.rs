//! A 24-bit pixel buffer with a foreground pixel and a raster operation, like a server GC.
use super::ZeroWidthLineRule;
use super::glyphs::{FIXED_GLYPHS, GLYPH_ADVANCE, GLYPH_ASCENT};
use super::raster_op::RasterOp;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat};

/// Row-major `0xRRGGBB` pixels drawn with one foreground pixel and raster operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PixelCanvas {
    size: [i32; 2],
    pixels: Vec<u32>,
    foreground: u32,
    operation: RasterOp,
    line_rule: ZeroWidthLineRule,
}

impl PixelCanvas {
    /// A black canvas drawing in copy mode; `PixelCanvas::new([400, 400], ZeroWidthLineRule::FrameBuffer)`.
    pub(super) fn new(size: [i32; 2], line_rule: ZeroWidthLineRule) -> Self {
        let count = size.map(|value| value.max(0) as usize);
        Self {
            size,
            pixels: vec![0; count[0] * count[1]],
            foreground: 0,
            operation: RasterOp::default(),
            line_rule,
        }
    }

    /// The segment rule this canvas draws with; `canvas.line_rule()`.
    pub(super) fn line_rule(&self) -> ZeroWidthLineRule {
        self.line_rule
    }

    /// `[width, height]`; `canvas.size()`.
    pub(super) fn size(&self) -> [i32; 2] {
        self.size
    }

    /// The current foreground pixel; `canvas.foreground()`.
    pub(super) fn foreground(&self) -> u32 {
        self.foreground
    }

    /// Select the foreground pixel; `canvas.set_foreground(0xff0000)`.
    pub(super) fn set_foreground(&mut self, pixel: u32) {
        self.foreground = pixel;
    }

    /// Select raster operation `code` (0..16), ignoring other codes; `canvas.set_operation(6)`.
    pub(super) fn set_operation(&mut self, code: i32) {
        if let Some(operation) = RasterOp::from_code(code) {
            self.operation = operation;
        }
    }

    /// Fill every pixel with pixel 0 through the current operation, as clearing does.
    ///
    /// Example: in copy mode `canvas.clear()` turns the canvas black; in xor mode it keeps it.
    pub(super) fn clear(&mut self) {
        let operation = self.operation;
        for pixel in &mut self.pixels {
            *pixel = operation.apply(0, *pixel);
        }
    }

    /// Combine the foreground into one pixel; points outside the canvas are ignored.
    ///
    /// Example: `canvas.plot([3, 4])`.
    pub(super) fn plot(&mut self, point: [i32; 2]) {
        let Some(index) = self.index(point) else {
            return;
        };
        self.pixels[index] = self.operation.apply(self.foreground, self.pixels[index]);
    }

    /// The pixel at `point`, `None` outside; `canvas.pixel([0, 0])`.
    pub(super) fn pixel(&self, point: [i32; 2]) -> Option<u32> {
        self.index(point).map(|index| self.pixels[index])
    }

    fn index(&self, [x, y]: [i32; 2]) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.size[0] || y >= self.size[1] {
            return None;
        }
        Some(y as usize * self.size[0] as usize + x as usize)
    }

    /// Draw a zero-width segment including both endpoints; `canvas.draw_segment([0, 0], [9, 3])`.
    pub(super) fn draw_segment(&mut self, start: [i32; 2], end: [i32; 2]) {
        let size = self.size;
        let rule = self.line_rule;
        rule.draw(size, start, end, &mut |point| self.plot(point));
    }

    /// Draw bytes up to the first NUL with the fixed font; `origin` is the first baseline point.
    ///
    /// Example: `canvas.draw_text([5, 20], b"label")` inks only glyph pixels, never a background.
    pub(super) fn draw_text(&mut self, origin: [i32; 2], text: &[u8]) {
        let visible = text.split(|byte| *byte == 0).next().unwrap_or_default();
        let mut x = i64::from(origin[0]);
        for byte in visible {
            if x >= i64::from(self.size[0]) {
                return;
            }
            if x > -i64::from(GLYPH_ADVANCE) {
                self.draw_glyph([x as i32, origin[1]], *byte);
            }
            x += i64::from(GLYPH_ADVANCE);
        }
    }

    fn draw_glyph(&mut self, origin: [i32; 2], byte: u8) {
        let top = origin[1] - GLYPH_ASCENT;
        for (row, bits) in FIXED_GLYPHS[usize::from(byte)].iter().enumerate() {
            for column in 0..8 {
                if bits & (0x80 >> column) != 0 {
                    self.plot([origin[0] + column, top + row as i32]);
                }
            }
        }
    }

    /// Top-down RGB bytes, three per pixel; `canvas.rgb_bytes().len() == width * height * 3`.
    pub(super) fn rgb_bytes(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|pixel| [(pixel >> 16) as u8, (pixel >> 8) as u8, *pixel as u8])
            .collect()
    }

    /// Write the pixels through the shared image writer, the same path as server captures.
    ///
    /// Example: `canvas.save("capture.png")?` writes a binary PPM (P6) whatever the extension.
    pub(super) fn save(&self, path: &str) -> Result<bool, Failure> {
        let size = self.size.map(|value| value as usize);
        let image = ImageAlienPixMap::default();
        if !image.init_trash(ImageFormat::Rgb, size[0], size[1], 0) {
            return Ok(false);
        }
        image.set_top_down(true);
        image
            .change_data()
            .data
            .change_data()
            .copy_from_slice(&self.rgb_bytes());
        image.save(&AsciiString::from(path))
    }
}

#[cfg(test)]
#[path = "canvas_tests.rs"]
mod tests;
