//! `BRep_CurveOnSurface`: a curve in the parametric space of a surface.
use super::curve_representation::{BRepCurveRepresentation, CurveRepresentationBase};
use super::dump::{dump_base, dump_class_name, dump_field, dump_handle};
use super::errors::null_object;
use super::gcurve::{BRepGCurve, GCurveBase, dump_gcurve};
use martensite_tkernel::impl_transient;
use martensite_tkernel::precision;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::toploc::Location;
use std::io;

/// The locked fields of `BRep_CurveOnSurface`: the protected `myUV1`, `myUV2` and the
/// private curve and surface.
#[derive(Clone)]
struct CurveOnSurfaceFields {
    uv1: Point2,
    uv2: Point2,
    pcurve: Handle<dyn Geom2dCurve>,
    surface: Handle<dyn GeomSurface>,
}

/// `BRep_CurveOnSurface`: a 2D curve on a located surface, with the UV points of its
/// range ends (recomputed by `Update`).
///
/// ```
/// use martensite_tkbrep::brep::{BRepCurveOnSurface, BRepCurveRepresentation};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
/// use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
/// use martensite_tkmath::gp::{Axis2d, Axis3};
/// use martensite_tkmath::toploc::Location;
/// let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).into_cast();
/// let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).into_cast();
/// let pcurve = BRepCurveOnSurface::new(&line, &plane, &Location::default()).unwrap();
/// assert!(pcurve.is_curve_on_surface_at(&plane, &Location::default()));
/// ```
pub struct BRepCurveOnSurface {
    transient: StandardTransient,
    gcurve: GCurveBase,
    fields: SharedState<CurveOnSurfaceFields>,
}
impl_transient!(BRepCurveOnSurface, "BRep_CurveOnSurface", dyn BRepGCurve, transient,
    interfaces: [dyn BRepCurveRepresentation, dyn BRepGCurve]);

