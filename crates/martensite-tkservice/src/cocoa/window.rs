//! `Cocoa_Window`: an AppKit window (NSWindow with its NSView) or a wrapper of an NSView.
use super::virtual_keys;
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectTypeOfResize, AspectVKey,
    AspectWindow, AspectWindowBase,
};
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;

/// The AppKit backend named by the unsupported errors.
const BACKEND: &str = "AppKit (macOS)";

/// The protected fields of a Cocoa window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CocoaWindowFields {
    /// `myHWindow`: the `NSWindow*` address, 0 for a wrapped view.
    pub h_window: usize,
    /// `myHView`: the `NSView*` address.
    pub h_view: usize,
    /// `myXLeft`.
    pub x_left: i32,
    /// `myYTop`.
    pub y_top: i32,
    /// `myXRight`.
    pub x_right: i32,
    /// `myYBottom`.
    pub y_bottom: i32,
}

/// An AppKit window. The backend is not implemented: both constructors raise
/// `Standard_NotImplemented`; `CocoaWindow::new("Viewer", 0, 0, 640, 480)`.
pub struct CocoaWindow {
    transient: StandardTransient,
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<CocoaWindowFields>,
}
martensite_tkernel::impl_transient!(CocoaWindow, "Cocoa_Window", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);

impl CocoaWindow {
    /// `VirtualKeyFromNative`: the Aspect key of a Carbon virtual key code (unknown outside
    /// the table); `CocoaWindow::virtual_key_from_native(0x00)` is `A`.
    pub fn virtual_key_from_native(key: i32) -> AspectVKey {
        virtual_keys::virtual_key_from_native(key)
    }
    /// `Cocoa_Window(title, left, top, width, height)`: a new NSWindow with its NSView;
    /// raises `Standard_NotImplemented` until the AppKit backend is implemented.
    /// Example: `CocoaWindow::new("Viewer", 0, 0, 640, 480)`.
    pub fn new(title: &str, left: i32, top: i32, width: i32, height: i32) -> Result<Self, Failure> {
        // TODO(platform): macOS — create the NSWindow and its NSView at the given frame.
        let _ = (title, left, top, width, height);
        Err(unsupported("Cocoa_Window", BACKEND))
    }
    /// `Cocoa_Window(NSView*)`: a wrapper of an existing NSView (an address); raises
    /// `Standard_NotImplemented` until the AppKit backend is implemented.
    /// Example: `CocoaWindow::from_view(view_address)`.
    pub fn from_view(view: usize) -> Result<Self, Failure> {
        // TODO(platform): macOS — retain the NSView and read its frame.
        let _ = view;
        Err(unsupported("Cocoa_Window", BACKEND))
    }
    /// `HView`: the NSView address; `window.h_view()`.
    pub fn h_view(&self) -> usize {
        self.fields.read().h_view
    }
    /// `SetHView`: replaces the NSView; raises `Standard_NotImplemented` until the AppKit
    /// backend is implemented. Example: `window.set_h_view(view_address)?`.
    pub fn set_h_view(&self, view: usize) -> Result<(), Failure> {
        // TODO(platform): macOS — release the old view, retain the new one.
        let _ = view;
        Err(unsupported("Cocoa_Window::SetHView", BACKEND))
    }
}

impl Drop for CocoaWindow {
    /// `~Cocoa_Window`: releases the NSView and the NSWindow created here.
    fn drop(&mut self) {
        // TODO(platform): macOS — release myHView and close myHWindow; no window exists
        // before the port.
        let _ = self.fields.read().h_window;
    }
}

/// The window interface over the stored frame; no window exists until the backend is
/// implemented, so these members only read the fields.
impl AspectWindow for CocoaWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    /// Unmapped without a window.
    fn is_mapped(&self) -> bool {
        // TODO(platform): macOS — `[myHView window] isVisible`.
        false
    }
    fn map(&self) {
        // TODO(platform): macOS — show the window or view.
    }
    fn unmap(&self) {
        // TODO(platform): macOS — hide the window or view.
    }
    /// Unknown without a window.
    fn do_resize(&self) -> AspectTypeOfResize {
        // TODO(platform): macOS — compare the NSView frame with the stored corners.
        AspectTypeOfResize::Unknown
    }
    fn do_mapping(&self) -> bool {
        true
    }
    /// The width over the height of the stored frame, 1 for an empty frame.
    fn ratio(&self) -> f64 {
        let (width, height) = self.size();
        if width != 0 && height != 0 {
            f64::from(width) / f64::from(height)
        } else {
            1.0
        }
    }
    /// The stored corners.
    fn position(&self) -> (i32, i32, i32, i32) {
        let fields = *self.fields.read();
        (fields.x_left, fields.y_top, fields.x_right, fields.y_bottom)
    }
    /// The stored frame size.
    fn size(&self) -> (i32, i32) {
        let fields = *self.fields.read();
        (
            fields.x_right.wrapping_sub(fields.x_left),
            fields.y_bottom.wrapping_sub(fields.y_top),
        )
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
    fn set_title(&self, title: &AsciiString) {
        // TODO(platform): macOS — `[myHWindow setTitle:]`.
        let _ = title;
    }
    fn invalidate_content(&self, display: &Handle<AspectDisplayConnection>) {
        // TODO(platform): macOS — `setNeedsDisplay` on the main thread.
        let _ = display;
    }
}
