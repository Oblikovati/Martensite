//! `BRep_PolygonOnSurface` and `BRep_PolygonOnClosedSurface`: 2D polygons of an edge
//! in the parametric space of a surface.
use super::curve_representation::{
    BRepCurveRepresentation, CurveRepresentationBase, dump_curve_representation,
};
use super::dump::{dump_base, dump_class_name, dump_handle};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::poly::Polygon2D;
use martensite_tkmath::toploc::Location;
use std::io;

/// The private fields of `BRep_PolygonOnSurface`.
#[derive(Clone)]
struct PolygonOnSurfaceFields {
    polygon: Handle<Polygon2D>,
    surface: Handle<dyn GeomSurface>,
}

/// `BRep_PolygonOnSurface`: a 2D polygon on a located surface.
///
/// ```
/// use martensite_tkbrep::brep::{BRepCurveRepresentation, BRepPolygonOnSurface};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkmath::toploc::Location;
/// let polygon = BRepPolygonOnSurface::new(&Handle::default(), &Handle::default(), &Location::default());
/// assert!(polygon.is_polygon_on_surface_at(&Handle::default(), &Location::default()));
/// ```
pub struct BRepPolygonOnSurface {
    state: PolygonOnSurfaceState,
}

struct PolygonOnSurfaceState {
    representation: CurveRepresentationBase,
    fields: SharedState<PolygonOnSurfaceFields>,
}
impl_transient!(BRepPolygonOnSurface, "BRep_PolygonOnSurface", dyn BRepCurveRepresentation, interfaces: [dyn BRepCurveRepresentation, dyn BRepPolygonOnSurfaceSource]);

/// Parent interface shared by ordinary and closed polygon representations.
/// Example: `polygon.upcast::<dyn BRepPolygonOnSurfaceSource>()`.
pub trait BRepPolygonOnSurfaceSource: BRepCurveRepresentation {}
parent_interface!(
    dyn BRepPolygonOnSurfaceSource,
    BRepPolygonOnSurface,
    [dyn BRepCurveRepresentation]
);

impl BRepPolygonOnSurfaceSource for BRepPolygonOnSurface {}

