//! `Aspect_DisplayConnection`: the connection to the window system (Wayland or X11 on
//! Linux), shared by the windows and the graphic driver.
//!
//! On Linux both Wayland and X11 are supported in parallel. [`AspectDisplayConnection::new`]
//! prefers native Wayland over XWayland whenever `WAYLAND_DISPLAY` is set, and falls back
//! to X11 (`DISPLAY`) when Wayland is not active. Explicit constructors select either
//! backend directly. Windows retains the process HINSTANCE used by Win32 surfaces.
//! On other platforms initializing a connection raises `Standard_NotImplemented`
//! (see [`crate::platform`]).
#[cfg(target_os = "linux")]
use super::exceptions::display_connection_definition_error;
use super::{AspectFbConfig, AspectXAtom};
use martensite_tkernel::standard::{Failure, SharedState, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use std::collections::HashMap;
use std::sync::Arc;
#[path = "display_native_calls.rs"]
mod native_calls;
#[cfg(target_os = "linux")]
use native_calls::WaylandDisplayCalls;
use native_calls::{NativeDisplayCalls, XlibDisplayCalls};
#[path = "display_lifecycle_failure.rs"]
mod lifecycle_failure;
pub use lifecycle_failure::*;
#[path = "display_connection_lease.rs"]
mod connection_lease;
pub use connection_lease::AspectDisplayConnectionLease;

/// The window-system backend backing an [`AspectDisplayConnection`].
///
/// Example: `assert_eq!(connection.backend(), AspectDisplayBackend::Wayland);`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AspectDisplayBackend {
    /// Native Wayland display (`wl_display*`).
    Wayland,
    /// X11 / Xlib display (`Display*`).
    X11,
    /// Win32 process module handle (`HINSTANCE`).
    Win32,
    /// Browser WebAssembly / WebGPU display context (#632).
    WebBrowser,
    /// macOS AppKit: windows reach the window server through AppKit, not a display socket.
    AppKit,
    /// Unconnected or platform-neutral display.
    #[default]
    Unknown,
}

/// The locked fields of the connection.
#[derive(Debug, Default)]
struct ConnectionState {
    /// `myDisplay`: the `wl_display*`, `Display*` or `HINSTANCE` address, 0 for none.
    display: usize,
    /// The active window-system backend for `display`.
    backend: AspectDisplayBackend,
    /// `myDefVisualInfo`: the `XVisualInfo*` address, 0 for none.
    default_visual_info: usize,
    /// `myDefFBConfig`.
    default_fb_config: AspectFbConfig,
    /// `myAtoms`.
    atoms: HashMap<AspectXAtom, u64>,
    /// `myDisplayName`.
    display_name: AsciiString,
    /// `myIsOwnDisplay`: the connection opened the display and closes it.
    is_own_display: bool,
    leases: AspectDisplayLeaseCounts,
}

/// Selects the preferred Linux backend from environment values; Wayland always takes
/// precedence over XWayland when `WAYLAND_DISPLAY` is non-empty.
#[cfg(target_os = "linux")]
pub(crate) fn select_linux_backend(
    wayland_display: Option<&str>,
    x11_display: Option<&str>,
) -> AspectDisplayBackend {
    if wayland_display.is_some_and(|value| !value.is_empty()) {
        return AspectDisplayBackend::Wayland;
    }
    if x11_display.is_some_and(|value| !value.is_empty()) {
        return AspectDisplayBackend::X11;
    }
    AspectDisplayBackend::Wayland
}

/// A window-system connection; `AspectDisplayConnection::new()?` prefers Wayland over
/// XWayland on Linux, falls back to `$DISPLAY` for X11, or borrows the process module on Windows.
pub struct AspectDisplayConnection {
    state: SharedState<ConnectionState>,
    native: Arc<dyn NativeDisplayCalls>,
}
martensite_tkernel::impl_transient!(
    AspectDisplayConnection,
    "Aspect_DisplayConnection",
    StandardTransient
);

