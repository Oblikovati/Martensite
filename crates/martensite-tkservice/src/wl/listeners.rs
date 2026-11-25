//! C listener tables and callbacks for Wayland core, `xdg-shell`, `wl_seat` and decoration.
#![allow(unsafe_code)]

use super::events::{WlEvent, evdev_button_to_mouse};
use super::state::{WlCallbackTarget, send_one};
use crate::wayland_client::{
    WaylandApi, WlArgument, WlArray, WlInterface, WlProxy, protocol::xdg_protocol, wayland,
    wl_fixed_to_f64,
};
use std::ffi::{CStr, c_char, c_int, c_void};

type RawFn = Option<unsafe extern "C" fn()>;

macro_rules! fn_ptr {
    ($f:ident as $ty:ty) => {{
        // SAFETY: libwayland-client expects listener tables as arrays of erased function pointers.
        Some(unsafe { std::mem::transmute::<$ty, unsafe extern "C" fn()>($f) })
    }};
}

/// Attaches the `wl_registry` listener to discover compositor, shell, seat and decoration globals.
pub(crate) fn attach_registry_listener(
    api: &WaylandApi,
    registry: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 2] = [
        fn_ptr!(
            on_registry_global
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, *const c_char, u32)
        ),
        fn_ptr!(on_registry_global_remove as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)),
    ];
    attach(api, registry, LISTENER.as_ptr(), target);
}

/// Attaches the `xdg_wm_base` listener so compositor ping events receive `pong`.
pub(crate) fn attach_wm_base_listener(
    api: &WaylandApi,
    wm_base: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 1] = [fn_ptr!(
        on_wm_base_ping as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)
    )];
    attach(api, wm_base, LISTENER.as_ptr(), target);
}

/// Attaches the `xdg_surface` listener to acknowledge configure serials.
pub(crate) fn attach_xdg_surface_listener(
    api: &WaylandApi,
    xdg_surface: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 1] = [fn_ptr!(
        on_xdg_surface_configure as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)
    )];
    attach(api, xdg_surface, LISTENER.as_ptr(), target);
}

/// Attaches the `xdg_toplevel` listener for configure and close events.
pub(crate) fn attach_xdg_toplevel_listener(
    api: &WaylandApi,
    xdg_toplevel: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 4] = [
        fn_ptr!(
            on_toplevel_configure
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, i32, i32, *mut WlArray)
        ),
        fn_ptr!(on_toplevel_close as unsafe extern "C" fn(*mut c_void, *mut WlProxy)),
        fn_ptr!(
            on_toplevel_configure_bounds
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, i32, i32)
        ),
        fn_ptr!(
            on_toplevel_wm_capabilities
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, *mut WlArray)
        ),
    ];
    attach(api, xdg_toplevel, LISTENER.as_ptr(), target);
}

/// Attaches the `wl_surface` listener to track preferred buffer scale.
pub(crate) fn attach_surface_listener(
    api: &WaylandApi,
    surface: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 4] = [
        fn_ptr!(on_surface_enter as unsafe extern "C" fn(*mut c_void, *mut WlProxy, *mut WlProxy)),
        fn_ptr!(on_surface_leave as unsafe extern "C" fn(*mut c_void, *mut WlProxy, *mut WlProxy)),
        fn_ptr!(on_surface_preferred_scale as unsafe extern "C" fn(*mut c_void, *mut WlProxy, i32)),
        fn_ptr!(
            on_surface_preferred_transform as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)
        ),
    ];
    attach(api, surface, LISTENER.as_ptr(), target);
}

/// Attaches the `wl_seat` listener to bind pointer and keyboard devices.
pub(crate) fn attach_seat_listener(
    api: &WaylandApi,
    seat: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 2] = [
        fn_ptr!(on_seat_capabilities as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)),
        fn_ptr!(on_seat_name as unsafe extern "C" fn(*mut c_void, *mut WlProxy, *const c_char)),
    ];
    attach(api, seat, LISTENER.as_ptr(), target);
}

/// Attaches the `zxdg_toplevel_decoration_v1` listener.
pub(crate) fn attach_decoration_listener(
    api: &WaylandApi,
    decoration: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 1] = [fn_ptr!(
        on_decoration_configure as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)
    )];
    attach(api, decoration, LISTENER.as_ptr(), target);
}

