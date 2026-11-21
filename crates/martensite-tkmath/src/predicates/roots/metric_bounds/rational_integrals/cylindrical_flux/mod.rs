// SPDX-License-Identifier: GPL-2.0-only
//! Conditional flux from every supplied ruled side and analytic cylindrical cap occurrence.
use super::{
    ExactRatio, RationalIntegralEvidence, RationalIntegralFailure, RationalIntegralOptions, curve,
    publish,
};
use crate::predicates::roots::metric_bounds::{
    MetricBezier, MetricCurve, MetricEmbedding, PolynomialIntegralFailure,
    PolynomialIntegralOptions, ResidualError, coefficients,
};
use crate::predicates::roots::{certificate_arithmetic::ExactRatio as Ratio, error::poll};
mod admission;
mod analytic_cone;
pub use analytic_cone::{
    AnalyticConicalFluxError, AnalyticConicalFluxFailure, AnalyticConicalFluxQuery,
    AnalyticConicalFluxSpan, CertifiedAnalyticConicalFlux, certify_analytic_conical_boundary_flux,
};
mod binding;
mod caps;
mod error;
mod execution;
mod side;
mod types;
use error::Checked;
pub use error::{CylindricalFluxError, CylindricalFluxFailure};
pub use types::*;
#[cfg(test)]
mod tests;

/// Integrate `F=x/3` over all declared ruled patches and cap boundary occurrences.
/// Caps are local +Z cylinders at the origin, first inward and last outward; side
/// orientation is explicit. No boundary coverage, closure or validity is inferred.
/// Example: `let flux = certify_cylindrical_boundary_flux(query, || false)?;`.
pub fn certify_cylindrical_boundary_flux(
    query: CylindricalFluxQuery<'_>,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CertifiedCylindricalFlux, CylindricalFluxError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalFluxError {
        requested_options: query.options,
        span_count: query.spans.len(),
        cause,
    })
}

fn execute(
    query: CylindricalFluxQuery<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<CertifiedCylindricalFlux> {
    poll(cancelled).map_err(|cause| error::integration(None, cause))?;
    admission::admit(query)?;
    let engine = query.options.engine(query.options.max_total_subintervals);
    let share = publish::width_share(engine, query.spans.len(), &Ratio::from_finite(1.));
    let (sum, work) = execution::integrate(query, &share, cancelled)?;
    let scalar =
        publish::rounded_scalar(sum, engine).map_err(|cause| error::integration(None, cause))?;
    poll(cancelled).map_err(|cause| error::integration(None, cause))?;
    Ok(CertifiedCylindricalFlux {
        scalar,
        bits: binding::source_bits(query),
        options: query.options,
        spans: query.spans.len(),
        work,
    })
}
