//! Curve engines retain live geometry and borrow controls without interpreter state.
mod planar;
mod spatial;
pub use planar::PlanarCurveRendering;
pub use spatial::SpatialCurveRendering;
