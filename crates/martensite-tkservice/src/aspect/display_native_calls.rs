//! Narrow connection I/O seam pins owned reset behavior without a live display server.
use super::AspectDisplayBackend;
use martensite_tkernel::tcollection::AsciiString;

pub(super) trait NativeDisplayCalls: Send + Sync {
    // Win32 opens a borrowed module binding; only Xlib interns display atoms.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    fn open(&self, name: &AsciiString) -> usize;
    fn close(&self, display: usize);
    fn free_visual(&self, visual: usize);
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    fn delete_atom(&self, display: usize) -> Option<u64>;
    fn backend(&self) -> AspectDisplayBackend {
        AspectDisplayBackend::X11
    }
}
pub(super) struct XlibDisplayCalls;
#[cfg(target_os = "linux")]
impl NativeDisplayCalls for XlibDisplayCalls {
    fn open(&self, name: &AsciiString) -> usize {
        let Ok(name) = std::ffi::CString::new(name.as_bytes()) else {
            return 0;
        };
        crate::xlib::xlib().map_or(0, |api| api.open_display(&name))
    }
    fn close(&self, display: usize) {
        if let Some(api) = crate::xlib::xlib() {
            api.close_display(display);
        }
    }
    fn free_visual(&self, visual: usize) {
        if let Some(api) = crate::xlib::xlib() {
            api.free(visual);
        }
    }
    fn delete_atom(&self, display: usize) -> Option<u64> {
        crate::xlib::xlib().map(|api| api.intern_atom(display, c"WM_DELETE_WINDOW", false))
    }
    fn backend(&self) -> AspectDisplayBackend {
        AspectDisplayBackend::X11
    }
}

#[cfg(target_os = "linux")]
pub(super) struct WaylandDisplayCalls;

#[cfg(target_os = "linux")]
impl NativeDisplayCalls for WaylandDisplayCalls {
    fn open(&self, name: &AsciiString) -> usize {
        let Some(api) = crate::wayland_client::wayland() else {
            return 0;
        };
        if name.is_empty() {
            return api.connect(None);
        }
        let Ok(socket) = std::ffi::CString::new(name.as_bytes()) else {
            return 0;
        };
        api.connect(Some(&socket))
    }
    fn close(&self, display: usize) {
        if let Some(api) = crate::wayland_client::wayland() {
            api.disconnect(display);
        }
    }
    fn free_visual(&self, _visual: usize) {}
    fn delete_atom(&self, _display: usize) -> Option<u64> {
        None
    }
    fn backend(&self) -> AspectDisplayBackend {
        AspectDisplayBackend::Wayland
    }
}
/// Without Xlib no display is ever opened (macOS stores a marker, other platforms raise
/// first), so these calls have nothing to release. On macOS they name AppKit, the window
/// system the connection's windows use.
#[cfg(not(any(target_os = "linux", windows)))]
impl NativeDisplayCalls for XlibDisplayCalls {
    fn open(&self, _name: &AsciiString) -> usize {
        0
    }
    fn close(&self, _display: usize) {}
    fn free_visual(&self, _visual: usize) {}
    fn delete_atom(&self, _display: usize) -> Option<u64> {
        None
    }
    fn backend(&self) -> AspectDisplayBackend {
        if cfg!(target_os = "macos") {
            AspectDisplayBackend::AppKit
        } else {
            AspectDisplayBackend::Unknown
        }
    }
}

// Win32 surfaces use the process HINSTANCE and an HWND, not an X server socket.
// GetModuleHandle borrows the executable module for the process lifetime.
#[cfg(windows)]
impl NativeDisplayCalls for XlibDisplayCalls {
    fn open(&self, _name: &AsciiString) -> usize {
        winsafe::HINSTANCE::GetModuleHandle(None).map_or(0, |instance| instance.ptr() as usize)
    }
    fn close(&self, _display: usize) {}
    fn free_visual(&self, _visual: usize) {}
    fn delete_atom(&self, _display: usize) -> Option<u64> {
        None
    }
    fn backend(&self) -> AspectDisplayBackend {
        AspectDisplayBackend::Win32
    }
}
