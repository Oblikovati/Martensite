// SPDX-License-Identifier: GPL-2.0-only
//! Checked native projection of one axial line onto one retained cylindrical face.
mod failure;
mod native;
mod storage;
mod target;
mod types;

pub use failure::{SurfaceProjectionCause, SurfaceProjectionFailure, SurfaceProjectionScope};
pub use types::*;

use martensite_tkernel::message::ProgressRange;
use martensite_tkmath::predicates::roots::metric_bounds::cylindrical_projection::{
    CylinderProjectionPlan, plan_axial_cylinder_projection,
};

type Checked<T> = Result<T, SurfaceProjectionCause>;

/// Publish detached actual native Section evidence selected by the requested projection mode.
/// Example: `project_axial_line_to_cylinder(source, target, mode, options, &progress)?`.
///
/// The caller pins both source revisions for the entire request. The native prism and Section
/// use private operands. The target domain is one full-period circular wall with an optional
/// retained cardinal rectangular hole and the line has a regular axial parameter.
pub fn project_axial_line_to_cylinder(
    source: SurfaceProjectionSource<'_>,
    target: SurfaceProjectionTarget<'_>,
    mode: AxialProjectionMode,
    options: SurfaceProjectionOptions,
    progress: &ProgressRange,
) -> Result<SurfaceProjectionResult, SurfaceProjectionFailure> {
    let parents = SurfaceProjectionParents {
        source: source.stamp,
        target: target.stamp,
    };
    let outcome = match options.behavior {
        SurfaceProjectionBehavior::AxialCardinalV1 => {
            project(source, target, mode, options, progress)
        }
    }
    .map_err(|cause| SurfaceProjectionFailure { parents, cause })?;
    Ok(SurfaceProjectionResult {
        parents,
        mode,
        options,
        outcome,
    })
}

fn project(
    source: SurfaceProjectionSource<'_>,
    target: SurfaceProjectionTarget<'_>,
    mode: AxialProjectionMode,
    options: SurfaceProjectionOptions,
    progress: &ProgressRange,
) -> Checked<SurfaceProjectionOutcome> {
    poll(progress)?;
    options.validate()?;
    let source_value = storage::source_line(source.edge)?;
    let target_value = target::capture(
        target.face,
        target.boundary,
        options.limits.spatial,
        progress,
    )?;
    let plan =
        plan_axial_cylinder_projection(target_value.cylinder, source_value.line, mode, || {
            progress.user_break()
        })?;
    poll(progress)?;
    match plan {
        CylinderProjectionPlan::Empty(reason) => Ok(SurfaceProjectionOutcome::Empty(reason)),
        CylinderProjectionPlan::Regular(witness) => native::publish(
            source.edge,
            source_value,
            target.face,
            target_value,
            &witness,
            options,
            progress,
        )
        .map(|value| SurfaceProjectionOutcome::Projected(Box::new(value))),
    }
}

fn poll(progress: &ProgressRange) -> Checked<()> {
    if progress.user_break() {
        return Err(SurfaceProjectionCause::Cancelled);
    }
    Ok(())
}
