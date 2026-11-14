// SPDX-License-Identifier: GPL-2.0-only
//! Conditional closed-profile area and exact-depth normal-volume enclosures.
mod admission;
mod error;
mod profile;
mod publication;
#[cfg(test)]
mod tests;
mod types;
use super::CylindricalProfileSource;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_curve as curve,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
use error::Checked;
pub use error::{CylindricalMeasureError, CylindricalMeasureFailure};
pub use types::*;

/// Bound closed-profile area displacement and its normal-extrusion volume consequence.
/// The caller must establish complete closed source and stored contours, shared lifted endpoints,
/// consistent orientation and injectivity before interpreting the signed integrals as body measures.
/// This does not certify represented BRep flux, radial storage error or topology.
/// Example: `let measure = certify_cylindrical_normal_measure(query, || false)?;`.
pub fn certify_cylindrical_normal_measure(
    query: CylindricalMeasureQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalNormalMeasure, CylindricalMeasureError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalMeasureError {
        requested_options: query.options,
        requested_limit: query.limit,
        requested_depths: query.depths,
        source_count: query.sources.len(),
        cause,
    })
}
fn execute(
    query: CylindricalMeasureQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalNormalMeasure> {
    admission::admit(query)?;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: query.options.fractional_bits,
            max_operations: query.options.max_operations,
        },
        cancelled,
    );
    let (area, error) = accumulate(query.sources, &mut arithmetic)?;
    publication::publish(query, area, error, &mut arithmetic)
}
fn accumulate(
    sources: &[CylindricalProfileSource<'_>],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(Interval, Interval)> {
    let mut area = arithmetic.zero();
    let mut error = arithmetic.zero();
    for source in sources {
        let poles = profile::capture(source, arithmetic)?;
        let integral = profile::integral(&poles, arithmetic)?;
        let discrepancy = profile::discrepancy(source, &poles, arithmetic)?;
        area = arithmetic.add(&area, &integral)?;
        error = arithmetic.add(&error, &discrepancy)?;
    }
    Ok((area, error))
}
