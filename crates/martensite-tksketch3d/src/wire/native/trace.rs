// SPDX-License-Identifier: GPL-2.0-only
//! Check stored identity, order, flags and whole affine line trace before publication.
use super::{NativeEdgeBinding, NativeWireValidation, native_result};
use crate::{
    EntityId, PointId,
    wire::{LineDirection, WireClosure, WireFailure, WirePhase, WireRecipe, WireSegment, poll},
};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        access::{edge_tshape, vertex_tshape},
    },
    topexp,
    topods::{Edge, ShapeIterator, Vertex, Wire},
};
use martensite_tkernel::precision;
use martensite_tkg3d::{geom::GeomLine, top_abs::Orientation};
use martensite_tkmath::gp::{Line3, Point3};
use std::collections::BTreeMap;

pub(super) fn validate(
    recipe: &WireRecipe,
    wire: &Wire,
    edges: &[NativeEdgeBinding],
    points: &BTreeMap<PointId, Vertex>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<NativeWireValidation, WireFailure> {
    poll(cancelled, WirePhase::VerifyNative)?;
    let closed = recipe.closure() == WireClosure::Closed;
    let native_closed = native_result(BRepTool::is_closed(wire), WirePhase::VerifyNative, None)?;
    let flag = native_result(wire.closed(), WirePhase::VerifyNative, None)?;
    require(
        closed == native_closed && closed == flag,
        None,
        "wire closure",
    )?;
    let bound = check_occurrences(wire, recipe.segments(), edges, points, cancelled)?;
    poll(cancelled, WirePhase::VerifyNative)?;
    Ok(NativeWireValidation {
        stored_tolerance: precision::confusion(),
        curve_error_bound: bound,
    })
}

fn check_occurrences(
    wire: &Wire,
    segments: &[WireSegment],
    edges: &[NativeEdgeBinding],
    points: &BTreeMap<PointId, Vertex>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<f64, WireFailure> {
    let mut occurrence = ShapeIterator::new(wire, true, true);
    let mut bound = 0.0_f64;
    for (source, binding) in segments.iter().zip(edges) {
        poll(cancelled, WirePhase::VerifyNative)?;
        let line = Some(EntityId::Line(source.source.line));
        let actual = native_result(occurrence.value(), WirePhase::VerifyNative, line)?;
        require(
            actual.is_equal(&binding.edge),
            line,
            "ordered edge identity and sense",
        )?;
        check_vertices(source, &binding.edge, points)?;
        bound = bound.max(check_carrier(source, &binding.edge)?);
        occurrence.next();
    }
    require(
        !occurrence.more() && segments.len() == edges.len(),
        None,
        "edge occurrence count",
    )?;
    Ok(bound)
}

fn check_vertices(
    source: &WireSegment,
    edge: &Edge,
    points: &BTreeMap<PointId, Vertex>,
) -> Result<(), WireFailure> {
    let line = Some(EntityId::Line(source.source.line));
    let sense = match source.source.direction {
        LineDirection::Forward => Orientation::Forward,
        LineDirection::Reversed => Orientation::Reversed,
    };
    require(edge.orientation() == sense, line, "authored orientation")?;
    let (first, last) = topexp::vertices(edge, false);
    let actual = [first, last];
    for ((id, at), vertex) in source
        .authored
        .line
        .endpoints
        .into_iter()
        .zip(source.authored.points)
        .zip(actual)
    {
        check_vertex(id, at, &vertex, points.get(&id))?;
    }
    Ok(())
}

fn check_vertex(
    id: PointId,
    position: Point3,
    actual: &Vertex,
    expected: Option<&Vertex>,
) -> Result<(), WireFailure> {
    let entity = Some(EntityId::Point(id));
    require(
        expected.is_some_and(|expected| actual.is_same(expected)),
        entity,
        "shared point identity",
    )?;
    let point = native_result(BRepTool::pnt(actual), WirePhase::VerifyNative, entity)?;
    let native = native_result(
        vertex_tshape(actual, "Sketch3D wire vertex tolerance"),
        WirePhase::VerifyNative,
        entity,
    )?;
    let tolerance = native.tolerance();
    require(
        point.coordinates().map(f64::to_bits) == position.coordinates().map(f64::to_bits),
        entity,
        "authored point coordinates",
    )?;
    require(
        tolerance == precision::confusion(),
        entity,
        "stored vertex tolerance",
    )
}

fn check_carrier(source: &WireSegment, edge: &Edge) -> Result<f64, WireFailure> {
    let line = source.source.line;
    let entity = Some(EntityId::Line(line));
    let stored = native_result(
        BRepTool::curve_with_location(edge),
        WirePhase::VerifyNative,
        entity,
    )?;
    let Some(curve) = stored.curve.downcast_ref::<GeomLine>() else {
        return Err(WireFailure::NativeMismatch {
            entity,
            expected: "analytic line carrier",
        });
    };
    require(
        stored.location.is_identity(),
        entity,
        "model-space line location",
    )?;
    let parameters = [stored.first, stored.last];
    require(
        parameters == [0.0, source.authored.length_estimate],
        entity,
        "authored trim order and range",
    )?;
    let native = native_result(
        edge_tshape(edge, "Sketch3D wire edge tolerance"),
        WirePhase::VerifyNative,
        entity,
    )?;
    let tolerance = native.tolerance();
    require(
        tolerance == precision::confusion(),
        entity,
        "stored edge tolerance",
    )?;
    let bound = trace_bound(curve.lin(), parameters, source.authored.points);
    if !bound.is_finite() || bound > tolerance {
        return Err(WireFailure::NativeErrorBound {
            line,
            bound,
            limit: tolerance,
        });
    }
    Ok(bound)
}

fn require(
    passed: bool,
    entity: Option<EntityId>,
    expected: &'static str,
) -> Result<(), WireFailure> {
    if !passed {
        return Err(WireFailure::NativeMismatch { entity, expected });
    }
    Ok(())
}

fn trace_bound(line: Line3, parameters: [f64; 2], points: [Point3; 2]) -> f64 {
    // The difference from the authored segment is affine under the normalized trim.
    // Its L1 norm is bounded by the larger endpoint bound and bounds Euclidean error.
    let first = endpoint_bound(line, parameters[0], points[0]);
    let last = endpoint_bound(line, parameters[1], points[1]);
    first.max(last)
}

fn endpoint_bound(line: Line3, t: f64, point: Point3) -> f64 {
    let origin = line.location();
    let direction = line.direction();
    let mut bound = 0.0;
    for ((&base, &dir), &target) in origin
        .coordinates()
        .iter()
        .zip(direction.coordinates())
        .zip(point.coordinates())
    {
        let component = component_bound(base, dir, t, target);
        bound = add_up(bound, component);
    }
    bound
}

fn component_bound(base: f64, direction: f64, t: f64, target: f64) -> f64 {
    if t == 0. || direction == 0. {
        return abs_difference(base, target);
    }
    // FMA performs one rounding. Neighboring floats enclose the exact affine value,
    // even if the unrounded product and origin nearly cancel.
    let rounded = direction.mul_add(t, base);
    if !rounded.is_finite() {
        return f64::INFINITY;
    }
    abs_difference(rounded.next_down(), target).max(abs_difference(rounded.next_up(), target))
}

fn abs_difference(left: f64, right: f64) -> f64 {
    if left == right {
        return 0.;
    }
    (left - right).abs().next_up()
}

fn add_up(a: f64, b: f64) -> f64 {
    if a == 0. || b == 0. {
        return a + b;
    }
    (a + b).next_up()
}
