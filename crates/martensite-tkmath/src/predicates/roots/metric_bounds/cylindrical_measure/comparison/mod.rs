// SPDX-License-Identifier: GPL-2.0-only
//! Compare actual retained flux with the source-bound ideal profile using outward enclosures.
mod binding;
mod bounds;
mod error;
#[cfg(test)]
mod tests;
mod types;
use super::super::{
    CertifiedCylindricalFlux, CylindricalFluxQuery, CylindricalFluxSpan, CylindricalProfileQuery,
};
use super::*;
pub use error::{CylindricalMeasureComparisonError, CylindricalMeasureComparisonFailure};
pub use types::*;
type Checked<T> = Result<T, CylindricalMeasureComparisonFailure>;

/// Compare completed measures only after binding them to the same actual first-depth curves.
/// Takes ownership of the input certificates so the report retains their evidence without copies.
/// Closed-boundary, consistent orientation and injectivity premises remain external.
/// Example: `let comparison = compare_cylindrical_measures(flux, normal, query, || false)?;`.
pub fn compare_cylindrical_measures(
    flux: CertifiedCylindricalFlux,
    normal: CertifiedCylindricalNormalMeasure,
    query: CylindricalMeasureComparisonQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalMeasureComparison, CylindricalMeasureComparisonError> {
    execute(flux, normal, query, &mut cancelled).map_err(|cause| {
        CylindricalMeasureComparisonError {
            requested_options: query.options,
            requested_limit: query.limit,
            flux_spans: query.flux_source.spans.len(),
            normal_sources: query.normal_source.sources.len(),
            cause,
        }
    })
}
fn execute(
    flux: CertifiedCylindricalFlux,
    normal: CertifiedCylindricalNormalMeasure,
    query: CylindricalMeasureComparisonQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalMeasureComparison> {
    binding::admit(query)?;
    let mut arithmetic = arithmetic(query.options, cancelled);
    binding::bind(query, &flux, &normal, &mut arithmetic)?;
    let bounds = bounds::calculate(&flux, &normal, &mut arithmetic)?;
    bounds::check_limit(query.limit, &bounds)?;
    Ok(assemble(
        flux,
        normal,
        query,
        bounds,
        arithmetic.operations(),
    ))
}
fn arithmetic<'a>(
    options: CylindricalMeasureOptions,
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
) -> Arithmetic<'a> {
    Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
        },
        cancelled,
    )
}

fn assemble(
    flux: CertifiedCylindricalFlux,
    normal: CertifiedCylindricalNormalMeasure,
    query: CylindricalMeasureComparisonQuery<'_>,
    bounds: bounds::ComparisonBounds,
    operations: u32,
) -> CertifiedCylindricalMeasureComparison {
    CertifiedCylindricalMeasureComparison {
        flux,
        normal,
        enclosure: bounds.enclosure,
        discrepancy: bounds.discrepancy,
        representative_error: bounds.representative_error,
        options: query.options,
        limit: query.limit,
        operations,
    }
}
