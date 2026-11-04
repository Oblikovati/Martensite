//! Specialized retained curves and surfaces keep live geometry and actual controls.
mod planar;
pub mod rendering;
mod spatial;
pub use planar::PlanarBezierDrawable;
pub use spatial::SpatialBezierDrawable;

mod surface;
pub mod surface_rendering;
pub use surface::BezierSurfaceDrawable;
