//! A native pixmap implements the existing viewer surface; no drawing command is recorded or synthesized.
use crate::{MAXCOLOR, PixelSegment, RasterWindow};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::xw::XwRasterCanvas;
use std::{cell::RefCell, rc::Rc};

pub(super) struct VirtualWindow {
    canvas: XwRasterCanvas,
    palette: Rc<RefCell<[u64; MAXCOLOR]>>,
    failure: Option<Failure>,
}
impl VirtualWindow {
    pub(super) fn new(mut canvas: XwRasterCanvas, palette: Rc<RefCell<[u64; MAXCOLOR]>>) -> Self {
        canvas.set_pixel(palette.borrow()[0]);
        Self {
            canvas,
            palette,
            failure: None,
        }
    }
}
impl RasterWindow for VirtualWindow {
    fn position(&self) -> (i32, i32) {
        let [x, y] = self.canvas.position();
        (x, y)
    }
    fn set_position(&mut self, x: i32, y: i32) {
        self.canvas.set_position([x, y]);
    }
    fn width(&self) -> i32 {
        self.canvas.dimensions()[0]
    }
    fn height(&self) -> i32 {
        self.canvas.dimensions()[1]
    }
    fn set_dimension(&mut self, width: i32, height: i32) {
        self.failure = self.canvas.resize([width, height]).err();
    }
    fn title(&self) -> String {
        self.canvas.title().to_string()
    }
    fn set_title(&mut self, title: &str) {
        self.canvas.set_title(title);
    }
    fn is_mapped(&self) -> bool {
        false
    }
    fn display(&mut self) { /* Capture windows remain unmapped. */
    }
    fn hide(&mut self) { /* This backend never maps its window. */
    }
    fn clear(&mut self) {
        self.canvas.clear();
    }
    fn set_color(&mut self, color: i32) {
        if let Ok(index) = usize::try_from(color)
            && let Some(pixel) = self.palette.borrow().get(index)
        {
            self.canvas.set_pixel(*pixel);
        }
    }
    fn set_mode(&mut self, mode: i32) {
        self.canvas.set_mode(mode);
    }
    fn draw_string(&mut self, x: i32, y: i32, text: &str) {
        self.canvas.draw_text([x, y], text);
    }
    fn draw_segments(&mut self, segments: &[PixelSegment]) {
        let segments: Vec<[i16; 4]> = segments
            .iter()
            .map(|segment| {
                let [[x1, y1], [x2, y2]] = segment.points;
                [x1, y1, x2, y2]
            })
            .collect();
        self.canvas.draw_segments(&segments);
    }
    fn redraw(&mut self) {
        self.canvas.flush();
    }
    fn save(&self, file_name: &str) -> bool {
        self.failure.is_none() && self.canvas.save(file_name).unwrap_or(false)
    }
    fn wait(&mut self, _to_wait: bool) { /* Capture-window event polling returns immediately. */
    }
    fn is_window(&self, window: u64) -> bool {
        self.canvas.window_id() == window
    }
}
