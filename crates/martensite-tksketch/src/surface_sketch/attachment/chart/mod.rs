//! Native hit parameters map analytically; trim search is separate from coordinate evidence.
use super::*;
use martensite_tkbo::{body_check::CheckMode, measured::Measured};
use martensite_tkg3d::top_abs::State;
mod arithmetic;
mod mapping;
pub(super) use mapping::classify;
#[cfg(test)]
mod tests;

/// Caller-owned chart for repeated reads of one cylindrical support revision and seam.
pub struct CylinderAuthoringChart {
    attempt: Attempt,
    face: Face,
    radius: f64,
    seam_angle: f64,
    tolerance: f64,
}

/// Authoring policy for parameters outside the selected face's trims.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutOfSupportPolicy {
    /// Refuse points classified outside the selected face.
    Refuse,
    /// Return coordinates and the outside state for application feedback.
    Report,
}
/// Native surface parameters and an explicit developed-chart seam.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderParameterRequest {
    /// Unwrapped native `(u,v)` from the selected face's intersector.
    pub uv: [f64; 2],
    /// Native angular parameter assigned developed arc length zero.
    pub seam_angle: f64,
    /// Treatment of parameters outside the selected face's trims.
    pub outside: OutOfSupportPolicy,
}
/// Coordinates for authoring; this does not certify a wrapped modelling operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DevelopedCylinderHit {
    /// Original unwrapped surface parameters, without automatic periodic reduction.
    pub uv: [f64; 2],
    /// `(radius * (u - seam_angle), v)` in the ideal analytic native chart.
    /// Bounds cover arithmetic on supplied binary64 parameters and the native radius;
    /// they exclude intersector uncertainty and stored-frame metric deviation.
    pub coordinates: [Measured; 2],
    /// Actual native trim classification, kept separate from arithmetic evidence.
    pub trim_state: State,
    /// Trim classification coverage; currently `NativeSearch`.
    pub trim_check: CheckMode,
}
impl CylinderFaceAttachment {
    /// Open an independent authoring chart at the chosen unwrapped native seam.
    /// Example: `let chart = attachment.authoring_chart(revision, seam_angle)?;`.
    pub fn authoring_chart(
        &self,
        revision: SupportRevision,
        seam_angle: f64,
    ) -> Checked<CylinderAuthoringChart> {
        mapping::open_chart(self, revision, seam_angle)
    }

    /// Map a native hit's UV into the developed chart of this revision's selected face.
    /// Example: `attachment.develop_parameters(revision, request)?`.
    /// Callers supply parameters from this face; this method does not project world points.
    pub fn develop_parameters(
        &self,
        revision: SupportRevision,
        request: CylinderParameterRequest,
    ) -> Checked<DevelopedCylinderHit> {
        mapping::apply(self, revision, request)
    }
}

impl CylinderAuthoringChart {
    /// Map a native hit without decoding its support again.
    /// Example: `chart.develop_parameters(revision, uv, OutOfSupportPolicy::Refuse)?`.
    pub fn develop_parameters(
        &self,
        revision: SupportRevision,
        uv: [f64; 2],
        outside: OutOfSupportPolicy,
    ) -> Checked<DevelopedCylinderHit> {
        mapping::apply_chart(self, revision, uv, outside)
    }
}
