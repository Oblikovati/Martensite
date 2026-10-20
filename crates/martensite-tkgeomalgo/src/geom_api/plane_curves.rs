//! The `GeomAPI` class: the conversions between a 3D curve lying in a plane and the
//! 2D curve of its parametric space.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg2d::geom2d_adaptor::{self, Geom2dAdaptorCurve};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dCurveOnSurface, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomCurve, GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::{self, GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkgeombase::proj_lib::ProjectedCurve;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::Plane3;

/// `GeomAPI::To2d(C, P)`: the 2D curve, in the parametric space of the plane `P`,
/// of the 3D curve `C` projected on `P` (ProjLib_ProjectedCurve); a null handle when
/// the projection is an offset curve or another curve. The exceptions of the
/// adaptor (a null curve), of the projection and of Geom2dAdaptor::MakeCurve are
/// returned.
///
/// ```
/// use martensite_tkgeomalgo::geom_api::to_2d;
/// use martensite_tkg3d::geom::{GeomCurve, GeomLine};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkmath::gp::{Direction3, Plane3, Point3};
/// let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
///     Point3::new(1.0, 2.0, 0.0), Direction3::new(1.0, 0.0, 0.0).unwrap())).cast();
/// let curve = to_2d(&line, &Plane3::default()).unwrap();
/// assert!(!curve.is_null());
/// ```
pub fn to_2d(c: &Handle<dyn GeomCurve>, p: &Plane3) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let hc: Handle<dyn Adaptor3dCurve> = Handle::new(GeomAdaptorCurve::with_curve(c)?).cast();
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(*p)).cast();
    let hs: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&plane)?).cast();
    let proj: ProjectedCurve = ProjectedCurve::with_curve(&hs, &hc)?;
    let kind: CurveType = proj.get_type()?;
    if kind == CurveType::OffsetCurve || kind == CurveType::OtherCurve {
        return Ok(Handle::default());
    }
    geom2d_adaptor::make_curve(&proj)
}

/// `GeomAPI::To3d(C, P)`: the 3D curve of the 2D curve `C` of the parametric space
/// of the plane `P` (Adaptor3d_CurveOnSurface made into a curve by
/// GeomAdaptor::MakeCurve), of the same nature as `C`.
///
/// ```
/// use martensite_tkgeomalgo::geom_api::to_3d;
/// use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkmath::gp::{Direction2, Plane3, Point2};
/// let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
///     Point2::new(1.0, 2.0), Direction2::new(1.0, 0.0).unwrap())).cast();
/// let curve = to_3d(&line, &Plane3::default()).unwrap();
/// assert!(!curve.is_null());
/// ```
pub fn to_3d(c: &Handle<dyn Geom2dCurve>, p: &Plane3) -> Result<Handle<dyn GeomCurve>, Failure> {
    let ahc: Handle<dyn Adaptor2dCurve2d> = Handle::new(Geom2dAdaptorCurve::with_curve(c)?).cast();
    let the_plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(*p)).cast();
    let ahs: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&the_plane)?).cast();
    let cos = Adaptor3dCurveOnSurface::with_curve(&ahc, &ahs)?;
    geom_adaptor::make_curve(&cos)
}
