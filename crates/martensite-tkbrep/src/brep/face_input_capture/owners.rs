//! Typed retained owners keep every discovered cell alive until all leases drop.
use super::types::FaceCaptureIncomplete as Incomplete;
use crate::{brep::*, topods::TShape};
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCurve, GeomLine, GeomSurface, GeomSurfaceOfLinearExtrusion,
};

#[derive(Clone)]
pub(super) enum Owner {
    Topology(Handle<dyn TShape>),
    Spatial(Handle<BRepCurve3D>),
    Parametric(Handle<BRepCurveOnSurface>),
    Seam(Handle<BRepCurveOnClosedSurface>),
    Line(Handle<GeomLine>),
    PlanarLine(Handle<Geom2dLine>),
    Spline(Handle<GeomBSplineCurve>),
    Extrusion(Handle<GeomSurfaceOfLinearExtrusion>),
}
impl Owner {
    pub(super) fn key(&self) -> usize {
        match self {
            Self::Topology(h) => h.object_address() as usize,
            Self::Spatial(h) => h.object_address() as usize,
            Self::Parametric(h) => h.object_address() as usize,
            Self::Seam(h) => h.object_address() as usize,
            Self::Line(h) => h.object_address() as usize,
            Self::PlanarLine(h) => h.object_address() as usize,
            Self::Spline(h) => h.object_address() as usize,
            Self::Extrusion(h) => h.object_address() as usize,
        }
    }
}

pub(super) fn unsupported<T: Transient + RuntimeTyped + ?Sized>(
    handle: &Handle<T>,
    expected: &'static str,
) -> Incomplete {
    Incomplete::Unsupported {
        owner: 0,
        actual: handle
            .dynamic_type()
            .map_or_else(|| "null".into(), |kind| kind.system_name().into()),
        expected,
    }
}

pub(super) fn spatial_owner(handle: &Handle<dyn GeomCurve>) -> Result<Owner, Incomplete> {
    let line = handle.cast::<GeomLine>();
    if exact(&line) {
        return Ok(Owner::Line(line));
    }
    let spline = handle.cast::<GeomBSplineCurve>();
    if exact(&spline) {
        return Ok(Owner::Spline(spline));
    }
    Err(unsupported(handle, "stored line or clamped spline curve"))
}

pub(super) fn surface_owner(handle: &Handle<dyn GeomSurface>) -> Result<Owner, Incomplete> {
    let surface = handle.cast::<GeomSurfaceOfLinearExtrusion>();
    if exact(&surface) {
        return Ok(Owner::Extrusion(surface));
    }
    Err(unsupported(handle, "stored linear extrusion surface"))
}

pub(super) fn planar_owner(handle: &Handle<dyn Geom2dCurve>) -> Result<Owner, Incomplete> {
    let line = handle.cast::<Geom2dLine>();
    if exact(&line) {
        return Ok(Owner::PlanarLine(line));
    }
    Err(unsupported(handle, "stored planar line"))
}

pub(super) fn representation_owner(
    handle: &Handle<dyn BRepCurveRepresentation>,
) -> Result<Owner, Incomplete> {
    let seam = handle.cast::<BRepCurveOnClosedSurface>();
    if exact(&seam) {
        return Ok(Owner::Seam(seam));
    }
    let pcurve = handle.cast::<BRepCurveOnSurface>();
    if exact(&pcurve) {
        return Ok(Owner::Parametric(pcurve));
    }
    let curve = handle.cast::<BRepCurve3D>();
    if exact(&curve) {
        return Ok(Owner::Spatial(curve));
    }
    Err(unsupported(handle, "spatial curve or one/two line pcurves"))
}

fn exact<T: Transient + RuntimeTyped>(handle: &Handle<T>) -> bool {
    handle
        .get()
        .is_some_and(|value| value.is_instance(T::static_type()))
}
