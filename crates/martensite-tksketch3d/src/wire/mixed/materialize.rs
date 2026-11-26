// SPDX-License-Identifier: GPL-2.0-only
//! Bind checked mixed geometry to actual private BRep edge occurrences.
use super::{
    CardinalWireArc, DirectedCurveUse, LineDirection, MixedWireGeometry, MixedWireRecipe,
    MixedWireSegment, WireClosure, WireFailure, WirePhase, pairs::exact_line, poll,
};
use crate::{
    EntityId, PointId,
    wire::{
        NativePointBinding, NativeWireValidation,
        native::{
            edge_vertices, make_curve_edge, make_line_edge, native_result, point_bindings,
            trace::{check_bound_vertices, check_root, require},
        },
    },
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepCurve3D, BRepTool, EdgeCurve3D, access::edge_tshape},
    topods::{Edge, ShapeIterator, Vertex, Wire},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomLine};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use std::collections::BTreeMap;

/// A certified tangent at a native trim end in the original occurrence direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeTangentBound {
    /// Ideal directed tangent proven against the original cardinal geometry.
    pub direction: Direction3,
    /// Certified angular deviation of the analytic stored trim, in radians.
    pub angle_error_upper: f64,
}

/// Literal native parameter ranges and original control evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NativeMixedTrim {
    /// Whole affine span read from the actual edge.
    Line {
        /// Native parameters in wire direction, descending for a reversed occurrence.
        parameters: [f64; 2],
    },
    /// Whole circular span read from the actual edge.
    Arc {
        /// Native parameters in wire direction, descending for a reversed occurrence.
        parameters: [f64; 2],
        /// Original center-control identity.
        center: PointId,
        /// Actual native center coordinates.
        center_position: Point3,
        /// Actual native X, Y and normal directions.
        axes: [Direction3; 3],
        /// Actual radius read from the stored circle.
        radius: f64,
        /// Whole normalized-range spatial discrepancy from the ideal real quarter.
        position_error_upper: f64,
        /// Analytic trim tangents in wire order.
        tangents: [NativeTangentBound; 2],
    },
}

/// An actual private edge with stable source IDs and native trim evidence.
pub struct MixedNativeEdgeBinding {
    /// Original curve and oriented use.
    pub source: DirectedCurveUse,
    /// Authored endpoint IDs in wire order.
    pub endpoints: [PointId; 2],
    /// Actual edge used by this native wire.
    pub edge: Edge,
    /// Verified original geometry and parameter transport.
    pub trim: NativeMixedTrim,
}

/// Fresh mutable native consumer topology with its own geometry and bindings.
pub struct DetachedMixedWire {
    /// Actual root retaining the recipe's checked connectivity and occurrence order.
    pub wire: Wire,
    /// Fresh native occurrences in directed recipe order.
    pub edges: Vec<MixedNativeEdgeBinding>,
    /// Fresh original endpoint vertices in ID order.
    pub points: Vec<NativePointBinding>,
    /// Actual whole-wire readback and achieved spatial bound.
    pub validation: NativeWireValidation,
}

