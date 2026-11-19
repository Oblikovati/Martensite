// SPDX-License-Identifier: GPL-2.0-only
//! Conditional analytic-cone cap flux and the shared rational ruled-side integration.
use super::*;
use crate::predicates::roots::metric_bounds::rational_integrals::subinterval::ContributionBounds;
use crate::predicates::roots::metric_bounds::{ConicalTrimCarrier, CylindricalProfileOptions};
mod admission;
mod binding;
mod caps;
mod error;
mod execution;
mod types;
use error::Checked;
pub use error::{AnalyticConicalFluxError, AnalyticConicalFluxFailure};
pub use types::*;

/// Integrate every actual cone-cap and ruled-side occurrence using `F(x)=x/3`.
/// This conditional flux implies no closure, topology, injectivity or volume verdict.
/// Example: `let flux = certify_analytic_conical_boundary_flux(query, || false)?;`.
pub fn certify_analytic_conical_boundary_flux(
    query: AnalyticConicalFluxQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedAnalyticConicalFlux, AnalyticConicalFluxError> {
    let mut work = error::ExecutionWork::default();
    execute(query, &mut cancelled, &mut work).map_err(|cause| AnalyticConicalFluxError {
        requested_options: query.options,
        requested_arithmetic: query.arithmetic,
        span_count: query.spans.len(),
        operations: work.operations,
        integration_subintervals: work.cells,
        cause,
    })
}

fn execute(
    query: AnalyticConicalFluxQuery<'_>,
    cancelled: &mut (impl FnMut() -> bool + Send),
    work: &mut error::ExecutionWork,
) -> Checked<CertifiedAnalyticConicalFlux> {
    poll(cancelled).map_err(|cause| error::integration(None, cause))?;
    admission::admit(query)?;
    let densities = caps::coefficients(query, cancelled, &mut work.operations)?;
    let (sum, span_work) = execution::integrate(query, &densities, work, cancelled)?;
    let mut scalar = publish::rounded_scalar(
        sum.finish(),
        super::super::conical_sum::term_engine(query.options, query.options.max_total_subintervals),
    )
    .map_err(|cause| error::integration(None, cause))?;
    // Cap coefficients retain transcendental enclosures even when side work is exact.
    scalar.evidence = RationalIntegralEvidence::ErrorBoundedIntegration;
    poll(cancelled).map_err(|cause| error::integration(None, cause))?;
    Ok(CertifiedAnalyticConicalFlux::assemble(
        query,
        scalar,
        span_work,
        work.operations,
    ))
}

#[cfg(test)]
mod tests;
