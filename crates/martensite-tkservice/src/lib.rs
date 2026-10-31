//! The visualization services: windows, images, fonts, the presentation data model and
//! the graphic driver interfaces.
//!
//! - [`aspect`]: window, grid, background and input definitions shared by the viewers.
//! - [`image`]: pixel maps, pixel formats and image comparison.
//! - [`font`]: font manager, FreeType fonts and text layout.
//! - [`graphic3d`]: the 3D presentation data model and the graphic driver interfaces.
//! - [`media`]: the media player context (built without FFmpeg).
//! - `xw`: the Xlib window (Linux).
//! - [`wnt`]: native Windows windows with owned message dispatch or borrowed HWNDs.
//! - [`cocoa`]: the macOS window interface, whose native backend is unfinished;
//!   [`platform`] names its typed error.
//!
//! See `docs/features/martensite-tkservice/README.md` for the platform interfaces and
//! run-time libraries.
pub mod aspect;
pub mod cocoa;
pub mod font;
pub mod graphic3d;
pub mod image;
pub mod media;
pub mod null_driver;
pub mod platform;
pub mod wasm;
pub mod wnt;
#[cfg(target_os = "linux")]
pub mod xw;
/// Xw without the Xlib backend: the window interface whose constructors raise.
#[cfg(not(target_os = "linux"))]
#[path = "xw_unsupported.rs"]
pub mod xw;

mod cast;
mod dynlib;
pub mod json_dump;
#[cfg(target_os = "linux")]
mod xlib;
