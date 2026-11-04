//! Planar retained spline owns actual controls and live shared geometry once.
use super::{SplineCurveLook, SplineRendering};
use crate::{CurveLook, KnotsStyle, PlanarCurveDrawable, PolesStyle};
use martensite_raster_core::{RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
/// Retained rendering state is Send but not Sync; geometry remains live and shared.
/// ```compile_fail
/// use martensite_geometry_raster::PlanarSplineDrawable;
/// fn shared<T: Sync>() {}
/// shared::<PlanarSplineDrawable>();
/// ```
pub struct PlanarSplineDrawable {
    curve: PlanarCurveDrawable,
    poles: PolesStyle,
    knots: KnotsStyle,
}
impl PlanarSplineDrawable {
    /// Creates green curves, red poles and violet knots; `Self::new(geometry)`.
    pub fn new(curve: Handle<Geom2dBSplineCurve>) -> Self {
        Self::with_style(curve, &SplineCurveLook::default(), 100)
    }
    /// Creates explicit retained controls; `Self::with_style(curve, &look, settings)`.
    pub fn with_style(
        curve: Handle<Geom2dBSplineCurve>,
        look: &SplineCurveLook,
        settings: i32,
    ) -> Self {
        Self {
            curve: PlanarCurveDrawable::with_look(
                curve.cast::<dyn Geom2dCurve>(),
                CurveLook::new(look.curve_color),
                settings,
            ),
            poles: look.poles(),
            knots: look.knots(),
        }
    }
    /// Borrows the live generic curve; `curve.curve_part().style().show_curvature()`.
    pub fn curve_part(&self) -> &PlanarCurveDrawable {
        &self.curve
    }
    /// Borrows the live pole options; `curve.poles().clear()`.
    pub fn poles(&self) -> &PolesStyle {
        &self.poles
    }
    /// Borrows the live knot options; `curve.knots().clear()`.
    pub fn knots(&self) -> &KnotsStyle {
        &self.knots
    }
    fn bspline(&self) -> Handle<Geom2dBSplineCurve> {
        self.curve.curve().cast::<Geom2dBSplineCurve>()
    }
    fn rendering(&self) -> SplineRendering<'_> {
        SplineRendering::new(
            self.curve.sampling(),
            self.curve.style(),
            &self.poles,
            &self.knots,
        )
    }
    /// Finds first pole after `index` in pixel units; `curve.find_pole((x,y), display, 1., 0)?`.
    pub fn find_pole(
        &self,
        point: (f64, f64),
        display: &RasterDisplay<'_>,
        precision: f64,
        index: i32,
    ) -> Result<i32, Failure> {
        let handle = self.bspline();
        self.rendering()
            .find_planar_pole(&handle, point, display, precision, index)
    }
    /// Finds first knot after `index` in model units; `curve.find_knot((x,y), display, 1., 0)?`.
    pub fn find_knot(
        &self,
        point: (f64, f64),
        display: &RasterDisplay<'_>,
        precision: f64,
        index: i32,
    ) -> Result<i32, Failure> {
        let handle = self.bspline();
        self.rendering()
            .find_planar_knot(&handle, point, display, precision, index)
    }
}
impl RasterDrawable for PlanarSplineDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        let handle = self.bspline();
        self.rendering()
            .draw_planar(&handle, display, |display| self.curve.draw_on(display))
    }
    fn is_3d(&self) -> bool {
        self.curve.is_3d()
    }
    fn bounds(&self) -> [f64; 4] {
        self.curve.bounds()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.curve.set_bounds(bounds);
    }
    fn visible(&self) -> bool {
        self.curve.visible()
    }
    fn set_visible(&self, visible: bool) {
        self.curve.set_visible(visible);
    }
}
