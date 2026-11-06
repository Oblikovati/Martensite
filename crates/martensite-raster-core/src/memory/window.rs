//! An in-memory raster window: the same drawing semantics as a server pixmap, no display.
use super::ZeroWidthLineRule;
use super::canvas::PixelCanvas;
use super::raster_op::RASTER_COPY;
use crate::{MAXCOLOR, PixelSegment, RasterWindow};
use martensite_tkernel::standard::Failure;
use std::{cell::RefCell, rc::Rc};

/// A window whose pixels live in process memory; create it with
/// `MemoryWindowSystem::new().try_create_memory_window("Preview", (0, 0), (400, 400))?`.
/// It is never mapped and never receives pointer events; [`RasterWindow::save`] writes
/// the same bytes a server-backed capture of the same drawing writes.
#[derive(Debug)]
pub struct MemoryWindow {
    canvas: PixelCanvas,
    palette: Rc<RefCell<[u32; MAXCOLOR]>>,
    identifier: u64,
    position: (i32, i32),
    title: String,
    failure: Option<Failure>,
}

impl MemoryWindow {
    pub(super) fn new(
        identifier: u64,
        title: &str,
        position: (i32, i32),
        size: [i32; 2],
        palette: Rc<RefCell<[u32; MAXCOLOR]>>,
        line_rule: ZeroWidthLineRule,
    ) -> Self {
        let mut canvas = PixelCanvas::new(size, line_rule);
        canvas.set_foreground(palette.borrow()[0]);
        Self {
            canvas,
            palette,
            identifier,
            position,
            title: title.to_string(),
            failure: None,
        }
    }

    /// The `0xRRGGBB` pixel at `(x, y)`, `None` outside the window.
    ///
    /// ```
    /// use martensite_raster_core::MemoryWindowSystem;
    /// let window = MemoryWindowSystem::new()
    ///     .try_create_memory_window("Preview", (0, 0), (4, 4))
    ///     .unwrap();
    /// assert_eq!(window.pixel(0, 0), Some(0x000000));
    /// assert_eq!(window.pixel(4, 0), None);
    /// ```
    pub fn pixel(&self, x: i32, y: i32) -> Option<u32> {
        self.canvas.pixel([x, y])
    }

    /// Top-down RGB bytes, three per pixel, as a capture stores them.
    ///
    /// ```
    /// use martensite_raster_core::MemoryWindowSystem;
    /// let window = MemoryWindowSystem::new()
    ///     .try_create_memory_window("Preview", (0, 0), (2, 3))
    ///     .unwrap();
    /// assert_eq!(window.rgb_bytes(), vec![0; 2 * 3 * 3]);
    /// ```
    pub fn rgb_bytes(&self) -> Vec<u8> {
        self.canvas.rgb_bytes()
    }

    /// A fresh raster of `size`, black, in copy mode, keeping the foreground pixel.
    fn reallocate(&mut self, size: [i32; 2]) {
        let foreground = self.canvas.foreground();
        self.canvas = PixelCanvas::new(size, self.canvas.line_rule());
        self.canvas.set_foreground(foreground);
        self.canvas.set_operation(RASTER_COPY);
    }
}

impl RasterWindow for MemoryWindow {
    fn position(&self) -> (i32, i32) {
        self.position
    }
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
    fn width(&self) -> i32 {
        self.canvas.size()[0]
    }
    fn height(&self) -> i32 {
        self.canvas.size()[1]
    }
    fn set_dimension(&mut self, width: i32, height: i32) {
        self.failure = super::checked_size([width, height]).err();
        if self.failure.is_none() {
            self.reallocate([width, height]);
        }
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    fn is_mapped(&self) -> bool {
        false
    }
    fn display(&mut self) { /* Memory windows are never shown. */
    }
    fn hide(&mut self) { /* Memory windows are never shown. */
    }
    fn clear(&mut self) {
        self.canvas.clear();
    }
    fn set_color(&mut self, color: i32) {
        if let Ok(index) = usize::try_from(color)
            && let Some(pixel) = self.palette.borrow().get(index)
        {
            self.canvas.set_foreground(*pixel);
        }
    }
    fn set_raster_op(&mut self, mode: i32) {
        self.canvas.set_operation(mode);
    }
    fn draw_string(&mut self, x: i32, y: i32, text: &str) {
        // Text origins travel as signed 16-bit protocol coordinates.
        self.canvas
            .draw_text([i32::from(x as i16), i32::from(y as i16)], text.as_bytes());
    }
    fn draw_segments(&mut self, segments: &[PixelSegment]) {
        for segment in segments {
            let [start, end] = segment.points.map(|point| point.map(i32::from));
            self.canvas.draw_segment(start, end);
        }
    }
    fn redraw(&mut self) { /* Drawing is immediate in memory. */
    }
    fn save(&self, file_name: &str) -> bool {
        self.failure.is_none() && self.canvas.save(file_name).unwrap_or(false)
    }
    fn wait(&mut self, _to_wait: bool) { /* Memory windows have no pointer events. */
    }
    fn is_window(&self, window: u64) -> bool {
        self.identifier == window
    }
}
