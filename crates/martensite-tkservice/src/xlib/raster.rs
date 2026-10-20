//! Raster calls share the existing loaded Xlib and never implement a software renderer.
#![allow(unsafe_code)]
use super::XlibApi;
use std::ffi::CStr;
use x11_dl::xlib::{Display, GC, XColor, XSegment};

#[derive(Clone, Copy)]
pub(crate) struct RasterResources {
    pub pixmap: u64,
    pub gc: usize,
}

impl XlibApi {
    pub(crate) fn create_raster(
        &self,
        display: usize,
        window: u64,
        size: [u32; 2],
    ) -> Option<RasterResources> {
        // SAFETY: display/window are retained by the owning canvas; positive dimensions are checked there.
        unsafe {
            let display = display as *mut Display;
            let screen = (self.lib.XDefaultScreen)(display);
            let depth = (self.lib.XDefaultDepth)(display, screen);
            let pixmap = (self.lib.XCreatePixmap)(display, window, size[0], size[1], depth as u32);
            let gc = (self.lib.XCreateGC)(display, pixmap, 0, std::ptr::null_mut());
            if gc.is_null() {
                (self.lib.XFreePixmap)(display, pixmap);
                return None;
            }
            (self.lib.XSetLineAttributes)(display, gc, 0, 0, 1, 0);
            Some(RasterResources {
                pixmap,
                gc: gc as usize,
            })
        }
    }
    pub(crate) fn free_raster(&self, display: usize, raster: RasterResources) {
        // SAFETY: canvas releases each live GC/pixmap exactly once before releasing its connection lease.
        unsafe {
            (self.lib.XFreeGC)(display as *mut Display, raster.gc as GC);
            (self.lib.XFreePixmap)(display as *mut Display, raster.pixmap);
        }
    }
    pub(crate) fn named_pixel(&self, display: usize, name: &CStr) -> Option<u64> {
        // SAFETY: display is pinned and name is NUL terminated; Xlib initializes the color before use.
        unsafe {
            let display = display as *mut Display;
            let screen = (self.lib.XDefaultScreen)(display);
            let colormap = (self.lib.XDefaultColormap)(display, screen);
            let mut color: XColor = std::mem::zeroed();
            if (self.lib.XParseColor)(display, colormap, name.as_ptr(), &mut color) == 0 {
                return None;
            }
            if (self.lib.XAllocColor)(display, colormap, &mut color) == 0 {
                return None;
            }
            Some(color.pixel)
        }
    }
    pub(crate) fn raster_color(&self, display: usize, raster: RasterResources, pixel: u64) {
        // SAFETY: both handles are live resources of the pinned canvas.
        unsafe {
            (self.lib.XSetForeground)(display as *mut Display, raster.gc as GC, pixel);
        }
    }
    pub(crate) fn raster_mode(&self, display: usize, raster: RasterResources, mode: i32) {
        // SAFETY: canvas accepts only the sixteen X11 raster operations.
        unsafe {
            (self.lib.XSetFunction)(display as *mut Display, raster.gc as GC, mode);
        }
    }
    pub(crate) fn clear_raster(
        &self,
        display: usize,
        raster: RasterResources,
        size: [u32; 2],
        foreground: u64,
    ) {
        self.raster_color(display, raster, 0);
        // SAFETY: the rectangle lies within the retained pixmap and the GC is live.
        unsafe {
            (self.lib.XFillRectangle)(
                display as *mut Display,
                raster.pixmap,
                raster.gc as GC,
                0,
                0,
                size[0],
                size[1],
            );
        }
        self.raster_color(display, raster, foreground);
    }
    pub(crate) fn raster_segments(
        &self,
        display: usize,
        raster: RasterResources,
        segments: &[[i16; 4]],
    ) {
        let mut native: Vec<XSegment> = segments
            .iter()
            .map(|p| XSegment {
                x1: p[0],
                y1: p[1],
                x2: p[2],
                y2: p[3],
            })
            .collect();
        // SAFETY: XDrawSegments reads this initialized array only during the call.
        unsafe {
            (self.lib.XDrawSegments)(
                display as *mut Display,
                raster.pixmap,
                raster.gc as GC,
                native.as_mut_ptr(),
                native.len() as i32,
            );
        }
    }
    pub(crate) fn raster_text(
        &self,
        display: usize,
        raster: RasterResources,
        position: [i32; 2],
        text: &[u8],
    ) {
        let text = text.split(|byte| *byte == 0).next().unwrap_or_default();
        // SAFETY: XDrawString reads the bounded byte span during this call, as the native strlen interface does.
        unsafe {
            (self.lib.XDrawString)(
                display as *mut Display,
                raster.pixmap,
                raster.gc as GC,
                position[0],
                position[1],
                text.as_ptr().cast(),
                text.len() as i32,
            );
        }
    }
    pub(crate) fn raster_resize_window(&self, display: usize, window: u64, size: [u32; 2]) {
        // SAFETY: the window is retained and dimensions are positive.
        unsafe {
            (self.lib.XResizeWindow)(display as *mut Display, window, size[0], size[1]);
        }
    }
    pub(crate) fn raster_move_window(&self, display: usize, window: u64, position: [i32; 2]) {
        // SAFETY: the window and display remain live for this request.
        unsafe {
            (self.lib.XMoveWindow)(display as *mut Display, window, position[0], position[1]);
        }
    }
}
