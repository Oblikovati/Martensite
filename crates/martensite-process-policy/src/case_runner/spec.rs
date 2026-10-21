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

/// The exact libtest identity and already evaluated positive budget.
///
/// Example: `CaseSpec::new("curve_cases::bounded", 600, FactorProfile::RequireUnset)`.
/// ```
/// use martensite_process_policy::case_runner::{CaseSpec, FactorProfile};
/// let case = CaseSpec::new("curve_cases::bounded", 600, FactorProfile::RequireUnset);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaseSpec<'a> {
    pub(crate) identity: &'a str,
    pub(crate) seconds: i64,
    pub(crate) factor: FactorProfile,
}

impl<'a> CaseSpec<'a> {
    /// Declare a case; execution validates values before any child or policy starts.
    ///
    /// Example: `CaseSpec::new("cases::point", 60, FactorProfile::RequireUnset)`.
    pub fn new(identity: &'a str, seconds: i64, factor: FactorProfile) -> Self {
        Self {
            identity,
            seconds,
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
        if self.seconds <= 0 {
            return Err(CaseFailure::InvalidSpec {
                observed: self.seconds.to_string(),
                expected: "positive already evaluated CPU/elapsed seconds",
            });
        }
        Ok(())
    }
}

fn valid_identity(identity: &str) -> bool {
    !identity.is_empty()
        && !identity.starts_with('-')
        && !identity
            .chars()
            .any(|c| c.is_control() || c.is_whitespace())
}
