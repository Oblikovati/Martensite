// SPDX-License-Identifier: GPL-2.0-only
use super::super::{BoundedArithmeticFailure, CylindricalProfileFailure};
use crate::predicates::roots::{DyadicBound, cartesian_equation::CartesianEquationError};
use std::{error::Error, fmt};

pub(super) type Checked<T> = Result<T, AngularEquationError>;

/// A coordinate singularity of an angular equation chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AngularChartQuantity {
    /// Radius must stay strictly positive.
    Radius,
    /// Spherical inclination must stay strictly between zero and exact pi.
    Inclination,
}

/// A typed input, proof, evaluation or fitting refusal.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AngularEquationError {
    /// Invalid typed polynomial law or parameter interval.
    Law(CartesianEquationError),
    /// Requested parameter lies outside the closed interval.
    Parameter {
        /// Requested value.
        parameter: f64,
        /// Accepted authored interval.
        domain: [f64; 2],
    },
    /// A closed parameter interval includes a witnessed chart violation.
    ChartViolation(AngularChartQuantity),
    /// The chart could not be proved strict within the subdivision limit.
    ChartUnproven(AngularChartQuantity),
    /// The Cartesian derivative is known to vanish.
    Singular,
    /// A positive Cartesian speed could not be proved within the fixed subdivision depth.
    RegularityUnproven,
    /// Outward fixed-grid work failed before a certified result.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// The original argument could not be reduced to the supported Taylor chart.
    ArgumentReduction {
        /// Reduced argument lower bound.
        lower: DyadicBound,
        /// Reduced argument upper bound.
        upper: DyadicBound,
    },
    /// The reused bounded trig entry point refused the request.
    Trigonometry(Box<CylindricalProfileFailure>),
    /// Requested model-space distance must be finite and nonnegative.
    DistanceLimit(f64),
    /// No certified fit met the requested distance under the fixed fitting policy.
    FitLimit {
        /// Maximum dyadic subdivision level.
        depth: u8,
        /// Remaining certified error bound at the unsuccessful span.
        bound: f64,
        /// Requested inclusive spatial distance.
        limit: f64,
    },
    /// Actual rounded native controls failed their independent finite regularity proof.
    RoundedCarrier(CartesianEquationError),
    /// A candidate or final proof was cancelled before publication.
    Cancelled,
}

impl fmt::Display for AngularEquationError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Law(cause) => cause.fmt(output),
            Self::Parameter { parameter, domain } => {
                write!(output, "parameter {parameter} must lie in {domain:?}")
            }
            Self::ChartViolation(quantity) => write!(
                output,
                "the closed parameter interval violates the open {quantity:?} chart"
            ),
            Self::ChartUnproven(quantity) => write!(
                output,
                "the bounded proof could not establish the open {quantity:?} chart"
            ),
            Self::Singular => output.write_str("the angular equation derivative vanishes"),
            Self::RegularityUnproven => output.write_str(
                "the bounded proof could not establish a positive angular equation speed",
            ),
            Self::Arithmetic(cause) => write!(output, "bounded angular arithmetic: {cause:?}"),
            Self::ArgumentReduction { lower, upper } => write!(
                output,
                "reduced angle [{lower}, {upper}] exceeds the bounded Taylor chart"
            ),
            Self::Trigonometry(cause) => cause.fmt(output),
            Self::DistanceLimit(limit) => write!(
                output,
                "distance limit {limit} must be finite and nonnegative"
            ),
            Self::FitLimit {
                depth,
                bound,
                limit,
            } => write!(
                output,
                "fitting level {depth} achieved bound {bound}, expected at most {limit}"
            ),
            Self::RoundedCarrier(cause) => write!(output, "rounded angular carrier: {cause}"),
            Self::Cancelled => output.write_str("angular equation was cancelled"),
        }
    }
}

impl Error for AngularEquationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Law(cause) | Self::RoundedCarrier(cause) => Some(cause),
            Self::Trigonometry(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<CartesianEquationError> for AngularEquationError {
    fn from(cause: CartesianEquationError) -> Self {
        match cause {
            CartesianEquationError::Cancelled => Self::Cancelled,
            cause => Self::Law(cause),
        }
    }
}
impl From<BoundedArithmeticFailure> for AngularEquationError {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        match cause {
            BoundedArithmeticFailure::Cancelled => Self::Cancelled,
            cause => Self::Arithmetic(Box::new(cause)),
        }
    }
}
impl From<CylindricalProfileFailure> for AngularEquationError {
    fn from(cause: CylindricalProfileFailure) -> Self {
        match cause {
            CylindricalProfileFailure::Arithmetic(cause) => (*cause).into(),
            CylindricalProfileFailure::TrigonometricRange { lower, upper } => {
                Self::ArgumentReduction { lower, upper }
            }
            cause => Self::Trigonometry(Box::new(cause)),
        }
    }
}

pub(super) fn check_cancelled(cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    if cancelled() {
        return Err(AngularEquationError::Cancelled);
    }
    Ok(())
}
