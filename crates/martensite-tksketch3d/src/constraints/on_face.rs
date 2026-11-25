// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound planar attachment ownership layered over the actual hard sketch program.
mod drag;
mod equation;
mod failure;

use super::{
    ConstrainedSketch, ConstraintRank, Program, Relation, RelationId, adopt::verify_relations, poll,
};
use crate::{PointId, Snapshot};
use martensite_tkbrep::brep::planar_attachment::{
    AttachmentSource, FaceAttachment, PlanarAttachmentLocation, PlanarAttachmentQuery,
};
use std::{num::NonZeroU64, sync::Arc};

pub use drag::PlanarFaceDragUpdate;
pub(in crate::constraints) use equation::PlanarEquation;
pub use failure::PlanarFaceConstraintFailure;
use failure::PlanarFaceConstraintFailure as Failure;

/// Host-authoritative identity and version of a captured native face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlanarFaceRevision {
    /// Original named body and face with their host incarnation and revision.
    pub face: AttachmentSource,
    /// Host input reservation for the exact immutable source binding.
    pub input_salt: NonZeroU64,
}

impl PlanarFaceRevision {
    fn may_replace(self, old: Self) -> bool {
        let a = self.face;
        let b = old.face;
        a.document == b.document
            && a.body == b.body
            && a.face == b.face
            && a.incarnation == b.incarnation
            && a.revision > b.revision
            && self.input_salt != old.input_salt
    }
}

/// Immutable face evidence and its host-owned input reservation.
#[derive(Clone)]
pub struct PlanarFaceBinding {
    proof: Arc<FaceAttachment>,
    revision: PlanarFaceRevision,
}

impl PlanarFaceBinding {
    /// Associate a read-only native proof with its host input reservation.
    /// Example: `PlanarFaceBinding::new(proof, input_salt)`.
    pub fn new(proof: FaceAttachment, input_salt: NonZeroU64) -> Self {
        let revision = PlanarFaceRevision {
            face: proof.source(),
            input_salt,
        };
        Self {
            proof: Arc::new(proof),
            revision,
        }
    }

    /// The named source and input version required for publication. Example: `binding.revision()`.
    pub fn revision(&self) -> PlanarFaceRevision {
        self.revision
    }

