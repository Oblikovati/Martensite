//! Testing only: a RenderDoc frame capture of a headless Vulkan device. `renderdoccmd capture`
//! crashes on this host, so the test process loads RenderDoc's capture layer itself
//! (`VK_ADD_IMPLICIT_LAYER_PATH` naming RenderDoc's layer manifest and
//! `ENABLE_VULKAN_RENDERDOC_CAPTURE=1`) and brackets the work with the in-application API
//! (renderdoc_app.h 1.6.0), keyed on the instance's dispatch pointer
//! (`RENDERDOC_DEVICEPOINTER_FROM_VKINSTANCE`). Without the layer `RenderDoc::injected()` is
//! None. Example: `RenderDoc::injected()?.capture(instance, "/tmp/frame", || dump())`.
#![allow(unsafe_code)]
use std::ffi::{CString, c_char, c_int, c_void};

unsafe extern "C" {
    fn dlopen(name: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
}

/// dlfcn.h: resolve now; only find a library that is already loaded.
const RTLD_NOW: c_int = 2;
const RTLD_NOLOAD: c_int = 4;
/// eRENDERDOC_API_Version_1_6_0.
const API_1_6_0: c_int = 10600;
/// Slots of RENDERDOC_API_1_6_0 (function pointers in declaration order).
const SET_CAPTURE_FILE_PATH_TEMPLATE: usize = 11;
const GET_NUM_CAPTURES: usize = 13;
const START_FRAME_CAPTURE: usize = 19;
const END_FRAME_CAPTURE: usize = 21;

type GetApi = unsafe extern "C" fn(c_int, *mut *mut c_void) -> c_int;
type SetTemplate = unsafe extern "C" fn(*const c_char);
type NumCaptures = unsafe extern "C" fn() -> u32;
type StartFrame = unsafe extern "C" fn(*mut c_void, *mut c_void);
type EndFrame = unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32;

/// RenderDoc's in-application API table, present only when its layer is in this process.
pub struct RenderDoc {
    api: *const *const c_void,
}

impl RenderDoc {
    /// The API of an injected RenderDoc, None when its library is not loaded.
    pub fn injected() -> Option<Self> {
        // SAFETY: RTLD_NOLOAD never loads a library; the symbol is renderdoc_app.h's
        // RENDERDOC_GetAPI, called with its documented signature.
        unsafe {
            let library = dlopen(c"librenderdoc.so".as_ptr(), RTLD_NOW | RTLD_NOLOAD);
            if library.is_null() {
                return None;
            }
            let get = dlsym(library, c"RENDERDOC_GetAPI".as_ptr());
            if get.is_null() {
                return None;
            }
            let get: GetApi = std::mem::transmute(get);
            let mut api = std::ptr::null_mut();
            (get(API_1_6_0, &mut api) == 1 && !api.is_null()).then_some(Self {
                api: api as *const *const c_void,
            })
        }
    }

    /// Capture every submission `body` makes on the device of `instance` (a raw `VkInstance`,
    /// Graphic3dRenderDriver::raw_vulkan_instance) into `<template>_frame<N>.rdc`; the result
    /// and whether RenderDoc wrote a capture.
    pub fn capture<T>(&self, instance: u64, template: &str, body: impl FnOnce() -> T) -> (T, bool) {
        let template = CString::new(template).unwrap();
        // SAFETY: the slots follow RENDERDOC_API_1_6_0; `instance` is a live dispatchable
        // VkInstance, whose first word is the loader dispatch pointer RenderDoc keys on.
        unsafe {
            let slot = |index: usize| *self.api.add(index);
            let set: SetTemplate = std::mem::transmute(slot(SET_CAPTURE_FILE_PATH_TEMPLATE));
            let count: NumCaptures = std::mem::transmute(slot(GET_NUM_CAPTURES));
            let start: StartFrame = std::mem::transmute(slot(START_FRAME_CAPTURE));
            let end: EndFrame = std::mem::transmute(slot(END_FRAME_CAPTURE));
            set(template.as_ptr());
            let device = *(instance as usize as *const *mut c_void);
            let before = count();
            start(device, std::ptr::null_mut());
            let value = body();
            let ended = end(device, std::ptr::null_mut()) == 1;
            (value, ended && count() > before)
        }
    }
}
