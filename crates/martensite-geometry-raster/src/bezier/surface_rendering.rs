// Bezier surface pole drawing/picking from GeometryBezierSurface.
//! Typed lazy geometry access retains the existing nonperiodic net and pick order.
use crate::spline_style::{draw_pole_net, find_net_index, scaled_target};
use crate::{PolesStyle, geometry_access::checked_geometry as geometry};
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomBezierSurface;
/// Reads geometry only when poles are shown; `draw_poles(|| handle.clone(), style, display)?`.
pub fn draw_poles(
    surface: impl FnOnce() -> Handle<GeomBezierSurface>,
    style: &PolesStyle,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    if style.shown() {
        let handle = surface();
        let bezier = geometry(&handle)?;
        let counts = (bezier.nb_u_poles(), bezier.nb_v_poles());
        draw_pole_net(display, style.color(), counts, (false, false), |u, v| {
            bezier.pole(u, v)
        })?;
    }
    Ok(())
}
/// Finds first-after-indices poles in pixel units; `find_pole(handle,(x,y),display,1.,(0,0))?`.
pub fn find_pole(
    handle: &Handle<GeomBezierSurface>,
    (x, y): (f64, f64),
    display: &RasterDisplay<'_>,
    precision: f64,
    indices: (i32, i32),
) -> Result<(i32, i32), Failure> {
    let bezier = geometry(handle)?;
    let target = scaled_target(display, (x, y), precision);
    let counts = (bezier.nb_u_poles(), bezier.nb_v_poles());
    find_net_index(display, target, indices, counts, |u, v| bezier.pole(u, v))
}
