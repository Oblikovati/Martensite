// SPDX-License-Identifier: GPL-2.0-only
//! One checked Sketch3D point on a captured finite full-period cylinder.
mod drag;
mod failure;

use super::{
    AttachmentEquation, ConstrainedSketch, ConstraintRank, FaceAttachmentRevision, Program,
    Relation, RelationId, equation::CylindricalEquation, poll, verify_relations,
};
use crate::{PointId, Snapshot};
use martensite_tkbrep::brep::cylindrical_split::{
    CylinderPointQuery, CylinderWallPlace, CylindricalSplitDomain, CylindricalTrimTrace,
};
use std::{num::NonZeroU64, sync::Arc};

pub use drag::CylindricalFaceDragUpdate;
pub use failure::CylindricalFaceFailure;
use failure::CylindricalFaceFailure as Failure;

/// Immutable source-bound cylinder and its host input reservation.
#[derive(Clone)]
pub struct CylindricalFaceBinding {
    proof: Arc<CylindricalSplitDomain>,
    revision: FaceAttachmentRevision,
}

impl CylindricalFaceBinding {
    /// Bind a proved full native wall to the host input salt.
    /// Example: `CylindricalFaceBinding::new(native_wall, input_salt)`.
    pub fn new(proof: CylindricalSplitDomain, input_salt: NonZeroU64) -> Self {
        let revision = FaceAttachmentRevision {
            face: proof.source(),
            input_salt,
        };
        Self {
            proof: Arc::new(proof),
            revision,
        }
    }

    /// Read the exact named source and input reservation. Example: `binding.revision()`.
    pub fn revision(&self) -> FaceAttachmentRevision {
        self.revision
    }

