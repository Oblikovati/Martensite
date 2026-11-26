// SPDX-License-Identifier: GPL-2.0-only
//! Read only the authoritative stored 3D line and vertex cells under held leases.
use super::{LineEdgeCaptureError as Error, LineEdgeReadCell as Cell};
use crate::{
    brep::{
        BRepCurve3D, BRepCurveOn2Surfaces, BRepCurveOnSurfaceSource, BRepCurveRepresentation,
        BRepPolygon3D, BRepPolygonOnSurfaceSource, BRepPolygonOnTriangulationSource, BRepTEdge,
        BRepTVertex, tedge::DEGENERATED_MASK,
    },
    topods::{Edge, Shape, TShape, Vertex, bit_layout::BITS_SHAPE_TYPE_MASK},
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, SharedState};
use martensite_tkg3d::{
    geom::GeomLine,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Line3, Point3},
    toploc::Location,
};
use std::sync::RwLockReadGuard;

const REPRESENTATIONS: usize = 16;
const LOCATION_STEPS: usize = 64;
type Rows = [[f64; 4]; 3];
type Checked<T> = Result<T, Error>;

pub(super) struct StoredLine {
    pub carrier: Line3,
    pub placement: Rows,
    pub range: [f64; 2],
    pub orientation: Orientation,
    pub edge_tolerance: f64,
    pub vertices: [StoredVertex; 2],
}

pub(super) struct StoredVertex {
    pub point: Point3,
    pub tolerance: f64,
    pub binding: usize,
    pub placement: Rows,
}

pub(super) fn read(edge: &Edge, nominated: [&Vertex; 2]) -> Checked<StoredLine> {
    let reversed = direction(edge.orientation())?;
    placement(edge.location())?;
    let handle = edge.tshape().downcast::<BRepTEdge>();
    let native = handle.get().ok_or(Error::Topology)?;
    let topology = read_cell(&native.tshape_base().fields, Cell::EdgeTopology)?;
    let directed = directed_vertices(topology.state, &topology.shapes, reversed)?;
    let fields = read_cell(&native.fields, Cell::EdgeRepresentations)?;
    if fields.flags & DEGENERATED_MASK != 0 {
        return Err(Error::Representation);
    }
    let tolerance = checked_tolerance(fields.tolerance)?;
    let record = spatial_record(&fields.curves)?;
    read_spatial(record, edge, directed, nominated, tolerance)
}

fn read_spatial(
    record: &BRepCurve3D,
    edge: &Edge,
    vertices: [&Shape; 2],
    nominated: [&Vertex; 2],
    edge_tolerance: f64,
) -> Checked<StoredLine> {
    let range = read_cell(&record.gcurve.range, Cell::SpatialRepresentation)?;
    let place = read_cell(
        &record.gcurve.representation.location,
        Cell::SpatialRepresentation,
    )?;
    let stored_curve = read_cell(&record.curve, Cell::SpatialRepresentation)?;
    let carrier = stored_curve
        .downcast_ref::<GeomLine>()
        .ok_or(Error::Carrier)?;
    let axis = carrier
        .try_input_lease()
        .map_err(|cause| Error::Read(Cell::Carrier, cause))?;
    let placement = composed_placement(edge.location(), &place)?;
    let vertices = read_vertices(vertices, nominated, edge.location())?;
    Ok(StoredLine {
        carrier: Line3::from_axis(axis.axis()),
        placement,
        range: [range.0, range.1],
        orientation: edge.orientation(),
        edge_tolerance,
        vertices,
    })
}

fn directed_vertices(state: u16, children: &[Shape], reversed: bool) -> Checked<[&Shape; 2]> {
    let [a, b] = children else {
        return Err(Error::Topology);
    };
    if shape_kind(state) != ShapeEnum::Edge as u16 || a.is_partner(b) {
        return Err(Error::Topology);
    }
    let [ar, br] = [direction(a.orientation())?, direction(b.orientation())?];
    if ar == br {
        return Err(Error::Topology);
    }
    Ok(if ar ^ reversed { [b, a] } else { [a, b] })
}

