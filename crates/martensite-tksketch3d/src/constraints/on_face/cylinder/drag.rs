// SPDX-License-Identifier: GPL-2.0-only
//! Preferred motion on the original lifted chart, followed by complete trim and solver replay.
use super::{
    ConstraintRank, CylindricalAttachedSketch, CylindricalFaceRelation, FaceAttachmentRevision,
    Failure, point_xyz, poll, require_same, require_trim,
};
use crate::{
    PointId,
    constraints::{ConstrainedSketch, PreferenceReport, PreferenceSettlement, residual},
};
use martensite_tkbrep::brep::cylindrical_split::{CylinderPointQuery, CylinderWallPlace};
use martensite_tkmath::gp::Point3;
use std::sync::Arc;

/// One accepted immutable cylinder drag and its solver and native evidence.
pub struct CylindricalFaceDragUpdate {
    /// Next source-bound sketch and persisted unwrapped UV lift.
    pub sketch: CylindricalAttachedSketch,
    /// Retained hard rank and degrees of freedom.
    pub rank: ConstraintRank,
    /// Largest replayed dimensionless numerical hard error.
    pub hard_error: f64,
    /// Cumulative numeric work charged by the prepared solver and replay.
    pub residual_work_bound: usize,
    /// Preferred-cursor evidence for the fully replayed accepted state.
    pub preference: PreferenceReport,
    /// Exact world/UV axial classifications and certified model discrepancy.
    pub attachment: CylinderPointQuery,
}

impl CylindricalAttachedSketch {
    /// Follow a world cursor across the stored periodic seam without resetting the angular sheet.
    /// Example: `attached.drag_preferred(point, cursor, 1e-8, || source, || false)?`.
    pub fn drag_preferred(
        &self,
        point: PointId,
        cursor: Point3,
        tangent_tolerance: f64,
        mut current_source: impl FnMut() -> FaceAttachmentRevision,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylindricalFaceDragUpdate, Failure> {
        poll(&mut cancelled)?;
        require_same(self.record.source, current_source())?;
        self.check_cursor_direction(point, cursor)?;
        let updated = self.solve((point, cursor), tangent_tolerance, &mut cancelled)?;
        poll(&mut cancelled)?;
        require_same(self.record.source, current_source())?;
        Ok(updated)
    }

    fn check_cursor_direction(&self, point: PointId, cursor: Point3) -> Result<(), Failure> {
        if point == self.record.point {
            let xyz = point_xyz(&self.sketch.geometry, point)?;
            let prior = self.record.uv[0];
            self.binding.proof.point_branch(
                self.record.source.face,
                prior,
                prior,
                Some([xyz, *cursor.coordinates()]),
            )?;
        }
        Ok(())
    }

    fn solve(
        &self,
        cursor: (PointId, Point3),
        tangent_tolerance: f64,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<CylindricalFaceDragUpdate, Failure> {
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
            self.preserve_rim_cursor(cursor.1, &mut output.values, cancelled)?;
        }
        let uv = sketch.program.parameter_values(&output.values)?;
        self.binding
            .proof
            .point_branch(self.record.source.face, self.record.uv[0], uv[0], None)?;
        let geometry = sketch.program.adopt(
            &sketch.geometry,
            &sketch.relations,
            &output.values,
            sketch.policy,
            cancelled,
        )?;
        let xyz = point_xyz(&geometry, self.record.point)?;
        let attachment =
            self.binding
                .qualify(xyz, uv, sketch.policy.length_tolerance.value(), cancelled)?;
        let equation = self
            .binding
            .equation(uv, sketch.policy.length_scale.value(), cancelled)?;
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
        let (output, preference) = next.program.recheck_preferred(
            &geometry,
            cursor,
            output,
            tangent_tolerance,
            cancelled,
        )?;
        let rank = next.program.rank_report(output.rank, output.dof);
        Ok(CylindricalFaceDragUpdate {
            sketch: Self {
                sketch: ConstrainedSketch {
                    geometry: Arc::new(geometry),
                    program: next.program,
                    ..sketch.clone()
                },
                binding: self.binding.clone(),
                record: CylindricalFaceRelation { uv, ..self.record },
            },
            rank,
            hard_error: output.hard_error.max(observed),
            preference,
            attachment,
            residual_work_bound: residual::work_bound(output.work.work),
        })
    }

    fn preserve_rim_cursor(
        &self,
        cursor: Point3,
        values: &mut [f64],
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<(), Failure> {
        let xyz = *cursor.coordinates();
        let current = self.record.source.face;
        let place = self.binding.proof.point_place(current, xyz)?;
        require_trim(place)?;
        if matches!(
            place,
            CylinderWallPlace::FirstRim | CylinderWallPlace::LastRim
        ) {
            let uv = self
                .binding
                .proof
                .point_seed(current, xyz, self.record.uv[0])?;
            self.binding.qualify(
                xyz,
                uv,
                self.sketch.policy.length_tolerance.value(),
                cancelled,
            )?;
            self.sketch
                .program
                .set_point_and_uv(self.record.point, xyz, uv, values)?;
        }
        Ok(())
    }
}
