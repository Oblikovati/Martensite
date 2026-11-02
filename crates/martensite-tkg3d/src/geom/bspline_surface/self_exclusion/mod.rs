//! Coherent numeric admission of a clamped, single-span bilinear support.
mod support;
mod types;
use super::{GeomBSplineSurface, SurfaceState, cell};
use martensite_tkmath::predicates::bilinear_patch::{
    BilinearExclusion, BilinearPatchInput, certify_bilinear_patch,
};
pub use types::{
    BilinearSplineCertificate, BilinearSplineError, BilinearSplineExclusion,
    BilinearSplineInconclusive, BilinearSplineLayout,
};

enum SplineAdmission {
    Supported(BilinearPatchInput, BilinearSplineLayout),
    Inconclusive(BilinearSplineInconclusive),
}
impl GeomBSplineSurface {
    /// Certify an owned support snapshot; `surface.bilinear_self_exclusion()?`.
    /// A later mutation of the surface is not covered by the returned certificate.
    pub fn bilinear_self_exclusion(&self) -> Result<BilinearSplineExclusion, BilinearSplineError> {
        Ok(match self.bilinear_support_snapshot()? {
            SplineAdmission::Supported(input, layout) => conclusion(input, layout),
            SplineAdmission::Inconclusive(reason) => BilinearSplineExclusion::Inconclusive(reason),
        })
    }
    fn bilinear_support_snapshot(&self) -> Result<SplineAdmission, BilinearSplineError> {
        let state = self.state.read();
        let layout = support::layout(&state);
        if layout.evaluation_override {
            return Ok(SplineAdmission::Inconclusive(
                BilinearSplineInconclusive::EvaluationOverride(layout),
            ));
        }
        support::validate_stored_values(&state)?;
        if !support::supported(&layout) {
            return Ok(SplineAdmission::Inconclusive(
                BilinearSplineInconclusive::UnsupportedLayout(layout),
            ));
        }
        Ok(SplineAdmission::Supported(copied_patch(&state)?, layout))
    }
}

fn conclusion(input: BilinearPatchInput, layout: BilinearSplineLayout) -> BilinearSplineExclusion {
    match certify_bilinear_patch(input) {
        BilinearExclusion::Certified(polynomial) => {
            BilinearSplineExclusion::Certified(BilinearSplineCertificate { polynomial, layout })
        }
        BilinearExclusion::Inconclusive { input, reason } => {
            BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::Patch {
                input,
                reason,
            })
        }
    }
}
fn copied_patch(state: &SurfaceState) -> Result<BilinearPatchInput, BilinearSplineError> {
    let mut corners = [[0.; 3]; 4];
    let mut weights = [0.; 4];
    for (slot, (u, v)) in [(1, 1), (2, 1), (2, 2), (1, 2)].into_iter().enumerate() {
        let point = cell(&state.poles, u, v).map_err(BilinearSplineError::Storage)?;
        corners[slot] = [point.x(), point.y(), point.z()];
        weights[slot] = cell(&state.weights, u, v).map_err(BilinearSplineError::Storage)?;
    }
    let domain = [support::interval(&state.u)?, support::interval(&state.v)?];
    BilinearPatchInput::new(corners, domain, weights).map_err(BilinearSplineError::Input)
}
#[cfg(test)]
mod tests;
