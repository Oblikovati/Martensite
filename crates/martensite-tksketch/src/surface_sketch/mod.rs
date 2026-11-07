//! Owned cylindrical surface sketches and physical normal extrusion (#416).
mod admission;
mod construction;
mod cylinder_sweep;
mod encoding;
mod failure;
mod frame;
mod history;
mod pipeline;
mod request;
mod result;
mod semantic;
mod sweep;
mod tolerance;
mod trim_binding;
mod validation;
mod volume;

pub use failure::{SurfaceFailureCause, SurfaceSketchFailure, SurfaceStage};
pub use history::{HistoryKey, HistoryRole, HistorySource};
pub use request::*;
pub use result::{
    DetachedNormalBody, NormalExtrusionResult, SurfaceDiagnostic, VolumeErrorEstimates,
};

/// Extrude one developed rectangle through an ordered physical normal-depth interval.
/// The returned body is complete, validated and independently owned.
/// Example: `let result = normal_extrude(&rectangle, [-1.0, 2.0], options)?;`.
pub fn normal_extrude(
    rectangle: &CylinderRectangle,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let admitted =
        admission::admit(rectangle, depths, options).map_err(|cause| SurfaceSketchFailure {
            region: rectangle.region,
            stage: SurfaceStage::Admission,
            cause,
        })?;
    let sweep = cylinder_sweep::prepare(&admitted);
    pipeline::build(&sweep, &|bindings| semantic::validate(&admitted, bindings))
}

#[cfg(test)]
mod semantic_tests;