impl BRepCurveOnSurface {
    /// `BRep_CurveOnSurface(PC, S, L)`: the range is the p-curve range. A null p-curve
    /// raises Standard_NullObject (a null dereference natively);
    /// `BRepCurveOnSurface::new(&pcurve, &surface, &location)?`.
    pub fn new(
        pcurve: &Handle<dyn Geom2dCurve>,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<Self, Failure> {
        let curve = pcurve
            .get()
            .ok_or_else(|| null_object("BRep_CurveOnSurface: the p-curve is null"))?;
        let transient = StandardTransient::default();
        let gcurve = GCurveBase::new(location, curve.first_parameter(), curve.last_parameter());
        Ok(Self::from_components(
            transient,
            gcurve,
            pcurve.clone(),
            surface.clone(),
        ))
    }

    fn from_components(
        transient: StandardTransient,
        gcurve: GCurveBase,
        pcurve: Handle<dyn Geom2dCurve>,
        surface: Handle<dyn GeomSurface>,
    ) -> Self {
        Self {
            transient,
            gcurve,
            fields: SharedState::new(CurveOnSurfaceFields {
                uv1: Point2::default(),
                uv2: Point2::default(),
                pcurve,
                surface,
            }),
        }
    }

    // Match the retained Linux x86-64 evaluation order; no language rule mandates it.
    pub(super) fn copy_from_current_binding(&self) -> Result<Self, Failure> {
        let transient = StandardTransient::default();
        let last = self.parameter_from_current_binding(|curve| curve.try_last_parameter())?;
        let first = self.parameter_from_current_binding(|curve| curve.try_first_parameter())?;
        let gcurve = GCurveBase::new(&self.location(), first, last);
        Ok(Self::from_components(
            transient,
            gcurve,
            self.pcurve()?,
            self.surface()?,
        ))
    }

    fn parameter_from_current_binding(
        &self,
        query: fn(&dyn Geom2dCurve) -> Result<f64, Failure>,
    ) -> Result<f64, Failure> {
        let pcurve = self.fields.read().pcurve.clone();
        let curve = pcurve
            .get()
            .ok_or_else(|| null_object("BRep_CurveOnSurface: the p-curve is null"))?;
        query(curve)
    }

    /// `SetUVPoints(P1, P2)`; `pcurve.set_uv_points(first, last)`.
    pub fn set_uv_points(&self, first: Point2, last: Point2) {
        let mut fields = self.fields.write();
        fields.uv1 = first;
        fields.uv2 = last;
    }

    /// `UVPoints(P1, P2)`; `let (first, last) = pcurve.uv_points();`.
    pub fn uv_points(&self) -> (Point2, Point2) {
        let fields = self.fields.read();
        (fields.uv1, fields.uv2)
    }

    /// The body of `BRep_CurveOnSurface::Update`: the UV points at the finite range ends.
    pub(crate) fn update_uv_points(&self) -> Result<(), Failure> {
        let (first, last) = self.gcurve.range();
        if !precision::is_negative_infinite(first) {
            self.fields.write().uv1 = self.endpoint_uv(first)?;
        }
        if !precision::is_positive_infinite(last) {
            self.fields.write().uv2 = self.endpoint_uv(last)?;
        }
        Ok(())
    }

    // Each callback can replace the binding used by the following endpoint.
    fn endpoint_uv(&self, parameter: f64) -> Result<Point2, Failure> {
        let pcurve = self.fields.read().pcurve.clone();
        let curve = pcurve
            .get()
            .ok_or_else(|| null_object("BRep_CurveOnSurface::Update: the p-curve is null"))?;
        curve.d0(parameter)
    }

    /// The body of `BRep_CurveOnSurface::DumpJson`, for the derived closed-surface class.
    pub(crate) fn dump_body(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        dump_class_name(stream, "BRep_CurveOnSurface")?;
        dump_base(stream, depth, |nested, nested_depth| {
            dump_gcurve(&self.gcurve, nested, nested_depth)
        })?;
        let fields = self.fields.read().clone();
        dump_field(stream, depth, "UV1", |nested, _| {
            fields.uv1.dump_json(nested)
        })?;
        dump_field(stream, depth, "UV2", |nested, _| {
            fields.uv2.dump_json(nested)
        })?;
        dump_handle(stream, depth, "PCurve", &fields.pcurve, |c, s, d| {
            c.dump_json(s, d)
        })?;
        dump_handle(stream, depth, "Surface", &fields.surface, |c, s, d| {
            c.dump_json(s, d)
        })
    }
}

impl BRepCurveRepresentation for BRepCurveOnSurface {
    fn curve_representation_base(&self) -> &CurveRepresentationBase {
        self.gcurve.representation()
    }
    /// Returns true.
    fn is_curve_on_surface(&self) -> bool {
        true
    }
    /// `(S == mySurface) && (L == myLocation)`.
    fn is_curve_on_surface_at(
        &self,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> bool {
        *surface == self.fields.read().surface && *location == self.location()
    }
    fn surface(&self) -> Result<Handle<dyn GeomSurface>, Failure> {
        Ok(self.fields.read().surface.clone())
    }
    fn pcurve(&self) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        Ok(self.fields.read().pcurve.clone())
    }
    fn set_pcurve(&self, pcurve: &Handle<dyn Geom2dCurve>) -> Result<(), Failure> {
        self.fields.write().pcurve = pcurve.clone();
        Ok(())
    }
    /// A new representation on the same data, then `SetRange` and `SetUVPoints`.
    fn copy(&self) -> Result<Handle<dyn BRepCurveRepresentation>, Failure> {
        let copy = self.copy_from_current_binding()?;
        let (first, last) = self.range();
        copy.set_range(first, last)?;
        let (uv1, uv2) = self.uv_points();
        copy.set_uv_points(uv1, uv2);
        Ok(Handle::new(copy).into_cast())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        self.dump_body(stream, depth)
    }
}

impl BRepGCurve for BRepCurveOnSurface {
    fn gcurve_base(&self) -> &GCurveBase {
        &self.gcurve
    }
    /// The p-curve point, evaluated on the surface and moved by the location.
    fn d0(&self, u: f64) -> Result<Point3, Failure> {
        let fields = self.fields.read().clone();
        let pcurve = fields
            .pcurve
            .get()
            .ok_or_else(|| null_object("BRep_CurveOnSurface::D0: the p-curve is null"))?;
        // Native Value(U) runs before surface access; preserve extension-curve failures.
        let uv = pcurve.value(u)?;
        let surface = fields
            .surface
            .get()
            .ok_or_else(|| null_object("BRep_CurveOnSurface::D0: the surface is null"))?;
        let point = surface.value(uv.x(), uv.y())?;
        Ok(point.transformed(self.location().transformation()))
    }
    /// `BRep_CurveOnSurface::Update`: the UV points at the finite range ends.
    fn update(&self) -> Result<(), Failure> {
        self.update_uv_points()
    }
}
