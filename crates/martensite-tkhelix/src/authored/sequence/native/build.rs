// SPDX-License-Identifier: GPL-2.0-only
//! One fit per authored smooth span, with literal shared geometry and topology at every join.
use super::super::{
    HelixSequence, HelixSequenceError as Failure,
    stations::{Form, Span},
};
use crate::authored::{
    HelixFailure, HelixFitOptions, HelixNativeInvariant as Invariant, HelixStage,
    failure::{native, poll},
    fit,
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Edge, Vertex, Wire},
};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};

const STAGE: HelixStage = HelixStage::Assembly;

pub(super) struct Work {
    pub estimate: f64,
    pub evaluations: u32,
    pub native_spans: usize,
}

pub(super) struct Candidate {
    pub wire: Wire,
    pub work: Vec<Work>,
}

pub(super) fn assemble(
    law: &HelixSequence,
    options: HelixFitOptions,
    max_segments: usize,
    flat_axis: Option<usize>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Candidate, Failure> {
    let mut remaining = options;
    remaining.max_segments = max_segments;
    let mut builder = BRepBuilderAPIMakeWire::new();
    let mut work = Vec::with_capacity(law.spans.len());
    let mut last = None;
    for span in &law.spans {
        let (edge, vertex, consumed) = build(law, span, remaining, flat_axis, last, cancelled)
            .map_err(|cause| Failure::SpanFailure {
                source: span.source,
                cause: Box::new(cause),
            })?;
        native(builder.add_edge(&edge), STAGE)?;
        remaining.max_evaluations -= consumed.evaluations;
        remaining.max_segments -= consumed.native_spans;
        work.push(consumed);
        last = Some(vertex);
    }
    poll(cancelled, STAGE)?;
    Ok(Candidate {
        wire: native(builder.wire(), STAGE)?,
        work,
    })
}

fn build(
    law: &HelixSequence,
    span: &Span,
    remaining: HelixFitOptions,
    flat_axis: Option<usize>,
    previous: Option<Vertex>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Edge, Vertex, Work), HelixFailure> {
    if remaining.max_segments == 0 || remaining.max_evaluations == 0 {
        return Err(HelixFailure::InvalidWorkLimits {
            max_segments: remaining.max_segments,
            max_evaluations: remaining.max_evaluations,
        });
    }
    let range = span.stations.each_ref().map(|station| station.parameter);
    let fitted = fit::build_range(
        range,
        remaining,
        remaining.max_segments as i32,
        |parameter, order| component(law, span, range, parameter, order),
        cancelled,
    )?;
    poll(cancelled, STAGE)?;
    let carrier = fitted
        .curve
        .get()
        .ok_or(HelixFailure::Topology(Invariant::PolynomialCarrier))?;
    let end_points = [
        point(law.components(span, range[0])?[0]),
        point(law.components(span, range[1])?[0]),
    ];
    pin(carrier, end_points, span.form, flat_axis)?;
    let first = match previous {
        Some(vertex) => vertex,
        None => vertex(end_points[0])?,
    };
    let last = vertex(end_points[1])?;
    let spans = (carrier.nb_knots() - 1) as usize;
    if spans == 0 || spans > remaining.max_segments {
        return Err(HelixFailure::Topology(Invariant::PolynomialExtent));
    }
    let curve = fitted.curve.into_upcast::<dyn GeomCurve>();
    let edge = native(
        BRepBuilderAPIMakeEdge::from_curve_vertices_params(
            &curve, &first, &last, range[0], range[1],
        ),
        STAGE,
    )?;
    let work = Work {
        estimate: fitted.estimate,
        evaluations: fitted.evaluations,
        native_spans: spans,
    };
    Ok((native(edge.edge(), STAGE)?, last, work))
}

fn component(
    law: &HelixSequence,
    span: &Span,
    [start, end]: [f64; 2],
    parameter: f64,
    order: u8,
) -> Result<[f64; 3], HelixFailure> {
    if !(start..=end).contains(&parameter) {
        return Err(HelixFailure::Parameter { parameter, end });
    }
    Ok(law.components(span, parameter)?[usize::from(order)])
}

fn vertex(point: Point3) -> Result<Vertex, HelixFailure> {
    let mut vertex = Vertex::default();
    native(
        BRepBuilder::default().make_vertex_at(
            &mut vertex,
            point,
            martensite_tkernel::precision::confusion(),
        ),
        STAGE,
    )?;
    Ok(vertex)
}

fn pin(
    curve: &GeomBSplineCurve,
    [first, last]: [Point3; 2],
    form: Form,
    flat_axis: Option<usize>,
) -> Result<(), HelixFailure> {
    let count = curve.nb_poles();
    if matches!(form, Form::Accelerating | Form::Decelerating) && count < 4 {
        return Err(HelixFailure::Topology(Invariant::PolynomialExtent));
    }
    native(curve.set_pole(1, first), STAGE)?;
    native(curve.set_pole(count, last), STAGE)?;
    if let Some(axis) = flat_axis {
        match form {
            Form::Flat => pin_axis(curve, 2..count, axis, first)?,
            Form::Accelerating => pin_axis(curve, 2..4, axis, first)?,
            Form::Decelerating => pin_axis(curve, (count - 2)..count, axis, last)?,
            Form::Constant => {}
        }
    }
    Ok(())
}

fn pin_axis(
    curve: &GeomBSplineCurve,
    indices: std::ops::Range<i32>,
    axis: usize,
    anchor: Point3,
) -> Result<(), HelixFailure> {
    for i in indices {
        let pole = native(curve.pole(i), STAGE)?;
        let mut xyz = *pole.coordinates();
        xyz[axis] = anchor.coordinates()[axis];
        native(curve.set_pole(i, point(xyz)), STAGE)?;
    }
    Ok(())
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}
