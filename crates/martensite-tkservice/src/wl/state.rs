//! Pinned callback state and proxy ownership for one [`super::WlWindow`].
#![allow(unsafe_code)]

use super::events::WlEvent;
use crate::aspect::AspectVKeyMouse;
use crate::wayland_client::{
    WL_MARSHAL_FLAG_DESTROY, WaylandApi, WlArgument, WlProxy, xkb::XkbKeymapState,
};
use std::collections::VecDeque;
use std::ptr;
use std::sync::Mutex;

/// Raw Wayland proxy addresses owned by a [`super::WlWindow`].
pub(crate) struct WlProxies {
    pub(crate) registry: *mut WlProxy,
    pub(crate) compositor: *mut WlProxy,
    pub(crate) xdg_wm_base: *mut WlProxy,
    pub(crate) decoration_manager: *mut WlProxy,
    pub(crate) seat: *mut WlProxy,
    pub(crate) pointer: *mut WlProxy,
    pub(crate) keyboard: *mut WlProxy,
    pub(crate) surface: *mut WlProxy,
    pub(crate) xdg_surface: *mut WlProxy,
    pub(crate) xdg_toplevel: *mut WlProxy,
    pub(crate) decoration: *mut WlProxy,
    pub(crate) owns_surface: bool,
}

impl Default for WlProxies {
    fn default() -> Self {
        Self {
            registry: ptr::null_mut(),
            compositor: ptr::null_mut(),
            xdg_wm_base: ptr::null_mut(),
            decoration_manager: ptr::null_mut(),
            seat: ptr::null_mut(),
            pointer: ptr::null_mut(),
            keyboard: ptr::null_mut(),
            surface: ptr::null_mut(),
            xdg_surface: ptr::null_mut(),
            xdg_toplevel: ptr::null_mut(),
            decoration: ptr::null_mut(),
            owns_surface: false,
        }
    }
}

/// Mutable state shared between Wayland C callbacks and `WlWindow` methods.
pub(crate) struct WlInner {
    pub(crate) proxies: WlProxies,
    pub(crate) xkb: Option<XkbKeymapState>,
    pub(crate) pending_width: i32,
    pub(crate) pending_height: i32,
    pub(crate) configured_width: i32,
    pub(crate) configured_height: i32,
    pub(crate) configured: bool,
    pub(crate) scale_factor: i32,
    pub(crate) pointer_x: i32,
    pub(crate) pointer_y: i32,
    pub(crate) pointer_buttons: AspectVKeyMouse,
    pub(crate) closed: bool,
    pub(crate) events: VecDeque<WlEvent>,
}

/// Heap-pinned callback target passed as `user_data` to `wl_proxy_add_listener`.
pub(crate) struct WlCallbackTarget {
    pub(crate) inner: Mutex<WlInner>,
}

// SAFETY: All raw proxy pointers are guarded by `Mutex<WlInner>` and only touched on the owning connection.
unsafe impl Send for WlCallbackTarget {}
// SAFETY: All raw proxy pointers are guarded by `Mutex<WlInner>`.
unsafe impl Sync for WlCallbackTarget {}

impl WlCallbackTarget {
    pub(crate) fn new(width: i32, height: i32, owns_surface: bool) -> Box<Self> {
        let proxies = WlProxies {
            owns_surface,
            ..Default::default()
        };
        Box::new(Self {
            inner: Mutex::new(WlInner {
                proxies,
                xkb: XkbKeymapState::new(),
                pending_width: width,
                pending_height: height,
                configured_width: width,
                configured_height: height,
                configured: false,
                scale_factor: 1,
                pointer_x: 0,
                pointer_y: 0,
                pointer_buttons: 0,
                closed: false,
                events: VecDeque::new(),
            }),
        })
    }
}

/// Destroys all Wayland proxies owned by `proxies` in strict child-before-parent order.
pub(crate) fn destroy_proxies(api: &WaylandApi, proxies: &mut WlProxies) {
    destroy_with_request(api, &mut proxies.decoration, 0);
    destroy_with_request(api, &mut proxies.xdg_toplevel, 0);
    destroy_with_request(api, &mut proxies.xdg_surface, 0);
    if proxies.owns_surface {
        destroy_with_request(api, &mut proxies.surface, 0);
    } else {
        proxies.surface = ptr::null_mut();
    }
    destroy_versioned_release(api, &mut proxies.pointer, 3, 1);
    destroy_versioned_release(api, &mut proxies.keyboard, 3, 0);
    destroy_versioned_release(api, &mut proxies.seat, 5, 3);
    destroy_with_request(api, &mut proxies.decoration_manager, 0);
    destroy_with_request(api, &mut proxies.xdg_wm_base, 0);
    destroy_client_only(api, &mut proxies.compositor);
    destroy_client_only(api, &mut proxies.registry);
}

fn destroy_with_request(api: &WaylandApi, slot: &mut *mut WlProxy, opcode: u32) {
    let proxy = std::mem::replace(slot, ptr::null_mut());
    if proxy.is_null() {
        return;
    }
    // SAFETY: `proxy` is a live Wayland proxy owned by this window and destroyed once.
    unsafe {
        let version = api.proxy_version(proxy);
        api.marshal(
            proxy,
            opcode,
            ptr::null(),
            version,
            WL_MARSHAL_FLAG_DESTROY,
            &mut [],
        );
    }
}

fn destroy_versioned_release(
    api: &WaylandApi,
    slot: &mut *mut WlProxy,
    min_version: u32,
    opcode: u32,
) {
    let proxy = std::mem::replace(slot, ptr::null_mut());
    if proxy.is_null() {
        return;
    }
    // SAFETY: `proxy` is a live Wayland proxy owned by this window and destroyed once.
    unsafe {
        let version = api.proxy_version(proxy);
        if version >= min_version {
            api.marshal(
                proxy,
                opcode,
                ptr::null(),
                version,
                WL_MARSHAL_FLAG_DESTROY,
                &mut [],
            );
        } else {
            api.proxy_destroy(proxy);
        }
    }
}

fn destroy_client_only(api: &WaylandApi, slot: &mut *mut WlProxy) {
    let proxy = std::mem::replace(slot, ptr::null_mut());
    if !proxy.is_null() {
        // SAFETY: `proxy` is a live Wayland proxy owned by this window and destroyed once.
        unsafe { api.proxy_destroy(proxy) };
    }
}

/// Sends a zero-argument request on `proxy` without destroying it.
pub(crate) fn send_empty(api: &WaylandApi, proxy: *mut WlProxy, opcode: u32) {
    if proxy.is_null() {
        return;
    }
    // SAFETY: Caller passes a live proxy and valid zero-argument opcode.
    unsafe {
        let version = api.proxy_version(proxy);
        api.marshal(proxy, opcode, ptr::null(), version, 0, &mut []);
    }
}

/// Sends a single-argument request on `proxy` without destroying it.
pub(crate) fn send_one(api: &WaylandApi, proxy: *mut WlProxy, opcode: u32, arg: WlArgument) {
    if proxy.is_null() {
        return;
    }
    let mut args = [arg];
    // SAFETY: Caller passes a live proxy and matching single-argument opcode.
    unsafe {
        let version = api.proxy_version(proxy);
        api.marshal(proxy, opcode, ptr::null(), version, 0, &mut args);
    }
}
