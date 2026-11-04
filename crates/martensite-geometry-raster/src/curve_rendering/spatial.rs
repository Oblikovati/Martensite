//
//! Live spatial geometry rendering borrows the actual caller-owned controls.
use crate::curve_properties::{CurvatureDisplay, draw_orientation_arrow, finite_range};
use crate::{CurveSampling, CurveStyle, geometry_access::checked_geometry as geometry};
use martensite_raster_core::{RasterDisplay as DrawDisplay, sampling::interval_bounds};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve, geom::GeomCurve, geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkgeombase::geom_lprop::CLProps;
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::{Point3, Vector2},
};
/// A live geometry record; drawing borrows mutable controls and the thread-confined display.
/// The retained geometry remains live and shared with its owner. Each record is
/// movable between threads but its rendering context is confined to one thread.
/// ```compile_fail
/// use martensite_geometry_raster::SpatialCurveRendering;
/// fn shared<T: Sync>() {}
/// shared::<SpatialCurveRendering>();
/// ```
pub struct SpatialCurveRendering {
    curve: Handle<dyn GeomCurve>,
    // A live rendering record is confined even when the underlying geometry handle is shared.
    thread_confined: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl SpatialCurveRendering {
    /// Retains the existing geometry handle; `SpatialCurveRendering::new(curve)`.
    pub fn new(curve: Handle<dyn GeomCurve>) -> Self {
        Self {
            curve,
            thread_confined: std::marker::PhantomData,
        }
    }
    /// Shares the live geometry handle; `let geometry = rendering.curve();`.
    pub fn curve(&self) -> Handle<dyn GeomCurve> {
        self.curve.clone()
    }
    /// Borrows geometry without increasing its reference count; `rendering.curve_ref()`.
    pub fn curve_ref(&self) -> &Handle<dyn GeomCurve> {
        &self.curve
    }
    /// Draws using the actual retained controls; `rendering.draw_on(&sampling, &style, &mut display)?;`.
    pub fn draw_on(
        &self,
        sampling: &CurveSampling,
        style: &CurveStyle,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        let curve = geometry(&self.curve)?;
        let range = (curve.first_parameter(), curve.last_parameter());
        let (first, last) = finite_range::<Point3>(range, |t| curve.d0(t))?;
        display.set_color(style.color());
        let adaptor = GeomAdaptorCurve::with_range(&self.curve, first, last)?;
        sampling.draw_curve_on(&adaptor, display)?;
        if style.display_origin() {
            draw_origin(&adaptor, last, display)?;
        }
        if style.curvature_shown() && adaptor.get_type()? != CurveType::Line {
            self.draw_curvature(sampling, style, &adaptor, display)?;
        }
        Ok(())
    }
    /// The curvature radii of a non-linear curve.
    fn draw_curvature(
        &self,
        sampling: &CurveSampling,
        style: &CurveStyle,
        adaptor: &GeomAdaptorCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        let bounds = interval_bounds(adaptor.nb_intervals(Shape::CN)?, |b| {
            adaptor.intervals(b, Shape::CN)
        })?;
        let radii = CurvatureDisplay::of(sampling.discretisation(), style);
        let mut props = CLProps::new(self.curve.clone(), 2, radii.resolution())?;
        let curve = geometry(&self.curve)?;
        radii.draw(display, &bounds, &mut props, |t| curve.d0(t))
    }
}
/// The orientation arrow at the end of the drawn range, from the projection of the
/// end point and of the end point moved by the first derivative.
fn draw_origin(
    adaptor: &GeomAdaptorCurve,
    last: f64,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    let (point, vector) = adaptor.d1(last)?;
    let p1 = display.project(point);
    let p2 = display.project(point.translated(vector));
    draw_orientation_arrow(display, p1, Vector2::between(p1, p2))
}
