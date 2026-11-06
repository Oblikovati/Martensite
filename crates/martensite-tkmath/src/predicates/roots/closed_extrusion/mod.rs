//! Exact sufficient self-exclusion for closed quadratic spline extrusion supports.
//! The certificate is on a declared endpoint seam quotient, not on trims or mutable handles.
mod budget;
mod input;
pub(super) mod input_view;
pub(super) mod operation;
mod report;
mod self_span;
mod types;
pub(super) mod validation;
pub use input::ClosedQuadraticExtrusionInput;
pub use report::{ClosedExtrusionCertificate, ClosedExtrusionReport};
pub use types::{
    ClosedExtrusionCause, ClosedExtrusionCompletion, ClosedExtrusionInputError,
    ClosedExtrusionPolicy, ClosedExtrusionProjectionContact, ClosedExtrusionStage,
    ClosedExtrusionWork,
};

/// Certify complete exact hull/monotonicity coverage; `certify_closed_quadratic_extrusion(input, ClosedExtrusionPolicy::bounded(), || false)`.
/// Unproved hull pairs remain incomplete; this operation does not use sampled acceptance.
pub fn certify_closed_quadratic_extrusion(
    input: ClosedQuadraticExtrusionInput,
    policy: ClosedExtrusionPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> ClosedExtrusionReport {
    operation::execute(input, policy, &mut cancelled)
}

#[cfg(test)]
mod tests;
