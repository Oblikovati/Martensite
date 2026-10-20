//! `WNT_Window`: a Win32 window (created here or wrapping an `HWND`).
//!
//! The Win32 backend is not implemented: both constructors raise `Standard_NotImplemented` on
//! every platform (see [`crate::platform`]).
// TODO(platform): Windows — register the window class, create or wrap the HWND, and implement
// Map/Unmap/DoResize/DoMapping, SetTitle, InvalidateContent and the raw input of WNT_Window.
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectHandle, AspectTypeOfResize,
    AspectWindow, AspectWindowBase,
};
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;

/// The Win32 backend named by the unsupported errors.
const BACKEND: &str = "Win32 (Windows)";

/// The fields of a Win32 window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WntWindowFields {
    /// `myHWindow`: the `HWND` address.
    pub h_window: AspectHandle,
    /// `myHParentWindow`: the parent `HWND` address.
    pub h_parent_window: AspectHandle,
    /// `aXLeft`, `aYTop`, `aXRight`, `aYBottom`: the window corners.
    pub corners: [i32; 4],
}

/// A Win32 window; `WntWindow::new("Viewer", 0, 0, 640, 480)` raises until implemented.
pub struct WntWindow {
    transient: StandardTransient,
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<WntWindowFields>,
}
martensite_tkernel::impl_transient!(WntWindow, "WNT_Window", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);

/// `WNT_WindowPtr`: a reference to a Win32 window.
pub type WntWindowPtr = Handle<WntWindow>;

impl WntWindow {
    /// `WNT_Window(title, class, style, left, top, width, height, ...)`: a new Win32 window;
    /// raises `Standard_NotImplemented` until the Win32 backend is implemented.
    /// Example: `WntWindow::new("Viewer", 0, 0, 640, 480)`.
    pub fn new(title: &str, left: i32, top: i32, width: i32, height: i32) -> Result<Self, Failure> {
        // TODO(platform): Windows — CreateWindowExW with the registered class.
        let _ = (title, left, top, width, height);
        Err(unsupported("WNT_Window", BACKEND))
    }
    /// `WNT_Window(HWND)`: a wrapper of an existing window; raises `Standard_NotImplemented`
    /// until the Win32 backend is implemented. Example: `WntWindow::from_handle(hwnd)`.
    pub fn from_handle(handle: AspectHandle) -> Result<Self, Failure> {
        // TODO(platform): Windows — read the HWND rectangle and parent.
        let _ = handle;
        Err(unsupported("WNT_Window", BACKEND))
    }
    /// `HWindow`: the `HWND` address; `window.h_window()`.
    pub fn h_window(&self) -> AspectHandle {
        self.fields.read().h_window
    }
}

/// The window interface over the stored rectangle; no window exists until the backend is
/// implemented, so these members only read the fields.
impl AspectWindow for WntWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    fn is_mapped(&self) -> bool {
        // TODO(platform): Windows — IsWindowVisible.
        false
    }
    fn map(&self) {
        // TODO(platform): Windows — ShowWindow.
    }
    fn unmap(&self) {
        // TODO(platform): Windows — ShowWindow(SW_HIDE).
    }
    fn do_resize(&self) -> AspectTypeOfResize {
        // TODO(platform): Windows — compare the window rectangle with the stored one.
        AspectTypeOfResize::Unknown
    }
    fn do_mapping(&self) -> bool {
        true
    }
    /// The width over the height of the stored rectangle, 1 for an empty one.
    fn ratio(&self) -> f64 {
        let (width, height) = self.size();
        if width != 0 && height != 0 {
            f64::from(width) / f64::from(height)
        } else {
            1.0
        }
    }
    fn position(&self) -> (i32, i32, i32, i32) {
        let [left, top, right, bottom] = self.fields.read().corners;
        (left, top, right, bottom)
    }
    fn size(&self) -> (i32, i32) {
        let [left, top, right, bottom] = self.fields.read().corners;
        (right.wrapping_sub(left), bottom.wrapping_sub(top))
    }
    fn native_handle(&self) -> AspectDrawable {
        self.h_window() as AspectDrawable
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        self.fields.read().h_parent_window as AspectDrawable
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        AspectFbConfig::default()
    }
    fn set_title(&self, title: &AsciiString) {
        // TODO(platform): Windows — SetWindowTextW.
        let _ = title;
    }
    fn invalidate_content(&self, display: &Handle<AspectDisplayConnection>) {
        // TODO(platform): Windows — InvalidateRect.
        let _ = display;
    }
}
