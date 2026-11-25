// SPDX-License-Identifier: GPL-2.0-only
//! Typed finite Cartesian polynomials with exact analytic jets and whole-domain proofs.
mod export;
mod net;
mod program;
mod refusal;

use super::certificate_arithmetic::{ExactRatio as R, rounding::nearest_binary64};

pub use export::EquationBezierCertificate;
pub use program::{EquationInstruction, MAX_CARTESIAN_EQUATION_INSTRUCTIONS};
pub use refusal::{CartesianEquationError, EquationExpressionError};

/// Highest degree admitted by the first Cartesian equation curve.
pub const MAX_CARTESIAN_EQUATION_DEGREE: usize = 3;

/// An owned Cartesian polynomial curve in its authored parameter.
#[derive(Clone, Debug)]
pub struct CartesianEquationCurve {
    domain: [f64; 2],
    span: R,
    rows: [net::Net; 4],
    certificate: EquationDomainCertificate,
}

/// Whole-interval position, derivative and regularity enclosures in the authored parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EquationDomainCertificate {
    /// Inclusive XYZ intervals indexed by derivative order, including order zero.
    pub coordinate_bounds: [[[f64; 2]; 3]; 4],
    /// Strictly positive Euclidean speed lower bound on the entire closed parameter domain.
    pub speed_lower: f64,
}

/// D0–D3 in the authored parameter, rounded once to binary64. Exact zeros are positive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EquationJet {
    /// Position.
    pub position: [f64; 3],
    /// First derivative.
    pub first: [f64; 3],
    /// Second derivative.
    pub second: [f64; 3],
    /// Third derivative.
    pub third: [f64; 3],
}

impl CartesianEquationCurve {
    /// Compile all three coordinate programs and certify the complete closed parameter interval.
    /// Example: `CartesianEquationCurve::compile([&x, &y, &z], [-1., 1.], || false)`.
    pub fn compile(
        programs: [&[EquationInstruction]; 3],
        domain: [f64; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, CartesianEquationError> {
        let (first, span) = parameter_basis(domain, &mut cancelled)?;
        let coordinates = program::compile(programs, &first, &span, &mut cancelled)?;
        let rows = net::jet(coordinates.into_bernstein(), &span);
        let certificate = net::certify(&rows, &mut cancelled)?;
        poll(&mut cancelled)?;
        Ok(Self {
            domain,
            span,
            rows,
            certificate,
        })
    }

    /// Read the authored parameter interval. Example: `curve.domain()`.
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }

    /// Read the certified interval evidence. Example: `curve.certificate().speed_lower`.
    pub fn certificate(&self) -> EquationDomainCertificate {
        self.certificate
    }

    /// Evaluate the analytic Cartesian jet inside the authored interval.
    /// Example: `curve.jet(0.5, || false)?.second`.
    pub fn jet(
        &self,
        parameter: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<EquationJet, CartesianEquationError> {
        let t = self.normalized(parameter, &mut cancelled)?;
        let jet = EquationJet {
            position: self.rounded_row(0, &t, &mut cancelled)?,
            first: self.rounded_row(1, &t, &mut cancelled)?,
            second: self.rounded_row(2, &t, &mut cancelled)?,
            third: self.rounded_row(3, &t, &mut cancelled)?,
        };
        poll(&mut cancelled)?;
        Ok(jet)
    }

    fn normalized(
        &self,
        parameter: f64,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<R, CartesianEquationError> {
        poll(cancelled)?;
        if !(self.domain[0]..=self.domain[1]).contains(&parameter) {
            return Err(CartesianEquationError::Parameter {
                value: parameter,
                domain: self.domain,
            });
        }
        let delta = R::from_finite(parameter).subtract(&R::from_finite(self.domain[0]));
        Ok(delta.divide(&self.span))
    }

    fn rounded_row(
        &self,
        order: usize,
        t: &R,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<[f64; 3], CartesianEquationError> {
        poll(cancelled)?;
        let row = self.rows[order].value(t);
        let [x, y, z] = [0, 1, 2].map(|axis| {
            nearest_binary64(&row[axis])
                .ok_or(CartesianEquationError::Unrepresentable { order, axis })
        });
        Ok([x?, y?, z?])
    }
}

fn parameter_basis(
    [first, last]: [f64; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(R, R), CartesianEquationError> {
    poll(cancelled)?;
    if !first.is_finite() || !last.is_finite() || first >= last {
        return Err(CartesianEquationError::Domain([first, last]));
    }
    let first = R::from_finite(first);
    let span = R::from_finite(last).subtract(&first);
    Ok((first, span))
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), CartesianEquationError> {
    if cancelled() {
        return Err(CartesianEquationError::Cancelled);
    }
    Ok(())
}