    fn qualify(
        &self,
        xyz: [f64; 3],
        uv: [f64; 2],
        length_limit: f64,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<CylinderPointQuery, Failure> {
        let query =
            self.proof
                .point_qualify(self.revision.face, uv, xyz, length_limit, cancelled)?;
        require_trim(query.world_place)?;
        require_trim(query.uv_place)?;
        Ok(query)
    }

    fn equation(
        &self,
        uv: [f64; 2],
        scale: f64,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<AttachmentEquation, Failure> {
        let chart = self.proof.point_chart(self.revision.face, uv, cancelled)?;
        let row = CylindricalEquation::new(chart, scale).ok_or(Failure::InvalidChartScale)?;
        Ok(AttachmentEquation::Cylinder(row))
    }
}

/// Persisted point attachment with its continuous original angular parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalFaceRelation {
    /// Stable caller-reserved relation identity.
    pub id: RelationId,
    /// Owned point this constraint binds.
    pub point: PointId,
    /// Original face occurrence and exact host input reservation.
    pub source: FaceAttachmentRevision,
    /// Accepted unwrapped angle in radians and closed axial coordinate.
    pub uv: [f64; 2],
}

/// Immutable, source-qualified sketch with one cylindrical Point On Face.
#[derive(Clone)]
pub struct CylindricalAttachedSketch {
    sketch: ConstrainedSketch,
    binding: CylindricalFaceBinding,
    record: CylindricalFaceRelation,
}

impl ConstrainedSketch {
    /// Attach the original owned point near a supplied initial angular lift.
    /// Example: `sketch.attach_point_on_cylinder(id, point, binding, 0., || false)?`.
    pub fn attach_point_on_cylinder(
        &self,
        id: RelationId,
        point: PointId,
        binding: CylindricalFaceBinding,
        near_u: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylindricalAttachedSketch, Failure> {
        poll(&mut cancelled)?;
        let world = point_xyz(&self.geometry, point)?;
        let uv = binding
            .proof
            .point_seed(binding.revision.face, world, near_u)?;
        let record = CylindricalFaceRelation {
            id,
            point,
            source: binding.revision,
            uv,
        };
        CylindricalAttachedSketch::build(self, binding, record, &mut cancelled)
    }

    /// Restore the exact persisted authored UV lift against the same captured native source.
    /// Example: `sketch.restore_point_on_cylinder(record, binding, || false)?`.
    pub fn restore_point_on_cylinder(
        &self,
        record: CylindricalFaceRelation,
        binding: CylindricalFaceBinding,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylindricalAttachedSketch, Failure> {
        CylindricalAttachedSketch::build(self, binding, record, &mut cancelled)
    }
}

impl CylindricalAttachedSketch {
    fn build(
        sketch: &ConstrainedSketch,
        binding: CylindricalFaceBinding,
        record: CylindricalFaceRelation,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<Self, Failure> {
        poll(cancelled)?;
        require_same(record.source, binding.revision)?;
        if sketch.relations.contains_key(&record.id) {
            return Err(super::super::ConstraintFailure::DuplicateId(record.id).into());
        }
        let world = point_xyz(&sketch.geometry, record.point)?;
        binding.qualify(
            world,
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

    /// Borrow the immutable accepted model. Example: `attached.geometry().point(point)`.
    pub fn geometry(&self) -> &Snapshot {
        self.sketch.geometry()
    }

    /// Read the persisted relation. Example: `attached.relation().uv`.
    pub fn relation(&self) -> CylindricalFaceRelation {
        self.record
    }

    /// Read the actual source trim fidelity. Example: `attached.trim_trace().upper_bound`.
    pub fn trim_trace(&self) -> CylindricalTrimTrace {
        self.binding.proof.trim_trace()
    }

    /// Read ordinary persisted relations in stable order. Example: `attached.ordinary_relations()`.
    pub fn ordinary_relations(&self) -> impl Iterator<Item = (RelationId, Relation)> + '_ {
        self.sketch.relations()
    }

    /// Inspect the prepared ordinary and cylindrical rank. Example: `attached.inspect_rank()?`.
    pub fn inspect_rank(&self) -> Result<ConstraintRank, Failure> {
        Ok(self
            .sketch
            .program
            .inspect_with_uv(&self.sketch.geometry, Some(self.record.uv))?)
    }

    /// Recapture the same named occurrence at a newer input, retaining the accepted world point.
    /// Example: `attached.rebind(next_binding, || false)?`.
    pub fn rebind(
        &self,
        binding: CylindricalFaceBinding,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<Self, Failure> {
        poll(&mut cancelled)?;
        if !binding.revision.may_replace(self.record.source) {
            return Err(Failure::RebindSource {
                prior: self.record.source,
                next: binding.revision,
            });
        }
        let world = point_xyz(&self.sketch.geometry, self.record.point)?;
        let uv = binding
            .proof
            .point_seed(binding.revision.face, world, self.record.uv[0])?;
        let record = CylindricalFaceRelation {
            uv,
            source: binding.revision,
            ..self.record
        };
        Self::build(&self.sketch, binding, record, &mut cancelled)
    }

    /// Explicitly remove source authority while keeping accepted model geometry and ordinary relations.
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
            &mut cancelled,
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
    binding: &CylindricalFaceBinding,
    record: CylindricalFaceRelation,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<Arc<Program>, Failure> {
    let equation = binding.equation(record.uv, sketch.policy.length_scale.value(), cancelled)?;
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
        cancelled,
    )?);
    program.verify_attachment_state(&sketch.geometry, record.uv, cancelled)?;
    Ok(program)
}

fn point_xyz(geometry: &Snapshot, point: PointId) -> Result<[f64; 3], Failure> {
    Ok(*geometry
        .point(point)
        .ok_or(super::super::ConstraintFailure::MissingPoint(point))?
        .position
        .coordinates())
}

fn require_trim(place: CylinderWallPlace) -> Result<(), Failure> {
    if matches!(place, CylinderWallPlace::Below | CylinderWallPlace::Above) {
        return Err(Failure::Trim(place));
    }
    Ok(())
}

fn require_same(
    expected: FaceAttachmentRevision,
    current: FaceAttachmentRevision,
) -> Result<(), Failure> {
    if expected != current {
        return Err(Failure::StaleSource { expected, current });
    }
    Ok(())
}
