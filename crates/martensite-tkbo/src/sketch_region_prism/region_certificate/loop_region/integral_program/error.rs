// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::predicates::roots::{
    RootIsolationError,
    metric_bounds::{
        IntegralProgramError, IntegralProgramFailure, RationalIntegralFailure, ResidualError,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum IntegralLedgerError {
    Math(Box<IntegralProgramError>),
    Accounting {
        grant: u64,
        consumed: u64,
        reported: u64,
    },
}
impl IntegralLedgerError {
    pub(crate) fn is_cancelled(&self) -> bool {
        match self {
            Self::Math(error) => matches!(&error.cause, IntegralProgramFailure::Numerical(cause)
                if matches!(cause.as_ref(), RationalIntegralFailure::Metric(metric)
                    if matches!(metric.as_ref(), ResidualError::Roots(RootIsolationError::Cancelled)))),
            Self::Accounting { .. } => false,
        }
    }
    pub(crate) fn reported_subintervals(&self) -> u64 {
        match self {
            Self::Math(error) => error.integration_subintervals,
            Self::Accounting { reported, .. } => *reported,
        }
    }
}
impl std::fmt::Display for IntegralLedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Math(error) => error.fmt(f),
            Self::Accounting {
                grant,
                consumed,
                reported,
            } => write!(
                f,
                "integral grant {grant}, consumed {consumed}, reported {reported}; expected checked consumed+reported <= grant"
            ),
        }
    }
}
impl std::error::Error for IntegralLedgerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Math(error) => Some(error.as_ref()),
            Self::Accounting { .. } => None,
        }
    }
}
