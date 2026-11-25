// SPDX-License-Identifier: GPL-2.0-only
//! Materialize exact recipe connectivity into a private, analytic BRep wire.
mod trace;

use super::{
    DirectedLineUse, LineDirection, WireClosure, WireFailure, WirePhase, WireRecipe, WireSegment,
    poll,
};
use crate::{EntityId, PointId};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Edge, Vertex, Wire},
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine},
    top_abs::Orientation,
};
use martensite_tkmath::gp::Point3;
use std::collections::BTreeMap;

/// One fresh native edge occurrence and its authored lineage.
pub struct NativeEdgeBinding {
    /// Authored line identity and sense.
    pub source: DirectedLineUse,
    /// Point identities in directed wire order.
    pub endpoints: [PointId; 2],
    /// Edge belonging to this exported wire, with the selected orientation.
    pub edge: Edge,
}

/// One fresh native vertex and its authored identity.
pub struct NativePointBinding {
    /// Point identity shared by every incident occurrence.
    pub point: PointId,
    /// Vertex belonging to this exported wire.
    pub vertex: Vertex,
}

/// Actual native storage checks performed on a detached export.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeWireValidation {
    /// Verified common tolerance of every exported vertex and edge, in model units.
    pub stored_tolerance: f64,
    /// Outward Euclidean error upper bound from analytic carrier readback over every trim.
    pub curve_error_bound: f64,
}

/// A mutable consumer export, whose native geometry is owned by this export alone.
/// Its binding handles belong to the same native topology.
pub struct DetachedWire {
    /// Fresh root with the recipe's verified closure and order.
    pub wire: Wire,
    /// Fresh occurrences in the recipe's order.
    pub edges: Vec<NativeEdgeBinding>,
    /// Fresh vertices in point-identity order.
    pub points: Vec<NativePointBinding>,
    /// Checks completed before returning the mutable export.
    pub validation: NativeWireValidation,
}

impl WireRecipe {
    /// Build and verify a fresh native wire. Example: `recipe.materialize(|| false)`.
    pub fn materialize(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedWire, WireFailure> {
        poll(&mut cancelled, WirePhase::Materialize)?;
        let builder = BRepBuilder::default();
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        let mut points = BTreeMap::new();
        let edges = build_occurrences(
            self.segments(),
            &builder,
            &mut wire,
            &mut points,
            &mut cancelled,
        )?;
        native_result(
            wire.set_closed(self.closure() == WireClosure::Closed),
            WirePhase::Materialize,
            None,
        )?;
        let validation = trace::validate(self, &wire, &edges, &points, &mut cancelled)?;
        Ok(DetachedWire {
            wire,
            edges,
            points: points
                .into_iter()
                .map(|(point, vertex)| NativePointBinding { point, vertex })
                .collect(),
            validation,
        })
    }
}

fn build_occurrences(
    segments: &[WireSegment],
    builder: &BRepBuilder,
    wire: &mut Wire,
    points: &mut BTreeMap<PointId, Vertex>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<NativeEdgeBinding>, WireFailure> {
    let mut edges = Vec::with_capacity(segments.len());
    for segment in segments {
        poll(cancelled, WirePhase::Materialize)?;
        let edge = build_edge(builder, segment, points)?;
        let line = Some(EntityId::Line(segment.source.line));
        native_result(builder.add(wire, &edge), WirePhase::Materialize, line)?;
        edges.push(NativeEdgeBinding {
            source: segment.source,
            endpoints: segment.endpoints(),
            edge,
        });
    }
    Ok(edges)
}

fn build_edge(
    builder: &BRepBuilder,
    segment: &WireSegment,
    points: &mut BTreeMap<PointId, Vertex>,
) -> Result<Edge, WireFailure> {
    let [a, b] = segment.authored.line.endpoints;
    let [pa, pb] = segment.authored.points;
    let mut first = vertex(builder, points, a, pa)?;
    let mut last = vertex(builder, points, b, pb)?;
    first.set_orientation(Orientation::Forward);
    last.set_orientation(Orientation::Reversed);
    let mut edge = make_line_edge(builder, segment, [&first, &last])?;
    if segment.source.direction == LineDirection::Reversed {
        edge.reverse();
    }
    Ok(edge)
}

fn make_line_edge(
    builder: &BRepBuilder,
    segment: &WireSegment,
    [first, last]: [&Vertex; 2],
) -> Result<Edge, WireFailure> {
    let mut edge = Edge::default();
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomLine::from_lin(segment.authored.carrier)).into_upcast();
    let entity = Some(EntityId::Line(segment.source.line));
    // BRepLib's edge maker reads a mutable process precision. This export owns its tolerance.
    native_result(
        builder.make_edge_with_curve(&mut edge, &curve, precision::confusion()),
        WirePhase::Materialize,
        entity,
    )?;
    native_result(
        builder.add(&mut edge, first),
        WirePhase::Materialize,
        entity,
    )?;
    native_result(builder.add(&mut edge, last), WirePhase::Materialize, entity)?;
    native_result(
        builder.range(&edge, 0.0, segment.authored.length_estimate, true),
        WirePhase::Materialize,
        entity,
    )?;
    Ok(edge)
}

fn vertex(
    builder: &BRepBuilder,
    points: &mut BTreeMap<PointId, Vertex>,
    point: PointId,
    position: Point3,
) -> Result<Vertex, WireFailure> {
    use std::collections::btree_map::Entry;
    let entry = match points.entry(point) {
        Entry::Occupied(entry) => return Ok(entry.get().clone()),
        Entry::Vacant(entry) => entry,
    };
    let mut vertex = Vertex::default();
    native_result(
        builder.make_vertex_at(&mut vertex, position, precision::confusion()),
        WirePhase::Materialize,
        Some(EntityId::Point(point)),
    )?;
    Ok(entry.insert(vertex).clone())
}

fn native_result<T>(
    result: Result<T, Failure>,
    phase: WirePhase,
    entity: Option<EntityId>,
) -> Result<T, WireFailure> {
    result.map_err(|source| WireFailure::Native {
        phase,
        entity,
        source: Box::new(source),
    })
}
