//! The `Aspect_Window` interface of `Cocoa_Window` over AppKit, falling back to the stored
//! frame where AppKit cannot answer (off the main thread or without a view).
use super::super::resize::{CocoaCorners, bounds_corners, resize_kind, screen_corners};
use super::CocoaWindow;
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectTypeOfResize, AspectWindow,
    AspectWindowBase, AspectWindowMembers as _,
};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;

impl CocoaWindow {
    /// The stored corners `(left, top, right, bottom)`.
    fn stored_corners(&self) -> CocoaCorners {
        let fields = *self.fields.read();
        (fields.x_left, fields.y_top, fields.x_right, fields.y_bottom)
    }
    /// The live view bounds in backing pixels as corners, `None` where AppKit cannot answer.
    fn live_corners(&self) -> Option<CocoaCorners> {
        let view = self.h_view();
        if view == 0 {
            return None;
        }
        self.calls.backing_bounds(view).map(bounds_corners)
    }
    fn store_corners(&self, corners: CocoaCorners) {
        let mut fields = self.fields.write();
        (fields.x_left, fields.y_top, fields.x_right, fields.y_bottom) = corners;
    }
}

/// The width and height spanned by `corners`.
fn corner_size(corners: CocoaCorners) -> (i32, i32) {
    (
        corners.2.wrapping_sub(corners.0),
        corners.3.wrapping_sub(corners.1),
    )
}

impl AspectWindow for CocoaWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    /// A virtual window is always mapped; otherwise the window of the view is visible.
    /// Off the main thread AppKit cannot answer, and the window reads as unmapped.
    fn is_mapped(&self) -> bool {
        if self.is_virtual() {
            return true;
        }
        let view = self.h_view();
        view != 0 && self.calls.is_window_visible(view).unwrap_or(false)
    }
    /// Orders the window of the view front (`orderFront:`), except for a virtual window.
    fn map(&self) {
        let view = self.h_view();
        if !self.is_virtual() && view != 0 {
            self.calls.order_window(view, true);
        }
    }
    /// Orders the window of the view out (`orderOut:`).
    fn unmap(&self) {
        let view = self.h_view();
        if view != 0 {
            self.calls.order_window(view, false);
        }
    }
    /// Compares the view bounds (backing pixels) with the stored corners, stores the new
    /// corners and reports which borders moved; unknown without a live view.
    fn do_resize(&self) -> AspectTypeOfResize {
        let Some(corners) = self.live_corners() else {
            return AspectTypeOfResize::Unknown;
        };
        let kind = resize_kind(self.stored_corners(), corners);
        self.store_corners(corners);
        kind
    }
    fn do_mapping(&self) -> bool {
        true
    }
    /// The width over the height of the view (or of the stored frame), 1 when empty.
    fn ratio(&self) -> f64 {
        let (width, height) = self.size();
        if width != 0 && height != 0 {
            f64::from(width) / f64::from(height)
        } else {
            1.0
        }
    }
    /// The window frame in top-left based screen points, or the stored corners where
    /// AppKit cannot answer.
    fn position(&self) -> (i32, i32, i32, i32) {
        let view = self.h_view();
        let frame = (view != 0).then(|| self.calls.window_frame(view)).flatten();
        match (frame, self.calls.main_screen_height()) {
            (Some(frame), Some(screen_height)) => screen_corners(frame, screen_height),
            _ => self.stored_corners(),
        }
    }
    /// The view size in backing pixels, or the stored frame size where AppKit cannot answer.
    fn size(&self) -> (i32, i32) {
        corner_size(self.live_corners().unwrap_or_else(|| self.stored_corners()))
    }
    /// The NSView address.
    fn native_handle(&self) -> AspectDrawable {
        self.h_view() as AspectDrawable
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        0
    }
    /// Nothing on macOS.
    fn native_fb_config(&self) -> AspectFbConfig {
        AspectFbConfig::default()
    }
    /// Sets the title of the window of the view.
    fn set_title(&self, title: &AsciiString) {
        let view = self.h_view();
        if view != 0 {
            self.calls
                .set_window_title(view, &String::from_utf8_lossy(title.as_bytes()));
        }
    }
    /// Marks the view as needing display on the main thread; the display is unused on macOS.
    fn invalidate_content(&self, _display: &Handle<AspectDisplayConnection>) {
        let view = self.h_view();
        if view != 0 {
            self.calls.invalidate_view(view);
        }
    }
}