impl AspectDisplayConnection {
    fn with_name(display_name: AsciiString) -> Self {
        Self::with_native(display_name, Arc::new(XlibDisplayCalls))
    }
    #[cfg(target_os = "linux")]
    fn with_wayland_calls(display_name: AsciiString) -> Self {
        Self::with_native(display_name, Arc::new(WaylandDisplayCalls))
    }
    fn with_native(display_name: AsciiString, native: Arc<dyn NativeDisplayCalls>) -> Self {
        let state = ConnectionState {
            display_name,
            backend: native.backend(),
            ..Default::default()
        };
        Self {
            state: SharedState::new(state),
            native,
        }
    }
    /// Opens the preferred window-system connection for this host.
    ///
    /// On Linux, native Wayland (`WAYLAND_DISPLAY`) is preferred over XWayland (`DISPLAY`).
    /// When `WAYLAND_DISPLAY` is unset, X11 (`DISPLAY`) is used.
    ///
    /// Example: `let display = AspectDisplayConnection::new()?;`.
    pub fn new() -> Result<Self, Failure> {
        #[cfg(target_os = "linux")]
        {
            let wl = std::env::var("WAYLAND_DISPLAY").ok();
            let x11 = std::env::var("DISPLAY").ok();
            match select_linux_backend(wl.as_deref(), x11.as_deref()) {
                AspectDisplayBackend::Wayland => {
                    let name = wl.unwrap_or_default();
                    Self::with_wayland_name(&AsciiString::from(name.as_str()))
                }
                _ => {
                    let name = x11.unwrap_or_default();
                    Self::with_display_name(&AsciiString::from(name.as_str()))
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            #[cfg(not(windows))]
            let name = std::env::var("DISPLAY").unwrap_or_default();
            #[cfg(windows)]
            let name = String::new();
            Self::with_display_name(&AsciiString::from(name.as_str()))
        }
    }
    /// Opens a native Wayland display connection using `WAYLAND_DISPLAY` (or `wayland-0`).
    ///
    /// Example: `let display = AspectDisplayConnection::new_wayland()?;`.
    pub fn new_wayland() -> Result<Self, Failure> {
        let name = std::env::var("WAYLAND_DISPLAY").unwrap_or_default();
        Self::with_wayland_name(&AsciiString::from(name.as_str()))
    }
    /// Opens a native Wayland display connection to the named compositor socket.
    ///
    /// Example: `AspectDisplayConnection::with_wayland_name(&AsciiString::from("wayland-0"))?`.
    pub fn with_wayland_name(socket_name: &AsciiString) -> Result<Self, Failure> {
        #[cfg(target_os = "linux")]
        {
            let connection = Self::with_wayland_calls(socket_name.clone());
            connection.init(0)?;
            Ok(connection)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = socket_name;
            Err(crate::platform::unsupported(
                "Aspect_DisplayConnection",
                "Wayland (Linux)",
            ))
        }
    }
    /// Wraps an existing caller-owned `wl_display*` address without taking ownership.
    ///
    /// Example: `AspectDisplayConnection::with_wayland_display(wl_display_addr)?`.
    pub fn with_wayland_display(display: usize) -> Result<Self, Failure> {
        #[cfg(target_os = "linux")]
        {
            let connection = Self::with_wayland_calls(AsciiString::default());
            connection.init(display)?;
            Ok(connection)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = display;
            Err(crate::platform::unsupported(
                "Aspect_DisplayConnection",
                "Wayland (Linux)",
            ))
        }
    }
    /// Opens an X11 display connection using `DISPLAY`, even when Wayland is active.
    ///
    /// Example: `let display = AspectDisplayConnection::new_x11()?;`.
    pub fn new_x11() -> Result<Self, Failure> {
        let name = std::env::var("DISPLAY").unwrap_or_default();
        Self::with_display_name(&AsciiString::from(name.as_str()))
    }
    /// `Aspect_DisplayConnection(theDisplayName)`: opens the named X11 display.
    ///
    /// Example: `AspectDisplayConnection::with_display_name(&AsciiString::from(":0"))?`.
    pub fn with_display_name(display_name: &AsciiString) -> Result<Self, Failure> {
        let connection = Self::with_name(display_name.clone());
        connection.init(0)?;
        Ok(connection)
    }
    /// `Aspect_DisplayConnection(Aspect_XDisplay*)`: wraps an existing X11 display (the
    /// caller keeps ownership); `AspectDisplayConnection::with_display(address)?`.
    pub fn with_display(display: usize) -> Result<Self, Failure> {
        let connection = Self::with_name(AsciiString::default());
        connection.init(display)?;
        Ok(connection)
    }
    /// The window-system backend of this connection; `connection.backend()`.
    pub fn backend(&self) -> AspectDisplayBackend {
        self.state.read().backend
    }
    /// The native display address (`wl_display*`, `Display*` or `HINSTANCE`), 0 for none;
    /// `connection.get_display_aspect()`.
    pub fn get_display_aspect(&self) -> usize {
        self.state.read().display
    }
    /// True when the connection opened (and closes) the display; `connection.is_own_display()`.
    pub fn is_own_display(&self) -> bool {
        self.state.read().is_own_display
    }
    /// `GetAtom`: the interned atom; `Standard_NoSuchObject` when it was not interned, as
    /// `NCollection_DataMap::Find` raises. Example: `connection.get_atom(AspectXAtom::DeleteWindow)?`.
    pub fn get_atom(&self, atom: AspectXAtom) -> Result<u64, Failure> {
        self.state.read().atoms.get(&atom).copied().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::no_such_object(&format!(
                "NCollection_DataMap::Find: atom {atom:?} is not interned"
            ))
        })
    }
    /// The display name; `connection.get_display_name()`.
    pub fn get_display_name(&self) -> AsciiString {
        self.state.read().display_name.clone()
    }
    /// The default `XVisualInfo*` address, 0 for none; `connection.get_default_visual_info()`.
    pub fn get_default_visual_info(&self) -> usize {
        self.state.read().default_visual_info
    }
    /// The default frame-buffer configuration; `connection.get_default_fb_config()`.
    pub fn get_default_fb_config(&self) -> AspectFbConfig {
        self.state.read().default_fb_config
    }
    /// Replaces the default visual (freeing the previous one with `XFree`) and FB config.
    ///
    /// Example: `connection.set_default_visual_info(visual, fb_config)`.
    pub fn set_default_visual_info(&self, visual: usize, fb_config: AspectFbConfig) {
        let mut state = self.state.write();
        if state.default_visual_info != 0 {
            self.native.free_visual(state.default_visual_info);
        }
        state.default_visual_info = visual;
        state.default_fb_config = fb_config;
    }
    /// Legacy error adapter: refuses retained reset, then initializes when unleased.
    /// Structured cause is lost; migrate to `connection.init_checked(0)?`.
    /// Example: `connection.init(0)?`.
    pub fn init(&self, display: usize) -> Result<(), Failure> {
        self.init_checked(display)
            .map_err(AspectDisplayLifecycleFailure::into_legacy_failure)
    }
    /// Initialize with structured lifecycle diagnostics; `connection.init_checked(0)?`.
    pub fn init_checked(&self, display: usize) -> Result<(), AspectDisplayLifecycleFailure> {
        let mut state = self.state.write();
        let failure = self.lifecycle_failure(
            &state,
            AspectDisplayOperation::Initialize,
            display,
            AspectDisplayLifecycleCause::Retained,
        );
        if state.leases != AspectDisplayLeaseCounts::default() {
            return Err(failure);
        }
        platform::init(&mut state, display, self.native.as_ref()).map_err(|native| {
            AspectDisplayLifecycleFailure {
                cause: AspectDisplayLifecycleCause::Native(Box::new(native)),
                ..failure
            }
        })
    }
}

