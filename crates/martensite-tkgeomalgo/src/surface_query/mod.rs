//! Coherent owned query embedding; this API does not certify trim or whole-face validity.
mod domain;
mod extract;
mod layout;
mod source;
mod types;
use martensite_tkbrep::{
    brep::face_input_capture::{FaceCaptureBehavior, capture_face_input_with_behavior},
    topods::Face,
};
use martensite_tkmath::predicates::roots::surface_query::certify_surface_query;
pub use types::*;

/// Capture and prove the actual face support on a declared query domain; `certify_face_query(&face, request, FaceQueryPolicy::bounded(), || false)`.
pub fn certify_face_query(
    face: &Face,
    request: SurfaceSelfQuery,
    policy: FaceQueryPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> FaceQueryReport {
    let mut report = FaceQueryReport {
        request,
        policy,
        input: None,
        proof: None,
        surface_owner: None,
        incomplete: None,
    };
    report.incomplete = run(face, &mut report, &mut cancelled).err();
    report
}
fn run(
    face: &Face,
    report: &mut FaceQueryReport,
    cancelled: &mut dyn FnMut() -> bool,
) -> Result<(), FaceQueryIncomplete> {
    if cancelled() {
        return Err(FaceQueryIncomplete::Cancelled);
    }
    if report.policy.version != 1 {
        return Err(FaceQueryIncomplete::Version(report.policy.version));
    }
    report.input = Some(capture(face, report.policy, cancelled)?);
    prove(report, cancelled)
}
fn capture(
    face: &Face,
    policy: FaceQueryPolicy,
    cancelled: &mut dyn FnMut() -> bool,
) -> Result<martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot, FaceQueryIncomplete> {
    capture_face_input_with_behavior(
        face,
        policy.capture,
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        cancelled,
    )
    .map_err(FaceQueryIncomplete::Capture)
}
fn prove(
    report: &mut FaceQueryReport,
    cancelled: &mut dyn FnMut() -> bool,
) -> Result<(), FaceQueryIncomplete> {
    let input = report.input.as_ref().expect("successful owned capture");
    let (owner, surface) = source::bind(input, report.request)?;
    report.surface_owner = Some(owner);
    let numeric = extract::extract(surface, owner)?;
    let proof = certify_surface_query(numeric, report.policy.proof, cancelled);
    let incomplete = proof.incomplete().cloned();
    report.proof = Some(proof);
    match incomplete {
        Some(cause) => Err(FaceQueryIncomplete::Proof { owner, cause }),
        None => Ok(()),
    }
}
#[cfg(test)]
mod tests;
