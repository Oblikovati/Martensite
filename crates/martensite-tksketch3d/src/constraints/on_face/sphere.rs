// SPDX-License-Identifier: GPL-2.0-only
//! One checked Sketch3D point on a captured finite full-period non-pole spherical band.
mod drag;
mod failure;

use super::{
    AttachmentEquation, ConstrainedSketch, ConstraintRank, FaceAttachmentRevision, Relation,
    RelationId,
    equation::SphericalEquation,
    periodic::{detach, point_xyz, prepare},
    poll,
};
use crate::{PointId, Snapshot};
use martensite_tkbrep::brep::spherical_attachment::{
    SphericalBandPlace, SphericalFaceDomain, SphericalPointQuery,
};
use std::{num::NonZeroU64, sync::Arc};

pub use drag::SphericalFaceDragUpdate;
pub use failure::SphericalFaceFailure;
use failure::SphericalFaceFailure as Failure;

/// Immutable source-bound spherical band and its host input reservation.
#[derive(Clone)]
pub struct SphericalFaceBinding {
    proof: Arc<SphericalFaceDomain>,
    revision: FaceAttachmentRevision,
}

impl SphericalFaceBinding {
    /// Bind a proved full native wall to the host input salt.
    /// Example: `SphericalFaceBinding::new(native_wall, input_salt)`.
    pub fn new(proof: SphericalFaceDomain, input_salt: NonZeroU64) -> Self {
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
    ) -> Result<SphericalPointQuery, Failure> {
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
        let row = SphericalEquation::new(chart, scale).ok_or(Failure::InvalidChartScale)?;
        Ok(AttachmentEquation::Sphere(row))
    }
}

/// Persisted point attachment with its continuous original angular parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphericalFaceRelation {
    /// Stable caller-reserved relation identity.
    pub id: RelationId,
    /// Owned point this constraint binds.
    pub point: PointId,
    /// Original face occurrence and exact host input reservation.
    pub source: FaceAttachmentRevision,
    /// Accepted unwrapped angle in radians and closed latitude.
    pub uv: [f64; 2],
}

/// Immutable, source-qualified sketch with one spherical Point On Face.
/// The selected native face has a closed full-period longitude and two nondegenerate latitude rims.
/// A sufficiently regular chart, a unique lifted step, both latitude comparisons, and the
/// original model-length limit are required before a candidate can replace the accepted state.
#[derive(Clone)]
pub struct SphericalAttachedSketch {
    sketch: ConstrainedSketch,
    binding: SphericalFaceBinding,
    record: SphericalFaceRelation,
}

impl ConstrainedSketch {
    /// Attach the original owned point near a supplied initial angular lift.
    /// Example: `sketch.attach_point_on_sphere(id, point, binding, 0., || false)?`.
    pub fn attach_point_on_sphere(
        &self,
        id: RelationId,
        point: PointId,
        binding: SphericalFaceBinding,
        near_u: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<SphericalAttachedSketch, Failure> {
        poll(&mut cancelled)?;
        let world = point_xyz(&self.geometry, point)?;
        let uv = binding
            .proof
            .point_seed(binding.revision.face, world, near_u)?;
        let record = SphericalFaceRelation {
            id,
            point,
            source: binding.revision,
            uv,
        };
        SphericalAttachedSketch::build(self, binding, record, &mut cancelled)
    }

    /// Restore the exact persisted authored UV lift against the same captured native source.
    /// Example: `sketch.restore_point_on_sphere(record, binding, || false)?`.
    pub fn restore_point_on_sphere(
        &self,
        record: SphericalFaceRelation,
        binding: SphericalFaceBinding,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<SphericalAttachedSketch, Failure> {
        SphericalAttachedSketch::build(self, binding, record, &mut cancelled)
    }
}

impl SphericalAttachedSketch {
    fn build(
        sketch: &ConstrainedSketch,
        binding: SphericalFaceBinding,
        record: SphericalFaceRelation,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<Self, Failure> {
        poll(cancelled)?;
        require_same(record.source, binding.revision)?;
        if sketch.relations.contains_key(&record.id) {
            return Err(super::super::ConstraintFailure::DuplicateId(record.id).into());
        }
        let lower = binding.proof.relative_area_lower();
        let limit = sketch.policy.solve.rank_tolerance;
        if lower <= limit {
            return Err(Failure::SingularChart { lower, limit });
        }
        let world = point_xyz(&sketch.geometry, record.point)?;
        binding.qualify(
            world,
            record.uv,
            sketch.policy.length_tolerance.value(),
            cancelled,
        )?;
        let equation =
            binding.equation(record.uv, sketch.policy.length_scale.value(), cancelled)?;
        let program = prepare(
            sketch,
            (record.id, record.point, record.uv),
            equation,
            cancelled,
        )?;
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
    pub fn relation(&self) -> SphericalFaceRelation {
        self.record
    }

    /// Read the achieved source trace and requested cap. Example: `attached.trim_trace()`.
    pub fn trim_trace(&self) -> [f64; 2] {
        self.binding.proof.trace()
    }

    /// Read ordinary persisted relations in stable order. Example: `attached.ordinary_relations()`.
    pub fn ordinary_relations(&self) -> impl Iterator<Item = (RelationId, Relation)> + '_ {
        self.sketch.relations()
    }

    /// Inspect the prepared ordinary and spherical rank. Example: `attached.inspect_rank()?`.
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
        binding: SphericalFaceBinding,
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
        let record = SphericalFaceRelation {
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
        Ok(detach(&self.sketch, &mut cancelled)?)
    }
}

fn require_trim(place: SphericalBandPlace) -> Result<(), Failure> {
    if matches!(place, SphericalBandPlace::Below | SphericalBandPlace::Above) {
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
