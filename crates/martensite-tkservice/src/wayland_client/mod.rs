//! Dynamic loader and minimal C ABI wrapper for `libwayland-client.so.0`.
//!
//! Loaded at run time through [`crate::dynlib::DynamicLibrary`], so the crate builds
//! without Wayland development headers and runs on both Wayland and X11-only systems.
#![allow(unsafe_code)]

pub(crate) mod protocol;
pub(crate) mod xkb;

use crate::dynlib::{DynamicLibrary, bind_symbols, cast_symbol};
use std::ffi::{CStr, c_char, c_int, c_void};
use std::ptr;
use std::sync::OnceLock;

/// Flag passed to `wl_proxy_marshal_array_flags` to destroy the proxy on send.
pub(crate) const WL_MARSHAL_FLAG_DESTROY: u32 = 1;

/// Opaque `struct wl_display`.
#[repr(C)]
pub(crate) struct WlDisplay {
    _private: [u8; 0],
}

/// Opaque `struct wl_proxy`.
#[repr(C)]
pub(crate) struct WlProxy {
    _private: [u8; 0],
}

/// Wayland wire array (`struct wl_array`).
#[repr(C)]
pub(crate) struct WlArray {
    pub(crate) size: usize,
    pub(crate) alloc: usize,
    pub(crate) data: *mut c_void,
}

/// Wayland protocol message descriptor (`struct wl_message`).
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct WlMessage {
    pub(crate) name: *const c_char,
    pub(crate) signature: *const c_char,
    pub(crate) types: *const *const WlInterface,
}

/// Wayland protocol interface descriptor (`struct wl_interface`).
#[repr(C)]
pub(crate) struct WlInterface {
    pub(crate) name: *const c_char,
    pub(crate) version: c_int,
    pub(crate) method_count: c_int,
    pub(crate) methods: *const WlMessage,
    pub(crate) event_count: c_int,
    pub(crate) events: *const WlMessage,
}

// SAFETY: WlInterface descriptors are immutable static tables owned for the process lifetime.
unsafe impl Sync for WlInterface {}
// SAFETY: WlMessage descriptors are immutable static tables owned for the process lifetime.
unsafe impl Sync for WlMessage {}

/// Wayland marshalling argument (`union wl_argument`).
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union WlArgument {
    pub(crate) i: i32,
    pub(crate) u: u32,
    pub(crate) s: *const c_char,
    pub(crate) o: *mut WlProxy,
    pub(crate) n: u32,
}

/// Raw function pointers resolved from `libwayland-client.so.0`.
#[allow(non_snake_case)]
struct WaylandSymbols {
    wl_display_connect: unsafe extern "C" fn(*const c_char) -> *mut WlDisplay,
    wl_display_disconnect: unsafe extern "C" fn(*mut WlDisplay),
    wl_display_roundtrip: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_dispatch_pending: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_flush: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_prepare_read: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_read_events: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_cancel_read: unsafe extern "C" fn(*mut WlDisplay),
    wl_display_get_fd: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_display_get_error: unsafe extern "C" fn(*mut WlDisplay) -> c_int,
    wl_proxy_marshal_array_flags: unsafe extern "C" fn(
        *mut WlProxy,
        u32,
        *const WlInterface,
        u32,
        u32,
        *mut WlArgument,
    ) -> *mut WlProxy,
    wl_proxy_add_listener: unsafe extern "C" fn(
        *mut WlProxy,
        *mut Option<unsafe extern "C" fn()>,
        *mut c_void,
    ) -> c_int,
    wl_proxy_destroy: unsafe extern "C" fn(*mut WlProxy),
    wl_proxy_get_version: unsafe extern "C" fn(*mut WlProxy) -> u32,
}

/// Core `wl_interface` pointers exported by `libwayland-client.so.0`.
pub(crate) struct CoreInterfaces {
    pub(crate) registry: *const WlInterface,
    pub(crate) compositor: *const WlInterface,
    pub(crate) surface: *const WlInterface,
    pub(crate) seat: *const WlInterface,
    pub(crate) pointer: *const WlInterface,
    pub(crate) keyboard: *const WlInterface,
    pub(crate) output: *const WlInterface,
}

