//! Specialized spline rendering borrows actual controls; retained variants own them once.
mod look;
mod planar;
mod range;
mod retained_planar;
mod retained_spatial;
mod spatial;
use crate::{CurveSampling, CurveStyle, KnotsStyle, PolesStyle};
pub use look::SplineCurveLook;
pub use retained_planar::PlanarSplineDrawable;
pub use retained_spatial::SpatialSplineDrawable;
/// Borrowed Cell-based rendering controls are confined to the current drawing thread.
/// This temporary command view is neither Send nor Sync; retained owners are Send.
/// ```compile_fail
/// use martensite_geometry_raster::bspline::SplineRendering;
/// fn transferable<T: Send>() {}
/// transferable::<SplineRendering<'static>>();
/// ```
/// ```compile_fail
/// use martensite_geometry_raster::bspline::SplineRendering;
/// fn shared<T: Sync>() {}
/// shared::<SplineRendering<'static>>();
/// ```
pub struct SplineRendering<'a> {
    sampling: &'a CurveSampling,
    style: &'a CurveStyle,
    poles: &'a PolesStyle,
    knots: &'a KnotsStyle,
}
impl<'a> SplineRendering<'a> {
    /// Borrows live cells without snapshots; `SplineRendering::new(sampling, style, poles, knots)`.
    pub fn new(
        sampling: &'a CurveSampling,
        style: &'a CurveStyle,
        poles: &'a PolesStyle,
        knots: &'a KnotsStyle,
    ) -> Self {
        Self {
            sampling,
            style,
            poles,
            knots,
        }
    }
}