    fn seed(
        &self,
        point: [f64; 3],
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<[f64; 2], Failure> {
        let projected =
            admitted(
                self.proof
                    .classify(self.revision.face, point, limit, cancelled)?,
            )?;
        // The lower endpoint is deterministic and is independently checked as the actual lift.
        Ok(projected.uv.map(|interval| interval[0]))
    }

    fn qualify(
        &self,
        point: [f64; 3],
        uv: [f64; 2],
        limit: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentLocation, Failure> {
        let projected =
            admitted(
                self.proof
                    .classify(self.revision.face, point, limit, &mut *cancelled)?,
            )?;
        admitted(
            self.proof
                .classify_uv(self.revision.face, uv, point, limit, cancelled)?,
        )?;
        Ok(projected.location)
    }
}

/// Persisted value for one Point On Face relation and its accepted original chart state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarFaceRelation {
    /// Caller-reserved identity in the ordinary or attached relation namespace.
    pub id: RelationId,
    /// The owned sketch point being attached.
    pub point: PointId,
    /// Stable source selection and exact active input revision.
    pub source: PlanarFaceRevision,
    /// Last accepted values of the two internal solver coordinates.
    pub uv: [f64; 2],
}

/// A constrained line sketch with one source-bound planar point attachment.
#[derive(Clone)]
pub struct PlanarAttachedSketch {
    sketch: ConstrainedSketch,
    binding: PlanarFaceBinding,
    record: PlanarFaceRelation,
}

impl ConstrainedSketch {
    /// Attach one owned point to an already captured planar face, without moving any geometry.
    /// Example: `sketch.attach_point_on_face(relation, point, binding, || false)?`.
    pub fn attach_point_on_face(
        &self,
        id: RelationId,
        point: PointId,
        binding: PlanarFaceBinding,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachedSketch, Failure> {
        poll(&mut cancelled)?;
        let uv = binding.seed(
            self.point_xyz(point)?,
            self.policy.length_tolerance.value(),
            &mut cancelled,
        )?;
        let record = PlanarFaceRelation {
            id,
            point,
            source: binding.revision,
            uv,
        };
        PlanarAttachedSketch::build(self, record, binding, &mut cancelled)
    }

    /// Restore a persisted attachment with currently resolved source evidence and accepted inputs.
    /// Example: `sketch.restore_point_on_face(stored, binding, || false)?`.
    pub fn restore_point_on_face(
        &self,
        record: PlanarFaceRelation,
        binding: PlanarFaceBinding,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachedSketch, Failure> {
        PlanarAttachedSketch::build(self, record, binding, &mut cancelled)
    }

    fn point_xyz(&self, point: PointId) -> Result<[f64; 3], Failure> {
        Ok(*self
            .geometry
            .point(point)
            .ok_or(super::ConstraintFailure::MissingPoint(point))?
            .position
            .coordinates())
    }
}

impl PlanarAttachedSketch {
    fn build(
        sketch: &ConstrainedSketch,
        record: PlanarFaceRelation,
        binding: PlanarFaceBinding,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        poll(cancelled)?;
        require_same(record.source, binding.revision)?;
        if sketch.relations.contains_key(&record.id) {
            return Err(super::ConstraintFailure::DuplicateId(record.id).into());
        }
        binding.qualify(
            sketch.point_xyz(record.point)?,
            record.uv,
            sketch.policy.length_tolerance.value(),
            cancelled,
        )?;
        let program = build_program(sketch, &binding, record, cancelled)?;
        poll(cancelled)?;
        Ok(Self {
            sketch: ConstrainedSketch {
                program,
                ..sketch.clone()
            },
            binding,
            record,
        })
    }

    /// Borrow the accepted geometry. Example: `attached.geometry().point(point)`.
    pub fn geometry(&self) -> &Snapshot {
        self.sketch.geometry()
    }

    /// The complete persisted attachment state. Example: `attached.relation().uv`.
    pub fn relation(&self) -> PlanarFaceRelation {
        self.record
    }

    /// Borrow the ordinary persisted relation values in their stable order.
    /// Example: attached.ordinary_relations().collect::<Vec<_>>().
    pub fn ordinary_relations(&self) -> impl Iterator<Item = (RelationId, Relation)> + '_ {
        self.sketch.relations()
    }

    /// Inspect rank of the ordinary rows and the two-coordinate attachment.
    /// Example: `attached.inspect_rank()?.degrees_of_freedom`.
    pub fn inspect_rank(&self) -> Result<ConstraintRank, Failure> {
        Ok(self
            .sketch
            .program
            .inspect_with_uv(&self.sketch.geometry, Some(self.record.uv))?)
    }

    /// Resolve the same named face at a later revision while retaining the current point.
    /// Example: `attached.rebind(new_binding, || false)?`.
    pub fn rebind(
        &self,
        binding: PlanarFaceBinding,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        poll(&mut cancelled)?;
        if !binding.revision.may_replace(self.record.source) {
            return Err(Failure::RebindSource {
                prior: self.record.source,
                next: binding.revision,
            });
        }
        let point = self.sketch.point_xyz(self.record.point)?;
        let uv = binding.seed(
            point,
            self.sketch.policy.length_tolerance.value(),
            &mut cancelled,
        )?;
        let record = PlanarFaceRelation {
            uv,
            source: binding.revision,
            ..self.record
        };
        Self::build(&self.sketch, record, binding, &mut cancelled)
    }

    /// Remove the relation while retaining the accepted geometry and ordinary relations.
    /// Example: `attached.detach(|| false)?`.
    pub fn detach(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<ConstrainedSketch, Failure> {
        poll(&mut cancelled)?;
        let program = Arc::new(Program::new(
            &self.sketch.geometry,
            &self.sketch.relations,
            self.sketch.policy,
        )?);
        poll(&mut cancelled)?;
        Ok(ConstrainedSketch {
            program,
            ..self.sketch.clone()
        })
    }
}

fn build_program(
    sketch: &ConstrainedSketch,
    binding: &PlanarFaceBinding,
    record: PlanarFaceRelation,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Arc<Program>, Failure> {
    let chart = binding.proof.solver_chart(record.source.face, record.uv)?;
    let equation = PlanarEquation::new(chart, sketch.policy.length_scale.value())
        .ok_or(Failure::InvalidChartScale)?;
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
        Some((record.id, record.point, equation)),
    )?);
    program.verify_attachment_state(&sketch.geometry, record.uv, cancelled)?;
    Ok(program)
}

fn admitted(query: PlanarAttachmentQuery) -> Result<PlanarAttachmentQuery, Failure> {
    use PlanarAttachmentLocation::{HoleInterior, OutsideOuter};
    match query.location {
        OutsideOuter | HoleInterior => Err(Failure::Trim(query.location)),
        _ => Ok(query),
    }
}

fn require_same(expected: PlanarFaceRevision, current: PlanarFaceRevision) -> Result<(), Failure> {
    if expected != current {
        return Err(Failure::StaleSource { expected, current });
    }
    Ok(())
}
