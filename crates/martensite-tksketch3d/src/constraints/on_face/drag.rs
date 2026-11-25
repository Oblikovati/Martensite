// SPDX-License-Identifier: GPL-2.0-only
//! Adopt only a fully replayed hard result inside both exact interpretations of the trim.
use super::{
    Failure, PlanarAttachedSketch, PlanarFaceRelation, PlanarFaceRevision, admitted, require_same,
};
use crate::{
    PointId,
    constraints::{
        ConstrainedSketch, ConstraintRank, PreferenceReport, PreferenceSettlement, poll, residual,
    },
};
use martensite_tkbrep::brep::planar_attachment::{
    FaceAttachmentError, PlanarAttachmentError, PlanarAttachmentLocation,
};
use martensite_tkmath::gp::Point3;
use std::sync::Arc;

/// A completed trimmed attachment update and its hard solver evidence.
pub struct PlanarFaceDragUpdate {
    /// Next immutable attached sketch and its accepted UV state.
    pub sketch: PlanarAttachedSketch,
    /// Prepared hard rows and retained coordinate directions.
    pub rank: ConstraintRank,
    /// Largest dimensionless numerical hard residual at the retained update.
    pub hard_error: f64,
    /// Cumulative charged built-in numeric work of the prepared solve and replay.
    pub residual_work_bound: usize,
    /// Projected cursor evidence from the retained coordinates.
    pub preference: PreferenceReport,
    /// Exact trim class of the accepted world point's support projection.
    pub trim: PlanarAttachmentLocation,
}

impl PlanarAttachedSketch {
    /// Follow a preferred world cursor while keeping all hard rows and the actual face trim.
    /// The authority callback supplies the current face before solving and before publication.
    /// Example: `attached.drag_preferred(point, target, 1e-7, || source, || false)?`.
    pub fn drag_preferred(
        &self,
        point: PointId,
        target: Point3,
        tangent_tolerance: f64,
        mut current_source: impl FnMut() -> PlanarFaceRevision,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarFaceDragUpdate, Failure> {
        poll(&mut cancelled)?;
        require_same(self.record.source, current_source())?;
        let updated = self.solve((point, target), tangent_tolerance, &mut cancelled)?;
        poll(&mut cancelled)?;
        require_same(self.record.source, current_source())?;
        Ok(updated)
    }

    fn solve(
        &self,
        cursor: (PointId, Point3),
        tangent_tolerance: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<PlanarFaceDragUpdate, Failure> {
        let sketch = &self.sketch;
        let (mut output, preference) = sketch.program.prefer_with_uv(
            (&sketch.geometry, Some(self.record.uv)),
            cursor,
            tangent_tolerance,
            cancelled,
        )?;
        if cursor.0 == self.record.point
            && preference.settlement == PreferenceSettlement::TargetReached
        {
            self.preserve_exact_cursor(cursor.1, &mut output.values, cancelled)?;
        }
        let uv = sketch.program.parameter_values(&output.values)?;
        let geometry = sketch.program.adopt(
            &sketch.geometry,
            &sketch.relations,
            &output.values,
            sketch.policy,
            cancelled,
        )?;
        let point = geometry.point(self.record.point).ok_or(
            crate::constraints::ConstraintFailure::MissingPoint(self.record.point),
        )?;
        let trim = self.binding.qualify(
            *point.position.coordinates(),
            uv,
            sketch.policy.length_tolerance.value(),
            cancelled,
        )?;
        let next = sketch.program.reframe_with_uv(
            &geometry,
            Some(uv),
            sketch.policy,
            Some(output.work),
            cancelled,
        )?;
        output.work = next.work;
        output.values = next.program.values_with_uv(&geometry, Some(uv))?;
        let observed_error = output.hard_error.max(next.hard_error);
        let (output, preference) = next.program.recheck_preferred(
            &geometry,
            cursor,
            output,
            tangent_tolerance,
            cancelled,
        )?;
        let rank = next.program.rank_report(output.rank, output.dof);
        Ok(PlanarFaceDragUpdate {
            sketch: PlanarAttachedSketch {
                sketch: ConstrainedSketch {
                    geometry: Arc::new(geometry),
                    program: next.program,
                    ..sketch.clone()
                },
                binding: self.binding.clone(),
                record: PlanarFaceRelation { uv, ..self.record },
            },
            rank,
            hard_error: output.hard_error.max(observed_error),
            residual_work_bound: residual::work_bound(output.work.work),
            preference,
            trim,
        })
    }

    fn preserve_exact_cursor(
        &self,
        cursor: Point3,
        values: &mut [f64],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        use PlanarAttachmentLocation::{HoleBoundary, OuterBoundary};
        let xyz = *cursor.coordinates();
        let exact = match self
            .binding
            .proof
            .classify(self.record.source.face, xyz, 0., cancelled)
        {
            Err(FaceAttachmentError::Numeric(PlanarAttachmentError::OffSupport { .. })) => {
                return Ok(());
            }
            result => admitted(result?)?,
        };
        if matches!(exact.location, HoleBoundary | OuterBoundary) {
            if exact.uv.iter().any(|range| range[0] != range[1]) {
                return Err(Failure::BoundaryParameters(exact.uv));
            }
            // The cursor itself is the exact closed-boundary candidate. Full hard and
            // preferred replay must still accept this vector before publishing it.
            self.sketch.program.set_point_and_uv(
                self.record.point,
                xyz,
                exact.uv.map(|range| range[0]),
                values,
            )?;
        }
        Ok(())
    }
}
