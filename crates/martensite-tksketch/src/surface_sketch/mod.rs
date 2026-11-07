//! Owned cylindrical surface sketches and physical normal extrusion (#416).
mod admission;
mod construction;
mod encoding;
mod failure;
mod frame;
mod history;
mod request;
mod result;
mod semantic;
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
    build(&admitted)
}
fn build(admitted: &admission::Admitted) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let (section, mut maker, solid) = construction::construct(admitted)
        .map_err(|cause| admitted.failure(SurfaceStage::Construction, cause))?;
    let bindings = history::collect(admitted, &section, &mut maker, &solid)
        .map_err(|cause| admitted.failure(SurfaceStage::History, cause))?;
    semantic::validate(admitted, &bindings)
        .map_err(|cause| admitted.failure(SurfaceStage::Validation, cause))?;
    let bytes = encoding::encode(&solid, &bindings)
        .map_err(|cause| admitted.failure(SurfaceStage::Encoding, cause))?;
    publish(
        admitted,
        bytes,
        bindings.iter().map(|(key, _)| *key).collect(),
    )
}
fn publish(
    admitted: &admission::Admitted,
    bytes: Vec<u8>,
    keys: Vec<HistoryKey>,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let (solid, bindings) = encoding::decode(&bytes, &keys)
        .map_err(|cause| admitted.failure(SurfaceStage::Encoding, cause))?;
    let checked = validation::check_body(&solid, &bindings, admitted)
        .map_err(|cause| admitted.failure(SurfaceStage::Validation, cause))?;
    let (volume, volume_errors) = volume::measure(&solid, admitted)
        .map_err(|cause| admitted.failure(SurfaceStage::Volume, cause))?;
    Ok(result::assemble(
        admitted,
        bytes,
        keys,
        checked,
        volume,
        volume_errors,
    ))
}

#[cfg(test)]
mod semantic_tests;
