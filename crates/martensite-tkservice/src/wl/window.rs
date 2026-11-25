//! `Wl_Window`: a native Wayland `xdg-shell` window, created here or wrapping an external `wl_surface`.
#![allow(unsafe_code)]

use super::events::{WlEvent, dispatch_event, virtual_key_from_keysym};
use super::listeners::{
    attach_decoration_listener, attach_registry_listener, attach_surface_listener,
    attach_xdg_surface_listener, attach_xdg_toplevel_listener,
};
use super::state::{WlCallbackTarget, destroy_proxies, send_empty, send_one};
use crate::aspect::exceptions::window_definition_error;
use crate::aspect::{
    AspectDisplayBackend, AspectDisplayConnection, AspectDisplayConnectionLease,
    AspectDisplayLeasePurpose, AspectDisplayLifecycleFailure, AspectDrawable, AspectFbConfig,
    AspectNativeWindowClosure, AspectNativeWindowClosureCause, AspectTypeOfResize as Resize,
    AspectVKey, AspectWindow, AspectWindowBase, AspectWindowInputListener,
    AspectWindowMembers as _,
};
use crate::wayland_client::{WaylandApi, WlArgument, WlProxy, protocol::xdg_protocol, wayland};
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkernel::tcollection::AsciiString;
use std::ffi::CString;
use std::ptr;

/// The protected fields of a Wayland window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WlWindowFields {
    /// The `wl_surface*` address exposed as [`AspectDrawable`], 0 when retired.
    pub wl_surface: AspectDrawable,
    /// Current width in logical pixels.
    pub width: i32,
    /// Current height in logical pixels.
    pub height: i32,
    /// `true` when mapped on screen.
    pub is_mapped: bool,
    /// `true` when this window created and owns the `wl_surface`.
    pub is_own_win: bool,
}

/// A native Wayland window; `WlWindow::create(&display, "Viewer", 640, 480)?`.
pub struct WlWindow {
    base: AspectWindowBase,
    /// The fields shared by every handle.
    pub fields: SharedState<WlWindowFields>,
    native_closure: SharedState<Option<AspectNativeWindowClosure>>,
    target: Box<WlCallbackTarget>,
    connection_lease: Option<AspectDisplayConnectionLease>,
}
martensite_tkernel::impl_transient!(WlWindow, "Wl_Window", dyn AspectWindow, interfaces: [dyn AspectWindow]);

impl Drop for WlWindow {
    fn drop(&mut self) {
        let is_own = self.fields.read().is_own_win;
        if !is_own {
            return;
        }
        let Some(api) = wayland() else { return };
        if let Ok(mut inner) = self.target.inner.lock() {
            destroy_proxies(api, &mut inner.proxies);
        }
        let display = self.display_address();
        if display != 0 {
            api.flush(display);
        }
    }
}

impl WlWindow {
    /// Creates an owned Wayland `xdg_toplevel` window on `display`.
    ///
    /// Example: `WlWindow::create(&display, "Viewer", 640, 480)?`.
    pub fn create(
        display: &Handle<AspectDisplayConnection>,
        title: &str,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        if width <= 0 || height <= 0 {
            return Err(window_definition_error(&format!(
                "Wl_Window, Coordinate(s) out of range: {width}x{height}; expected a positive size"
            )));
        }
        let lease = retain_wayland_connection(display)?;
        let api = wayland().ok_or_else(|| {
            window_definition_error("Wl_Window, libwayland-client.so.0 is unavailable")
        })?;
        let mut target = WlCallbackTarget::new(width, height, true);
        setup_xdg_window(
            api,
            lease.display_address(),
            &mut target,
            title,
            width,
            height,
        )?;
        let surface_addr = surface_address(&target);
        Ok(Self::finish(
            display,
            lease,
            target,
            surface_addr,
            width,
            height,
            true,
        ))
    }

    /// Wraps an externally owned `wl_surface*` on `display` without taking ownership.
    ///
    /// Example: `WlWindow::wrap(&display, wl_surface_addr, 640, 480)?`.
    pub fn wrap(
        display: &Handle<AspectDisplayConnection>,
        wl_surface: AspectDrawable,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        if wl_surface == 0 {
            return Err(window_definition_error(
                "Wl_Window, given invalid wl_surface",
            ));
        }
        if width <= 0 || height <= 0 {
            return Err(window_definition_error(&format!(
                "Wl_Window, Coordinate(s) out of range: {width}x{height}; expected a positive size"
            )));
        }
        let lease = retain_wayland_connection(display)?;
        let target = WlCallbackTarget::new(width, height, false);
        if let Ok(mut inner) = target.inner.lock() {
            inner.proxies.surface = wl_surface as *mut WlProxy;
        }
        Ok(Self::finish(
            display, lease, target, wl_surface, width, height, false,
        ))
    }

