//! Iso dispatch preserves spline conversion before shared curve sampling.
use super::CurveSampling;
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dIsoCurve},
    geom::GeomCurve,
    geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkmath::geom_abs::{CurveType, IsoType};
pub(super) fn draw_curve(
    settings: &CurveSampling,
    curve: &Adaptor3dIsoCurve,
    (iso, parameter): (IsoType, f64),
    (first, last): (f64, f64),
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    curve.load_iso_range(iso, parameter, (first, last))?;
    let Some(geometry) = spline_geometry(curve)? else {
        return settings.draw_curve_on(curve, display);
    };
    let adaptor = GeomAdaptorCurve::default();
    adaptor.load_range(&geometry, first, last)?;
    settings.draw_curve_on(&adaptor, display)
}
fn spline_geometry(curve: &Adaptor3dIsoCurve) -> Result<Option<Handle<dyn GeomCurve>>, Failure> {
    match curve.get_type()? {
        CurveType::BezierCurve => Ok(Some(curve.bezier()?.cast::<dyn GeomCurve>())),
        CurveType::BSplineCurve => Ok(Some(curve.bspline()?.cast::<dyn GeomCurve>())),
        _ => Ok(None),
    }
}
