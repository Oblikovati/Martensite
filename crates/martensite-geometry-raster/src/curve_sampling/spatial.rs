//! Spatial sampling keeps the legacy interval refinement state local to one draw.
use super::{CurveSampling, bound, interval_bounds, plot_deflection};
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::{ncollection::Array1, precision, standard::Failure};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::gcpnts::UniformDeflection;
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::Point3,
};
/// `IsoRatio` of `PlotCurve`: a chord is kept when the midpoint is at most 0.1% off.
const ISO_RATIO: f64 = 1.001;
pub(super) fn draw_curve(
    settings: &CurveSampling,
    curve: &dyn Adaptor3dCurve,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    if settings.draw_mode() == 1 {
        let deflection = settings.deflection() / display.zoom();
        let line = UniformDeflection::with_curve(curve, deflection, true)?;
        return plot_deflection(
            &line,
            display,
            |d, p| d.move_to_3d(p),
            |d, p| d.draw_to_3d(p),
        );
    }
    draw_intervals(settings, curve, display)
}

/// The discrete mode of `DrawCurveOn`: circles and ellipses take `discret` steps,
/// the other non-linear curves `discret / 2` double steps refined by `PlotCurve`.
///
/// Native quirk kept: the previous point of the refinement is not moved to the end
/// of an interval, so the next interval starts its first chord there.
fn draw_intervals(
    settings: &CurveSampling,
    curve: &dyn Adaptor3dCurve,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    let bounds = interval_bounds(curve.nb_intervals(Shape::CN)?, |b| {
        curve.intervals(b, Shape::CN)
    })?;
    let first = curve.d0(curve.first_parameter()?)?;
    display.move_to_3d(first);
    let mut drawing = SpatialDrawing::new(curve, display, first)?;
    for interval in 1..bounds.len() as i32 {
        drawing.draw_interval(settings, &bounds, interval)?;
    }
    Ok(())
}
struct SpatialDrawing<'curve, 'display, 'window> {
    curve: &'curve dyn Adaptor3dCurve,
    display: &'display mut DrawDisplay<'window>,
    curve_type: CurveType,
    previous: Point3,
}
impl<'curve, 'display, 'window> SpatialDrawing<'curve, 'display, 'window> {
    fn new(
        curve: &'curve dyn Adaptor3dCurve,
        display: &'display mut DrawDisplay<'window>,
        previous: Point3,
    ) -> Result<Self, Failure> {
        let curve_type = curve.get_type()?;
        Ok(Self {
            curve,
            display,
            curve_type,
            previous,
        })
    }
    fn draw_interval(
        &mut self,
        settings: &CurveSampling,
        bounds: &Array1<'_, f64>,
        interval: i32,
    ) -> Result<(), Failure> {
        let mut t = bound(bounds, interval)?;
        let end = bound(bounds, interval + 1)?;
        let step = (end - t) / f64::from(settings.discretisation());
        match self.curve_type {
            CurveType::Line => {}
            CurveType::Circle | CurveType::Ellipse => self.draw_conic(settings, &mut t, step)?,
            _ => self.draw_adaptive(settings, &mut t, step)?,
        }
        self.finish_interval(t, step, end)
    }
    fn draw_conic(
        &mut self,
        settings: &CurveSampling,
        t: &mut f64,
        step: f64,
    ) -> Result<(), Failure> {
        for _ in 1..settings.discretisation() {
            *t += step;
            self.display.draw_to_3d(self.curve.d0(*t)?);
        }
        Ok(())
    }
    fn draw_adaptive(
        &mut self,
        settings: &CurveSampling,
        t: &mut f64,
        step: f64,
    ) -> Result<(), Failure> {
        for _ in 1..settings.discretisation() / 2 {
            self.draw_adaptive_step(t, step)?;
        }
        Ok(())
    }
    fn draw_adaptive_step(&mut self, t: &mut f64, step: f64) -> Result<(), Failure> {
        let t1 = *t + step * 2.;
        let next = self.curve.d0(t1)?;
        plot_curve(self.display, self.curve, *t, step, self.previous, next)?;
        self.previous = next;
        *t = t1;
        Ok(())
    }
    fn finish_interval(&mut self, t: f64, step: f64, end: f64) -> Result<(), Failure> {
        let last = self.curve.d0(end)?;
        plot_curve(self.display, self.curve, t, step, self.previous, last)?;
        self.display.draw_to_3d(last);
        Ok(())
    }
}
/// `PlotCurve`: draws to `last` when the curve midpoint is within `ISO_RATIO` of the
/// chord `first`-`last` (or the chord is shorter than the confusion), else halves the
/// step on both halves.
fn plot_curve(
    display: &mut DrawDisplay<'_>,
    curve: &dyn Adaptor3dCurve,
    first_param: f64,
    half_step: f64,
    first: Point3,
    last: Point3,
) -> Result<(), Failure> {
    let middle = curve.d0(first_param + half_step)?;
    let length = first.distance(&last);
    if length < precision::confusion()
        || middle.distance(&first) + middle.distance(&last) <= ISO_RATIO * length
    {
        display.draw_to_3d(last);
        return Ok(());
    }
    plot_curve(display, curve, first_param, half_step / 2., first, middle)?;
    let local_first = first_param + half_step;
    plot_curve(display, curve, local_first, half_step / 2., middle, last)
}
