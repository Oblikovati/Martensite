// SPDX-License-Identifier: GPL-2.0-only
use super::super::BoundedArithmeticFailure;
use super::CylindricalConeOptions;
pub(super) type Checked<T> = Result<T, CylindricalConeFailure>;

/// Incomplete tangent-cone evidence, never a successful topology verdict.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalConeError {
    /// Original precision and work policy.
    pub requested_options: CylindricalConeOptions,
    /// Original number of oriented sources.
    pub source_count: usize,
    /// Original typed failure.
    pub cause: CylindricalConeFailure,
}
/// Invalid source binding or a projection that could not be established.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalConeFailure {
    /// Settings outside the stated finite arithmetic policy.
    Policy(CylindricalConeOptions),
    /// Expected one regularity source or two adjacent sources.
    SourceCount(usize),
    /// Original polynomial, interval or chart did not match its retained certificate.
    SourceMismatch {
        /// Position in the input source slice.
        index: usize,
    },
    /// Adjacent sources declared different developed support charts.
    ChartMismatch {
        /// Position of the source outside the first chart.
        index: usize,
        /// Supplied support radius.
        radius: f64,
        /// Supplied angular origin.
        origin: f64,
        /// First source's support radius.
        expected_radius: f64,
        /// First source's angular origin.
        expected_origin: f64,
    },
    /// Original bounded-arithmetic cause.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// No candidate established a strict common cone; geometry is not declared invalid.
    Unresolved {
        /// Candidate directions tested without a strict proof.
        attempted: u32,
    },
}
impl From<BoundedArithmeticFailure> for CylindricalConeFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalConeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical tangent cone sources={}: {:?}; expected source-bound O2 evidence, admitted arithmetic and a positive directional derivative margin",
            self.source_count, self.cause
        )
    }
}
impl std::error::Error for CylindricalConeError {}
