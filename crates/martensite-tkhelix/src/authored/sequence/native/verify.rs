// SPDX-License-Identifier: GPL-2.0-only
//! Validate actual child order, shared native vertices and whole retained active trims.
use super::{
    super::{
        HelixSequenceError as Failure,
        stations::{Form, Span},
    },
    HelixSequence, HelixSequenceSpanReceipt,
    build::Work,
};
use crate::authored::{
    HelixFailure, HelixFitOptions, HelixNativeInvariant as Invariant, HelixRotation, HelixStage,
    failure::{native, poll},
    native::verify::{Captured, STAGE, tolerance},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Edge, ShapeIterator, Vertex, Wire},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::predicates::roots::{
    DyadicBound,
    metric_bounds::{
        HelixCylindricalSpanLaw, HelixSpanProgress, HelixTraceFailure,
        certify_cylindrical_helix_span,
    },
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

pub(super) fn edges(
    wire: &Wire,
    count: usize,
    stage: HelixStage,
) -> Result<Vec<Edge>, HelixFailure> {
    let mut iterator = ShapeIterator::new(wire, true, true);
    let mut edges = Vec::with_capacity(count);
    while iterator.more() {
        let edge = native(topods::edge(native(iterator.value(), stage)?), stage)?;
        if edge.orientation() != Orientation::Forward || edges.len() == count {
            return Err(HelixFailure::Topology(Invariant::OrderedWire));
        }
        edges.push(edge);
        iterator.next();
    }
    if edges.len() != count || native(wire.closed(), stage)? {
        return Err(HelixFailure::Topology(Invariant::OrderedWire));
    }
    Ok(edges)
}

pub(super) fn retained(
    law: &HelixSequence,
    wire: &Wire,
    options: HelixFitOptions,
    work: &[Work],
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<Box<[HelixSequenceSpanReceipt]>, Failure> {
    poll(cancelled, STAGE)?;
    let ordered = edges(wire, law.spans.len(), STAGE)?;
    if work.len() != ordered.len() {
        return Err(HelixFailure::Topology(Invariant::OrderedWire).into());
    }
    let check = native(BRepCheckAnalyzer::new(wire, true, false, false), STAGE)?;
    if !native(check.is_valid(), STAGE)? || native(BRepTool::is_closed(wire), STAGE)? {
        return Err(HelixFailure::Topology(Invariant::ValidWire).into());
    }
    let mut previous = None;
    let mut remaining = options;
    let mut receipts = Vec::with_capacity(ordered.len());
    for ((edge, span), work) in ordered.iter().zip(&law.spans).zip(work) {
        let endpoints = station_vertices(edge, previous.as_ref())?;
        let receipt = verify_span(law, span, edge, &endpoints, remaining, work, cancelled)
            .map_err(|cause| Failure::SpanFailure {
                source: span.source,
                cause: Box::new(cause),
            })?;
        remaining.trace.max_operations -= receipt.trace.operations;
        remaining.trace.max_cells -= receipt.trace.cells;
        remaining.trace.max_native_spans -= receipt.trace.native_spans;
        let [_, last] = endpoints;
        previous = Some(last);
        receipts.push(receipt);
    }
    Ok(receipts.into_boxed_slice())
}

fn verify_span(
    law: &HelixSequence,
    span: &Span,
    edge: &Edge,
    endpoints: &[Vertex; 2],
    remaining: HelixFitOptions,
    work: &Work,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<HelixSequenceSpanReceipt, HelixFailure> {
    poll(cancelled, STAGE)?;
    let captured = Captured::read(edge, remaining.trace.max_native_spans)?;
    let source = captured.source();
    literal_endpoints(endpoints, source.poles)?;
    let stored_topology_tolerance = tolerance(edge, endpoints, remaining.max_topology_tolerance)?;
    let axial_controls = controls(span);
    let trace = certify_cylindrical_helix_span(
        reference(law, span, &axial_controls),
        source,
        remaining.trace,
        &mut *cancelled,
    )
    .map_err(trace_failure)?;
    if trace.native_spans != work.native_spans {
        return Err(HelixFailure::Topology(Invariant::OrderedWire));
    }
    Ok(HelixSequenceSpanReceipt {
        span: span.summary(),
        native_estimate: work.estimate,
        evaluations: work.evaluations,
        trace,
        stored_topology_tolerance,
    })
}

fn station_vertices(edge: &Edge, previous: Option<&Vertex>) -> Result<[Vertex; 2], HelixFailure> {
    let (first, last) = topexp::vertices(edge, true);
    if first.is_null()
        || last.is_null()
        || first.is_same(&last)
        || !first.location().is_identity()
        || !last.location().is_identity()
        || previous.is_some_and(|previous| !previous.is_same(&first))
        || native(BRepTool::degenerated(edge), STAGE)?
    {
        return Err(HelixFailure::Topology(Invariant::SharedStation));
    }
    Ok([first, last])
}

fn literal_endpoints([first, last]: &[Vertex; 2], poles: &[[f64; 3]]) -> Result<(), HelixFailure> {
    if native(BRepTool::pnt(first), STAGE)?.coordinates() != &poles[0]
        || native(BRepTool::pnt(last), STAGE)?.coordinates() != &poles[poles.len() - 1]
    {
        return Err(HelixFailure::Topology(Invariant::LiteralEndpoint));
    }
    Ok(())
}

fn trace_failure(cause: HelixTraceFailure) -> HelixFailure {
    match cause {
        HelixTraceFailure::Cancelled => HelixFailure::Cancelled(STAGE),
        other => HelixFailure::Trace(Box::new(other)),
    }
}

fn reference<'a>(
    law: &HelixSequence,
    span: &'a Span,
    axial_controls: &'a [DyadicBound],
) -> HelixCylindricalSpanLaw<'a> {
    let frame = law.placement.frame;
    HelixCylindricalSpanLaw {
        origin: *frame.location().coordinates(),
        basis: [frame.x_direction(), frame.y_direction(), frame.direction()]
            .map(|direction| *direction.coordinates()),
        radius: law.placement.radius,
        turns: span.stations.each_ref().map(|station| &station.turns),
        native_parameter: span.summary().parameters,
        axial_controls,
        positive_rotation: law.placement.rotation == HelixRotation::Positive,
        progress: match span.form {
            Form::Flat => HelixSpanProgress::Flat,
            Form::Constant => HelixSpanProgress::Advancing,
            Form::Accelerating | Form::Decelerating => HelixSpanProgress::Ramp,
        },
    }
}

fn controls(span: &Span) -> Vec<DyadicBound> {
    let [opening, closing] = span.stations.each_ref().map(|station| &station.height);
    match span.form {
        Form::Flat => vec![opening.clone()],
        Form::Constant => vec![opening.clone(), closing.clone()],
        Form::Accelerating => vec![
            opening.clone(),
            opening.clone(),
            opening.clone(),
            opening.midpoint(closing),
            closing.clone(),
        ],
        Form::Decelerating => vec![
            opening.clone(),
            opening.midpoint(closing),
            closing.clone(),
            closing.clone(),
            closing.clone(),
        ],
    }
}
