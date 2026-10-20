//! Xw: the Xlib window interface on platforms without the Xlib backend.
//!
//! The constructors raise `Standard_NotImplemented` (see [`crate::platform`]); the members
//! only read the stored fields. The raster canvas and X event dispatch are Linux-only.
// TODO(platform): macOS with XQuartz — load Xlib and share the Linux Xw implementation.
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectTypeOfResize, AspectWindow,
    AspectWindowBase,
};
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};

/// The Xlib backend named by the unsupported errors.
const BACKEND: &str = "Xlib (Linux)";

/// The protected fields of an X window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XwWindowFields {
    /// `myXWindow`: the Xlib window id.
    pub x_window: AspectDrawable,
    /// `myFBConfig`: the GLX framebuffer configuration.
    pub fb_config: AspectFbConfig,
    /// `myXLeft`.
    pub x_left: i32,
    /// `myYTop`.
    pub y_top: i32,
    /// `myXRight`.
    pub x_right: i32,
    /// `myYBottom`.
    pub y_bottom: i32,
    /// `myIsOwnWin`: the window was created here and is destroyed with it.
    pub is_own_win: bool,
}

/// An Xlib window; `XwWindow::create(&display, "Viewer", 0, 0, 640, 480)` raises here.
pub struct XwWindow {
    transient: StandardTransient,
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<XwWindowFields>,
}
martensite_tkernel::impl_transient!(XwWindow, "Xw_Window", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);

impl XwWindow {
    /// `Xw_Window(display, title, left, top, width, height)`; raises `Standard_NotImplemented`
    /// without the Xlib backend. Example: `XwWindow::create(&display, "Viewer", 0, 0, 640, 480)`.
    pub fn create(
        display: &Handle<AspectDisplayConnection>,
        title: &str,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        let _ = (display, title, left, top, width, height);
        Err(unsupported("Xw_Window", BACKEND))
    }
    /// `Xw_Window(display, xWindow, fbConfig)`; raises `Standard_NotImplemented` without the
    /// Xlib backend. Example: `XwWindow::wrap(&display, xid, 0)`.
    pub fn wrap(
        display: &Handle<AspectDisplayConnection>,
        x_window: AspectDrawable,
        fb_config: AspectFbConfig,
    ) -> Result<Self, Failure> {
        let _ = (display, x_window, fb_config);
        Err(unsupported("Xw_Window", BACKEND))
    }
    /// `XWindow`; `window.x_window()`.
    pub fn x_window(&self) -> AspectDrawable {
        self.fields.read().x_window
    }
}

/// The window interface over the stored corners; no window exists without the backend.
impl AspectWindow for XwWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    fn is_mapped(&self) -> bool {
        false
    }
    fn map(&self) {}
    fn unmap(&self) {}
    fn do_resize(&self) -> AspectTypeOfResize {
        AspectTypeOfResize::Unknown
    }
    fn do_mapping(&self) -> bool {
        true
    }
    /// The width over the height of the stored corners, 1 for an empty frame.
    fn ratio(&self) -> f64 {
        let (width, height) = self.size();
        if width != 0 && height != 0 {
            f64::from(width) / f64::from(height)
        } else {
            1.0
        }
    }
    fn position(&self) -> (i32, i32, i32, i32) {
        let fields = *self.fields.read();
        (fields.x_left, fields.y_top, fields.x_right, fields.y_bottom)
    }
    fn size(&self) -> (i32, i32) {
        let fields = *self.fields.read();
        (
            fields.x_right.wrapping_sub(fields.x_left),
            fields.y_bottom.wrapping_sub(fields.y_top),
        )
    }
    fn native_handle(&self) -> AspectDrawable {
        self.x_window()
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        0
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        self.fields.read().fb_config
    }
}
