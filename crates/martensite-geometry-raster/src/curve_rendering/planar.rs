//
//! Live planar rendering keeps replacement and the original per-stage handle reads local.
use crate::curve_properties::{CurvatureDisplay, draw_orientation_arrow, finite_range};
use crate::{CurveSampling, CurveStyle, geometry_access::checked_geometry as geometry};
use martensite_raster_core::{RasterDisplay as DrawDisplay, sampling::interval_bounds};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d, geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeombase::geom_lprop::CLProps2d;
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::Point2,
};
use std::cell::RefCell;
/// Thread-confined mutable geometry selection; the retained handle remains live.
/// ```compile_fail
/// use martensite_geometry_raster::PlanarCurveRendering;
/// fn shared<T: Sync>() {}
/// shared::<PlanarCurveRendering>();
/// ```
pub struct PlanarCurveRendering {
    curve: RefCell<Handle<dyn Geom2dCurve>>,
}
impl PlanarCurveRendering {
    /// Retains the existing geometry handle; `PlanarCurveRendering::new(curve)`.
    pub fn new(curve: Handle<dyn Geom2dCurve>) -> Self {
        Self {
            curve: RefCell::new(curve),
        }
    }
    /// Shares the current geometry handle; `let geometry = rendering.curve();`.
    pub fn curve(&self) -> Handle<dyn Geom2dCurve> {
        self.curve.borrow().clone()
    }
    /// Replaces the live geometry selection; `rendering.set_curve(curve)`.
    pub fn set_curve(&self, curve: Handle<dyn Geom2dCurve>) {
        *self.curve.borrow_mut() = curve;
    }
    /// Draws with actual mutable controls; `rendering.draw_on(&sampling, &style, &mut display)?;`.
    pub fn draw_on(
        &self,
        sampling: &CurveSampling,
        style: &CurveStyle,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        let handle = self.curve();
        let (first, last) = finite_parameters(&handle)?;
        display.set_color(style.color());
        let adaptor = Geom2dAdaptorCurve::with_range(&handle, first, last)?;
        sampling.draw_curve2d_on(&adaptor, display)?;
        if style.display_origin() {
            self.draw_origin(&adaptor, last, display)?;
        }
        if style.curvature_shown() && adaptor.get_type()? != CurveType::Line {
            self.draw_curvature(sampling, style, &adaptor, display)?;
        }
        Ok(())
    }
    /// The orientation arrow at the end of the drawn range.
    fn draw_origin(
        &self,
        adaptor: &Geom2dAdaptorCurve,
        last: f64,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        let (point, vector) = adaptor.d1(last)?;
        draw_orientation_arrow(display, point, vector)
    }
    /// The curvature radii of a non-linear curve.
    fn draw_curvature(
        &self,
        sampling: &CurveSampling,
        style: &CurveStyle,
        adaptor: &Geom2dAdaptorCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        let bounds = interval_bounds(adaptor.nb_intervals(Shape::CN)?, |b| {
            adaptor.intervals(b, Shape::CN)
        })?;
        let radii = CurvatureDisplay::of(sampling.discretisation(), style);
        let mut props = CLProps2d::new(self.curve(), 2, radii.resolution())?;
        let handle = self.curve();
        let curve = geometry(&handle)?;
        radii.draw(display, &bounds, &mut props, |t| curve.d0(t))
    }
}

fn finite_parameters(handle: &Handle<dyn Geom2dCurve>) -> Result<(f64, f64), Failure> {
    let curve = geometry(handle)?;
    let range = (curve.first_parameter(), curve.last_parameter());
    finite_range::<Point2>(range, |t| curve.d0(t))
}