impl BRepPolygonOnSurface {
    /// `BRep_PolygonOnSurface(P, S, L)`; `BRepPolygonOnSurface::new(&polygon, &surface, &location)`.
    pub fn new(
        polygon: &Handle<Polygon2D>,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Self {
        Self {
            state: PolygonOnSurfaceState::new(polygon, surface, location),
        }
    }
}

impl PolygonOnSurfaceState {
    fn new(
        polygon: &Handle<Polygon2D>,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Self {
        Self {
            representation: CurveRepresentationBase::new(location),
            fields: SharedState::new(PolygonOnSurfaceFields {
                polygon: polygon.clone(),
                surface: surface.clone(),
            }),
        }
    }

    /// The body of `BRep_PolygonOnSurface::DumpJson`, for the derived closed-surface class.
    fn dump_body(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        dump_class_name(stream, "BRep_PolygonOnSurface")?;
        dump_base(stream, depth, |nested, nested_depth| {
            dump_curve_representation(&self.representation, nested, nested_depth)
        })?;
        let fields = self.fields.read().clone();
        dump_handle(stream, depth, "Polygon2D", &fields.polygon, |p, s, _| {
            p.dump_json(s)
        })?;
        dump_handle(stream, depth, "Surface", &fields.surface, |c, s, d| {
            c.dump_json(s, d)
        })
    }
    fn is_on(&self, surface: &Handle<dyn GeomSurface>, location: &Location) -> bool {
        *surface == self.fields.read().surface && *location == self.representation.location()
    }

    fn surface(&self) -> Handle<dyn GeomSurface> {
        self.fields.read().surface.clone()
    }

    fn polygon(&self) -> Handle<Polygon2D> {
        self.fields.read().polygon.clone()
    }

    fn set_polygon(&self, polygon: &Handle<Polygon2D>) {
        self.fields.write().polygon = polygon.clone();
    }
}

impl BRepCurveRepresentation for BRepPolygonOnSurface {
    fn curve_representation_base(&self) -> &CurveRepresentationBase {
        &self.state.representation
    }
    /// Returns true.
    fn is_polygon_on_surface(&self) -> bool {
        true
    }
    /// `(S == mySurface) && (L == myLocation)`.
    fn is_polygon_on_surface_at(
        &self,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> bool {
        self.state.is_on(surface, location)
    }
    fn surface(&self) -> Result<Handle<dyn GeomSurface>, Failure> {
        Ok(self.state.surface())
    }
    fn polygon(&self) -> Result<Handle<Polygon2D>, Failure> {
        Ok(self.state.polygon())
    }
    fn set_polygon(&self, polygon: &Handle<Polygon2D>) -> Result<(), Failure> {
        self.state.set_polygon(polygon);
        Ok(())
    }
    /// A new representation on the same polygon, surface and location.
    fn copy(&self) -> Result<Handle<dyn BRepCurveRepresentation>, Failure> {
        let fields = self.state.fields.read().clone();
        let copy = Self::new(&fields.polygon, &fields.surface, &self.location());
        Ok(Handle::new(copy).into_upcast())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        self.state.dump_body(stream, depth)
    }
}

/// `BRep_PolygonOnClosedSurface`: the two 2D polygons of a seam edge. It derives from
/// `BRep_PolygonOnSurface`, whose state it shares.
///
/// ```
/// use martensite_tkbrep::brep::{BRepCurveRepresentation, BRepPolygonOnClosedSurface};
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkmath::toploc::Location;
/// let none = Handle::default();
/// let seam = BRepPolygonOnClosedSurface::new(&none, &none, &Handle::default(), &Location::default());
/// assert!(seam.is_polygon_on_closed_surface() && seam.polygon2().unwrap().is_null());
/// ```
pub struct BRepPolygonOnClosedSurface {
    first_polygon: PolygonOnSurfaceState,
    polygon2: SharedState<Handle<Polygon2D>>,
}
impl_transient!(BRepPolygonOnClosedSurface, "BRep_PolygonOnClosedSurface", dyn BRepPolygonOnSurfaceSource, interfaces: [dyn BRepCurveRepresentation, dyn BRepPolygonOnSurfaceSource]);

impl BRepPolygonOnClosedSurface {
    /// `BRep_PolygonOnClosedSurface(P1, P2, S, L)`;
    /// `BRepPolygonOnClosedSurface::new(&p1, &p2, &surface, &location)`.
    pub fn new(
        polygon1: &Handle<Polygon2D>,
        polygon2: &Handle<Polygon2D>,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Self {
        Self {
            first_polygon: PolygonOnSurfaceState::new(polygon1, surface, location),
            polygon2: SharedState::new(polygon2.clone()),
        }
    }
}

impl BRepCurveRepresentation for BRepPolygonOnClosedSurface {
    fn curve_representation_base(&self) -> &CurveRepresentationBase {
        &self.first_polygon.representation
    }
    fn is_polygon_on_surface(&self) -> bool {
        true
    }
    fn is_polygon_on_surface_at(
        &self,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> bool {
        self.first_polygon.is_on(surface, location)
    }
    /// Returns true.
    fn is_polygon_on_closed_surface(&self) -> bool {
        true
    }
    fn surface(&self) -> Result<Handle<dyn GeomSurface>, Failure> {
        Ok(self.first_polygon.surface())
    }
    fn polygon(&self) -> Result<Handle<Polygon2D>, Failure> {
        Ok(self.first_polygon.polygon())
    }
    fn set_polygon(&self, polygon: &Handle<Polygon2D>) -> Result<(), Failure> {
        self.first_polygon.set_polygon(polygon);
        Ok(())
    }
    fn polygon2(&self) -> Result<Handle<Polygon2D>, Failure> {
        Ok(self.polygon2.read().clone())
    }
    fn set_polygon2(&self, polygon: &Handle<Polygon2D>) -> Result<(), Failure> {
        *self.polygon2.write() = polygon.clone();
        Ok(())
    }
    /// A new representation on the same polygons, surface and location.
    fn copy(&self) -> Result<Handle<dyn BRepCurveRepresentation>, Failure> {
        let copy = Self::new(
            &self.first_polygon.polygon(),
            &self.polygon2.read(),
            &self.first_polygon.surface(),
            &self.location(),
        );
        Ok(Handle::new(copy).into_upcast())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        dump_class_name(stream, "BRep_PolygonOnClosedSurface")?;
        dump_base(stream, depth, |nested, nested_depth| {
            self.first_polygon.dump_body(nested, nested_depth)
        })?;
        let polygon2 = self.polygon2.read().clone();
        dump_handle(stream, depth, "Polygon2", &polygon2, |p, s, _| {
            p.dump_json(s)
        })
    }
}

impl BRepPolygonOnSurfaceSource for BRepPolygonOnClosedSurface {}
