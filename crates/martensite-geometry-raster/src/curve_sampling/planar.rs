//! Planar sampling retains interval and deflection evaluation order.
use super::{CurveSampling, bound, interval_bounds, plot_deflection};
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkgeombase::gcpnts::UniformDeflection;
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::{Point2, Point3},
};
pub(super) fn draw_curve(
    settings: &CurveSampling,
    curve: &dyn Adaptor2dCurve2d,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    if settings.draw_mode() == 1 {
        return draw_deflection(settings, curve, display);
    }
    draw_intervals(settings, curve, display)
}
fn draw_deflection(
    settings: &CurveSampling,
    curve: &dyn Adaptor2dCurve2d,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    let deflection = settings.deflection() / display.zoom();
    let line = UniformDeflection::with_curve(curve, deflection, true)?;
    // The C++ code reads the gpPnt of each point through a gpPnt2d pointer: x, y.
    let to_2d = |point: Point3| Point2::new(point.x(), point.y());
    plot_deflection(
        &line,
        display,
        |d, p| d.move_to(to_2d(p)),
        |d, p| d.draw_to(to_2d(p)),
    )
}
fn draw_intervals(
    settings: &CurveSampling,
    curve: &dyn Adaptor2dCurve2d,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    let bounds = interval_bounds(curve.nb_intervals(Shape::CN)?, |b| {
        curve.intervals(b, Shape::CN)
    })?;
    display.move_to(curve.d0(curve.first_parameter()?)?);
    let is_line = curve.get_type()? == CurveType::Line;
    for interval in 1..bounds.len() as i32 {
        draw_interval(settings, curve, display, &bounds, (interval, is_line))?;
    }
    Ok(())
}
fn draw_interval(
    settings: &CurveSampling,
    curve: &dyn Adaptor2dCurve2d,
    display: &mut RasterDisplay<'_>,
    bounds: &Array1<'_, f64>,
    (interval, is_line): (i32, bool),
) -> Result<(), Failure> {
    let mut t = bound(bounds, interval)?;
    if !is_line {
        let step = (bound(bounds, interval + 1)? - t) / f64::from(settings.discretisation());
        for _ in 1..settings.discretisation() {
            t += step;
            display.draw_to(curve.d0(t)?);
        }
    }
    display.draw_to(curve.d0(bound(bounds, interval + 1)?)?);
    Ok(())
}
