//! Published values never expose native interior-mutable geometry.
use super::{PeriodicImageKey, encoding, failure::*, history::*, request::*};
use martensite_tkbo::{
    body_check::ValidationReport, measured::Measured, profile_extrusion::AchievedTolerance,
};
use martensite_tkbrep::topods::Shape;

/// Non-failing observations kept with a complete result.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceDiagnostic {
    /// Explicit report-only cap was exceeded; no refusing cap was bypassed.
    ToleranceCapReported {
        /// Largest stored native tolerance.
        achieved: f64,
        /// Caller-selected report threshold.
        limit: f64,
    },
}
/// Fresh native owners with authored bindings, for a standard normal extrusion.
pub type DetachedNormalBody = KeyedDetachedNormalBody<HistoryKey>;

/// Fresh native owners with bindings that include the periodic seam images.
pub type DetachedPeriodicBody = KeyedDetachedNormalBody<PeriodicImageKey>;

/// Fresh native owners with semantic bindings belonging to the same exported solid.
pub struct KeyedDetachedNormalBody<K> {
    /// Mutable native export, independent of every other export and the published result.
    pub solid: Shape,
    /// Explicit native references, keyed by authored parent and operation role.
    pub entities: Vec<(K, Shape)>,
}
/// Separate estimated contributions to the reported volume uncertainty.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VolumeErrorEstimates {
    /// Native quadrature estimate plus floating-point accumulation allowance.
    pub integration: f64,
    /// Forward coordinate/radius rounding estimate times analytic surface-area scale.
    pub construction: f64,
}
/// Complete immutable result of a standard normal extrusion.
pub type NormalExtrusionResult = KeyedNormalExtrusionResult<HistoryKey>;

/// Complete immutable result with authored and periodic seam bindings.
pub type PeriodicNormalExtrusionResult = KeyedNormalExtrusionResult<PeriodicImageKey>;

/// Complete immutable normal extrusion, with value-only metadata and an owned encoding.
#[derive(Clone, Debug)]
pub struct KeyedNormalExtrusionResult<K> {
    pub(super) bytes: Vec<u8>,
    pub(super) keys: Vec<K>,
    pub(super) region: SurfaceRegionId,
    pub(super) validation: ValidationReport,
    pub(super) volume: Measured,
    pub(super) volume_errors: VolumeErrorEstimates,
    pub(super) achieved: AchievedTolerance,
    pub(super) diagnostics: Vec<SurfaceDiagnostic>,
    pub(super) options: NormalExtrusionOptions,
}
impl<K: Copy> KeyedNormalExtrusionResult<K> {
    /// Ordered checks of the decoded published body; `result.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }
    /// Complete semantic keys, in total order; `result.history().len()`.
    pub fn history(&self) -> &[K] {
        &self.keys
    }
    /// Integrated native-body volume with estimated uncertainty; `result.volume().value`.
    pub fn volume(&self) -> Measured {
        self.volume
    }
    /// Estimated integration and construction contributions; `result.volume_error_estimates()`.
    pub fn volume_error_estimates(&self) -> VolumeErrorEstimates {
        self.volume_errors
    }
    /// Largest stored tolerances in the decoded body; `result.achieved_tolerance().max()`.
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.achieved
    }
    /// Report-only findings; `result.diagnostics().is_empty()`.
    pub fn diagnostics(&self) -> &[SurfaceDiagnostic] {
        &self.diagnostics
    }
    /// Requested settings used without silent convergence clamps; `result.options()`.
    pub fn options(&self) -> NormalExtrusionOptions {
        self.options
    }
    /// Effective native and requested admission limits; `result.effective_limits()`.
    pub fn effective_limits(&self) -> NormalExtrusionLimits {
        super::request::effective_limits(self.options)
    }
    /// Decode independent native owners and their semantic references; `result.export_brep()?`.
    pub fn export_brep(&self) -> Result<KeyedDetachedNormalBody<K>, SurfaceSketchFailure> {
        let (solid, entities) =
            encoding::decode(&self.bytes, &self.keys).map_err(|cause| SurfaceSketchFailure {
                region: self.region,
                stage: SurfaceStage::Encoding,
                cause,
            })?;
        Ok(KeyedDetachedNormalBody { solid, entities })
    }
}

pub(super) fn assemble<K>(
    admitted: &super::sweep::Sweep,
    bytes: Vec<u8>,
    keys: Vec<K>,
    checked: super::validation::CheckedBody,
    volume: Measured,
    volume_errors: VolumeErrorEstimates,
) -> KeyedNormalExtrusionResult<K> {
    KeyedNormalExtrusionResult {
        bytes,
        keys,
        region: admitted.region,
        validation: checked.report,
        volume,
        volume_errors,
        achieved: checked.achieved,
        diagnostics: checked.diagnostics,
        options: admitted.options,
    }
}
