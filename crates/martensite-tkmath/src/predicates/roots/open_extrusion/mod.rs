//! Exact sufficient injectivity on a finite curve interval with no endpoint identification.
//! A support certificate does not validate face trims, boundary incidences or mutable handles.
mod input;
mod policy;
mod report;
mod types;
pub use super::closed_extrusion::{
    ClosedExtrusionCause as OpenExtrusionCause,
    ClosedExtrusionProjectionContact as OpenExtrusionProjectionContact,
    ClosedExtrusionStage as OpenExtrusionStage, ClosedExtrusionWork as OpenExtrusionWork,
};
pub use input::{OpenExtrusionInputError, OpenQuadraticExtrusionInput};
pub use policy::OpenExtrusionPolicy;
pub use report::{OpenExtrusionCertificate, OpenExtrusionReport};
pub use types::OpenExtrusionCompletion;

/// Prove the entire supplied interval without a cyclic seam; `certify_open_quadratic_extrusion(input, OpenExtrusionPolicy::bounded(), || false)`.
/// Every unseparated product remains incomplete, including numerically coincident endpoints.
pub fn certify_open_quadratic_extrusion(
    input: OpenQuadraticExtrusionInput,
    policy: OpenExtrusionPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> OpenExtrusionReport {
    let proof = super::closed_extrusion::operation::traverse(
        &input.proof_view(),
        policy.proof_policy(),
        &mut cancelled,
    );
    OpenExtrusionReport::from_proof(input, policy, proof)
}

#[cfg(test)]
mod tests;
