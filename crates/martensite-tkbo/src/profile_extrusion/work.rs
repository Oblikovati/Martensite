// SPDX-License-Identifier: GPL-2.0-only
//! Explicit computational work is independent of geometric accuracy.
use super::{OperationDecline, PlanarProofWorkReceipt, PlanarRegionSet, ProfileCurveId};

/// Select the exact planar proof behavior while retaining earlier replay semantics.
/// Example: `PlanarProofVersion::BernsteinV3` enables source-bound exact exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlanarProofVersion {
    /// Released common-carrier and pair solver behavior.
    CarrierV2,
    /// Exact Bernstein exclusion followed by that solver under the remaining budget.
    BernsteinV3,
    /// Bernstein exclusions with exact bracket filtering for generated membership signs.
    FilteredBernsteinV4,
    /// Filtered Bernstein graph proof with bounded whole-orbit and face integral programs.
    FilteredCompositeBernsteinV5 {
        /// Maximum components in each complete signed-boundary or face program.
        max_components: u16,
        /// Graph-wide examined integration cells shared by every program.
        max_total_subintervals: u64,
    },
    /// Exact geometric pair leaves with the same bounded integral programs and solver fallback.
    GeometricCompositeV6 {
        /// Maximum components in each complete signed-boundary or face program.
        max_components: u16,
        /// Graph-wide examined integration cells shared by every program.
        max_total_subintervals: u64,
    },
}
impl PlanarProofVersion {
    pub(crate) fn integral_options(
        self,
    ) -> Option<martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions> {
        use martensite_tkmath::predicates::roots::metric_bounds::*;
        let (Self::FilteredCompositeBernsteinV5 {
            max_components,
            max_total_subintervals,
        }
        | Self::GeometricCompositeV6 {
            max_components,
            max_total_subintervals,
        }) = self
        else {
            return None;
        };
        Some(IntegralProgramOptions {
            version: IntegralProgramVersion::V1,
            integration: RationalIntegralOptions::default(),
            max_components,
            max_total_subintervals,
        })
    }
    pub(crate) fn with_integration_limit(self, maximum: u64) -> Self {
        match self {
            Self::FilteredCompositeBernsteinV5 { max_components, .. } => {
                Self::FilteredCompositeBernsteinV5 {
                    max_components,
                    max_total_subintervals: maximum,
                }
            }
            Self::GeometricCompositeV6 { max_components, .. } => Self::GeometricCompositeV6 {
                max_components,
                max_total_subintervals: maximum,
            },
            _ => self,
        }
    }
    pub(crate) fn request_version(self) -> u32 {
        match self {
            Self::CarrierV2 => 2,
            Self::BernsteinV3 => 3,
            Self::FilteredBernsteinV4 => 4,
            Self::FilteredCompositeBernsteinV5 { .. } => 5,
            Self::GeometricCompositeV6 { .. } => 6,
        }
    }
}

/// Original root-isolation refusal and its authored parent curves.
/// Example: `failure.pair_root_failure().map(|pair| pair.curves)`.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPairRootFailure {
    /// Authored parents; multiple proof pieces may belong to the same curve.
    pub curves: [ProfileCurveId; 2],
    /// Actual retained numerical cause, never inferred from spent work.
    pub cause: martensite_tkmath::predicates::roots::RootIsolationError,
}

/// Finite shared pair-work choice; zero permits only completion without subdivision.
/// Example: `PlanarProofWorkLimit::PairBisections(8192)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarProofWorkLimit {
    /// Retain the existing graph policy; the observed receipt records its numeric limit.
    CurrentDefault,
    /// Apply exactly this shared bisection limit without changing other proof settings.
    PairBisections(u16),
}
impl PlanarProofWorkLimit {
    pub(crate) fn override_limit(self) -> Option<u16> {
        match self {
            Self::CurrentDefault => None,
            Self::PairBisections(maximum) => Some(maximum),
        }
    }
}
/// Owned typed decline and work evidence, with no partial geometry.
#[derive(Clone, PartialEq)]
pub struct PlanarRegionWorkFailure {
    decline: OperationDecline,
    requested: PlanarProofWorkLimit,
    observed: Option<PlanarProofWorkReceipt>,
    root_failure: Option<Box<PlanarPairRootFailure>>,
    cancelled: bool,
    pub(super) integral: Option<Box<super::PlanarIntegralFailure>>,
    policy: Option<Box<super::PlanarIntegralPolicyFailure>>,
}
impl PlanarRegionWorkFailure {
    /// Specific unsupported-selector diagnostic; the legacy decline remains coarse.
    /// Example: `failure.integral_policy_failure().map(|cause| cause.class())`.
    pub fn integral_policy_failure(&self) -> Option<&super::PlanarIntegralPolicyFailure> {
        self.policy.as_deref()
    }
    pub(crate) fn with_integral_policy(
        mut self,
        actual: PlanarProofVersion,
        allowance: u64,
    ) -> Self {
        self.policy = Some(Box::new(super::PlanarIntegralPolicyFailure {
            actual,
            allowance,
            invalid: None,
        }));
        self
    }

