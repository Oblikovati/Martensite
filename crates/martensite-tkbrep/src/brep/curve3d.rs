//! `BRep_Curve3D`: the 3D curve of an edge.
use super::curve_representation::{BRepCurveRepresentation, CurveRepresentationBase};
use super::dump::{dump_base, dump_class_name, dump_handle};
use super::errors::null_object;
use super::gcurve::{BRepGCurve, GCurveBase, dump_gcurve};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::real::{real_first, real_last};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::toploc::Location;
use std::io;

/// `BRep_Curve3D`: a 3D curve with its location and range. A null curve is allowed
/// (degenerated edges); its range is then `RealFirst()..RealLast()`.
///
/// ```
/// use martensite_tkbrep::brep::{BRepCurve3D, BRepGCurve};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkmath::toploc::Location;
/// let curve = BRepCurve3D::new(&Handle::default(), &Location::default());
/// assert_eq!(curve.first(), -f64::MAX);
/// ```
pub struct BRepCurve3D {
    transient: StandardTransient,
    gcurve: GCurveBase,
    curve: SharedState<Handle<dyn GeomCurve>>,
}
impl_transient!(BRepCurve3D, "BRep_Curve3D", dyn BRepGCurve, transient,
    interfaces: [dyn BRepCurveRepresentation, dyn BRepGCurve]);

impl BRepCurve3D {
    /// `BRep_Curve3D(C, L)`: the range is the curve range, or `RealFirst()..RealLast()`
    /// for a null curve; `BRepCurve3D::new(&curve, &location)`.
    pub fn new(curve: &Handle<dyn GeomCurve>, location: &Location) -> Self {
        let (first, last) = curve.get().map_or((real_first(), real_last()), |c| {
            (c.first_parameter(), c.last_parameter())
        });
        Self::from_components(curve.clone(), location, (first, last))
    }

    fn copy_from_current_binding(&self) -> Result<Self, Failure> {
        // Match the retained Linux x86-64 evaluation order; no language rule mandates it.
        let last = self.current_parameter(|curve| curve.try_last_parameter(), real_last())?;
        let first = self.current_parameter(|curve| curve.try_first_parameter(), real_first())?;
        let curve = self.curve.read().clone();
        Ok(Self::from_components(
            curve,
            &self.location(),
            (first, last),
        ))
    }

    fn current_parameter(
        &self,
        query: fn(&dyn GeomCurve) -> Result<f64, Failure>,
        when_null: f64,
    ) -> Result<f64, Failure> {
        let curve = self.curve.read().clone();
        curve.get().map_or_else(|| Ok(when_null), query)
    }

    fn from_components(
        curve: Handle<dyn GeomCurve>,
        location: &Location,
        range: (f64, f64),
    ) -> Self {
        Self {
            transient: StandardTransient::default(),
            gcurve: GCurveBase::new(location, range.0, range.1),
            curve: SharedState::new(curve),
        }
    }
}

impl BRepCurveRepresentation for BRepCurve3D {
    fn curve_representation_base(&self) -> &CurveRepresentationBase {
        self.gcurve.representation()
    }
    /// Returns true.
    fn is_curve3d(&self) -> bool {
        true
    }
    fn curve3d(&self) -> Result<Handle<dyn GeomCurve>, Failure> {
        Ok(self.curve.read().clone())
    }
    fn set_curve3d(&self, curve: &Handle<dyn GeomCurve>) -> Result<(), Failure> {
        *self.curve.write() = curve.clone();
        Ok(())
    }
    /// A new `BRep_Curve3D` on the same curve and location, then `SetRange`.
    fn copy(&self) -> Result<Handle<dyn BRepCurveRepresentation>, Failure> {
        let copy = self.copy_from_current_binding()?;
        let (first, last) = self.range();
        copy.set_range(first, last)?;
        Ok(Handle::new(copy).into_cast())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        dump_class_name(stream, "BRep_Curve3D")?;
        dump_base(stream, depth, |nested, nested_depth| {
            dump_gcurve(&self.gcurve, nested, nested_depth)
        })?;
        let curve = self.curve.read().clone();
        dump_handle(stream, depth, "Curve", &curve, |curve, nested, d| {
            curve.dump_json(nested, d)
        })
    }
}

impl BRepGCurve for BRepCurve3D {
    fn gcurve_base(&self) -> &GCurveBase {
        &self.gcurve
    }
    /// `P = myCurve->Value(U)`; a null curve raises Standard_NullObject (a null
    /// dereference natively).
    fn d0(&self, u: f64) -> Result<Point3, Failure> {
        let curve = self.curve.read().clone();
        let curve = curve
            .get()
            .ok_or_else(|| null_object("BRep_Curve3D::D0: the 3D curve is null"))?;
        curve.value(u)
    }
}
