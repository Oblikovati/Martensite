//! Planar retained Bezier drawing uses live generic controls and pole options.
use super::rendering;
use crate::{CurveLook, PlanarCurveDrawable, PolesStyle};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
/// Geometry is shared live; mutable rendering cells require one rendering thread.
/// ```compile_fail
/// use martensite_geometry_raster::PlanarBezierDrawable;
/// fn shared<T: Sync>() {}
/// shared::<PlanarBezierDrawable>();
/// ```
pub struct PlanarBezierDrawable {
    curve: PlanarCurveDrawable,
    poles: PolesStyle,
}
impl PlanarBezierDrawable {
    /// Creates green curves with shown red poles; `let curve = Self::new(geometry);`.
    pub fn new(curve: Handle<Geom2dBezierCurve>) -> Self {
        Self::with_style(
            curve,
            (
                PaletteColor::new(PaletteKind::Vert),
                PaletteColor::new(PaletteKind::Rouge),
            ),
            true,
            50,
        )
    }
    /// Creates explicit retained styles; `Self::with_style(curve, colors, true, settings)`.
    pub fn with_style(
        curve: Handle<Geom2dBezierCurve>,
        colors: (PaletteColor, PaletteColor),
        shown: bool,
        settings: i32,
    ) -> Self {
        Self {
            curve: PlanarCurveDrawable::with_look(
                curve.cast::<dyn Geom2dCurve>(),
                CurveLook::new(colors.0),
                settings,
            ),
            poles: PolesStyle::new(shown, colors.1),
        }
    }
    /// Borrows actual generic controls; `curve.curve_part().style().show_curvature()`.
    pub fn curve_part(&self) -> &PlanarCurveDrawable {
        &self.curve
    }
    /// Borrows actual pole options; `curve.poles().clear()`.
    pub fn poles(&self) -> &PolesStyle {
        &self.poles
    }
    fn bezier(&self) -> Handle<Geom2dBezierCurve> {
        self.curve.curve().cast::<Geom2dBezierCurve>()
    }
    /// Finds the first pole after `index`; `curve.find_pole((x,y), display, 1., 0)?`.
    pub fn find_pole(
        &self,
        point: (f64, f64),
        display: &RasterDisplay<'_>,
        precision: f64,
        index: i32,
    ) -> Result<i32, Failure> {
        let handle = self.bezier();
        rendering::find_planar(&handle, point, display, precision, index)
    }
}
impl RasterDrawable for PlanarBezierDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        rendering::draw_planar(|| self.bezier(), &self.poles, display)?;
        self.curve.draw_on(display)
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