impl Drop for AspectDisplayConnection {
    fn drop(&mut self) {
        let state = self.state.read();
        if state.default_visual_info != 0 {
            self.native.free_visual(state.default_visual_info);
        }
        if state.display != 0 && state.is_own_display {
            self.native.close(state.display);
        }
    }
}

/// The Linux backend (`Wayland` or `X11`); ordering is pinned by the owned unleased fixtures.
#[cfg(target_os = "linux")]
mod platform {
    use super::{
        AspectDisplayBackend, AspectXAtom, ConnectionState, NativeDisplayCalls,
        display_connection_definition_error,
    };
    use martensite_tkernel::standard::Failure;
    pub(super) fn init(
        state: &mut ConnectionState,
        display: usize,
        native: &dyn NativeDisplayCalls,
    ) -> Result<(), Failure> {
        prepare_display(state, display, native);
        if state.display == 0 {
            return Err(connection_error(state));
        }
        state.backend = native.backend();
        if state.backend == AspectDisplayBackend::Wayland {
            state.is_own_display = display == 0;
            return Ok(());
        }
        let Some(atom) = native.delete_atom(state.display) else {
            return Err(connection_error(state));
        };
        state.is_own_display = display == 0;
        state.atoms.insert(AspectXAtom::DeleteWindow, atom);
        Ok(())
    }
    fn prepare_display(
        state: &mut ConnectionState,
        display: usize,
        native: &dyn NativeDisplayCalls,
    ) {
        if state.display != 0 && state.is_own_display {
            native.close(state.display);
        }
        state.is_own_display = false;
        state.atoms.clear();
        state.display = if display != 0 {
            display
        } else {
            native.open(&state.display_name)
        };
    }
    fn connection_error(state: &ConnectionState) -> Failure {
        let name = String::from_utf8_lossy(state.display_name.as_bytes());
        display_connection_definition_error(&format!("Can not connect to the server \"{name}\""))
    }
}

/// Browser WebAssembly display connection: no X11/Wayland socket exists, so
/// initialization succeeds with `AspectDisplayBackend::WebBrowser` (#632).
#[cfg(target_arch = "wasm32")]
mod platform {
    use super::{AspectDisplayBackend, ConnectionState, NativeDisplayCalls};
    use martensite_tkernel::standard::Failure;

