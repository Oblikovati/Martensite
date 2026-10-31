//! `WNT_Window`: a Win32 window (created here or wrapping an `HWND`).
//!
//! Windows creates owned windows with a message-pumping owner thread or wraps an
//! application-owned HWND whose application supplies its message loop.
use crate::aspect::{
    AspectDisplayConnection, AspectDrawable, AspectFbConfig, AspectHandle, AspectTypeOfResize,
    AspectWindow, AspectWindowBase,
};
#[cfg(not(windows))]
use crate::platform::unsupported;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;

/// The Win32 backend named by the unsupported errors.
#[cfg(not(windows))]
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

/// A Win32 window wrapper; `WntWindow::from_handle(hwnd)` borrows an application HWND.
pub struct WntWindow {
    transient: StandardTransient,
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<WntWindowFields>,
    #[cfg(windows)]
    _owner: Option<super::owned::WindowOwner>,
    #[cfg(windows)]
    connection_lease: crate::aspect::AspectDisplayConnectionLease,
}
martensite_tkernel::impl_transient!(WntWindow, "WNT_Window", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);

/// `WNT_WindowPtr`: a reference to a Win32 window.
pub type WntWindowPtr = Handle<WntWindow>;

impl WntWindow {
    /// `WNT_Window(title, class, style, left, top, width, height, ...)`: a new Win32 window;
    /// creates and destroys the native window on a dedicated message-pumping thread.
    /// Example: `WntWindow::new("Viewer", 0, 0, 640, 480)`.
    pub fn new(title: &str, left: i32, top: i32, width: i32, height: i32) -> Result<Self, Failure> {
        #[cfg(windows)]
        {
            if width <= 0 || height <= 0 || title.contains('\0') {
                return Err(Failure::with_type(
                    "Standard_ConstructionError",
                    &format!(
                        "WNT_Window dimensions {width}x{height}, title {title:?}: expected positive dimensions and no NUL in the title"
                    ),
                    "",
                ));
            }
            let (owner, handle) =
                super::owned::WindowOwner::create(title, [left, top, width, height])?;
            let mut window = Self::from_handle(handle)?;
            window._owner = Some(owner);
            Ok(window)
        }
        #[cfg(not(windows))]
        {
            let _ = (title, left, top, width, height);
            Err(unsupported("WNT_Window", BACKEND))
        }
    }
    /// Borrows a live HWND without destroying it on drop; `WntWindow::from_handle(hwnd)`.
    /// The application keeps the native window and its message loop alive.
    pub fn from_handle(handle: AspectHandle) -> Result<Self, Failure> {
        #[cfg(windows)]
        {
            let fields = super::native::fields(handle)?;
            let display = Handle::new(AspectDisplayConnection::new()?);
            let lease = AspectDisplayConnection::acquire_lease(
                &display,
                crate::aspect::AspectDisplayLeasePurpose::Window,
            )
            .map_err(|error| error.into_legacy_failure())?;
            let base = AspectWindowBase::default();
            base.fields_mut().display = display;
            Ok(Self {
                transient: StandardTransient::default(),
                base,
                fields: SharedState::new(fields),
                _owner: None,
                connection_lease: lease,
            })
        }
        #[cfg(not(windows))]
        {
            let _ = handle;
            Err(unsupported("WNT_Window", BACKEND))
        }
    }
    /// `HWindow`: the `HWND` address; `window.h_window()`.
    pub fn h_window(&self) -> AspectHandle {
        self.fields.read().h_window
    }

    /// Opts this HWND into foreground multi-axis HID input; `window.enable_raw_input()?`.
    /// Windows delivers this process-wide device usage to the last registered HWND.
    pub fn enable_raw_input(&self) -> Result<(), Failure> {
        #[cfg(windows)]
        return super::raw_input::register(self.h_window());
        #[cfg(not(windows))]
        Err(unsupported("WNT raw input", BACKEND))
    }

