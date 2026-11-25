//! Dynamic `libxkbcommon.so.0` loader for Wayland keymap and keysym decoding.
#![allow(unsafe_code)]

use crate::dynlib::{DynamicLibrary, bind_symbols};
use std::ffi::{c_char, c_int, c_void};
use std::ptr;
use std::sync::OnceLock;

const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;

#[allow(non_snake_case)]
struct XkbSymbols {
    xkb_context_new: unsafe extern "C" fn(u32) -> *mut c_void,
    xkb_context_unref: unsafe extern "C" fn(*mut c_void),
    xkb_keymap_new_from_string:
        unsafe extern "C" fn(*mut c_void, *const c_char, u32, u32) -> *mut c_void,
    xkb_keymap_unref: unsafe extern "C" fn(*mut c_void),
    xkb_state_new: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    xkb_state_unref: unsafe extern "C" fn(*mut c_void),
    xkb_state_key_get_one_sym: unsafe extern "C" fn(*mut c_void, u32) -> u32,
    xkb_state_update_mask: unsafe extern "C" fn(*mut c_void, u32, u32, u32, u32, u32, u32) -> u32,
}

// SAFETY: Loaded function pointers live for the entire process lifetime.
unsafe impl Send for XkbSymbols {}
// SAFETY: Loaded function pointers are immutable for the process lifetime.
unsafe impl Sync for XkbSymbols {}

fn xkb() -> Option<&'static XkbSymbols> {
    static LOADED: OnceLock<Option<XkbSymbols>> = OnceLock::new();
    LOADED
        .get_or_init(|| {
            let lib = DynamicLibrary::open(&["libxkbcommon.so.0", "libxkbcommon.so"])?;
            bind_symbols!(
                &lib,
                XkbSymbols {
                    xkb_context_new,
                    xkb_context_unref,
                    xkb_keymap_new_from_string,
                    xkb_keymap_unref,
                    xkb_state_new,
                    xkb_state_unref,
                    xkb_state_key_get_one_sym,
                    xkb_state_update_mask,
                }
            )
        })
        .as_ref()
}

/// Owned XKB context, keymap and state for one Wayland keyboard.
pub(crate) struct XkbKeymapState {
    context: *mut c_void,
    keymap: *mut c_void,
    state: *mut c_void,
}

// SAFETY: Access to XkbKeymapState is serialized behind a mutex in WlWindow.
unsafe impl Send for XkbKeymapState {}

impl XkbKeymapState {
    /// Creates an XKB context without a keymap yet.
    pub(crate) fn new() -> Option<Self> {
        let api = xkb()?;
        // SAFETY: `xkb_context_new(0)` allocates a new context with default flags.
        let context = unsafe { (api.xkb_context_new)(0) };
        (!context.is_null()).then_some(Self {
            context,
            keymap: ptr::null_mut(),
            state: ptr::null_mut(),
        })
    }

    /// Loads a keymap from a Wayland `wl_keyboard::keymap` file descriptor and closes `fd`.
    pub(crate) fn load_from_fd(&mut self, fd: c_int, size: u32) {
        let len = size as usize;
        if fd < 0 || len == 0 {
            if fd >= 0 {
                // SAFETY: `fd` is an open file descriptor owned by the keymap event handler.
                unsafe { libc::close(fd) };
            }
            return;
        }
        // SAFETY: `mmap` maps `len` read-only bytes from the compositor-provided keymap fd.
        let map = unsafe {
            libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ,
                libc::MAP_PRIVATE,
                fd,
                0,
            )
        };
        // SAFETY: `fd` is no longer needed after `mmap`.
        unsafe { libc::close(fd) };
        if map == libc::MAP_FAILED {
            return;
        }
        self.replace_keymap_from_ptr(map.cast::<c_char>());
        // SAFETY: `map` was returned by `mmap` with length `len`.
        unsafe { libc::munmap(map, len) };
    }

    fn replace_keymap_from_ptr(&mut self, text: *const c_char) {
        let Some(api) = xkb() else { return };
        // SAFETY: `self.context` is live and `text` is a NUL-terminated XKB v1 keymap string.
        let keymap = unsafe {
            (api.xkb_keymap_new_from_string)(self.context, text, XKB_KEYMAP_FORMAT_TEXT_V1, 0)
        };
        if keymap.is_null() {
            return;
        }
        // SAFETY: `keymap` is a live `xkb_keymap` just created above.
        let state = unsafe { (api.xkb_state_new)(keymap) };
        if state.is_null() {
            // SAFETY: `keymap` is owned locally and has no state attached.
            unsafe { (api.xkb_keymap_unref)(keymap) };
            return;
        }
        self.clear_keymap();
        self.keymap = keymap;
        self.state = state;
    }

    /// Updates modifier bitmasks from `wl_keyboard::modifiers`.
    pub(crate) fn update_modifiers(
        &mut self,
        depressed: u32,
        latched: u32,
        locked: u32,
        group: u32,
    ) {
        let (Some(api), false) = (xkb(), self.state.is_null()) else {
            return;
        };
        // SAFETY: `self.state` is a live `xkb_state`.
        unsafe {
            (api.xkb_state_update_mask)(self.state, depressed, latched, locked, 0, 0, group);
        }
    }

    /// Translates a Linux evdev keycode (Wayland `key` argument) to an XKB keysym.
    pub(crate) fn keysym_for_evdev(&self, evdev_key: u32) -> u32 {
        let (Some(api), false) = (xkb(), self.state.is_null()) else {
            return 0;
        };
        // XKB keycodes are offset by +8 from Linux evdev scancodes.
        let xkb_keycode = evdev_key.saturating_add(8);
        // SAFETY: `self.state` is a live `xkb_state`.
        unsafe { (api.xkb_state_key_get_one_sym)(self.state, xkb_keycode) }
    }

    fn clear_keymap(&mut self) {
        let Some(api) = xkb() else { return };
        if !self.state.is_null() {
            // SAFETY: `self.state` is owned and released once.
            unsafe { (api.xkb_state_unref)(self.state) };
            self.state = ptr::null_mut();
        }
        if !self.keymap.is_null() {
            // SAFETY: `self.keymap` is owned and released once.
            unsafe { (api.xkb_keymap_unref)(self.keymap) };
            self.keymap = ptr::null_mut();
        }
    }
}

impl Drop for XkbKeymapState {
    fn drop(&mut self) {
        self.clear_keymap();
        if let (Some(api), false) = (xkb(), self.context.is_null()) {
            // SAFETY: `self.context` is owned and released once after state and keymap.
            unsafe { (api.xkb_context_unref)(self.context) };
            self.context = ptr::null_mut();
        }
    }
}
