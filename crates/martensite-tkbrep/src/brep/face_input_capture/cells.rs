//! Cell addresses are used only for deterministic acquisition, never geometry identity.
use super::{owners::Owner, types::FaceCaptureIncomplete as Incomplete};
use crate::{brep::*, topods::TShape};
use martensite_tkernel::standard::{Handle, RuntimeTyped, SharedState};
use martensite_tkg2d::geom2d::{PlanarLineInputLease, PlanarSplineInputLease};
use martensite_tkg3d::geom::{
    ExtrusionInputLease, GeomCurve, LineInputLease, PlaneInputLease, SplineInputLease,
    SplineSurfaceInputLease,
};
use martensite_tkmath::toploc::Location;
use std::sync::RwLockReadGuard;

pub(super) enum Cell<'a> {
    Topology(&'a SharedState<crate::topods::TShapeFields>),
    Face(&'a SharedState<crate::brep::tface::TFaceFields>),
    Edge(&'a SharedState<crate::brep::tedge::TEdgeFields>),
    Vertex(&'a SharedState<crate::brep::tvertex::TVertexFields>),
    PCurve(&'a SharedState<crate::brep::curve_on_surface::CurveOnSurfaceFields>),
    Seam(&'a SharedState<crate::brep::curve_on_closed_surface::ClosedSurfaceFields>),
    Spatial(&'a SharedState<Handle<dyn GeomCurve>>),
    Range(&'a SharedState<(f64, f64)>),
    Location(&'a SharedState<Location>),
    Regularity(&'a SharedState<crate::brep::curve_on_2_surfaces::TwoSurfacesFields>),
    Geometry(&'a Owner),
}
pub(super) enum Lease<'a> {
    Topology(RwLockReadGuard<'a, crate::topods::TShapeFields>),
    Face(RwLockReadGuard<'a, crate::brep::tface::TFaceFields>),
    Edge(RwLockReadGuard<'a, crate::brep::tedge::TEdgeFields>),
    Vertex(RwLockReadGuard<'a, crate::brep::tvertex::TVertexFields>),
    PCurve(RwLockReadGuard<'a, crate::brep::curve_on_surface::CurveOnSurfaceFields>),
    Seam(RwLockReadGuard<'a, crate::brep::curve_on_closed_surface::ClosedSurfaceFields>),
    Spatial(RwLockReadGuard<'a, Handle<dyn GeomCurve>>),
    Range(RwLockReadGuard<'a, (f64, f64)>),
    Location(RwLockReadGuard<'a, Location>),
    Line(LineInputLease<'a>),
    PlanarLine(PlanarLineInputLease<'a>),
    PlanarSpline(PlanarSplineInputLease<'a>),
    Spline(SplineInputLease<'a>),
    Plane(PlaneInputLease<'a>),
    Extrusion(ExtrusionInputLease<'a>),
    SplineSurface(SplineSurfaceInputLease<'a>),
    Regularity(RwLockReadGuard<'a, crate::brep::curve_on_2_surfaces::TwoSurfacesFields>),
}
impl Cell<'_> {
    pub(super) fn key(&self) -> usize {
        match self {
            Self::Topology(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Face(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Edge(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Vertex(cell) => std::ptr::from_ref(*cell) as usize,
            Self::PCurve(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Seam(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Spatial(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Range(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Location(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Regularity(cell) => std::ptr::from_ref(*cell) as usize,
            Self::Geometry(owner) => owner.key(),
        }
    }
}
impl<'a> Cell<'a> {
    pub(super) fn acquire(&self, owner: usize) -> Result<Lease<'a>, Incomplete> {
        let result = match self {
            Self::Topology(cell) => cell.try_read().map(Lease::Topology),
            Self::Face(cell) => cell.try_read().map(Lease::Face),
            Self::Edge(cell) => cell.try_read().map(Lease::Edge),
            Self::Vertex(cell) => cell.try_read().map(Lease::Vertex),
            Self::PCurve(cell) => cell.try_read().map(Lease::PCurve),
            Self::Seam(cell) => cell.try_read().map(Lease::Seam),
            Self::Spatial(cell) => cell.try_read().map(Lease::Spatial),
            Self::Range(cell) => cell.try_read().map(Lease::Range),
            Self::Location(cell) => cell.try_read().map(Lease::Location),
            Self::Regularity(cell) => cell.try_read().map(Lease::Regularity),
            Self::Geometry(cell) => {
                return geometry_lease(cell).map_err(|error| error.at_owner(owner));
            }
        };
        result.map_err(|cause| read_failure(owner, cause))
    }
}
fn live<T: RuntimeTyped + ?Sized>(handle: &Handle<T>) -> Result<&T, Incomplete> {
    handle.get().ok_or(Incomplete::Changed { owner: 0 })
}
fn geometry_lease(owner: &Owner) -> Result<Lease<'_>, Incomplete> {
    let result = match owner {
        Owner::Line(h) => live(h)?.try_input_lease().map(Lease::Line),
        Owner::PlanarLine(h) => live(h)?.try_input_lease().map(Lease::PlanarLine),
        Owner::PlanarSpline(h) => live(h)?.try_input_lease().map(Lease::PlanarSpline),
        Owner::Spline(h) => live(h)?.try_input_lease().map(Lease::Spline),
        Owner::Plane(h) => live(h)?.try_input_lease().map(Lease::Plane),
        Owner::Extrusion(h) => live(h)?.try_input_lease().map(Lease::Extrusion),
        Owner::SplineSurface(h) => live(h)?.try_input_lease().map(Lease::SplineSurface),
        _ => return Err(Incomplete::Changed { owner: 0 }),
    };
    result.map_err(|cause| Incomplete::Read {
        owner: 0,
        cell: "geometry evaluator",
        cause,
    })
}

pub(super) fn owner_cells(owner: &Owner) -> Result<Vec<Cell<'_>>, Incomplete> {
    match owner {
        Owner::Topology(h) => topology_cells(h),
        Owner::Spatial(h) => spatial_cells(live(h)?),
        Owner::Parametric(h) => Ok(parametric_cells(live(h)?)),
        Owner::Seam(h) => seam_cells(live(h)?),
        Owner::Regularity(h) => Ok(vec![
            Cell::Regularity(&live(h)?.fields),
            Cell::Location(&live(h)?.representation.location),
        ]),
        _ => Ok(vec![Cell::Geometry(owner)]),
    }
}

fn spatial_cells(curve: &BRepCurve3D) -> Result<Vec<Cell<'_>>, Incomplete> {
    Ok(vec![
        Cell::Spatial(&curve.curve),
        Cell::Range(&curve.gcurve.range),
        Cell::Location(&curve.gcurve.representation.location),
    ])
}
fn seam_cells(curve: &BRepCurveOnClosedSurface) -> Result<Vec<Cell<'_>>, Incomplete> {
    let mut cells = parametric_cells(&curve.parent);
    cells.push(Cell::Seam(&curve.fields));
    Ok(cells)
}

fn parametric_cells(curve: &BRepCurveOnSurface) -> Vec<Cell<'_>> {
    vec![
        Cell::PCurve(&curve.fields),
        Cell::Range(&curve.gcurve.range),
        Cell::Location(&curve.gcurve.representation.location),
    ]
}

fn topology_cells(handle: &Handle<dyn TShape>) -> Result<Vec<Cell<'_>>, Incomplete> {
    let shape = handle.get().ok_or(Incomplete::Changed { owner: 0 })?;
    let mut cells = Vec::new();
    topology_fields(handle, &mut cells)?;
    cells.push(Cell::Topology(&shape.tshape_base().fields));
    Ok(cells)
}

fn topology_fields<'a>(
    handle: &'a Handle<dyn TShape>,
    cells: &mut Vec<Cell<'a>>,
) -> Result<(), Incomplete> {
    let shape = handle.get().ok_or(Incomplete::Changed { owner: 0 })?;
    if shape.is_instance(BRepTFace::static_type()) {
        let fields = BRepTFace::project_from(shape).ok_or(Incomplete::Changed {
            owner: handle.object_address() as usize,
        })?;
        cells.push(Cell::Face(&fields.fields));
        return Ok(());
    }
    topology_edge_or_vertex(handle, cells)
}

fn topology_edge_or_vertex<'a>(
    handle: &'a Handle<dyn TShape>,
    cells: &mut Vec<Cell<'a>>,
) -> Result<(), Incomplete> {
    let shape = handle.get().ok_or(Incomplete::Changed { owner: 0 })?;
    if shape.is_instance(BRepTEdge::static_type()) {
        let edge = BRepTEdge::project_from(shape).ok_or(Incomplete::Changed {
            owner: handle.object_address() as usize,
        })?;
        cells.push(Cell::Edge(&edge.fields));
        return Ok(());
    }
    topology_vertex_or_wire(handle, cells)
}

fn topology_vertex_or_wire<'a>(
    handle: &'a Handle<dyn TShape>,
    cells: &mut Vec<Cell<'a>>,
) -> Result<(), Incomplete> {
    let shape = handle.get().ok_or(Incomplete::Changed { owner: 0 })?;
    if shape.is_instance(BRepTVertex::static_type()) {
        let vertex = BRepTVertex::project_from(shape).ok_or(Incomplete::Changed {
            owner: handle.object_address() as usize,
        })?;
        cells.push(Cell::Vertex(&vertex.fields));
        return Ok(());
    }
    if shape.is_instance(crate::topods::TWire::static_type()) {
        return Ok(());
    }
    Err(super::owners::unsupported(
        handle,
        "exact face, wire, edge or vertex type",
    ))
}

pub(super) fn lease_order(lease: &Lease<'_>) -> u8 {
    match lease {
        Lease::Topology(_) => 0,
        Lease::Face(_) => 1,
        Lease::Edge(_) => 2,
        Lease::Vertex(_) => 3,
        Lease::PCurve(_) => 4,
        Lease::Seam(_) => 5,
        Lease::Spatial(_) => 6,
        Lease::Range(_) => 7,
        Lease::Location(_) => 8,
        _ => geometry_order(lease),
    }
}

fn geometry_order(lease: &Lease<'_>) -> u8 {
    match lease {
        Lease::Line(_) => 9,
        Lease::PlanarLine(_) => 10,
        Lease::PlanarSpline(_) => 11,
        Lease::Spline(_) => 12,
        Lease::Plane(_) => 13,
        Lease::Extrusion(_) => 14,
        Lease::SplineSurface(_) => 15,
        Lease::Regularity(_) => 16,
        _ => unreachable!("topology and representation leases classified first"),
    }
}

fn read_failure(
    owner: usize,
    cause: martensite_tkernel::standard::SharedReadFailure,
) -> Incomplete {
    Incomplete::Read {
        owner,
        cell: "mutable input cell",
        cause,
    }
}
