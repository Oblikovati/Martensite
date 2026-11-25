// SPDX-License-Identifier: GPL-2.0-only
use std::{error::Error, fmt};

/// Failure to reduce one typed coordinate program to a supported polynomial.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EquationExpressionError {
    /// The constant operand must be finite.
    NonFiniteLiteral(f64),
    /// An operator is missing one or both operands.
    MissingOperand,
    /// The complete program must leave exactly one result on the stack.
    ResultCount(usize),
    /// Product requires an intermediate degree outside the supported range.
    Degree(usize),
    /// The first polynomial mode only divides by an exact nonzero constant.
    NonConstantDivisor,
    /// The denominator is exactly zero.
    ZeroDivisor,
}

impl fmt::Display for EquationExpressionError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        use super::MAX_CARTESIAN_EQUATION_DEGREE;
        match self {
            Self::NonFiniteLiteral(value) => write!(output, "literal {value} must be finite"),
            Self::MissingOperand => output.write_str("operator requires its full operands"),
            Self::ResultCount(count) => {
                write!(output, "program produced {count} results, expected one")
            }
            Self::Degree(degree) => write!(
                output,
                "degree {degree} exceeds {MAX_CARTESIAN_EQUATION_DEGREE}"
            ),
            Self::NonConstantDivisor => {
                output.write_str("division requires a constant denominator")
            }
            Self::ZeroDivisor => output.write_str("division requires a nonzero denominator"),
        }
    }
}

impl Error for EquationExpressionError {}

/// Why the Cartesian equation, an evaluation, or its native carrier could not be certified.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CartesianEquationError {
    /// Bounds must be finite and strictly increasing.
    Domain([f64; 2]),
    /// The combined coordinate program exceeds the fixed instruction budget.
    InstructionLimit {
        /// Observed combined instruction count.
        count: usize,
        /// Maximum supported combined instruction count.
        limit: usize,
    },
    /// Malformed or unsupported coordinate instruction.
    Expression {
        /// XYZ coordinate index.
        axis: usize,
        /// Zero-based instruction index, or program length for a final stack error.
        instruction: usize,
        /// Typed grammar failure.
        cause: EquationExpressionError,
    },
    /// An evaluation parameter falls outside the finite domain.
    Parameter {
        /// Requested parameter.
        value: f64,
        /// Accepted domain.
        domain: [f64; 2],
    },
    /// A certified coordinate or derivative range has no finite binary64 enclosure.
    Unrepresentable {
        /// Derivative order, zero for position.
        order: usize,
        /// XYZ coordinate index.
        axis: usize,
    },
    /// Exact derivative vanishes at a witnessed parameter or on the entire curve.
    Singular,
    /// Bounded interval subdivision could not finish a positive speed proof.
    RegularityUnproven {
        /// Maximum permitted dyadic subdivisions on one branch.
        depth: u8,
    },
    /// No positive binary64 lower bound on speed is representable.
    SpeedUnderflow,
    /// Export requires a finite, nonnegative inclusive model distance.
    DistanceLimit(f64),
    /// Exact matched-control discrepancy exceeds the requested certified distance.
    ExportDistance {
        /// Certified upper bound if representable.
        bound: Option<f64>,
        /// Requested inclusive distance limit.
        limit: f64,
    },
    /// The operation was cancelled before completion.
    Cancelled,
}

impl fmt::Display for CartesianEquationError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(domain) => write!(
                output,
                "domain {domain:?} must be finite and strictly increasing"
            ),
            Self::InstructionLimit { count, limit } => {
                write!(output, "instruction count {count} exceeds {limit}")
            }
            Self::Expression {
                axis,
                instruction,
                cause,
            } => write!(
                output,
                "coordinate {axis} instruction {instruction}: {cause}"
            ),
            Self::Parameter { value, domain } => {
                write!(output, "parameter {value} must lie in {domain:?}")
            }
            Self::Unrepresentable { order, axis } => write!(
                output,
                "derivative order {order}, axis {axis} requires a finite enclosure"
            ),
            Self::Singular => output.write_str("curve derivative vanishes in the closed domain"),
            Self::RegularityUnproven { depth } => write!(
                output,
                "regularity could not be certified within {depth} subdivision levels"
            ),
            Self::SpeedUnderflow => output.write_str(
                "the certified speed lower bound rounds below the smallest positive binary64",
            ),
            Self::DistanceLimit(limit) => write!(
                output,
                "distance limit {limit} must be finite and nonnegative"
            ),
            Self::ExportDistance { bound, limit } => write!(
                output,
                "export distance with upper bound {bound:?} exceeds {limit}"
            ),
            Self::Cancelled => output.write_str("Cartesian equation operation was cancelled"),
        }
    }
}

impl Error for CartesianEquationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Expression { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
