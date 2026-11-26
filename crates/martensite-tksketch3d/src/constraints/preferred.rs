// SPDX-License-Identifier: GPL-2.0-only
//! Opt-in pointer movement on the prepared feasible sketch.
use super::{ConstrainedSketch, ConstraintFailure, DragUpdate, PointId, poll};
use martensite_tkmath::gp::Point3;
use std::sync::Arc;

pub use martensite_tksolve::preview::{PreferenceReport, PreferenceSettlement};

/// The independently admitted sketch and scaled cursor settlement information.
pub struct PreferredDragUpdate {
    /// Geometry, rank, hard error and cumulative work for the accepted update.
    pub update: DragUpdate,
    /// Numerical cursor settlement at the retained point.
    pub preference: PreferenceReport,
}

impl ConstrainedSketch {
    /// Move one point toward a cursor that may lie outside its feasible set.
    ///
    /// A successful projection can keep a point stationary. The tangent tolerance
    /// is positive and finite in scaled coordinate units. The ordinary physical
    /// and geometry gates still apply before publication.
    /// Example: `let moved = sketch.drag_preferred(point, cursor, 1e-7, || false)?;`
    pub fn drag_preferred(
        &self,
        point: PointId,
        target: Point3,
        tangent_tolerance: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PreferredDragUpdate, ConstraintFailure> {
        poll(&mut cancelled)?;
        let (mut output, mut preference) = self.program.drag_preferred(
            &self.geometry,
            point,
            target,
            tangent_tolerance,
            &mut cancelled,
        )?;
        let geometry = self.program.adopt(
            &self.geometry,
            &self.relations,
            &output.values,
            self.policy,
            &mut cancelled,
        )?;
        let next =
            self.program
                .reframe(&geometry, self.policy, Some(output.work), &mut cancelled)?;
        output.work = next.work;
        let hard_error = output.hard_error.max(next.hard_error);
        if !Arc::ptr_eq(&self.program, &next.program) {
            (output, preference) = next.program.recheck_preferred(
                &geometry,
                (point, target),
                output,
                tangent_tolerance,
                &mut cancelled,
            )?;
        }
        output.hard_error = output.hard_error.max(hard_error);
        let update = self.drag_update(geometry, next.program, output);
        Ok(PreferredDragUpdate { update, preference })
    }
}