// SAFETY: Core interface symbols live in libwayland-client for the entire process lifetime.
unsafe impl Send for CoreInterfaces {}
// SAFETY: Core interface symbols are read-only descriptors in libwayland-client.
unsafe impl Sync for CoreInterfaces {}

/// Loaded `libwayland-client.so.0` entry points and core protocol descriptors.
pub(crate) struct WaylandApi {
    sym: WaylandSymbols,
    pub(crate) core: CoreInterfaces,
}

// SAFETY: Function pointers and immutable interface symbols are safe to share across threads.
unsafe impl Send for WaylandApi {}
// SAFETY: Function pointers and immutable interface symbols are safe to share across threads.
unsafe impl Sync for WaylandApi {}

/// Returns the loaded Wayland client library, or `None` when absent.
pub(crate) fn wayland() -> Option<&'static WaylandApi> {
    static LOADED: OnceLock<Option<WaylandApi>> = OnceLock::new();
    LOADED.get_or_init(load_wayland).as_ref()
}

fn load_wayland() -> Option<WaylandApi> {
    let lib = DynamicLibrary::open(&["libwayland-client.so.0", "libwayland-client.so"])?;
    let sym = bind_symbols!(
        &lib,
        WaylandSymbols {
            wl_display_connect,
            wl_display_disconnect,
            wl_display_roundtrip,
            wl_display_dispatch_pending,
            wl_display_flush,
            wl_display_prepare_read,
            wl_display_read_events,
            wl_display_cancel_read,
            wl_display_get_fd,
            wl_display_get_error,
            wl_proxy_marshal_array_flags,
            wl_proxy_add_listener,
            wl_proxy_destroy,
            wl_proxy_get_version,
        }
    )?;
    let core = load_core_interfaces(&lib)?;
    Some(WaylandApi { sym, core })
}

fn load_core_interfaces(lib: &DynamicLibrary) -> Option<CoreInterfaces> {
    let iface = |name| -> Option<*const WlInterface> {
        let addr = lib.symbol(name)?;
        // SAFETY: libwayland-client exports each named symbol as a `const struct wl_interface`.
        Some(unsafe { cast_symbol::<*const WlInterface>(addr) })
    };
    Some(CoreInterfaces {
        registry: iface("wl_registry_interface")?,
        compositor: iface("wl_compositor_interface")?,
        surface: iface("wl_surface_interface")?,
        seat: iface("wl_seat_interface")?,
        pointer: iface("wl_pointer_interface")?,
        keyboard: iface("wl_keyboard_interface")?,
        output: iface("wl_output_interface")?,
    })
}

impl WaylandApi {
    /// Connects to a Wayland compositor (`None` uses `$WAYLAND_DISPLAY` or `wayland-0`).
    pub(crate) fn connect(&self, name: Option<&CStr>) -> usize {
        let ptr = name.map_or(ptr::null(), CStr::as_ptr);
        // SAFETY: `ptr` is either null or a live NUL-terminated socket name.
        unsafe { (self.sym.wl_display_connect)(ptr) as usize }
    }

    /// Disconnects a display opened by [`Self::connect`].
    pub(crate) fn disconnect(&self, display: usize) {
        if display == 0 {
            return;
        }
        // SAFETY: Caller owns this `wl_display` handle and disconnects it once.
        unsafe { (self.sym.wl_display_disconnect)(display as *mut WlDisplay) };
    }

    /// Processes all pending requests and events up to a server sync point.
    pub(crate) fn roundtrip(&self, display: usize) -> c_int {
        // SAFETY: `display` is a live `wl_display` connection.
        unsafe { (self.sym.wl_display_roundtrip)(display as *mut WlDisplay) }
    }

    /// Flushes buffered outgoing requests to the compositor socket.
    pub(crate) fn flush(&self, display: usize) -> c_int {
        // SAFETY: `display` is a live `wl_display` connection.
        unsafe { (self.sym.wl_display_flush)(display as *mut WlDisplay) }
    }