fn attach_pointer_listener(api: &WaylandApi, pointer: *mut WlProxy, target: *mut WlCallbackTarget) {
    static LISTENER: [RawFn; 11] = [
        fn_ptr!(
            on_pointer_enter
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, *mut WlProxy, i32, i32)
        ),
        fn_ptr!(
            on_pointer_leave as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, *mut WlProxy)
        ),
        fn_ptr!(
            on_pointer_motion as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, i32, i32)
        ),
        fn_ptr!(
            on_pointer_button
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32, u32, u32)
        ),
        fn_ptr!(on_pointer_axis as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32, i32)),
        fn_ptr!(on_pointer_frame as unsafe extern "C" fn(*mut c_void, *mut WlProxy)),
        fn_ptr!(on_pointer_axis_source as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32)),
        fn_ptr!(on_pointer_axis_stop as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32)),
        fn_ptr!(
            on_pointer_axis_discrete as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, i32)
        ),
        fn_ptr!(
            on_pointer_axis_value120 as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, i32)
        ),
        fn_ptr!(
            on_pointer_axis_rel_dir as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32)
        ),
    ];
    attach(api, pointer, LISTENER.as_ptr(), target);
}

fn attach_keyboard_listener(
    api: &WaylandApi,
    keyboard: *mut WlProxy,
    target: *mut WlCallbackTarget,
) {
    static LISTENER: [RawFn; 6] = [
        fn_ptr!(
            on_keyboard_keymap as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, c_int, u32)
        ),
        fn_ptr!(
            on_keyboard_enter
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, *mut WlProxy, *mut WlArray)
        ),
        fn_ptr!(
            on_keyboard_leave as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, *mut WlProxy)
        ),
        fn_ptr!(
            on_keyboard_key as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32, u32, u32)
        ),
        fn_ptr!(
            on_keyboard_modifiers
                as unsafe extern "C" fn(*mut c_void, *mut WlProxy, u32, u32, u32, u32, u32)
        ),
        fn_ptr!(
            on_keyboard_repeat_info as unsafe extern "C" fn(*mut c_void, *mut WlProxy, i32, i32)
        ),
    ];
    attach(api, keyboard, LISTENER.as_ptr(), target);
}

fn attach(
    api: &WaylandApi,
    proxy: *mut WlProxy,
    listener: *const RawFn,
    target: *mut WlCallbackTarget,
) {
    if proxy.is_null() {
        return;
    }
    // SAFETY: `listener` is a `'static` function-pointer table and `target` outlives `proxy`.
    unsafe {
        api.add_listener(proxy, listener.cast_mut(), target.cast::<c_void>());
    }
}

fn bind_global(
    api: &WaylandApi,
    registry: *mut WlProxy,
    name: u32,
    iface: *const WlInterface,
    version: u32,
) -> *mut WlProxy {
    // SAFETY: `iface` is a valid static `WlInterface` with a NUL-terminated name.
    let iface_name = unsafe { (*iface).name };
    let mut args = [
        WlArgument { u: name },
        WlArgument { s: iface_name },
        WlArgument { u: version },
        WlArgument { n: 0 },
    ];
    // SAFETY: `registry` is live; opcode 0 is `wl_registry::bind(usun)`.
    unsafe { api.marshal(registry, 0, iface, version, 0, &mut args) }
}

unsafe extern "C" fn on_registry_global(
    data: *mut c_void,
    registry: *mut WlProxy,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    let (Some(api), false) = (wayland(), data.is_null() || interface.is_null()) else {
        return;
    };
    // SAFETY: `interface` is a NUL-terminated string from libwayland; `data` is our pinned target.
    let (iface_name, target) =
        unsafe { (CStr::from_ptr(interface), &*data.cast::<WlCallbackTarget>()) };
    let proto = xdg_protocol(&api.core);
    let Ok(mut inner) = target.inner.lock() else {
        return;
    };
    match iface_name.to_bytes() {
        b"wl_compositor" if inner.proxies.compositor.is_null() => {
            inner.proxies.compositor =
                bind_global(api, registry, name, api.core.compositor, version.min(6));
        }
        b"xdg_wm_base" if inner.proxies.xdg_wm_base.is_null() => {
            let wm = bind_global(api, registry, name, proto.wm_base, version.min(6));
            attach_wm_base_listener(api, wm, data.cast());
            inner.proxies.xdg_wm_base = wm;
        }
        b"zxdg_decoration_manager_v1" if inner.proxies.decoration_manager.is_null() => {
            inner.proxies.decoration_manager = bind_global(
                api,
                registry,
                name,
                proto.decoration_manager,
                version.min(1),
            );
        }
        b"wl_seat" if inner.proxies.seat.is_null() => {
            let seat = bind_global(api, registry, name, api.core.seat, version.min(7));
            attach_seat_listener(api, seat, data.cast());
            inner.proxies.seat = seat;
        }
        _ => {}
    }
}