    pub(super) fn init(
        state: &mut ConnectionState,
        display: usize,
        _native: &dyn NativeDisplayCalls,
    ) -> Result<(), Failure> {
        state.display = if display != 0 { display } else { 1 };
        state.backend = AspectDisplayBackend::WebBrowser;
        state.is_own_display = false;
        state.atoms.clear();
        Ok(())
    }
}

#[cfg(target_os = "macos")]
#[path = "display_appkit_platform.rs"]
mod platform;

/// The platforms without an implemented window-system backend: initializing raises
/// `Standard_NotImplemented` and leaves the connection without a display.
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    windows,
    target_arch = "wasm32"
)))]
mod platform {
    use super::{ConnectionState, NativeDisplayCalls};
    use martensite_tkernel::standard::Failure;

    pub(super) fn init(
        state: &mut ConnectionState,
        display: usize,
        _native: &dyn NativeDisplayCalls,
    ) -> Result<(), Failure> {
        let _ = (state, display);
        Err(crate::platform::unsupported(
            "Aspect_DisplayConnection",
            "window-system connection (non-Linux)",
        ))
    }
}

#[cfg(windows)]
mod platform {
    use super::*;
    pub(super) fn init(
        state: &mut ConnectionState,
        display: usize,
        native: &dyn NativeDisplayCalls,
    ) -> Result<(), Failure> {
        let instance = if display == 0 {
            native.open(&state.display_name)
        } else {
            display
        };
        if instance == 0 {
            return Err(
                super::super::exceptions::display_connection_definition_error(
                    "Win32 process module handle is null; expected a live HINSTANCE",
                ),
            );
        }
        state.display = instance;
        state.is_own_display = false;
        state.atoms.clear();
        Ok(())
    }
}

#[cfg(all(test, windows))]
#[path = "display_windows_tests.rs"]
mod windows_tests;

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn an_unreachable_display_raises_the_definition_error() {
        let Err(failure) = AspectDisplayConnection::with_display_name(&AsciiString::from(":9999"))
        else {
            panic!("display :9999 must not exist");
        };
        assert_eq!(
            failure.exception_type(),
            "Aspect_DisplayConnectionDefinitionError"
        );
        assert_eq!(failure.message(), "Can not connect to the server \":9999\"");
    }

    #[test]
    fn an_unreachable_wayland_socket_raises_the_definition_error() {
        let Err(failure) = AspectDisplayConnection::with_wayland_name(&AsciiString::from(
            "martensite-no-such-wayland-socket",
        )) else {
            panic!("unreachable Wayland socket must fail");
        };
        assert_eq!(
            failure.exception_type(),
            "Aspect_DisplayConnectionDefinitionError"
        );
        assert_eq!(
            failure.message(),
            "Can not connect to the server \"martensite-no-such-wayland-socket\""
        );
    }

    #[test]
    fn linux_backend_preference_favors_wayland_over_xwayland() {
        assert_eq!(
            select_linux_backend(Some("wayland-1"), Some(":0")),
            AspectDisplayBackend::Wayland
        );
        assert_eq!(
            select_linux_backend(Some("wayland-0"), None),
            AspectDisplayBackend::Wayland
        );
        assert_eq!(
            select_linux_backend(None, Some(":99")),
            AspectDisplayBackend::X11
        );
        assert_eq!(
            select_linux_backend(Some(""), Some(":0")),
            AspectDisplayBackend::X11
        );
        assert_eq!(
            select_linux_backend(None, None),
            AspectDisplayBackend::Wayland
        );
    }

    #[test]
    fn wrapped_wayland_display_preserves_backend_and_non_ownership() {
        let conn = AspectDisplayConnection::with_wayland_display(0x1234).unwrap();
        assert_eq!(conn.backend(), AspectDisplayBackend::Wayland);
        assert_eq!(conn.get_display_aspect(), 0x1234);
        assert!(!conn.is_own_display());
    }
}

#[cfg(all(test, target_os = "linux"))]
#[path = "display_connection_baseline.rs"]
mod baseline;

#[cfg(all(test, target_os = "linux"))]
#[path = "display_connection_lifecycle_tests.rs"]
mod lifecycle_tests;

#[cfg(all(test, target_os = "linux"))]
#[path = "display_lease_drop_tests.rs"]
mod lease_drop_tests;

#[cfg(all(test, target_os = "linux"))]
#[path = "display_lease_thread_tests.rs"]
mod lease_thread_tests;

#[cfg(all(test, target_os = "linux"))]
pub(crate) fn recorded_display_fixture(
    address: usize,
) -> martensite_tkernel::standard::Handle<AspectDisplayConnection> {
    let connection = baseline::fake_connection(baseline::RecordingDisplayCalls::new(address));
    connection.init_checked(0).unwrap();
    martensite_tkernel::standard::Handle::new(connection)
}