fn read_vertices(
    [first, last]: [&Shape; 2],
    nominated: [&Vertex; 2],
    parent: &Location,
) -> Checked<[StoredVertex; 2]> {
    let first_binding = bind_vertex(first, nominated, parent)?;
    let last_binding = bind_vertex(last, nominated, parent)?;
    if first_binding == last_binding {
        return Err(Error::VertexBinding);
    }
    let a = first.tshape().downcast::<BRepTVertex>();
    let b = last.tshape().downcast::<BRepTVertex>();
    let [a, b] = [
        a.get().ok_or(Error::Topology)?,
        b.get().ok_or(Error::Topology)?,
    ];
    let at = read_cell(&a.tshape_base().fields, Cell::Endpoint(0))?;
    let bt = read_cell(&b.tshape_base().fields, Cell::Endpoint(1))?;
    check_vertex(at.state, &at.shapes)?;
    check_vertex(bt.state, &bt.shapes)?;
    let af = read_cell(&a.fields, Cell::Endpoint(0))?;
    let bf = read_cell(&b.fields, Cell::Endpoint(1))?;
    Ok([
        vertex(&af, first_binding, nominated)?,
        vertex(&bf, last_binding, nominated)?,
    ])
}

fn bind_vertex(vertex: &Shape, nominated: [&Vertex; 2], parent: &Location) -> Checked<usize> {
    placement(vertex.location())?;
    let located = vertex.moved(parent, false)?;
    let matches = nominated.map(|candidate| located.is_same(candidate));
    match matches {
        [true, false] => Ok(0),
        [false, true] => Ok(1),
        _ => Err(Error::VertexBinding),
    }
}

fn check_vertex(state: u16, children: &[Shape]) -> Checked<()> {
    if shape_kind(state) != ShapeEnum::Vertex as u16 || !children.is_empty() {
        return Err(Error::Topology);
    }
    Ok(())
}

fn vertex(
    value: &crate::brep::tvertex::TVertexFields,
    binding: usize,
    nominated: [&Vertex; 2],
) -> Checked<StoredVertex> {
    Ok(StoredVertex {
        point: value.point,
        tolerance: checked_tolerance(value.tolerance)?,
        binding,
        placement: placement(nominated[binding].location())?,
    })
}

fn spatial_record(curves: &[Handle<dyn BRepCurveRepresentation>]) -> Checked<&BRepCurve3D> {
    if curves.len() > REPRESENTATIONS {
        return Err(Error::RepresentationLimit {
            actual: curves.len(),
            maximum: REPRESENTATIONS,
        });
    }
    let mut spatial = None;
    for record in curves {
        if let Some(value) = record.downcast_ref::<BRepCurve3D>() {
            if spatial.replace(value).is_some() {
                return Err(Error::Representation);
            }
        } else if !known_nonspatial(record) {
            return Err(Error::Representation);
        }
    }
    spatial.ok_or(Error::Representation)
}

fn known_nonspatial(record: &Handle<dyn BRepCurveRepresentation>) -> bool {
    record.is_kind(<dyn BRepCurveOnSurfaceSource>::static_type())
        || record.is_kind(BRepCurveOn2Surfaces::static_type())
        || record.is_kind(BRepPolygon3D::static_type())
        || record.is_kind(<dyn BRepPolygonOnSurfaceSource>::static_type())
        || record.is_kind(<dyn BRepPolygonOnTriangulationSource>::static_type())
}

fn composed_placement(parent: &Location, raw: &Location) -> Checked<Rows> {
    placement(raw)?;
    placement(&parent.multiplied(raw)?)
}

fn placement(location: &Location) -> Checked<Rows> {
    let mut step = location;
    for _ in 0..LOCATION_STEPS {
        if step.is_identity() {
            break;
        }
        let datum = step.first_datum()?.get().ok_or(Error::Topology)?;
        datum.transformation().checked_unit_placement_rows()?;
        step = step.next_location();
    }
    if !step.is_identity() {
        return Err(Error::LocationDepth(LOCATION_STEPS));
    }
    Ok(location.transformation().checked_unit_placement_rows()?)
}

fn read_cell<T>(value: &SharedState<T>, part: Cell) -> Checked<RwLockReadGuard<'_, T>> {
    value.try_read().map_err(|cause| Error::Read(part, cause))
}

fn direction(value: Orientation) -> Checked<bool> {
    match value {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        other => Err(Error::Orientation(other)),
    }
}

fn shape_kind(state: u16) -> u16 {
    state & BITS_SHAPE_TYPE_MASK
}

fn checked_tolerance(value: f64) -> Checked<f64> {
    if !value.is_finite() || value < 0. {
        return Err(Error::Tolerance(value));
    }
    Ok(value)
}
