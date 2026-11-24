// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::RootIsolation;

/// Borrowed exact sign/domain/coefficient witness; construction stays inside the provider.
#[derive(Clone, Copy)]
pub struct BezierPairBernsteinExclusion<'a> {
    proof: &'a ExclusionProof,
}
impl<'a> BezierPairBernsteinExclusion<'a> {
    pub(super) fn new(proof: &'a ExclusionProof) -> Self {
        Self { proof }
    }
    /// Actual cleared coordinate equation, zero for X and one for Y; `proof.coordinate_axis()`.
    pub fn coordinate_axis(&self) -> usize {
        self.proof.axis
    }
    /// Strict common coefficient sign, always Less or Greater; `proof.sign()`.
    pub fn sign(&self) -> Ordering {
        self.proof.witness.sign
    }
    /// Whether exact `(t-u)` division preceded the test; `proof.divided_self_diagonal()`.
    pub fn divided_self_diagonal(&self) -> bool {
        self.proof.divided_self_diagonal
    }
    /// Original exact binary64 parameter ranges before affine restriction; `proof.parameter_rectangle()`.
    pub fn parameter_rectangle(&self) -> [[f64; 2]; 2] {
        self.proof.rectangle
    }
    /// Degrees of the tensor basis in T and U; `proof.bernstein_degrees()`.
    pub fn bernstein_degrees(&self) -> [usize; 2] {
        self.proof.witness.degrees
    }
    /// Reduced exact coefficient numerator; `proof.bernstein_numerator(0, 0).unwrap().to_string()`.
    pub fn bernstein_numerator(&self, t: usize, u: usize) -> Option<ExactPairCoefficient<'a>> {
        let value = self.proof.witness.values.get(t)?.get(u)?;
        Some(ExactPairCoefficient::new(value.numerator()))
    }
    /// Strictly positive reduced coefficient denominator; `proof.bernstein_denominator(0, 0)`.
    pub fn bernstein_denominator(&self, t: usize, u: usize) -> Option<ExactPairCoefficient<'a>> {
        let value = self.proof.witness.values.get(t)?.get(u)?;
        Some(ExactPairCoefficient::new(value.denominator()))
    }
    /// Actual cleared equations after the reported diagonal division, as U powers of T polynomials.
    /// Example: `proof.equation(proof.coordinate_axis()).unwrap().len()`.
    pub fn equation(
        &self,
        axis: usize,
    ) -> Option<impl ExactSizeIterator<Item = ExactPairPolynomial<'a>>> {
        let equation = self.proof.equations.0.get(axis)?;
        Some(
            equation
                .0
                .iter()
                .map(|row| ExactPairPolynomial::new(&row.0)),
        )
    }
    /// Complete nonzero-denominator proofs for both source domains; `proof.denominators()[0].roots()`.
    pub fn denominators(&self) -> [&'a RootIsolation; 2] {
        self.proof
            .domains
            .each_ref()
            .map(|domain| &domain.denominator)
    }
    /// Actual jointly normalized homogeneous X/Y/W power coefficients; `proof.homogeneous_sources()`.
    pub fn homogeneous_sources(&self) -> [[ExactPairPolynomial<'a>; 3]; 2] {
        self.proof.domains.each_ref().map(|domain| {
            domain
                .homogeneous
                .components
                .each_ref()
                .map(|axis| ExactPairPolynomial::new(&axis.0))
        })
    }
}