    fn finish(
        display: &Handle<AspectDisplayConnection>,
        lease: AspectDisplayConnectionLease,
        target: Box<WlCallbackTarget>,
        wl_surface: AspectDrawable,
        width: i32,
        height: i32,
        is_own_win: bool,
    ) -> Self {
        let fields = WlWindowFields {
            wl_surface,
            width,
            height,
            is_mapped: false,
            is_own_win,
        };
        let window = Self {
            base: AspectWindowBase::default(),
            fields: SharedState::new(fields),
            native_closure: SharedState::default(),
            target,
            connection_lease: Some(lease),
        };
        window.base.fields_mut().display = display.clone();
        window
    }

    /// Returns the `wl_surface*` address as [`AspectDrawable`]; `window.wl_surface()`.
    pub fn wl_surface(&self) -> AspectDrawable {
        self.fields.read().wl_surface
    }

    /// Translates an XKB keysym into an [`AspectVKey`]; `WlWindow::virtual_key_from_native(0x61)`.
    pub fn virtual_key_from_native(keysym: u32) -> AspectVKey {
        virtual_key_from_keysym(keysym)
    }

    /// Pumps pending Wayland socket events and delivers them to `listener`.
    ///
    /// Example: `window.dispatch_events(&mut listener)?;`.
    pub fn dispatch_events(
        &self,
        listener: &mut dyn AspectWindowInputListener,
    ) -> Result<usize, Failure> {
        let display = self.display_address();
        if let (Some(api), true) = (wayland(), display != 0) {
            api.poll_and_read(display);
        }
        let mut count = 0;
        while let Some(event) = self.pop_event() {
            if self.process_event(listener, event)? {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Processes a single [`WlEvent`] and forwards it to `listener`.
    ///
    /// Example: `window.process_event(&mut listener, WlEvent::Expose)?;`.
    pub fn process_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: WlEvent,
    ) -> Result<bool, Failure> {
        if matches!(event, WlEvent::Close) {
            return Ok(self.retire_closed_window(listener));
        }
        if self.native_window_closure().is_some() {
            return Ok(false);
        }
        if let WlEvent::Configure { width, height } = event
            && width > 0
            && height > 0
        {
            let mut fields = self.fields.write();
            fields.width = width;
            fields.height = height;
        }
        dispatch_event(listener, event)
    }

    fn pop_event(&self) -> Option<WlEvent> {
        self.target.inner.lock().ok()?.events.pop_front()
    }

    fn retire_closed_window(&self, listener: &mut dyn AspectWindowInputListener) -> bool {
        let mut fields = self.fields.write();
        let mut closure = self.native_closure.write();
        if closure.is_some() {
            return false;
        }
        let original = fields.wl_surface;
        *closure = Some(AspectNativeWindowClosure {
            original_handle: original,
            cause: AspectNativeWindowClosureCause::DestroyNotify,
            behavior_version: 1,
        });
        fields.wl_surface = 0;
        fields.is_mapped = false;
        drop(closure);
        drop(fields);
        listener.process_close();
        true
    }

    fn display_address(&self) -> usize {
        self.connection_lease
            .as_ref()
            .map_or(0, AspectDisplayConnectionLease::display_address)
    }
}

fn retain_wayland_connection(
    display: &Handle<AspectDisplayConnection>,
) -> Result<AspectDisplayConnectionLease, Failure> {
    let Some(connection) = display.get() else {
        return Err(window_definition_error(
            "Wl_Window, Wayland Display connection is undefined",
        ));
    };
    if connection.backend() != AspectDisplayBackend::Wayland {
        return Err(window_definition_error(&format!(
            "Wl_Window, received {:?} display connection; expected AspectDisplayBackend::Wayland",
            connection.backend()
        )));
    }
    AspectDisplayConnection::acquire_lease(display, AspectDisplayLeasePurpose::Window)
        .map_err(AspectDisplayLifecycleFailure::into_legacy_failure)
}

fn setup_xdg_window(
    api: &WaylandApi,
    display: usize,
    target: &mut Box<WlCallbackTarget>,
    title: &str,
    width: i32,
    height: i32,
) -> Result<(), Failure> {
    let target_ptr: *mut WlCallbackTarget = &mut **target;
    bind_registry(api, display, target_ptr)?;
    let (compositor, wm_base, decor_mgr) = {
        let inner = target.inner.lock().map_err(|_| {
            window_definition_error("Wl_Window, internal Wayland state lock poisoned")
        })?;
        (
            inner.proxies.compositor,
            inner.proxies.xdg_wm_base,
            inner.proxies.decoration_manager,
        )
    };
    if compositor.is_null() || wm_base.is_null() {
        cleanup_on_error(api, target);
        return Err(window_definition_error(
            "Wl_Window, Wayland compositor does not advertise wl_compositor and xdg_wm_base",
        ));
    }
    create_shell_objects(
        api, compositor, wm_base, decor_mgr, target, target_ptr, title, width, height,
    )?;
    if api.roundtrip(display) < 0 || api.get_error(display) != 0 {
        cleanup_on_error(api, target);
        return Err(window_definition_error(
            "Wl_Window, Wayland initial roundtrip failed",
        ));
    }
    Ok(())
}

fn bind_registry(
    api: &WaylandApi,
    display: usize,
    target_ptr: *mut WlCallbackTarget,
) -> Result<(), Failure> {
    let mut args = [WlArgument { n: 0 }];
    // SAFETY: `display` is a live `wl_display`; opcode 1 is `wl_display::get_registry(n)`.
    let registry = unsafe {
        api.marshal(
            display as *mut WlProxy,
            1,
            api.core.registry,
            1,
            0,
            &mut args,
        )
    };
    if registry.is_null() {
        return Err(window_definition_error(
            "Wl_Window, failed to create wl_registry",
        ));
    }
    attach_registry_listener(api, registry, target_ptr);
    // SAFETY: `target_ptr` is owned by the caller of `setup_xdg_window`.
    if let Ok(mut inner) = unsafe { &*target_ptr }.inner.lock() {
        inner.proxies.registry = registry;
    }
    if api.roundtrip(display) < 0 {
        return Err(window_definition_error(
            "Wl_Window, wl_registry roundtrip failed",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn create_shell_objects(
    api: &WaylandApi,
    compositor: *mut WlProxy,
    wm_base: *mut WlProxy,
    decor_mgr: *mut WlProxy,
    target: &mut Box<WlCallbackTarget>,
    target_ptr: *mut WlCallbackTarget,
    title: &str,
    width: i32,
    height: i32,
) -> Result<(), Failure> {
    let proto = xdg_protocol(&api.core);
    let surface = create_proxy(
        api,
        compositor,
        0,
        api.core.surface,
        &mut [WlArgument { n: 0 }],
    )?;
    attach_surface_listener(api, surface, target_ptr);
    let xdg_surface = create_proxy(
        api,
        wm_base,
        2,
        proto.surface,
        &mut [WlArgument { n: 0 }, WlArgument { o: surface }],
    )?;
    attach_xdg_surface_listener(api, xdg_surface, target_ptr);
    let xdg_toplevel = create_proxy(
        api,
        xdg_surface,
        1,
        proto.toplevel,
        &mut [WlArgument { n: 0 }],
    )?;
    attach_xdg_toplevel_listener(api, xdg_toplevel, target_ptr);
    let decoration = request_server_decorations(api, decor_mgr, xdg_toplevel, target_ptr);
    configure_toplevel_metadata(api, xdg_surface, xdg_toplevel, title, width, height);
    if let Ok(mut inner) = target.inner.lock() {
        inner.proxies.surface = surface;
        inner.proxies.xdg_surface = xdg_surface;
        inner.proxies.xdg_toplevel = xdg_toplevel;
        inner.proxies.decoration = decoration;
    }
    // Opcode 6 is `wl_surface::commit()`.
    send_empty(api, surface, 6);
    Ok(())
}

fn create_proxy(
    api: &WaylandApi,
    parent: *mut WlProxy,
    opcode: u32,
    iface: *const crate::wayland_client::WlInterface,
    args: &mut [WlArgument],
) -> Result<*mut WlProxy, Failure> {
    // SAFETY: Caller passes a live parent proxy, valid opcode, target interface and args.
    let proxy = unsafe {
        let ver = api.proxy_version(parent);
        api.marshal(parent, opcode, iface, ver, 0, args)
    };
    if proxy.is_null() {
        return Err(window_definition_error(
            "Wl_Window, failed to create Wayland shell object",
        ));
    }
    Ok(proxy)
}

fn request_server_decorations(
    api: &WaylandApi,
    decor_mgr: *mut WlProxy,
    xdg_toplevel: *mut WlProxy,
    target_ptr: *mut WlCallbackTarget,
) -> *mut WlProxy {
    if decor_mgr.is_null() {
        return ptr::null_mut();
    }
    let proto = xdg_protocol(&api.core);
    let mut args = [WlArgument { n: 0 }, WlArgument { o: xdg_toplevel }];
    // SAFETY: `decor_mgr` and `xdg_toplevel` are live; opcode 1 is `get_toplevel_decoration(no)`.
    let decor = unsafe {
        let ver = api.proxy_version(decor_mgr);
        api.marshal(decor_mgr, 1, proto.toplevel_decoration, ver, 0, &mut args)
    };
    if !decor.is_null() {
        attach_decoration_listener(api, decor, target_ptr);
        // Mode 2 = ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE.
        send_one(api, decor, 1, WlArgument { u: 2 });
    }
    decor
}

fn configure_toplevel_metadata(
    api: &WaylandApi,
    xdg_surface: *mut WlProxy,
    xdg_toplevel: *mut WlProxy,
    title: &str,
    width: i32,
    height: i32,
) {
    let clean_title = CString::new(title.replace('\0', "")).unwrap_or_default();
    // Opcode 2 is `xdg_toplevel::set_title(s)`.
    send_one(
        api,
        xdg_toplevel,
        2,
        WlArgument {
            s: clean_title.as_ptr(),
        },
    );
    // Opcode 3 is `xdg_toplevel::set_app_id(s)`.
    send_one(
        api,
        xdg_toplevel,
        3,
        WlArgument {
            s: c"org.oblikovati.martensite".as_ptr(),
        },
    );
    let mut geom = [
        WlArgument { i: 0 },
        WlArgument { i: 0 },
        WlArgument { i: width },
        WlArgument { i: height },
    ];
    // SAFETY: `xdg_surface` is live; opcode 3 is `xdg_surface::set_window_geometry(iiii)`.
    unsafe {
        let ver = api.proxy_version(xdg_surface);
        api.marshal(xdg_surface, 3, ptr::null(), ver, 0, &mut geom);
    }
}

fn surface_address(target: &WlCallbackTarget) -> AspectDrawable {
    target
        .inner
        .lock()
        .map_or(0, |inner| inner.proxies.surface as AspectDrawable)
}

fn cleanup_on_error(api: &WaylandApi, target: &mut WlCallbackTarget) {
    if let Ok(mut inner) = target.inner.lock() {
        destroy_proxies(api, &mut inner.proxies);
    }
}

impl AspectWindow for WlWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    fn native_display_connection(&self) -> Handle<AspectDisplayConnection> {
        self.connection_lease.as_ref().map_or_else(
            Handle::default,
            AspectDisplayConnectionLease::retained_connection,
        )
    }
    fn is_mapped(&self) -> bool {
        let fields = *self.fields.read();
        if fields.wl_surface == 0 {
            return false;
        }
        self.is_virtual() || fields.is_mapped
    }
    fn map(&self) {
        let mut fields = self.fields.write();
        if self.is_virtual() || fields.wl_surface == 0 {
            return;
        }
        fields.is_mapped = true;
        drop(fields);
        if let (Some(api), Ok(inner)) = (wayland(), self.target.inner.lock()) {
            send_empty(api, inner.proxies.surface, 6);
            let display = self.display_address();
            if display != 0 {
                api.flush(display);
            }
        }
    }
    fn unmap(&self) {
        let mut fields = self.fields.write();
        if self.is_virtual() || fields.wl_surface == 0 {
            return;
        }
        fields.is_mapped = false;
        drop(fields);
        if let (Some(api), Ok(inner)) = (wayland(), self.target.inner.lock()) {
            if !inner.proxies.xdg_toplevel.is_null() {
                // Opcode 13 is `xdg_toplevel::set_minimized()`.
                send_empty(api, inner.proxies.xdg_toplevel, 13);
            }
            let display = self.display_address();
            if display != 0 {
                api.flush(display);
            }
        }
    }
    fn do_resize(&self) -> Resize {
        let Ok(inner) = self.target.inner.lock() else {
            return Resize::Unknown;
        };
        let (new_w, new_h) = (inner.configured_width, inner.configured_height);
        drop(inner);
        if new_w <= 0 || new_h <= 0 {
            return Resize::Unknown;
        }
        let mut fields = self.fields.write();
        let dw = (new_w - fields.width).abs() > 2;
        let dh = (new_h - fields.height).abs() > 2;
        fields.width = new_w;
        fields.height = new_h;
        match (dw, dh) {
            (false, false) => Resize::NoBorder,
            (true, false) => Resize::RightBorder,
            (false, true) => Resize::BottomBorder,
            (true, true) => Resize::RightAndBottomBorder,
        }
    }
    fn do_mapping(&self) -> bool {
        self.native_window_closure().is_none()
    }
    fn ratio(&self) -> f64 {
        if self.native_window_closure().is_some() {
            return 0.0;
        }
        let (w, h) = self.size();
        if h <= 0 {
            0.0
        } else {
            f64::from(w) / f64::from(h)
        }
    }
    fn position(&self) -> (i32, i32, i32, i32) {
        let (w, h) = self.size();
        (0, 0, w, h)
    }
    fn size(&self) -> (i32, i32) {
        let fields = *self.fields.read();
        (fields.width, fields.height)
    }
    fn native_handle(&self) -> AspectDrawable {
        self.fields.read().wl_surface
    }
    fn native_window_closure(&self) -> Option<AspectNativeWindowClosure> {
        *self.native_closure.read()
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        0
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        0
    }
    fn set_title(&self, title: &AsciiString) {
        let clean = CString::new(
            title
                .as_bytes()
                .split(|b| *b == 0)
                .next()
                .unwrap_or_default(),
        )
        .unwrap_or_default();
        if let (Some(api), Ok(inner)) = (wayland(), self.target.inner.lock()) {
            send_one(
                api,
                inner.proxies.xdg_toplevel,
                2,
                WlArgument { s: clean.as_ptr() },
            );
            let display = self.display_address();
            if display != 0 {
                api.flush(display);
            }
        }
    }
    fn invalidate_content(&self, _display: &Handle<AspectDisplayConnection>) {
        if let Ok(mut inner) = self.target.inner.lock() {
            inner.events.push_back(WlEvent::Expose);
        }
    }
    fn device_pixel_ratio(&self) -> f64 {
        self.target
            .inner
            .lock()
            .map_or(1.0, |inner| f64::from(inner.scale_factor.max(1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aspect::recorded_display_fixture;

    #[test]
    fn non_positive_size_and_null_display_are_rejected() {
        assert!(WlWindow::create(&Handle::default(), "t", 0, 10).is_err());
        assert!(WlWindow::create(&Handle::default(), "t", 10, -1).is_err());
        assert!(WlWindow::create(&Handle::default(), "t", 64, 64).is_err());
        assert!(WlWindow::wrap(&Handle::default(), 0, 64, 64).is_err());
        assert!(WlWindow::wrap(&Handle::default(), 1, 0, 64).is_err());
    }

    #[test]
    fn x11_display_connection_is_rejected_by_wayland_window() {
        let x11_display = recorded_display_fixture(0x2000);
        let err = WlWindow::create(&x11_display, "wrong backend", 64, 64)
            .err()
            .expect("X11 display must be refused by WlWindow");
        assert!(err.message().contains("AspectDisplayBackend::Wayland"));
    }

    #[test]
    fn wrap_external_wayland_surface_preserves_size_and_non_ownership() {
        let wl_display =
            Handle::new(AspectDisplayConnection::with_wayland_display(0x4000).unwrap());
        let window = WlWindow::wrap(&wl_display, 0x8000, 320, 240).unwrap();
        assert_eq!(window.wl_surface(), 0x8000);
        assert_eq!(window.native_handle(), 0x8000);
        assert_eq!(window.size(), (320, 240));
        assert_eq!(window.position(), (0, 0, 320, 240));
        assert!(!window.fields.read().is_own_win);
        assert_eq!(window.device_pixel_ratio(), 1.0);
    }
}
