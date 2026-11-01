use super::failure::CaseFailure;

pub(crate) const FACTOR_ENV: &str = "CSF_CPULIMIT_FACTOR";
pub(crate) const CHILD_ENV: &str = "MARTENSITE_EXACT_CASE_CHILD_V1";
pub(crate) const HARNESS_PROFILE: &str = "rust-1.99.0-pretty-show-output";

/// An explicitly supported execution environment, not an expression parser.
///
/// Example: `let profile = martensite_process_policy::case_runner::FactorProfile::RequireUnset;`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactorProfile {
    /// Reject every present factor, including an empty value.
    RequireUnset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedProcessPolicy {
    Seconds(i64),
    Inherit,
}

impl RequestedProcessPolicy {
    pub(crate) fn description(self) -> String {
        match self {
            Self::Seconds(seconds) => format!("{seconds} seconds"),
            Self::Inherit => "inherited process policy (no runner installation)".into(),
        }
    }
    fn validate(self) -> Result<(), CaseFailure> {
        if let Self::Seconds(seconds) = self
            && seconds <= 0
        {
            return Err(CaseFailure::InvalidSpec {
                observed: seconds.to_string(),
                expected: "positive already evaluated CPU/elapsed seconds",
            });
        }
        Ok(())
    }
}

/// The exact libtest identity and explicitly requested process policy.
///
/// Example: `CaseSpec::new("curve_cases::bounded", 600, FactorProfile::RequireUnset)`.
/// ```
/// use martensite_process_policy::case_runner::{CaseSpec, FactorProfile};
/// let case = CaseSpec::new("curve_cases::bounded", 600, FactorProfile::RequireUnset);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaseSpec<'a> {
    pub(crate) identity: &'a str,
    pub(crate) process_policy: RequestedProcessPolicy,
    pub(crate) factor: FactorProfile,
}

impl<'a> CaseSpec<'a> {
    /// Declare a case; execution validates values before any child or policy starts.
    ///
    /// Example: `CaseSpec::new("cases::point", 60, FactorProfile::RequireUnset)`.
    pub fn new(identity: &'a str, seconds: i64, factor: FactorProfile) -> Self {
        Self {
            identity,
            process_policy: RequestedProcessPolicy::Seconds(seconds),
            factor,
        }
    }

    /// Preserve the dedicated child's inherited policy without starting a watchdog (#351).
    ///
    /// Example: `CaseSpec::inherit_process_policy("cases::inherited", FactorProfile::RequireUnset)`.
    pub fn inherit_process_policy(identity: &'a str, factor: FactorProfile) -> Self {
        Self {
            identity,
            process_policy: RequestedProcessPolicy::Inherit,
            factor,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), CaseFailure> {
        if !valid_identity(self.identity) {
            return Err(CaseFailure::InvalidSpec {
                observed: self.identity.into(),
                expected: "nonempty exact identity without whitespace/control characters or option prefix",
            });
        }
        self.process_policy.validate()
    }
}

fn valid_identity(identity: &str) -> bool {
    !identity.is_empty()
        && !identity.starts_with('-')
        && !identity
            .chars()
            .any(|c| c.is_control() || c.is_whitespace())
}