    /// Actual program refusal and graph cells; `failure.integral_program_failure()`.
    pub fn integral_program_failure(&self) -> Option<&super::PlanarIntegralFailure> {
        self.integral.as_deref()
    }
    pub(crate) fn with_integral_failure(
        mut self,
        integral: Option<super::PlanarIntegralFailure>,
    ) -> Self {
        self.integral = integral.map(Box::new);
        self
    }

    /// Existing operation, stage, entities and cause; `failure.decline().class()`.
    pub fn decline(&self) -> &OperationDecline {
        &self.decline
    }
    /// Original work choice; `failure.requested_work()`.
    pub fn requested_work(&self) -> PlanarProofWorkLimit {
        self.requested
    }
    /// Actual retained work, when observed; `failure.proof_work().is_some()`.
    /// Absence does not assert zero consumed work.
    pub fn proof_work(&self) -> Option<PlanarProofWorkReceipt> {
        self.observed
    }
    /// Original pair root refusal when present; `failure.pair_root_failure()`.
    pub fn pair_root_failure(&self) -> Option<&PlanarPairRootFailure> {
        self.root_failure.as_deref()
    }
    /// Whether the caller cancelled the operation; `failure.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    pub(crate) fn with_invalid_integral_policy(
        mut self,
        actual: PlanarProofVersion,
        allowance: u64,
        invalid: (&'static str, u64, u64, u64),
    ) -> Self {
        self.policy = Some(Box::new(super::PlanarIntegralPolicyFailure {
            actual,
            allowance,
            invalid: Some(invalid),
        }));
        self
    }
    pub(crate) fn with_completed_integral(
        mut self,
        requested: PlanarProofVersion,
        entry_allowance: u64,
        options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
        consumed: u64,
    ) -> Self {
        self.integral = Some(Box::new(super::PlanarIntegralFailure {
            requested,
            entry_allowance,
            options,
            consumed,
            source: None,
            failure: None,
        }));
        self
    }
    pub(crate) fn with_cancelled(mut self, cancelled: bool) -> Self {
        self.cancelled = cancelled;
        self
    }
    pub(crate) fn with_root_failure(mut self, root_failure: Option<PlanarPairRootFailure>) -> Self {
        self.root_failure = root_failure.map(Box::new);
        self
    }
    pub(crate) fn new(
        requested: PlanarProofWorkLimit,
        decline: OperationDecline,
        observed: Option<PlanarProofWorkReceipt>,
    ) -> Self {
        Self {
            decline,
            requested,
            observed,
            root_failure: None,
            cancelled: false,
            integral: None,
            policy: None,
        }
    }
    pub(crate) fn into_decline(self) -> OperationDecline {
        self.decline
    }
}
impl std::fmt::Display for PlanarRegionWorkFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; pair work requested={:?}, observed={:?}",
            self.decline, self.requested, self.observed
        )?;
        if let Some(policy) = &self.policy {
            write!(f, "; {policy}")?;
        }
        if let Some(integral) = &self.integral {
            write!(f, "; {integral}")?;
        }
        Ok(())
    }
}
impl std::error::Error for PlanarRegionWorkFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(policy) = &self.policy {
            return Some(policy.as_ref());
        }
        self.integral
            .as_deref()
            .map(|cause| cause as &(dyn std::error::Error + 'static))
            .or(Some(&self.decline))
    }
}
/// Complete planar regions or a typed work-bearing failure.
pub type PlanarRegionWorkResult = Result<PlanarRegionSet, PlanarRegionWorkFailure>;

impl std::fmt::Debug for PlanarRegionWorkFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = f.debug_struct("PlanarRegionWorkFailure");
        output
            .field("decline", &self.decline)
            .field("requested", &self.requested)
            .field("observed", &self.observed)
            .field("root_failure", &self.root_failure)
            .field("cancelled", &self.cancelled);
        if let Some(integral) = &self.integral {
            output.field("integral", integral);
        }
        if let Some(policy) = &self.policy {
            output.field("integral_policy", policy);
        }
        output.finish()
    }
}
