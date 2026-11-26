// SPDX-License-Identifier: GPL-2.0-only
//! Shared solver custody for one periodic point on an independently qualified source.
use super::{
    AttachmentEquation, ConstrainedSketch, ConstraintRank, Program, RelationId, Snapshot, poll,
    verify_relations,
};
use crate::{
    PointId,
    constraints::{ConstraintFailure as Failure, PreferenceReport, residual},
};
use martensite_tkmath::gp::Point3;
use martensite_tksolve::preview::Update;
use std::sync::Arc;

pub(super) fn prepare(
    sketch: &ConstrainedSketch,
    (id, point, uv): (RelationId, PointId, [f64; 2]),
    equation: AttachmentEquation,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Arc<Program>, Failure> {
    verify_relations(
        &sketch.geometry,
        &sketch.relations,
        sketch.policy,
        cancelled,
    )?;
    let program = Arc::new(Program::compile(
        &sketch.geometry,
        &sketch.relations,
        sketch.policy,
        Some((id, point, equation)),
        cancelled,
    )?);
    program.verify_attachment_state(&sketch.geometry, uv, cancelled)?;
    Ok(program)
}

pub(super) fn point_xyz(geometry: &Snapshot, point: PointId) -> Result<[f64; 3], Failure> {
    Ok(*geometry
        .point(point)
        .ok_or(Failure::MissingPoint(point))?
        .position
        .coordinates())
}

pub(super) fn detach(
    sketch: &ConstrainedSketch,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ConstrainedSketch, Failure> {
    poll(cancelled)?;
    let program = Arc::new(Program::new(
        &sketch.geometry,
        &sketch.relations,
        sketch.policy,
        cancelled,
    )?);
    poll(cancelled)?;
    Ok(ConstrainedSketch {
        program,
        ..sketch.clone()
    })
}

pub(super) struct Replayed {
    pub sketch: ConstrainedSketch,
    pub rank: ConstraintRank,
    pub hard_error: f64,
    pub residual_work_bound: usize,
    pub preference: PreferenceReport,
}

pub(super) fn replay(
    sketch: &ConstrainedSketch,
    (geometry, uv): (Snapshot, [f64; 2]),
    (equation, mut output): (AttachmentEquation, Update),
    (cursor, tolerance): ((PointId, Point3), f64),
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Replayed, Failure> {
    let next = sketch.program.reframe_attachment(
        &geometry,
        uv,
        equation,
        sketch.policy,
        output.work,
        cancelled,
    )?;
    output.work = next.work;
    output.values = next.program.values_with_uv(&geometry, Some(uv))?;
    let observed = output.hard_error.max(next.hard_error);
    let (output, preference) = next
        .program
        .recheck_preferred(&geometry, cursor, output, tolerance, cancelled)?;
    let rank = next.program.rank_report(output.rank, output.dof);
    Ok(Replayed {
        sketch: ConstrainedSketch {
            geometry: Arc::new(geometry),
            program: next.program,
            ..sketch.clone()
        },
        rank,
        preference,
        hard_error: output.hard_error.max(observed),
        residual_work_bound: residual::work_bound(output.work.work),
    })
}
