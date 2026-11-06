// Existing specialized spline drawing.
//! Drawing borrows actual geometry and actual sampling, pole and knot cells.
use super::SplineRendering;
use crate::geometry_access::checked_geometry as geometry;
use crate::spline_style::{draw_polygon, find_index, scaled_target};
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkmath::gp::Point2;

impl SplineRendering<'_> {
    /// Finds the first pole after `index` in pixel units; `rendering.find_planar_pole(handle, (x,y), display, 1., 0)?`.
    pub fn find_planar_pole(
        &self,
        handle: &Handle<Geom2dBSplineCurve>,
        (x, y): (f64, f64),
        display: &DrawDisplay<'_>,
        precision: f64,
        index: i32,
    ) -> Result<i32, Failure> {
        let bspline = geometry(handle)?;
        let target = scaled_target(display, (x, y), precision);
        find_index(display, target, (index, bspline.nb_poles()), |i| {
            bspline.pole(i)
        })
    }
    /// Finds the first knot after `index` in projected model units; `rendering.find_planar_knot(handle, (x,y), display, 1., 0)?`.
    pub fn find_planar_knot(
        &self,
        handle: &Handle<Geom2dBSplineCurve>,
        (x, y): (f64, f64),
        display: &DrawDisplay<'_>,
        precision: f64,
        index: i32,
    ) -> Result<i32, Failure> {
        let bspline = geometry(handle)?;
        let target = (Point2::new(x, y), precision);
        find_index(display, target, (index, bspline.nb_knots()), |i| {
            bspline.value(bspline.knot(i)?)
        })
    }
    /// Closes periodic poles, draws the curve, then all knots; `rendering.draw_planar(handle, display, draw_curve)?`.
    pub fn draw_planar(
        &self,
        handle: &Handle<Geom2dBSplineCurve>,
        display: &mut DrawDisplay<'_>,
        draw_curve: impl FnOnce(&mut DrawDisplay<'_>) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let bspline = geometry(handle)?;
        self.closed_planar_polygon(bspline, display)?;
        draw_curve(display)?;
        self.all_planar_knots(bspline, display)
    }
    fn closed_planar_polygon(
        &self,
        bspline: &Geom2dBSplineCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        if self.poles.shown() {
            draw_polygon(display, self.poles.color(), bspline.poles().as_slice());
            if bspline.is_periodic() {
                display.draw_to(bspline.pole(1)?);
            }
        }
        Ok(())
    }
    fn all_planar_knots(
        &self,
        bspline: &Geom2dBSplineCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        if self.knots.shown() {
            let knots = bspline.knots();
            self.knots
                .mark(display, knots.iter().map(|u| bspline.value(*u)))?;
        }
        Ok(())
    }
}
