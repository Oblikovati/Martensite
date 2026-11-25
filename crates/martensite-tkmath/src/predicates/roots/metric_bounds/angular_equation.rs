// SPDX-License-Identifier: GPL-2.0-only
//! Checked angular equation laws over finite authored parameter intervals.
mod fit;
mod interval;
mod law;
mod refusal;
mod series;

use crate::predicates::roots::{
    cartesian_equation::{EquationInstruction, EquationJet, component_rows, net::Net},
    certificate_arithmetic::{
        ExactRatio as R,
        bounded_dyadic::{Arithmetic, ArithmeticPolicy},
    },
};

pub use fit::{AngularBezierCertificate, AngularBezierSpan};
use refusal::check_cancelled;
pub use refusal::{AngularChartQuantity, AngularEquationError};

const POLICY: ArithmeticPolicy = ArithmeticPolicy {
    fractional_bits: 160,
    max_operations: 2_000_000,
};
const MAX_DEPTH: u8 = 8;

/// The meaning and order of three typed coordinate programs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AngularEquationMode {
    /// Radius in model units, azimuth in radians, Z in model units.
    Cylindrical,
    /// Radius in model units, inclination from +Z in radians, azimuth in radians.
    Spherical,
}

/// An owned bounded coordinate law and proof of its whole authored interval.
#[derive(Clone, Debug)]
pub struct AngularEquationCurve {
    mode: AngularEquationMode,
    domain: [f64; 2],
    span: R,
    rows: [Net; 4],
    certificate: AngularEquationCertificate,
}

/// Whole-interval finite Cartesian position, derivatives and regularity evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AngularEquationCertificate {
    /// Outward XYZ intervals for D0 through D4 in the authored parameter.
    pub coordinate_bounds: [[[f64; 2]; 3]; 5],
    /// Strictly positive Euclidean speed lower bound, before nominal evaluation rounding.
    pub speed_lower: f64,
}

/// Nominal analytic D0 through D3 and their certified exact-value enclosures.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AngularEquationJet {
    /// Nearest binary64 to the midpoint of each bounded analytic component.
    pub nominal: EquationJet,
    /// Outward XYZ bounds on each analytic derivative order, including position.
    pub bounds: [[[f64; 2]; 3]; 4],
}

impl AngularEquationCurve {
    /// Compile radius and angle laws and certify every point of the closed parameter interval.
    /// Example: `AngularEquationCurve::compile(mode, [&a, &b, &c], [0., 1.], || false)`.
    pub fn compile(
        mode: AngularEquationMode,
        programs: [&[EquationInstruction]; 3],
        domain: [f64; 2],
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<Self, AngularEquationError> {
        let (span, rows) = component_rows(programs, domain, &mut cancelled)?;
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let certificate = law::domain(mode, &rows, 0, &mut arithmetic)?;
        check_cancelled(&mut cancelled)?;
        Ok(Self {
            mode,
            domain,
            span,
            rows,
            certificate,
        })
    }

    /// Original coordinate mode. Example: `curve.mode()`.
    pub fn mode(&self) -> AngularEquationMode {
        self.mode
    }

    /// Authored interval, before native parameter normalization. Example: `curve.domain()`.
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }

    /// Whole authored interval evidence. Example: `curve.certificate().speed_lower`.
    pub fn certificate(&self) -> AngularEquationCertificate {
        self.certificate
    }

    /// Enclose the Cartesian analytic jet at one finite authored parameter.
    /// Example: `curve.jet(0.5, || false)?.nominal.first`.
    pub fn jet(
        &self,
        parameter: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<AngularEquationJet, AngularEquationError> {
        check_cancelled(&mut cancelled)?;
        if !(self.domain[0]..=self.domain[1]).contains(&parameter) {
            return Err(AngularEquationError::Parameter {
                parameter,
                domain: self.domain,
            });
        }
        let delta = R::from_finite(parameter).subtract(&R::from_finite(self.domain[0]));
        let normalized = delta.divide(&self.span);
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let jet = law::point_jet(self.mode, &self.rows, &normalized, &mut arithmetic)?;
        let output = interval::jet(&jet, &mut arithmetic)?;
        check_cancelled(&mut cancelled)?;
        Ok(output)
    }
}
