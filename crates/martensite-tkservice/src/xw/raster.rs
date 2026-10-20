//! Thread-confined server-side raster storage for native drawing and capture.
use super::XwWindow;
use crate::aspect::{AspectDisplayConnection, AspectDisplayConnectionLease, AspectWindow as _};
use crate::image::{ImageAlienPixMap, ImageFormat};
use crate::xlib::{raster::RasterResources, xlib};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use std::{ffi::CString, marker::PhantomData, rc::Rc};

/// An owned X-server pixmap and GC, kept on its creating thread because drawing is ordered.
/// For example, `XwRasterCanvas::create(&connection, "Preview", [0, 0], [400, 400])?`.
/// This operation-local canvas is intentionally neither `Send` nor `Sync`; saved files contain owned pixels.
pub struct XwRasterCanvas {
    window: XwWindow,
    raster: RasterResources,
    size: [u32; 2],
    foreground: u64,
    position: [i32; 2],
    title: String,
    thread_bound: PhantomData<Rc<()>>,
}

fn raster_failure(value: &str) -> Failure {
    Failure::with_type(
        "Aspect_WindowDefinitionError",
        &format!("X11 raster {value}; expected a live display, drawable and positive dimensions"),
        "XwRasterCanvas",
    )
}

impl XwRasterCanvas {
    /// Allocates actual native raster storage; `Self::create(&display, "Preview", [0, 0], [400, 400])?`.
    pub fn create(
        display: &Handle<AspectDisplayConnection>,
        title: &str,
        position: [i32; 2],
        size: [i32; 2],
    ) -> Result<Self, Failure> {
        let dimensions = raster_dimensions(size)?;
        let window = XwWindow::create(display, title, position[0], position[1], size[0], size[1])?;
        Self::with_window(window, title, position, dimensions)
    }
    fn with_window(
        window: XwWindow,
        title: &str,
        position: [i32; 2],
        dimensions: [u32; 2],
    ) -> Result<Self, Failure> {
        let api = xlib().ok_or_else(|| raster_failure("library unavailable"))?;
        let raster = api
            .create_raster(window.display_address(), window.x_window(), dimensions)
            .ok_or_else(|| raster_failure("allocation failed"))?;
        let mut canvas = Self {
            window,
            raster,
            size: dimensions,
            foreground: 0,
            position,
            title: title.to_string(),
            thread_bound: PhantomData,
        };
        canvas.clear();
        Ok(canvas)
    }
    /// Selects the actual GC foreground; `canvas.set_pixel(0xff0000)`.
    pub fn set_pixel(&mut self, pixel: u64) {
        self.foreground = pixel;
        if let Some(api) = xlib() {
            api.raster_color(self.window.display_address(), self.raster, pixel);
        }
    }
    /// Selects an X11 raster operation; `canvas.set_mode(3)` chooses copy.
    pub fn set_mode(&self, mode: i32) {
        if !(0..16).contains(&mode) {
            return;
        }
        if let Some(api) = xlib() {
            api.raster_mode(self.window.display_address(), self.raster, mode);
        }
    }
    /// Clears the server pixmap through its existing GC; `canvas.clear()`.
    pub fn clear(&mut self) {
        if let Some(api) = xlib() {
            api.clear_raster(
                self.window.display_address(),
                self.raster,
                self.size,
                self.foreground,
            );
        }
    }
    /// Draws actual signed-short window segments; `canvas.draw_segments(&[[1, 2, 3, 4]])`.
    pub fn draw_segments(&self, segments: &[[i16; 4]]) {
        if let Some(api) = xlib() {
            api.raster_segments(self.window.display_address(), self.raster, segments);
        }
    }
    /// Draws server-font bytes through XDrawString; `canvas.draw_text([5, 10], "label")`.
    pub fn draw_text(&self, position: [i32; 2], text: &str) {
        if let Some(api) = xlib() {
            api.raster_text(
                self.window.display_address(),
                self.raster,
                position,
                text.as_bytes(),
            );
        }
    }
    /// Captures the actual pixmap and uses the configured image writer; `canvas.save("preview.png")?`.
    pub fn save(&self, path: &str) -> Result<bool, Failure> {
        let Some(rgb) = xlib()
            .and_then(|api| api.raster_rgb(self.window.display_address(), self.raster, self.size))
        else {
            return Ok(false);
        };
        let image = ImageAlienPixMap::default();
        if !image.init_trash(
            ImageFormat::Rgb,
            self.size[0] as usize,
            self.size[1] as usize,
            0,
        ) {
            return Ok(false);
        }
        image.set_top_down(true);
        image.change_data().data.change_data().copy_from_slice(&rgb);
        image.save(&AsciiString::from(path))
    }
    /// The native window identity; `canvas.window_id()`.
    pub fn window_id(&self) -> u64 {
        self.window.x_window()
    }
    /// Current raster dimensions; `canvas.dimensions()`.
    pub fn dimensions(&self) -> [i32; 2] {
        self.size.map(|value| value as i32)
    }
    /// The requested native window position; `canvas.position()`.
    pub fn position(&self) -> [i32; 2] {
        self.position
    }
    /// The assigned window title; `canvas.title()`.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Assigns the actual native title; `canvas.set_title("Preview")`.
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(&AsciiString::from(title));
        self.title = title.to_string();
    }
    /// Moves the actual window without changing pixels; `canvas.set_position([10, 20])`.
    pub fn set_position(&mut self, position: [i32; 2]) {
        if let Some(api) = xlib() {
            api.raster_move_window(
                self.window.display_address(),
                self.window.x_window(),
                position,
            );
        }
        self.position = position;
    }
    /// Reallocates native pixels for new positive dimensions; `canvas.resize([640, 480])?`.
    pub fn resize(&mut self, size: [i32; 2]) -> Result<(), Failure> {
        let size = raster_dimensions(size)?;
        let api = xlib().ok_or_else(|| raster_failure("library unavailable"))?;
        let display = self.window.display_address();
        let replacement = api
            .create_raster(display, self.window.x_window(), size)
            .ok_or_else(|| raster_failure("resize allocation failed"))?;
        api.free_raster(display, self.raster);
        api.raster_resize_window(display, self.window.x_window(), size);
        self.raster = replacement;
        self.size = size;
        self.set_pixel(self.foreground);
        self.clear();
        Ok(())
    }
    /// Flushes this canvas's actual connection; `canvas.flush()`.
    pub fn flush(&self) {
        if let Some(api) = xlib() {
            api.flush(self.window.display_address());
        }
    }
}
impl Drop for XwRasterCanvas {
    fn drop(&mut self) {
        if let Some(api) = xlib() {
            api.free_raster(self.window.display_address(), self.raster);
        }
    }
}

/// Resolves a named color while a lease pins its display; `xw_raster_named_pixel(&lease, "Red")`.
pub fn xw_raster_named_pixel(lease: &AspectDisplayConnectionLease, name: &str) -> Option<u64> {
    let name = CString::new(name).ok()?;
    xlib()?.named_pixel(lease.display_address(), &name)
}

/// Flushes a pinned drawing connection; `xw_raster_flush(&lease)`.
pub fn xw_raster_flush(lease: &AspectDisplayConnectionLease) {
    if let Some(api) = xlib() {
        api.flush(lease.display_address());
    }
}

fn raster_dimensions(size: [i32; 2]) -> Result<[u32; 2], Failure> {
    if size.iter().any(|value| !(1..=65535).contains(value)) {
        return Err(raster_failure(&format!(
            "dimensions={size:?}, expected X11 sizes in 1..=65535"
        )));
    }
    Ok(size.map(|value| value as u32))
}
