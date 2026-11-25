// SPDX-License-Identifier: GPL-2.0-only
//! Read and prove the exact archived edge, including directed incidence and stored tolerances.
use super::super::{
    ConstantHelix, HelixFailure, HelixFitOptions, HelixNativeInvariant as Invariant, HelixStage,
    HelixTraceBounds,
    failure::{native, poll},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Edge, ShapeIterator, Vertex, Wire},
};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomCurve},
    top_abs::Orientation,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    HelixSplineSource, HelixTraceFailure, certify_polynomial_helix_trace,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const STAGE: HelixStage = HelixStage::Verification;

pub(super) fn edge(wire: &Wire) -> Result<Edge, HelixFailure> {
    let mut members = ShapeIterator::new(wire, true, true);
    let edge = native(topods::edge(native(members.value(), STAGE)?), STAGE)?;
    members.next();
    if edge.orientation() != Orientation::Forward || members.more() {
        return Err(HelixFailure::Topology(Invariant::SingleForwardEdge));
    }
    Ok(edge)
}

pub(super) struct CheckedCarrier {
    pub trace: HelixTraceBounds,
    pub stored_topology_tolerance: f64,
    pub terminal_error_upper: [f64; 2],
}

pub(super) fn retained(
    law: ConstantHelix,
    wire: &Wire,
    edge: &Edge,
    options: HelixFitOptions,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<CheckedCarrier, HelixFailure> {
    let terminals = topology(wire, edge, cancelled)?;
    let stored_topology_tolerance = tolerance(edge, &terminals, options.max_topology_tolerance)?;
    let captured = Captured::read(edge, options.trace.max_native_spans)?;
    let trace = certify_polynomial_helix_trace(
        law.trace_law(),
        captured.source(),
        options.trace,
        &mut *cancelled,
    )
    .map_err(|cause| match cause {
        HelixTraceFailure::Cancelled => HelixFailure::Cancelled(STAGE),
        other => HelixFailure::Trace(Box::new(other)),
    })?;
    let terminal_error_upper = terminal_errors(
        &terminals,
        &captured,
        trace.position_error_upper,
        options.trace.max_position_error,
    )?;
    poll(cancelled, STAGE)?;
    Ok(CheckedCarrier {
        trace,
        stored_topology_tolerance,
        terminal_error_upper,
    })
}

fn topology(
    wire: &Wire,
    edge: &Edge,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[Vertex; 2], HelixFailure> {
    poll(cancelled, STAGE)?;
    let (first, last) = topexp::vertices(edge, true);
    if first.is_null()
        || last.is_null()
        || first.is_same(&last)
        || !first.location().is_identity()
        || !last.location().is_identity()
        || native(wire.closed(), STAGE)?
        || native(BRepTool::is_closed(wire), STAGE)?
        || native(BRepTool::degenerated(edge), STAGE)?
    {
        return Err(HelixFailure::Topology(Invariant::OpenDistinctTerminals));
    }
    let check = native(BRepCheckAnalyzer::new(wire, true, false, false), STAGE)?;
    if !native(check.is_valid(), STAGE)? {
        return Err(HelixFailure::Topology(Invariant::ValidWire));
    }
    poll(cancelled, STAGE)?;
    Ok([first, last])
}

fn tolerance(edge: &Edge, [first, last]: &[Vertex; 2], limit: f64) -> Result<f64, HelixFailure> {
    let values = [
        native(BRepTool::edge_tolerance(edge), STAGE)?,
        native(BRepTool::vertex_tolerance(first), STAGE)?,
        native(BRepTool::vertex_tolerance(last), STAGE)?,
    ];
    let mut maximum: f64 = 0.;
    for achieved in values {
        if !achieved.is_finite() || achieved < 0. || achieved > limit {
            return Err(HelixFailure::TopologyTolerance { achieved, limit });
        }
        maximum = maximum.max(achieved);
    }
    Ok(maximum)
}

struct Captured {
    degree: usize,
    poles: Vec<[f64; 3]>,
    knots: Vec<f64>,
    trim: [f64; 2],
}

impl Captured {
    fn read(edge: &Edge, limit: usize) -> Result<Self, HelixFailure> {
        let carrier = native(BRepTool::curve_with_location(edge), STAGE)?;
        let expected = HelixFailure::Topology(Invariant::PolynomialCarrier);
        let spline = carrier
            .curve
            .downcast_ref::<GeomBSplineCurve>()
            .filter(|spline| {
                carrier.location.is_identity() && !spline.is_rational() && !spline.is_periodic()
            })
            .ok_or(expected)?;
        let degree = spline.degree() as usize;
        let count = spline.nb_poles() as usize;
        if !(1..=8).contains(&degree)
            || count <= degree
            || count > degree.saturating_mul(limit).saturating_add(1)
        {
            return Err(HelixFailure::Topology(Invariant::PolynomialExtent));
        }
        let poles = (1..=spline.nb_poles())
            .map(|i| native(spline.pole(i), STAGE).map(|point| *point.coordinates()))
            .collect::<Result<Vec<_>, _>>()?;
        let knots = spline.knot_sequence().iter().copied().collect();
        Ok(Self {
            degree,
            poles,
            knots,
            trim: [carrier.first, carrier.last],
        })
    }

    fn source(&self) -> HelixSplineSource<'_> {
        HelixSplineSource {
            degree: self.degree,
            poles: &self.poles,
            knots: &self.knots,
            trim: self.trim,
        }
    }
}

fn terminal_errors(
    terminals: &[Vertex; 2],
    curve: &Captured,
    bound: f64,
    limit: f64,
) -> Result<[f64; 2], HelixFailure> {
    let end = curve.poles.len() - 1;
    Ok([
        terminal(&terminals[0], curve.poles[0], bound, limit, 0)?,
        terminal(&terminals[1], curve.poles[end], bound, limit, 1)?,
    ])
}

fn terminal(
    vertex: &Vertex,
    pole: [f64; 3],
    bound: f64,
    limit: f64,
    endpoint: usize,
) -> Result<f64, HelixFailure> {
    let stored = native(BRepTool::pnt(vertex), STAGE)?;
    let gap = l1_gap(*stored.coordinates(), pole);
    let achieved = if gap == 0. {
        bound
    } else {
        (gap + bound).next_up()
    };
    if !achieved.is_finite() || achieved > limit {
        return Err(HelixFailure::TerminalPosition {
            endpoint,
            achieved,
            limit,
        });
    }
    Ok(achieved)
}

fn l1_gap(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).fold(0., |accumulated, (a, b)| {
        if a == b {
            accumulated
        } else {
            (accumulated + (a - b).abs().next_up()).next_up()
        }
    })
}

#[cfg(test)]
mod tests;
