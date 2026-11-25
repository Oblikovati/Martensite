//! Wl: the Wayland window interface on platforms without the Wayland backend.
//!
//! Constructors raise `Standard_NotImplemented` (see [`crate::platform`]); members read
//! stored fields only.
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectTypeOfResize, AspectWindow,
    AspectWindowBase,
};
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, Handle, SharedState};

const BACKEND: &str = "Wayland (Linux)";

/// The protected fields of a Wayland window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WlWindowFields {
    /// The `wl_surface*` address, 0 when unset.
    pub wl_surface: AspectDrawable,
    /// Width in logical pixels.
    pub width: i32,
    /// Height in logical pixels.
    pub height: i32,
    /// `true` when mapped.
    pub is_mapped: bool,
    /// `true` when owned by this window.
    pub is_own_win: bool,
}

/// A Wayland window; `WlWindow::create(&display, "Viewer", 640, 480)` raises here.
pub struct WlWindow {
    base: AspectWindowBase,
    /// The fields shared by every handle.
    pub fields: SharedState<WlWindowFields>,
}
martensite_tkernel::impl_transient!(WlWindow, "Wl_Window", dyn AspectWindow, interfaces: [dyn AspectWindow]);

impl WlWindow {
    /// Creates a Wayland window; raises `Standard_NotImplemented` on non-Linux hosts.
    ///
    /// Example: `WlWindow::create(&display, "Viewer", 640, 480)`.
    pub fn create(
        display: &Handle<AspectDisplayConnection>,
        title: &str,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        let _ = (display, title, width, height);
        Err(unsupported("Wl_Window", BACKEND))
    }
    /// Wraps an external `wl_surface*`; raises `Standard_NotImplemented` on non-Linux hosts.
    ///
    /// Example: `WlWindow::wrap(&display, wl_surface, 640, 480)`.
    pub fn wrap(
        display: &Handle<AspectDisplayConnection>,
        wl_surface: AspectDrawable,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        let _ = (display, wl_surface, width, height);
        Err(unsupported("Wl_Window", BACKEND))
    }
    /// The `wl_surface*` address; `window.wl_surface()`.
    pub fn wl_surface(&self) -> AspectDrawable {
        self.fields.read().wl_surface
    }
}

impl AspectWindow for WlWindow {
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
    fn ratio(&self) -> f64 {
        let (w, h) = self.size();
        if w != 0 && h != 0 {
            f64::from(w) / f64::from(h)
        } else {
            1.0
        }
    }
    fn position(&self) -> (i32, i32, i32, i32) {
        let (w, h) = self.size();
        (0, 0, w, h)
    }
    fn size(&self) -> (i32, i32) {
        let f = *self.fields.read();
        (f.width, f.height)
    }
    fn native_handle(&self) -> AspectDrawable {
        self.wl_surface()
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        0
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        0
    }
}
