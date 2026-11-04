//! Retained generic surfaces own one geometry record and actual mutable controls.
mod rendering;
mod retained;
mod style;
pub use rendering::SurfaceRendering;
pub use retained::SurfaceDrawable;
pub use style::{SurfaceLook, SurfaceStyle};