    /// Copies an application's live WM_INPUT lParam before message cleanup;
    /// `let reports = WntWindow::read_raw_input(lparam as usize)?;`.
    pub fn read_raw_input(handle: usize) -> Result<Vec<super::WntRawHidReport>, Failure> {
        #[cfg(windows)]
        return super::raw_input::read(handle);
        #[cfg(not(windows))]
        {
            let _ = handle;
            Err(unsupported("WNT raw input", BACKEND))
        }
    }

    /// Drains owned-window HID input and delivery diagnostics; `window.take_raw_input()`.
    /// Borrowed HWNDs use their application's loop and `read_raw_input` instead.
    pub fn take_raw_input(&self) -> super::WntRawInputBatch {
        #[cfg(windows)]
        return self
            ._owner
            .as_ref()
            .map_or_else(Default::default, |owner| owner.take_raw_input());
        #[cfg(not(windows))]
        Default::default()
    }
}

/// The native window interface with a cached rectangle for border-change detection.
impl AspectWindow for WntWindow {
    #[cfg(windows)]
    fn native_display_connection(&self) -> Handle<AspectDisplayConnection> {
        self.connection_lease.retained_connection()
    }
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    fn is_mapped(&self) -> bool {
        #[cfg(windows)]
        return super::native::visible(self.h_window());
        #[cfg(not(windows))]
        false
    }
    fn map(&self) {
        #[cfg(windows)]
        super::native::show(self.h_window(), true);
    }
    fn unmap(&self) {
        #[cfg(windows)]
        super::native::show(self.h_window(), false);
    }
    fn do_resize(&self) -> AspectTypeOfResize {
        #[cfg(windows)]
        {
            let Ok(updated) = super::native::fields(self.h_window()) else {
                return AspectTypeOfResize::Unknown;
            };
            let mut fields = self.fields.write();
            let mode = resize_mode(fields.corners, updated.corners);
            *fields = updated;
            mode
        }
        #[cfg(not(windows))]
        {
            AspectTypeOfResize::Unknown
        }
    }
    fn do_mapping(&self) -> bool {
        #[cfg(windows)]
        return super::native::fields(self.h_window()).is_ok();
        #[cfg(not(windows))]
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
        #[cfg(windows)]
        super::native::title(self.h_window(), &String::from_utf8_lossy(title.as_bytes()));
        let _ = title;
    }
    fn invalidate_content(&self, display: &Handle<AspectDisplayConnection>) {
        #[cfg(windows)]
        super::native::invalidate(self.h_window());
        let _ = display;
    }
}

#[cfg(windows)]
fn resize_mode(before: [i32; 4], after: [i32; 4]) -> AspectTypeOfResize {
    use AspectTypeOfResize::*;
    let mask = before
        .into_iter()
        .zip(after)
        .enumerate()
        .fold(0, |mask, (index, (a, b))| {
            mask | (i32::from(a != b) << index)
        });
    match mask {
        0 => NoBorder,
        1 => LeftBorder,
        2 => TopBorder,
        4 => RightBorder,
        8 => BottomBorder,
        3 => LeftAndTopBorder,
        6 => TopAndRightBorder,
        9 => BottomAndLeftBorder,
        12 => RightAndBottomBorder,
        _ => Unknown,
    }
}

#[cfg(all(test, windows))]
mod tests {
    #[test]
    fn border_changes_match_rectangle_order() {
        use crate::aspect::AspectTypeOfResize::*;
        let before = [10, 20, 110, 120];
        let expected = [
            NoBorder,
            LeftBorder,
            TopBorder,
            LeftAndTopBorder,
            RightBorder,
            Unknown,
            TopAndRightBorder,
            Unknown,
            BottomBorder,
            BottomAndLeftBorder,
            Unknown,
            Unknown,
            RightAndBottomBorder,
            Unknown,
            Unknown,
            Unknown,
        ];
        for (mask, expected) in expected.into_iter().enumerate() {
            let after =
                std::array::from_fn(|index| before[index] + i32::from(mask & (1 << index) != 0));
            assert_eq!(super::resize_mode(before, after), expected);
        }
    }
}
