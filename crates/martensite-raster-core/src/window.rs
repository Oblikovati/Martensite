//! Portable raster-window interface; native implementations own their platform resources.
use crate::PixelSegment;

/// One window of a view.
pub trait RasterWindow {
    /// the top-left corner on the screen.
    fn position(&self) -> (i32, i32);
    /// Set screen coordinates; `window.set_position(5, 8)`.
    fn set_position(&mut self, x: i32, y: i32);
    /// Read pixel width; `window.width()`.
    fn width(&self) -> i32;
    /// Read pixel height; `window.height()`.
    fn height(&self) -> i32;
    /// Resize raster storage; `window.set_dimension(80, 60)`.
    fn set_dimension(&mut self, width: i32, height: i32);
    /// Read the window title; `window.title()`.
    fn title(&self) -> String;
    /// Replace the title; `window.set_title("Preview")`.
    fn set_title(&mut self, title: &str);
    /// the window is shown.
    fn is_mapped(&self) -> bool;
    /// shows the window.
    fn display(&mut self);
    /// Hide an interactive window; `window.hide()`.
    fn hide(&mut self);
    /// fills the window with the background.
    fn clear(&mut self);
    /// the palette index for the following drawing.
    fn set_color(&mut self, color: i32);
    /// the raster operation (3 = copy, 6 = xor).
    fn set_raster_op(&mut self, mode: i32);
    /// Draw text at pixel coordinates; `window.draw_string(2, 3, "x")`.
    fn draw_string(&mut self, x: i32, y: i32, text: &str);
    /// Draw signed pixel segments; `window.draw_segments(&segments)`.
    fn draw_segments(&mut self, segments: &[PixelSegment]);
    /// shows the off-screen buffer.
    fn redraw(&mut self);
    /// writes a snapshot image; false on failure.
    fn save(&self, file_name: &str) -> bool;
    /// selects (true) or deselects pointer motion events.
    fn wait(&mut self, to_wait: bool);
    /// true when `window` is this window's native identifier.
    fn is_window(&self, window: u64) -> bool;
}

/// A pointer event of the window system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PointerEvent {
    /// 4 for a button press, 6 for a motion, 0 for anything else (X11 event types).
    pub kind: i32,
    /// The native identifier of the window.
    pub window: u64,
    /// The button, 0 for motion.
    pub button: i32,
    /// Pointer x in window pixels.
    pub x: i32,
    /// Pointer y in window pixels.
    pub y: i32,
}

/// The X11 `ButtonPress` event type.
pub const BUTTON_PRESS: i32 = 4;
/// The X11 `MotionNotify` event type.
pub const MOTION_NOTIFY: i32 = 6;

/// The window system connection.
pub trait RasterWindowSystem {
    /// Creates a window; `existing` attaches to a native
    /// window instead of creating one (0 for a new window).
    fn create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Box<dyn RasterWindow>;
    /// Creates a window with a typed allocation failure; `system.try_create_window("Preview", (0, 0), (400, 400), 0)?`.
    fn try_create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Result<Box<dyn RasterWindow>, martensite_tkernel::standard::Failure> {
        Ok(self.create_window(title, position, size, existing))
    }
    /// allocates palette entry `index` with the named color.
    fn define_color(&self, index: usize, name: &str) -> bool;
    /// sends pending drawing to the screen.
    fn flush(&self);
    /// waits for the next pointer event.
    fn next_event(&self) -> PointerEvent;
}