impl MixedWireRecipe {
    /// Build and check a fresh analytic native wire. Example: `recipe.materialize(|| false)?`.
    pub fn materialize(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedMixedWire, WireFailure> {
        poll(&mut cancelled, WirePhase::Materialize)?;
        let (wire, native_edges, points) = build(self, &mut cancelled)?;
        let edges = readback(self, &wire, native_edges, &points, &mut cancelled)?;
        let validation = summarize(&edges);
        let points = point_bindings(points, &mut cancelled)?;
        poll(&mut cancelled, WirePhase::VerifyNative)?;
        Ok(DetachedMixedWire {
            wire,
            edges,
            points,
            validation,
        })
    }
}

type Draft = (Wire, Vec<Edge>, BTreeMap<PointId, Vertex>);

fn build(
    recipe: &MixedWireRecipe,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Draft, WireFailure> {
    let builder = BRepBuilder::default();
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    let mut points = BTreeMap::new();
    let mut edges = Vec::with_capacity(recipe.segments().len());
    for segment in recipe.segments() {
        poll(cancelled, WirePhase::Materialize)?;
        let edge = build_edge(segment, &builder, &mut points)?;
        native_result(
            builder.add(&mut wire, &edge),
            WirePhase::Materialize,
            Some(entity(segment)),
        )?;
        edges.push(edge);
    }
    native_result(
        wire.set_closed(recipe.closure() == WireClosure::Closed),
        WirePhase::Materialize,
        None,
    )?;
    Ok((wire, edges, points))
}

fn build_edge(
    segment: &MixedWireSegment,
    builder: &BRepBuilder,
    points: &mut BTreeMap<PointId, Vertex>,
) -> Result<Edge, WireFailure> {
    let [first, last] = edge_vertices(
        builder,
        points,
        segment.authored_endpoints(),
        segment.points(),
    )?;
    let mut edge = match segment.geometry {
        MixedWireGeometry::Line(line) => make_line_edge(builder, &line, [&first, &last])?,
        MixedWireGeometry::Arc(arc) => build_arc(arc, builder, [&first, &last])?,
    };
    if segment.source.direction == LineDirection::Reversed {
        edge.reverse();
    }
    Ok(edge)
}

fn build_arc(
    arc: CardinalWireArc,
    builder: &BRepBuilder,
    vertices: [&Vertex; 2],
) -> Result<Edge, WireFailure> {
    let arc = arc.authored();
    let entity = EntityId::Arc(arc.recipe().id);
    let curve = native_result(
        GeomCircle::new(arc.frame(), arc.radius()),
        WirePhase::Materialize,
        Some(entity),
    )?;
    make_curve_edge(
        builder,
        &Handle::new(curve).into_upcast(),
        arc.range(),
        entity,
        vertices,
    )
}

fn readback(
    recipe: &MixedWireRecipe,
    wire: &Wire,
    edges: Vec<Edge>,
    points: &BTreeMap<PointId, Vertex>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<MixedNativeEdgeBinding>, WireFailure> {
    check_root(wire, recipe.closure())?;
    let mut occurrence = ShapeIterator::new(wire, true, true);
    let mut result = Vec::with_capacity(edges.len());
    require(
        edges.len() == recipe.segments().len(),
        None,
        "mixed edge count",
    )?;
    for (segment, edge) in recipe.segments().iter().zip(edges) {
        poll(cancelled, WirePhase::VerifyNative)?;
        let actual = native_result(
            occurrence.value(),
            WirePhase::VerifyNative,
            Some(entity(segment)),
        )?;
        require(
            actual.is_equal(&edge),
            Some(entity(segment)),
            "ordered edge identity and sense",
        )?;
        result.push(bind_occurrence(segment, edge, points)?);
        occurrence.next();
    }
    require(!occurrence.more(), None, "no extra edge occurrences")?;
    Ok(result)
}

fn bind_occurrence(
    segment: &MixedWireSegment,
    edge: Edge,
    points: &BTreeMap<PointId, Vertex>,
) -> Result<MixedNativeEdgeBinding, WireFailure> {
    check_bound_vertices(
        entity(segment),
        segment.source.direction,
        segment.authored_endpoints(),
        segment.points(),
        &edge,
        points,
    )?;
    let stored = read_carrier(segment, &edge)?;
    let trim = match segment.geometry {
        MixedWireGeometry::Line(line) => {
            let entity = Some(entity(segment));
            let line_curve =
                stored
                    .curve
                    .downcast_ref::<GeomLine>()
                    .ok_or(WireFailure::NativeMismatch {
                        entity,
                        expected: "literal analytic line carrier",
                    })?;
            require(
                exact_line(line_curve.lin(), [stored.first, stored.last], &line),
                entity,
                "exact line trace",
            )?;
            NativeMixedTrim::Line {
                parameters: directed_range(segment.source.direction, &stored),
            }
        }
        MixedWireGeometry::Arc(arc) => read_arc(arc, segment.source.direction, &stored)?,
    };
    Ok(MixedNativeEdgeBinding {
        source: segment.source,
        endpoints: segment.endpoints(),
        edge,
        trim,
    })
}

fn read_carrier(segment: &MixedWireSegment, edge: &Edge) -> Result<EdgeCurve3D, WireFailure> {
    let entity = Some(entity(segment));
    require(
        edge.nb_children() == 2,
        entity,
        "exact endpoint occurrence count",
    )?;
    let stored = native_result(
        BRepTool::curve_with_location(edge),
        WirePhase::VerifyNative,
        entity,
    )?;
    let owner = native_result(
        edge_tshape(edge, "Sketch3D mixed wire edge"),
        WirePhase::VerifyNative,
        entity,
    )?;
    require(
        matches!(owner.curves().as_slice(), [representation] if representation.downcast_ref::<BRepCurve3D>().is_some()),
        entity,
        "one literal spatial representation",
    )?;
    let range = match segment.geometry {
        MixedWireGeometry::Line(line) => [0., line.authored.length_estimate],
        MixedWireGeometry::Arc(arc) => arc.authored().range(),
    };
    require(
        stored.location.is_identity() && [stored.first, stored.last] == range,
        entity,
        "original model-space trim",
    )?;
    require(
        owner.tolerance() == precision::confusion() && !owner.degenerated(),
        entity,
        "stored edge tolerance and use",
    )?;
    Ok(stored)
}

fn read_arc(
    arc: CardinalWireArc,
    direction: LineDirection,
    stored: &EdgeCurve3D,
) -> Result<NativeMixedTrim, WireFailure> {
    let authored = arc.authored();
    let entity = Some(EntityId::Arc(authored.recipe().id));
    let circle = stored
        .curve
        .downcast_ref::<GeomCircle>()
        .ok_or(WireFailure::NativeMismatch {
            entity,
            expected: "literal analytic circle carrier",
        })?;
    let circle = native_result(circle.circ(), WirePhase::VerifyNative, entity)?;
    let frame = circle.position();
    require(
        frame.location() == authored.frame().location()
            && axes(frame) == axes(authored.frame())
            && circle.radius() == authored.radius(),
        entity,
        "source circle coefficients",
    )?;
    let tangent_axes = arc.tangents(direction);
    let errors = CardinalWireArc::tangent_errors(direction);
    Ok(NativeMixedTrim::Arc {
        parameters: directed_range(direction, stored),
        center: arc.center(),
        center_position: frame.location(),
        axes: axes(frame),
        radius: circle.radius(),
        position_error_upper: arc.terminal_error_upper(),
        tangents: [0, 1].map(|i| NativeTangentBound {
            direction: tangent_axes[i],
            angle_error_upper: errors[i],
        }),
    })
}

fn axes(frame: Axis2) -> [Direction3; 3] {
    [frame.x_direction(), frame.y_direction(), frame.direction()]
}

fn directed_range(direction: LineDirection, stored: &EdgeCurve3D) -> [f64; 2] {
    match direction {
        LineDirection::Forward => [stored.first, stored.last],
        LineDirection::Reversed => [stored.last, stored.first],
    }
}

fn entity(segment: &MixedWireSegment) -> EntityId {
    segment.source.curve.entity()
}

fn summarize(edges: &[MixedNativeEdgeBinding]) -> NativeWireValidation {
    let bound = edges
        .iter()
        .map(|edge| match edge.trim {
            NativeMixedTrim::Line { .. } => 0.,
            NativeMixedTrim::Arc {
                position_error_upper,
                ..
            } => position_error_upper,
        })
        .fold(0., f64::max);
    NativeWireValidation {
        stored_tolerance: precision::confusion(),
        curve_error_bound: bound,
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(
        DetachedMixedWire,
        MixedNativeEdgeBinding,
        NativeMixedTrim,
        NativeTangentBound,
    )>();
};

#[cfg(test)]
mod tests;
