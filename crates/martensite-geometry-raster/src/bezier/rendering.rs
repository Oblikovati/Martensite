// Drawing and FindPole bodies from GeometryBezierCurve{,2d}; geometry reads remain lazy.
//! Typed pole rendering borrows the caller's actual geometry and style.
use crate::spline_style::{draw_polygon, find_index, scaled_target};
use crate::{PolesStyle, curve_properties::PlotPoint, geometry_access::checked_geometry};
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::{
    ncollection::Array1,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::GeomBezierCurve;
/// Reads geometry only when poles are shown; `draw_spatial(|| handle.clone(), style, display)?`.
pub fn draw_spatial(
    curve: impl FnOnce() -> Handle<GeomBezierCurve>,
    style: &PolesStyle,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    draw_control_polygon(style, display, || {
        let handle = curve();
        Ok(checked_geometry(&handle)?.poles())
    })
}
/// Reads planar geometry only when shown; `draw_planar(|| handle.clone(), style, display)?`.
pub fn draw_planar(
    curve: impl FnOnce() -> Handle<Geom2dBezierCurve>,
    style: &PolesStyle,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    draw_control_polygon(style, display, || {
        let handle = curve();
        Ok(checked_geometry(&handle)?.poles())
    })
}
fn draw_control_polygon<P: PlotPoint + 'static>(
    style: &PolesStyle,
    display: &mut RasterDisplay<'_>,
    read: impl FnOnce() -> Result<Array1<'static, P>, Failure>,
) -> Result<(), Failure> {
    if style.shown() {
        let poles = read()?;
        draw_polygon(display, style.color(), poles.as_slice());
    }
    Ok(())
}
/// Finds the first pole after `index` in pixel units; `find_spatial(&handle, (x,y), display, 1., 0)?`.
pub fn find_spatial(
    handle: &Handle<GeomBezierCurve>,
    (x, y): (f64, f64),
    display: &RasterDisplay<'_>,
    precision: f64,
    index: i32,
) -> Result<i32, Failure> {
    let bezier = checked_geometry(handle)?;
    let target = scaled_target(display, (x, y), precision);
    find_index(display, target, (index, bezier.nb_poles()), |i| {
        bezier.pole(i)
    })
}
/// Projects planar poles as `(x,y,0)`; `find_planar(&handle, (x,y), display, 1., 0)?`.
pub fn find_planar(
    handle: &Handle<Geom2dBezierCurve>,
    (x, y): (f64, f64),
    display: &RasterDisplay<'_>,
    precision: f64,
    index: i32,
) -> Result<i32, Failure> {
    let bezier = checked_geometry(handle)?;
    let target = scaled_target(display, (x, y), precision);
    find_index(display, target, (index, bezier.nb_poles()), |i| {
        bezier.pole(i)
    })
}
