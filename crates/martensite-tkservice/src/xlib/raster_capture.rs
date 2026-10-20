//! Read actual X-server raster pixels; no synthesized capture or recorded pixel source.
#![allow(unsafe_code)]
use super::{XlibApi, raster::RasterResources};
use x11_dl::xlib::{Display, ZPixmap};

impl XlibApi {
    pub(crate) fn raster_rgb(
        &self,
        display: usize,
        raster: RasterResources,
        size: [u32; 2],
    ) -> Option<Vec<u8>> {
        // SAFETY: live pinned display/pixmap, matching positive dimensions; XGetImage owns its result until destroyed below.
        unsafe {
            (self.lib.XSync)(display as *mut Display, 0);
            let image = (self.lib.XGetImage)(
                display as *mut Display,
                raster.pixmap,
                0,
                0,
                size[0],
                size[1],
                !0,
                ZPixmap,
            );
            if image.is_null() {
                return None;
            }
            // Pixmaps have no associated visual, so XGetImage may report zero masks.
            // create_raster always uses this screen's default depth; use that exact visual.
            let screen = (self.lib.XDefaultScreen)(display as *mut Display);
            let visual = (self.lib.XDefaultVisual)(display as *mut Display, screen);
            let masks = [
                (*visual).red_mask,
                (*visual).green_mask,
                (*visual).blue_mask,
            ];
            let result = self.read_raster_pixels(image, size, masks);
            (self.lib.XDestroyImage)(image);
            result
        }
    }
    unsafe fn read_raster_pixels(
        &self,
        image: *mut x11_dl::xlib::XImage,
        size: [u32; 2],
        masks: [u64; 3],
    ) -> Option<Vec<u8>> {
        if masks.contains(&0) {
            return None;
        }
        let mut rgb = Vec::with_capacity(size[0] as usize * size[1] as usize * 3);
        for y in 0..size[1] {
            for x in 0..size[0] {
                // SAFETY: coordinates are within the successfully returned XImage dimensions.
                let pixel = unsafe { (self.lib.XGetPixel)(image, x as i32, y as i32) };
                rgb.extend(masks.map(|mask| {
                    (((pixel & mask) >> mask.trailing_zeros()) * 255
                        / (mask >> mask.trailing_zeros())) as u8
                }));
            }
        }
        Some(rgb)
    }
}
