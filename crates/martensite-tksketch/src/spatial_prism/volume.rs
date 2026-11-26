// SPDX-License-Identifier: GPL-2.0-only
//! Bind and integrate the actual planar faces of the retained native sweep.
pub(crate) mod affine_curve;
mod bilinear;
mod face;
pub(crate) mod interval;

use super::{SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure::native, poll};
use interval::{Point, Span, l1_difference, placed};
use martensite_tkbo::profile_extrusion::{
    HistoryRole, HistorySource, IntegrationMethod, Measured, OperationResult, ResultEntity,
};
use martensite_tkbrep::{
    brep::{BRepTool, access::vertex_tshape},
    topods::{Face, Vertex},
};
use martensite_tksketch3d::wire::WireRecipe;
use std::collections::BTreeMap;

pub(super) fn certify(
    result: &OperationResult,
    wire: &WireRecipe,
    direction: [f64; 3],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Measured, f64), Failure> {
    let first = result
        .vertices()
        .first()
        .ok_or_else(|| unbound(None, "a reference vertex"))?;
    let origin = *native(BRepTool::pnt(first), Stage::Sweep)?.coordinates();
    if origin.iter().any(|coordinate| !coordinate.is_finite()) {
        return Err(unbound(None, "a finite reference vertex"));
    }
    let vertex_trace = bind_authored_vertices(result, wire, origin, direction, limit, cancelled)?;
    certify_affine_flux(
        result.faces().iter(),
        origin,
        vertex_trace,
        limit,
        FluxMode::Prism,
        cancelled,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FluxMode {
    Prism,
    Ruled,
}

pub(crate) fn certify_affine_flux<'a>(
    faces: impl IntoIterator<Item = &'a Face>,
    origin: [f64; 3],
    vertex_trace: f64,
    limit: f64,
    mode: FluxMode,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Measured, f64), Failure> {
    let mut face_trace = 0.0_f64;
    let mut numerator = Span::ZERO;
    for (index, face) in faces.into_iter().enumerate() {
        poll(cancelled, Stage::Sweep)?;
        let (volume, trace) = face::contribution(face, index, origin, limit, mode, cancelled)?;
        let bound = (Span::at(vertex_trace) + Span::at(trace)).hi;
        if !bound.is_finite() || bound > limit {
            return Err(Failure::SweepFaceTrace {
                face: index,
                bound,
                limit,
            });
        }
        numerator = numerator + volume;
        face_trace = face_trace.max(bound);
    }
    Ok((to_measure(numerator)?, face_trace))
}

fn bind_authored_vertices(
    result: &OperationResult,
    wire: &WireRecipe,
    origin: [f64; 3],
    direction: [f64; 3],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<f64, Failure> {
    let points: BTreeMap<_, _> = wire
        .segments()
        .iter()
        .flat_map(|segment| {
            segment
                .authored
                .line
                .endpoints
                .into_iter()
                .zip(segment.authored.points)
        })
        .map(|(id, point)| (id.get().get(), (id, point)))
        .collect();
    let mut trace = 0.0_f64;
    for record in &result.history().records {
        poll(cancelled, Stage::Sweep)?;
        let (HistorySource::Vertex(source), ResultEntity::Vertex(image)) =
            (record.source, record.result)
        else {
            continue;
        };
        let Some((point_id, authored)) = points.get(&source.0) else {
            return Err(unbound(None, "an authored vertex for each cap image"));
        };
        let Some(actual) = result.vertices().get(image) else {
            return Err(unbound(None, "a decoded cap vertex for each history image"));
        };
        let step = if record.role == HistoryRole::EndVertex {
            direction
        } else {
            [0.; 3]
        };
        let desired = std::array::from_fn(|axis| {
            (Span::at(authored.coordinates()[axis]) - Span::at(origin[axis])) + Span::at(step[axis])
        });
        let bound = l1_difference(vertex_point(actual, origin)?, desired);
        if !bound.is_finite() || bound > limit {
            return Err(Failure::SweepVertexTrace {
                point: *point_id,
                role: record.role,
                bound,
                limit,
            });
        }
        trace = trace.max(bound);
    }
    Ok(trace)
}

fn vertex_point(vertex: &Vertex, reference: [f64; 3]) -> Result<Point, Failure> {
    let stored = native(
        vertex_tshape(vertex, "spatial prism result vertex"),
        Stage::Sweep,
    )?
    .pnt();
    Ok(placed(
        stored.coordinates().map(Span::at),
        vertex.location().transformation(),
        reference,
    ))
}

fn to_measure(numerator: Span) -> Result<Measured, Failure> {
    let divisor = 1.0_f64 / 6.0;
    let bounded = numerator
        * Span {
            lo: divisor.next_down(),
            hi: divisor.next_up(),
        };
    if !bounded.finite() || bounded.lo <= 0. {
        return Err(unbound(
            None,
            "a finite strictly positive oriented volume enclosure",
        ));
    }
    let value = bounded.lo + (bounded.hi - bounded.lo) * 0.5;
    let error_bound = (value - bounded.lo).max(bounded.hi - value).next_up();
    Ok(Measured {
        value,
        error_bound,
        method: IntegrationMethod::CertifiedEnclosure,
    })
}

fn unbound(face: Option<usize>, expected: &'static str) -> Failure {
    Failure::UnboundSweep { face, expected }
}
