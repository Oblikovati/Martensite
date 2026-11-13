// SPDX-License-Identifier: GPL-2.0-only
//! Strict directional derivatives for source-bound cylindrical profile homotopies.
mod directions;
mod error;
mod proof;
mod source;
#[cfg(test)]
mod tests;
mod types;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_curve as curve,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
use error::Checked;
pub use error::{CylindricalConeError, CylindricalConeFailure};
pub use types::*;

/// Prove a common strictly monotone projection throughout the straight profile homotopy.
/// One source excludes a self-contact; two sources additionally need externally checked
/// shared endpoints to exclude a second adjacent contact. No whole-body topology claim is made.
/// Example: `let cone = certify_cylindrical_cone(query, || false)?;`.
pub fn certify_cylindrical_cone(
    query: CylindricalConeQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalCone, CylindricalConeError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalConeError {
        requested_options: query.options,
        source_count: query.sources.len(),
        cause,
    })
}

fn execute(
    query: CylindricalConeQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalCone> {
    admit(query)?;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: query.options.fractional_bits,
            max_operations: query.options.max_operations,
        },
        cancelled,
    );
    let derivatives = query
        .sources
        .iter()
        .map(|s| source::derivatives(s, &mut arithmetic))
        .collect::<Checked<Vec<_>>>()?;
    search(query, &derivatives, &mut arithmetic)
}

fn search(
    query: CylindricalConeQuery<'_>,
    derivatives: &[Vec<[Interval; 2]>],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CertifiedCylindricalCone> {
    let mut attempted = 0;
    for direction in directions::candidates(derivatives, query.options.fractional_bits) {
        attempted += 1;
        if let Some(margins) = proof::margins(direction, derivatives, query, arithmetic)? {
            return Ok(CertifiedCylindricalCone::assemble(
                direction,
                margins,
                query,
                arithmetic.operations(),
                attempted,
            ));
        }
    }
    Err(CylindricalConeFailure::Unresolved { attempted })
}

fn admit(query: CylindricalConeQuery<'_>) -> Checked<()> {
    let policy = query.options;
    if !(8..=4096).contains(&policy.fractional_bits) || policy.max_operations > 1_000_000 {
        return Err(CylindricalConeFailure::Policy(policy));
    }
    if !(1..=2).contains(&query.sources.len()) {
        return Err(CylindricalConeFailure::SourceCount(query.sources.len()));
    }
    for (index, source) in query.sources.iter().enumerate() {
        admit_source(index, source)?;
    }
    common_chart(query.sources)
}

fn admit_source(index: usize, source: &CylindricalConeSource<'_>) -> Checked<()> {
    if !source.matches() {
        return Err(CylindricalConeFailure::SourceMismatch { index });
    }
    Ok(())
}

fn common_chart(sources: &[CylindricalConeSource<'_>]) -> Checked<()> {
    let expected = &sources[0];
    for (index, source) in sources.iter().enumerate().skip(1) {
        if source.support_radius.to_bits() != expected.support_radius.to_bits()
            || source.angular_origin.to_bits() != expected.angular_origin.to_bits()
        {
            return Err(chart_mismatch(index, source, expected));
        }
    }
    Ok(())
}

fn chart_mismatch(
    index: usize,
    source: &CylindricalConeSource<'_>,
    expected: &CylindricalConeSource<'_>,
) -> CylindricalConeFailure {
    CylindricalConeFailure::ChartMismatch {
        index,
        radius: source.support_radius,
        origin: source.angular_origin,
        expected_radius: expected.support_radius,
        expected_origin: expected.angular_origin,
    }
}
