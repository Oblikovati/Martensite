// SPDX-License-Identifier: GPL-2.0-only
//! One scalar cap and one aggregate cell allowance cover both rational terms of every span.
use super::super::conical_sum::ConicalSum;
use super::*;
use std::cmp::Ordering;
struct Integration<'a> {
    query: ConicalReferenceVolumeQuery<'a>,
    coefficients: &'a [ContributionBounds; 2],
    operations: u32,
    cells: u64,
    sum: ConicalSum,
    work: Vec<CylindricalFluxWork>,
}
pub(super) fn integrate(
    query: ConicalReferenceVolumeQuery<'_>,
    coefficients: &[ContributionBounds; 2],
    operations: u32,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<CertifiedConicalReferenceVolume, ConicalReferenceVolumeError> {
    let mut integration = Integration {
        query,
        coefficients,
        operations,
        cells: 0,
        sum: ConicalSum::new(query.options.behavior_version == 2, query.arithmetic),
        work: Vec::new(),
    };
    for (index, span) in query.spans.iter().copied().enumerate() {
        integration.span(index, span, cancelled)?;
    }
    integration.finish(cancelled)
}
impl Integration<'_> {
    fn failure(
        &self,
        span: Option<usize>,
        term: ConicalReferenceVolumeTerm,
        cause: RationalIntegralFailure,
    ) -> ConicalReferenceVolumeError {
        error::integration(self.query, span, (self.operations, self.cells), term, cause)
    }
    fn span(
        &mut self,
        index: usize,
        span: ConicalReferenceVolumeSpan<'_>,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<(), ConicalReferenceVolumeError> {
        let term = ConicalReferenceVolumeTerm::SquaredSlant;
        poll(cancelled).map_err(|cause| self.failure(Some(index), term, cause.into()))?;
        let quotients = quotient::capture(span, cancelled)
            .map_err(|cause| self.failure(Some(index), term, cause))?;
        let allowance = self
            .remaining()
            .min(self.query.options.max_subintervals_per_span);
        let started = self.cells;
        for term in 0..2 {
            let remaining = allowance - (self.cells - started) as u32;
            self.term(index, span.reversed, term, &quotients, remaining, cancelled)?;
        }
        self.work.push(CylindricalFluxWork {
            effective_limit: allowance,
            examined_subintervals: (self.cells - started) as u32,
        });
        Ok(())
    }
    fn term(
        &mut self,
        index: usize,
        reversed: bool,
        term: usize,
        quotient: &quotient::Quotients,
        remaining: u32,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<(), ConicalReferenceVolumeError> {
        let coefficient = &self.coefficients[term];
        if zero(coefficient) {
            return Ok(());
        }
        let options = engine(self.query.options, remaining.min(self.remaining()));
        let share = width_share(self.query, coefficient);
        let (result, examined) = integrate_quotient(quotient, term, options, &share, cancelled);
        self.cells += examined;
        let kind = term_kind(term);
        let result = result.map_err(|cause| self.failure(Some(index), kind, cause))?;
        let contribution = multiply(coefficient, &result.bounds);
        self.accumulate(index, reversed, contribution, examined, cancelled)
    }
    fn accumulate(
        &mut self,
        index: usize,
        reversed: bool,
        contribution: ContributionBounds,
        cells: u64,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<(), ConicalReferenceVolumeError> {
        self.sum
            .accumulate(
                publish::reversed_bounds(contribution, reversed),
                cells,
                &mut self.operations,
                cancelled,
            )
            .map_err(|cause| {
                ConicalReferenceVolumeError::new(
                    self.query,
                    Some(index),
                    None,
                    (self.operations, self.cells),
                    ConicalReferenceVolumeFailure::Arithmetic(Box::new(cause)),
                )
            })
    }
    fn remaining(&self) -> u32 {
        self.query.options.max_total_subintervals - self.cells as u32
    }
    fn finish(
        self,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<CertifiedConicalReferenceVolume, ConicalReferenceVolumeError> {
        let query = self.query;
        let work = (self.operations, self.cells);
        let fail = |cause| publication_error(query, work, cause);
        poll(cancelled).map_err(|cause| fail(cause.into()))?;
        let options = engine(query.options, self.remaining());
        let mut scalar = publish::rounded_scalar(self.sum.finish(), options).map_err(fail)?;
        scalar.evidence = RationalIntegralEvidence::ErrorBoundedIntegration;
        poll(cancelled).map_err(|cause| fail(cause.into()))?;
        Ok(CertifiedConicalReferenceVolume {
            scalar,
            bits: binding::source_bits(query),
            options: query.options,
            arithmetic: query.arithmetic,
            work: self.work,
            operations: self.operations,
        })
    }
}
fn engine(options: CylindricalFluxOptions, remaining: u32) -> RationalIntegralOptions {
    RationalIntegralOptions {
        behavior_version: 1, // Parent2 changes only accumulation; scalar term algorithm remains1.
        max_degree: options.max_degree,
        max_curves: 1,
        absolute_error_limit: options.absolute_error_limit,
        max_subintervals: options.max_subintervals_per_span.min(remaining),
    }
}
fn width_share(
    query: ConicalReferenceVolumeQuery<'_>,
    coefficient: &ContributionBounds,
) -> ExactRatio {
    let magnitude = coefficient
        .lower
        .negated()
        .max(coefficient.upper.clone())
        .max(ExactRatio::from_finite(1.));
    let occurrences = ExactRatio::new((4 * query.spans.len() as u64).into(), 1.into());
    ExactRatio::from_finite(query.options.absolute_error_limit)
        .divide(&occurrences.multiply(&magnitude))
}
fn multiply(a: &ContributionBounds, b: &ContributionBounds) -> ContributionBounds {
    let products = [
        a.lower.multiply(&b.lower),
        a.lower.multiply(&b.upper),
        a.upper.multiply(&b.lower),
        a.upper.multiply(&b.upper),
    ];
    let lower = products
        .iter()
        .fold(products[0].clone(), |old, value| old.min(value.clone()));
    let upper = products
        .iter()
        .fold(products[0].clone(), |old, value| old.max(value.clone()));
    ContributionBounds { lower, upper }
}

fn zero(coefficient: &ContributionBounds) -> bool {
    coefficient.lower.sign() == Ordering::Equal && coefficient.upper.sign() == Ordering::Equal
}
fn term_kind(term: usize) -> ConicalReferenceVolumeTerm {
    [
        ConicalReferenceVolumeTerm::SquaredSlant,
        ConicalReferenceVolumeTerm::Slant,
    ][term]
}
fn publication_error(
    query: ConicalReferenceVolumeQuery<'_>,
    work: (u32, u64),
    cause: RationalIntegralFailure,
) -> ConicalReferenceVolumeError {
    error::integration(
        query,
        None,
        work,
        ConicalReferenceVolumeTerm::Publication,
        cause,
    )
}
fn integrate_quotient(
    quotient: &quotient::Quotients,
    term: usize,
    options: RationalIntegralOptions,
    share: &ExactRatio,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> (Result<curve::CurveIntegral, RationalIntegralFailure>, u64) {
    curve::quotient_integral_accounted(
        quotient.numerators[term].clone(),
        quotient.denominator.clone(),
        options,
        share,
        cancelled,
    )
}
