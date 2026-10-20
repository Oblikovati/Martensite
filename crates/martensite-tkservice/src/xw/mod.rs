//! Xw: the Xlib window, over the project-owned Xlib wrapper.
mod raster;
mod window;
mod window_events;

pub use crate::xlib::XwXEvent;
pub use window::{XwWindow, XwWindowFields};

pub use raster::{XwRasterCanvas, xw_raster_flush, xw_raster_named_pixel};
