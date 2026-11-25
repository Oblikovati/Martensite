// SPDX-License-Identifier: GPL-2.0-only
//! The original program refusal and global accounting survive authored-source projection.
use super::*;
use crate::sketch_region_prism::ProgramGraphFailure;
/// Observed graph-program work and its original typed numerical cause.
/// Example: `failure.integral_program_failure().map(|proof| proof.integration_subintervals())`.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarIntegralFailure {
    pub(crate) requested: PlanarProofVersion,
    pub(crate) entry_allowance: u64,
    pub(crate) source: Option<ProfileCurveId>,
    pub(crate) failure: Option<Box<ProgramGraphFailure>>,
    pub(crate) options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
    pub(crate) consumed: u64,
}
impl PlanarIntegralFailure {
    /// Actual examined cells including the failing program; `proof.integration_subintervals()`.
    pub fn integration_subintervals(&self) -> u64 {
        self.consumed
    }
    /// Original selected policy; `proof.requested_proof_version()`.
    pub fn requested_proof_version(&self) -> PlanarProofVersion {
        self.requested
    }
    /// Finite caller allowance; `proof.entry_allowance()`.
    pub fn entry_allowance(&self) -> u64 {
        self.entry_allowance
    }
    /// Actual graph grant; `proof.granted_subintervals()`.
    pub fn granted_subintervals(&self) -> u64 {
        self.options.max_total_subintervals
    }
    /// Authored source of the numerical occurrence; `proof.offending_curve()`.
    pub fn offending_curve(&self) -> Option<ProfileCurveId> {
        self.source
    }
    /// Original typed Math error, when the refusal arose inside integration.
    /// Example: `proof.numerical_cause().map(|cause| &cause.cause)`.
    pub fn numerical_cause(
        &self,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramError> {
        self.failure.as_ref()?.numerical_cause()
    }
}
impl std::fmt::Display for PlanarIntegralFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "integral graph requested={:?}, entry={}, grant={}, consumed={}, source={:?}: {:?}",
            self.requested,
            self.entry_allowance,
            self.granted_subintervals(),
            self.integration_subintervals(),
            self.source,
            self.failure
        )
    }
}
impl std::error::Error for PlanarIntegralFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.numerical_cause().map(|cause| cause as _)
    }
}

/// The integration-budget entrypoint supports only explicit V5.
/// Example: `failure.integral_policy_failure().map(|cause| cause.actual())`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanarIntegralPolicyFailure {
    pub(crate) actual: PlanarProofVersion,
    pub(crate) allowance: u64,
    pub(crate) invalid: Option<(&'static str, u64, u64, u64)>,
}
impl PlanarIntegralPolicyFailure {
    /// Rejected scalar and inclusive admission bounds, if the V5 policy itself is invalid.
    /// Example: `cause.invalid_scalar().map(|(field, value, _, _)| (field, value))`.
    pub fn invalid_scalar(&self) -> Option<(&'static str, u64, u64, u64)> {
        self.invalid
    }
    /// Actual rejected selector; `cause.actual()`.
    pub fn actual(&self) -> PlanarProofVersion {
        self.actual
    }
    /// Required owned graph behavior; `cause.expected_request_version()`.
    pub fn expected_request_version(&self) -> u32 {
        5
    }
    /// Supplied finite caller allowance; `cause.entry_allowance()`.
    pub fn entry_allowance(&self) -> u64 {
        self.allowance
    }
    /// Specific refusal class; `cause.class()`.
    pub fn class(&self) -> DeclineClass {
        if self.invalid.is_some() {
            DeclineClass::InvalidInput
        } else {
            DeclineClass::Unsupported
        }
    }
}
impl std::fmt::Display for PlanarIntegralPolicyFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((field, actual, minimum, maximum)) = self.invalid {
            return write!(
                f,
                "BuildPlanarRegions Admission: {field}={actual}; expected {minimum}..={maximum}, requested {:?}, entry allowance {}",
                self.actual, self.allowance
            );
        }
        write!(
            f,
            "BuildPlanarRegions Admission: integration allowance {} with {:?}; expected FilteredCompositeBernsteinV5 (owned graph version 5)",
            self.allowance, self.actual
        )
    }
}
impl std::error::Error for PlanarIntegralPolicyFailure {}