unsafe extern "C" fn on_registry_global_remove(_: *mut c_void, _: *mut WlProxy, _: u32) {}

unsafe extern "C" fn on_wm_base_ping(_: *mut c_void, wm_base: *mut WlProxy, serial: u32) {
    if let Some(api) = wayland() {
        // Opcode 3 is `xdg_wm_base::pong(u)`.
        send_one(api, wm_base, 3, WlArgument { u: serial });
    }
}

unsafe extern "C" fn on_xdg_surface_configure(
    data: *mut c_void,
    xdg_surface: *mut WlProxy,
    serial: u32,
) {
    if let Some(api) = wayland() {
        // Opcode 4 is `xdg_surface::ack_configure(u)`.
        send_one(api, xdg_surface, 4, WlArgument { u: serial });
    }
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget` owned by `WlWindow`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    let Ok(mut inner) = target.inner.lock() else {
        return;
    };
    inner.configured = true;
    inner.configured_width = inner.pending_width;
    inner.configured_height = inner.pending_height;
    let (width, height) = (inner.configured_width, inner.configured_height);
    inner.events.push_back(WlEvent::Configure { width, height });
}

unsafe extern "C" fn on_toplevel_configure(
    data: *mut c_void,
    _: *mut WlProxy,
    width: i32,
    height: i32,
    _: *mut WlArray,
) {
    if data.is_null() || width <= 0 || height <= 0 {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        inner.pending_width = width;
        inner.pending_height = height;
    }
}

unsafe extern "C" fn on_toplevel_close(data: *mut c_void, _: *mut WlProxy) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        inner.closed = true;
        inner.events.push_back(WlEvent::Close);
    }
}

unsafe extern "C" fn on_toplevel_configure_bounds(_: *mut c_void, _: *mut WlProxy, _: i32, _: i32) {
}
unsafe extern "C" fn on_toplevel_wm_capabilities(_: *mut c_void, _: *mut WlProxy, _: *mut WlArray) {
}
unsafe extern "C" fn on_surface_enter(_: *mut c_void, _: *mut WlProxy, _: *mut WlProxy) {}
unsafe extern "C" fn on_surface_leave(_: *mut c_void, _: *mut WlProxy, _: *mut WlProxy) {}
unsafe extern "C" fn on_surface_preferred_transform(_: *mut c_void, _: *mut WlProxy, _: u32) {}
unsafe extern "C" fn on_decoration_configure(_: *mut c_void, _: *mut WlProxy, _: u32) {}
unsafe extern "C" fn on_seat_name(_: *mut c_void, _: *mut WlProxy, _: *const c_char) {}

unsafe extern "C" fn on_surface_preferred_scale(data: *mut c_void, _: *mut WlProxy, factor: i32) {
    if data.is_null() || factor <= 0 {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        inner.scale_factor = factor;
    }
}

unsafe extern "C" fn on_seat_capabilities(data: *mut c_void, seat: *mut WlProxy, caps: u32) {
    let (Some(api), false) = (wayland(), data.is_null()) else {
        return;
    };
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    let Ok(mut inner) = target.inner.lock() else {
        return;
    };
    // Bit 1 = WL_SEAT_CAPABILITY_POINTER, Bit 2 = WL_SEAT_CAPABILITY_KEYBOARD.
    if (caps & 1) != 0 && inner.proxies.pointer.is_null() {
        let mut args = [WlArgument { n: 0 }];
        // SAFETY: `seat` is live; opcode 0 is `wl_seat::get_pointer(n)`.
        let ptr_proxy = unsafe {
            let ver = api.proxy_version(seat);
            api.marshal(seat, 0, api.core.pointer, ver, 0, &mut args)
        };
        attach_pointer_listener(api, ptr_proxy, data.cast());
        inner.proxies.pointer = ptr_proxy;
    }
    if (caps & 2) != 0 && inner.proxies.keyboard.is_null() {
        let mut args = [WlArgument { n: 0 }];
        // SAFETY: `seat` is live; opcode 1 is `wl_seat::get_keyboard(n)`.
        let kbd_proxy = unsafe {
            let ver = api.proxy_version(seat);
            api.marshal(seat, 1, api.core.keyboard, ver, 0, &mut args)
        };
        attach_keyboard_listener(api, kbd_proxy, data.cast());
        inner.proxies.keyboard = kbd_proxy;
    }
}

unsafe extern "C" fn on_pointer_enter(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    _: *mut WlProxy,
    sx: i32,
    sy: i32,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        inner.pointer_x = wl_fixed_to_f64(sx).round() as i32;
        inner.pointer_y = wl_fixed_to_f64(sy).round() as i32;
    }
}

unsafe extern "C" fn on_pointer_leave(_: *mut c_void, _: *mut WlProxy, _: u32, _: *mut WlProxy) {}

unsafe extern "C" fn on_pointer_motion(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    sx: i32,
    sy: i32,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        let x = wl_fixed_to_f64(sx).round() as i32;
        let y = wl_fixed_to_f64(sy).round() as i32;
        inner.pointer_x = x;
        inner.pointer_y = y;
        let buttons = inner.pointer_buttons;
        inner
            .events
            .push_back(WlEvent::PointerMotion { x, y, buttons });
    }
}

unsafe extern "C" fn on_pointer_button(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    _: u32,
    button: u32,
    state: u32,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        let pressed = state == 1;
        let bit = evdev_button_to_mouse(button);
        if pressed {
            inner.pointer_buttons |= bit;
        } else {
            inner.pointer_buttons &= !bit;
        }
        let (x, y) = (inner.pointer_x, inner.pointer_y);
        inner.events.push_back(WlEvent::PointerButton {
            x,
            y,
            button,
            pressed,
        });
    }
}

unsafe extern "C" fn on_pointer_axis(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    axis: u32,
    value: i32,
) {
    // Only vertical axis (0) maps to AspectScrollDelta.
    if data.is_null() || axis != 0 {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        let (x, y) = (inner.pointer_x, inner.pointer_y);
        let delta = -wl_fixed_to_f64(value) / 10.0;
        inner
            .events
            .push_back(WlEvent::PointerScroll { x, y, delta });
    }
}

unsafe extern "C" fn on_pointer_frame(_: *mut c_void, _: *mut WlProxy) {}
unsafe extern "C" fn on_pointer_axis_source(_: *mut c_void, _: *mut WlProxy, _: u32) {}
unsafe extern "C" fn on_pointer_axis_stop(_: *mut c_void, _: *mut WlProxy, _: u32, _: u32) {}
unsafe extern "C" fn on_pointer_axis_discrete(_: *mut c_void, _: *mut WlProxy, _: u32, _: i32) {}
unsafe extern "C" fn on_pointer_axis_value120(_: *mut c_void, _: *mut WlProxy, _: u32, _: i32) {}
unsafe extern "C" fn on_pointer_axis_rel_dir(_: *mut c_void, _: *mut WlProxy, _: u32, _: u32) {}

unsafe extern "C" fn on_keyboard_keymap(
    data: *mut c_void,
    _: *mut WlProxy,
    format: u32,
    fd: c_int,
    size: u32,
) {
    if data.is_null() || format != 1 {
        if fd >= 0 {
            // SAFETY: Wayland transfers ownership of the keymap fd to the client.
            unsafe { libc::close(fd) };
        }
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        if let Some(xkb) = inner.xkb.as_mut() {
            xkb.load_from_fd(fd, size);
        } else if fd >= 0 {
            // SAFETY: Close fd when libxkbcommon is unavailable.
            unsafe { libc::close(fd) };
        }
    }
}

unsafe extern "C" fn on_keyboard_enter(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    _: *mut WlProxy,
    _: *mut WlArray,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        inner.events.push_back(WlEvent::Focus(true));
    }
}

unsafe extern "C" fn on_keyboard_leave(_: *mut c_void, _: *mut WlProxy, _: u32, _: *mut WlProxy) {
    // Focus loss is delivered when `wl_keyboard::leave` arrives.
}

unsafe extern "C" fn on_keyboard_key(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    _: u32,
    key: u32,
    state: u32,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock() {
        let keysym = inner
            .xkb
            .as_ref()
            .map_or(0, |xkb| xkb.keysym_for_evdev(key));
        inner.events.push_back(WlEvent::Key {
            keysym,
            pressed: state == 1,
        });
    }
}

unsafe extern "C" fn on_keyboard_modifiers(
    data: *mut c_void,
    _: *mut WlProxy,
    _: u32,
    depressed: u32,
    latched: u32,
    locked: u32,
    group: u32,
) {
    if data.is_null() {
        return;
    }
    // SAFETY: `data` points to the pinned `WlCallbackTarget`.
    let target = unsafe { &*data.cast::<WlCallbackTarget>() };
    if let Ok(mut inner) = target.inner.lock()
        && let Some(xkb) = inner.xkb.as_mut()
    {
        xkb.update_modifiers(depressed, latched, locked, group);
    }
}

unsafe extern "C" fn on_keyboard_repeat_info(_: *mut c_void, _: *mut WlProxy, _: i32, _: i32) {}
