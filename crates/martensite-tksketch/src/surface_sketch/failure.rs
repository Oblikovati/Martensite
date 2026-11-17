//! Failures retain stage, region identity and a typed cause; partial bodies never escape.
use super::request::SurfaceRegionId;
use martensite_tkbo::body_check::ValidationReport;
use std::fmt;

/// Stage that refused publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceStage {
    /// Invalid or numerically unresolved input.
    Admission,
    /// Native radial section or revolution construction.
    Construction,
    /// Incomplete source-to-result correspondence.
    History,
    /// Native immutable encoding or detached decoding.
    Encoding,
    /// Ordered body or semantic geometry check.
    Validation,
    /// Independent volume integration and comparison.
    Volume,
}

/// Reason a normal-extrusion operation did not complete.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceFailureCause {
    /// Input value outside its stated domain.
    InvalidInput {
        /// Offending input field.
        field: &'static str,
        /// Offending numeric value or identity.
        value: f64,
        /// Required domain or independent expected volume.
        expected: &'static str,
    },
    /// Caller identities must be distinct within each entity kind.
    DuplicateIdentity {
        /// Entity kind containing the duplicate.
        kind: &'static str,
        /// Repeated caller identity.
        value: u64,
    },
    /// A native operation failed, preserving its diagnostic.
    Native {
        /// Native operation or missing invariant.
        context: &'static str,
        /// Native diagnostic.
        message: String,
    },
    /// Required topology or semantic relation is absent or inconsistent.
    Incomplete {
        /// Native operation or missing invariant.
        context: &'static str,
        /// Observed value or missing correspondence.
        actual: String,
    },
    /// At least one required ordered body check did not pass.
    BodyValidation(Box<ValidationReport>),
    /// Measured tolerance exceeded an explicit refusing cap.
    ToleranceExceeded {
        /// Largest native tolerance.
        achieved: f64,
        /// Caller-selected refusing cap.
        limit: f64,
    },
    /// Native integration did not provide a usable completed estimate.
    IntegrationRejected {
        /// Native signed volume, before acceptance.
        volume: f64,
        /// Native error or negative failure status.
        status: f64,
        /// Relative estimate after the native unit conversion.
        relative: f64,
        /// Requested maximum relative error.
        requested: f64,
    },
    /// Volume or its uncertainty disagrees with the operation contract.
    VolumeMismatch {
        /// Observed value or missing correspondence.
        actual: f64,
        /// Required domain or independent expected volume.
        expected: f64,
        /// Estimated absolute uncertainty.
        error: f64,
        /// Maximum accepted error.
        budget: f64,
    },
}

/// Typed unsuccessful operation; no result body accompanies it.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceSketchFailure {
    /// Region involved in the operation.
    pub region: SurfaceRegionId,
    /// Failed stage.
    pub stage: SurfaceStage,
    /// Native, numeric or validation cause.
    pub cause: SurfaceFailureCause,
}
impl fmt::Display for SurfaceSketchFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "normal extrusion {:?} at {:?}: {:?}",
            self.region, self.stage, self.cause
        )
    }
}
impl std::error::Error for SurfaceSketchFailure {}

pub(super) type Checked<T> = Result<T, SurfaceFailureCause>;
pub(super) fn native(
    context: &'static str,
    error: martensite_tkernel::standard::Failure,
) -> SurfaceFailureCause {
    SurfaceFailureCause::Native {
        context,
        message: error.to_string(),
    }
}
pub(super) fn incomplete(context: &'static str, actual: impl Into<String>) -> SurfaceFailureCause {
    SurfaceFailureCause::Incomplete {
        context,
        actual: actual.into(),
    }
}
