//! Spatial retained spline owns actual controls and live shared geometry once.
use super::{SplineCurveLook, SplineRendering};
use crate::{CurveLook, KnotsStyle, PolesStyle, SpatialCurveDrawable};
use martensite_raster_core::{RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
/// Retained rendering state is Send but not Sync; geometry remains live and shared.
/// ```compile_fail
/// use martensite_geometry_raster::SpatialSplineDrawable;
/// fn shared<T: Sync>() {}
/// shared::<SpatialSplineDrawable>();
/// ```
pub struct SpatialSplineDrawable {
    curve: SpatialCurveDrawable,
    poles: PolesStyle,
    knots: KnotsStyle,
}
impl SpatialSplineDrawable {
    /// Creates green curves, red poles and violet knots; `Self::new(geometry)`.
    pub fn new(curve: Handle<GeomBSplineCurve>) -> Self {
        Self::with_style(curve, &SplineCurveLook::default(), (16, 0.05, 1))
    }
    /// Creates explicit retained controls; `Self::with_style(curve, &look, settings)`.
    pub fn with_style(
        curve: Handle<GeomBSplineCurve>,
        look: &SplineCurveLook,
        settings: (i32, f64, i32),
    ) -> Self {
        Self {
            curve: SpatialCurveDrawable::with_look(
                curve.cast::<dyn GeomCurve>(),
                CurveLook::new(look.curve_color),
                settings,
            ),
            poles: look.poles(),
            knots: look.knots(),
        }
    }
    /// Borrows the live generic curve; `curve.curve_part().style().show_curvature()`.
    pub fn curve_part(&self) -> &SpatialCurveDrawable {
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
    fn bspline(&self) -> Handle<GeomBSplineCurve> {
        self.curve.curve().cast::<GeomBSplineCurve>()
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
            .find_spatial_pole(&handle, point, display, precision, index)
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
            .find_spatial_knot(&handle, point, display, precision, index)
    }
    /// Draws the open pole polygon and all knots; `curve.draw_on_with(display,true,true)?`.
    pub fn draw_on_with(
        &self,
        display: &mut RasterDisplay<'_>,
        poles: bool,
        knots: bool,
    ) -> Result<(), Failure> {
        let handle = self.bspline();
        self.rendering()
            .draw_spatial_with(&handle, display, poles, knots, |display| {
                self.curve.draw_on(display)
            })
    }
    /// Draws selected spans and a pole neighborhood; `curve.draw_on_range(display,(a,b),0,(true,true))?`.
    pub fn draw_on_range(
        &self,
        display: &mut RasterDisplay<'_>,
        range: (f64, f64),
        pole: i32,
        flags: (bool, bool),
    ) -> Result<(), Failure> {
        let handle = self.bspline();
        self.rendering()
            .draw_spatial_range(&handle, display, range, pole, flags)
    }
}
impl RasterDrawable for SpatialSplineDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        let handle = self.bspline();
        self.rendering()
            .draw_spatial(&handle, display, |display| self.curve.draw_on(display))
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
