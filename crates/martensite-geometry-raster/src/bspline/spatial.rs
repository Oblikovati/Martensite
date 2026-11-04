// Existing specialized spline drawing.
//! Drawing borrows actual geometry and actual sampling, pole and knot cells.
use super::SplineRendering;
use super::range::{RangeSpans, draw_pole_neighbourhood};
use crate::geometry_access::checked_geometry as geometry;
use crate::spline_style::{draw_polygon, find_index, scaled_target};
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::standard::real::epsilon;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkmath::gp::Point2;

impl SplineRendering<'_> {
    /// Draws an open pole polygon, curve, then all knots; `rendering.draw_spatial_with(handle, display, true, true, draw_curve)?`.
    pub fn draw_spatial_with(
        &self,
        handle: &Handle<GeomBSplineCurve>,
        display: &mut DrawDisplay<'_>,
        show_poles: bool,
        show_knots: bool,
        draw_curve: impl FnOnce(&mut DrawDisplay<'_>) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let bspline = geometry(handle)?;
        if self.poles.shown() && show_poles {
            draw_polygon(display, self.poles.color(), bspline.poles().as_slice());
        }
        draw_curve(display)?;
        if self.knots.shown() && show_knots {
            let knots = bspline.knots();
            self.knots
                .mark(display, knots.iter().map(|u| bspline.value(*u)))?;
        }
        Ok(())
    }
    /// Preserves ranged spans and pole neighborhoods; `rendering.draw_spatial_range(handle, display, (a,b), 0, (true,true))?`.
    pub fn draw_spatial_range(
        &self,
        handle: &Handle<GeomBSplineCurve>,
        display: &mut DrawDisplay<'_>,
        (u1, u2): (f64, f64),
        pole_index: i32,
        (show_poles, show_knots): (bool, bool),
    ) -> Result<(), Failure> {
        let bspline = geometry(handle)?;
        let (i1, j1) = bspline.locate_u(u1, epsilon(u1).abs(), false)?;
        let (i2, j2) = bspline.locate_u(u2, epsilon(u2).abs(), false)?;
        self.range_poles(display, bspline, pole_index, show_poles)?;
        display.set_color(self.style.color());
        self.range_curve(display, bspline, (u1, u2), (i1, j2))?;
        self.range_knots(display, bspline, j1..=i2, show_knots)
    }
    /// Finds the first pole after `index` in pixel units; `rendering.find_spatial_pole(handle, (x,y), display, 1., 0)?`.
    pub fn find_spatial_pole(
        &self,
        handle: &Handle<GeomBSplineCurve>,
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
    /// Finds the first knot after `index` in projected model units; `rendering.find_spatial_knot(handle, (x,y), display, 1., 0)?`.
    pub fn find_spatial_knot(
        &self,
        handle: &Handle<GeomBSplineCurve>,
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
    /// Closes periodic poles, draws the curve, then used knots; `rendering.draw_spatial(handle, display, draw_curve)?`.
    pub fn draw_spatial(
        &self,
        handle: &Handle<GeomBSplineCurve>,
        display: &mut DrawDisplay<'_>,
        draw_curve: impl FnOnce(&mut DrawDisplay<'_>) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let bspline = geometry(handle)?;
        self.closed_spatial_polygon(bspline, display)?;
        draw_curve(display)?;
        self.used_spatial_knots(bspline, display)
    }
    fn closed_spatial_polygon(
        &self,
        bspline: &GeomBSplineCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        if self.poles.shown() {
            let poles = bspline.poles();
            draw_polygon(display, self.poles.color(), poles.as_slice());
            if bspline.is_periodic() {
                display.draw_to_3d(bspline.pole(1)?);
            }
        }
        Ok(())
    }
    fn used_spatial_knots(
        &self,
        bspline: &GeomBSplineCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        if self.knots.shown() {
            let first = bspline.first_u_knot_index()?;
            let last = bspline.last_u_knot_index()?;
            let marked = (first..=last).map(|i| bspline.value(bspline.knot(i)?));
            self.knots.mark(display, marked)?;
        }
        Ok(())
    }
    fn range_poles(
        &self,
        display: &mut DrawDisplay<'_>,
        bspline: &GeomBSplineCurve,
        pole_index: i32,
        show_poles: bool,
    ) -> Result<(), Failure> {
        if self.poles.shown() && show_poles {
            display.set_color(self.poles.color());
            draw_pole_neighbourhood(display, bspline, pole_index)?;
        }
        Ok(())
    }
    fn range_curve(
        &self,
        display: &mut DrawDisplay<'_>,
        bspline: &GeomBSplineCurve,
        (u1, u2): (f64, f64),
        indices: (i32, i32),
    ) -> Result<(), Failure> {
        if bspline.degree() == 1 {
            display.move_to_3d(bspline.value(u1)?);
            display.draw_to_3d(bspline.value(u2)?);
        } else {
            let span = RangeSpans {
                bspline,
                discret: self.sampling.discretisation(),
            };
            span.draw(display, (u1, u2), indices)?;
        }
        Ok(())
    }
    fn range_knots(
        &self,
        display: &mut DrawDisplay<'_>,
        bspline: &GeomBSplineCurve,
        indices: std::ops::RangeInclusive<i32>,
        show_knots: bool,
    ) -> Result<(), Failure> {
        if self.knots.shown() && show_knots {
            let marked = indices.map(|i| bspline.value(bspline.knot(i)?));
            self.knots.mark(display, marked)?;
        }
        Ok(())
    }
}
