// SPDX-License-Identifier: GPL-2.0-only
//! Explicit bounded composition of open-chain Green integrals; geometry is caller-proved.
mod admission;
mod binding;
mod error;
mod run;
mod types;
use super::*;
pub use error::*;
pub use types::*;

/// Integrate ordered open components with one error/work policy; no closure is asserted.
/// Example: `certify_boundary_integral_program(query, || false)?`.
pub fn certify_boundary_integral_program(
    query: BoundaryIntegralProgramQuery<'_>,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    let loops = [IntegralProgramLoop {
        components: query.components,
    }];
    run::certify(&loops, query.options, false, &mut cancelled)
}

/// Integrate the outer loop magnitude minus whole hole-loop magnitudes, rounding once.
/// Jordan, nesting and disjointness are caller obligations, not inferred from components.
/// Example: `certify_face_integral_program(query, || false)?`.
pub fn certify_face_integral_program(
    query: FaceIntegralProgramQuery<'_>,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    run::certify(query.loops, query.options, true, &mut cancelled)
}

#[cfg(test)]
mod tests;