    /// Reads any immediately readable events from the socket without blocking.
    pub(crate) fn poll_and_read(&self, display: usize) -> c_int {
        let dpy = display as *mut WlDisplay;
        // SAFETY: `dpy` is a live `wl_display` connection; prepare/cancel/read follow Wayland's protocol.
        unsafe {
            while (self.sym.wl_display_prepare_read)(dpy) != 0 {
                if (self.sym.wl_display_dispatch_pending)(dpy) < 0 {
                    return -1;
                }
            }
            if (self.sym.wl_display_flush)(dpy) < 0 {
                (self.sym.wl_display_cancel_read)(dpy);
                return -1;
            }
            let fd = (self.sym.wl_display_get_fd)(dpy);
            let mut pfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            if libc::poll(&mut pfd, 1, 0) > 0 && (pfd.revents & libc::POLLIN) != 0 {
                let res = (self.sym.wl_display_read_events)(dpy);
                if res < 0 {
                    return res;
                }
            } else {
                (self.sym.wl_display_cancel_read)(dpy);
            }
            (self.sym.wl_display_dispatch_pending)(dpy)
        }
    }

    /// Returns the last protocol or transport error code on `display`.
    pub(crate) fn get_error(&self, display: usize) -> c_int {
        // SAFETY: `display` is a live `wl_display` connection.
        unsafe { (self.sym.wl_display_get_error)(display as *mut WlDisplay) }
    }

    /// Marshals a request on `proxy` using `wl_proxy_marshal_array_flags`.
    ///
    /// # Safety
    /// `proxy` must be a live `wl_proxy` (or `wl_display` cast to `wl_proxy`), `opcode`
    /// and `args` must match the interface's request signature, and `interface` must be
    /// null or a valid static `wl_interface` descriptor.
    pub(crate) unsafe fn marshal(
        &self,
        proxy: *mut WlProxy,
        opcode: u32,
        interface: *const WlInterface,
        version: u32,
        flags: u32,
        args: &mut [WlArgument],
    ) -> *mut WlProxy {
        let args_ptr = if args.is_empty() {
            ptr::null_mut()
        } else {
            args.as_mut_ptr()
        };
        // SAFETY: Preconditions are upheld by the caller per this function's safety contract.
        unsafe {
            (self.sym.wl_proxy_marshal_array_flags)(
                proxy, opcode, interface, version, flags, args_ptr,
            )
        }
    }

    /// Attaches a C listener function-pointer table to `proxy`.
    ///
    /// # Safety
    /// `proxy` must be live, `listener` must remain valid for the proxy's lifetime with
    /// function signatures matching the proxy's events, and `data` must stay valid until
    /// the proxy is destroyed.
    pub(crate) unsafe fn add_listener(
        &self,
        proxy: *mut WlProxy,
        listener: *mut Option<unsafe extern "C" fn()>,
        data: *mut c_void,
    ) -> c_int {
        // SAFETY: Preconditions are upheld by the caller per this function's safety contract.
        unsafe { (self.sym.wl_proxy_add_listener)(proxy, listener, data) }
    }

    /// Destroys a client-side `wl_proxy` without sending a protocol request.
    ///
    /// # Safety
    /// `proxy` must be a live `wl_proxy` not already destroyed.
    pub(crate) unsafe fn proxy_destroy(&self, proxy: *mut WlProxy) {
        if !proxy.is_null() {
            // SAFETY: Caller guarantees `proxy` is live and destroyed once.
            unsafe { (self.sym.wl_proxy_destroy)(proxy) };
        }
    }

    /// Returns the negotiated protocol version of `proxy`.
    ///
    /// # Safety
    /// `proxy` must be a live `wl_proxy`.
    pub(crate) unsafe fn proxy_version(&self, proxy: *mut WlProxy) -> u32 {
        // SAFETY: Caller guarantees `proxy` is live.
        unsafe { (self.sym.wl_proxy_get_version)(proxy) }
    }
}

/// Converts a Wayland 24.8 fixed-point coordinate to `f64`.
pub(crate) fn wl_fixed_to_f64(fixed: i32) -> f64 {
    f64::from(fixed) / 256.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_conversion_decodes_positive_and_negative_values() {
        assert_eq!(wl_fixed_to_f64(256), 1.0);
        assert_eq!(wl_fixed_to_f64(-128), -0.5);
        assert_eq!(wl_fixed_to_f64(0), 0.0);
    }

    #[test]
    fn unreachable_wayland_socket_returns_zero() {
        let Some(api) = wayland() else {
            return;
        };
        assert_eq!(api.connect(Some(c"martensite-no-such-wayland-socket")), 0);
    }
}
