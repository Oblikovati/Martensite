// SPDX-License-Identifier: GPL-2.0-only
//! Certified developed-plane clearance with hierarchy pruning and cached curve domains.
mod admission;
mod bounds;
mod cache;
mod context;
mod error;
mod hierarchy;
mod pair;
mod projection;
mod report;
#[cfg(test)]
mod tests;
mod types;
mod witness;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_curve as curve,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    planar_hulls as hulls,
};
use error::Checked;
pub use error::{CylindricalClearanceError, CylindricalClearanceFailure};
pub use report::{
    CertifiedCylindricalClearance, CertifiedCylindricalImage,
    CertifiedCylindricalPeriodicClearance, CertifiedCylindricalPeriodicSlab,
    CylindricalPeriodicTranslation,
};
pub use types::*;
type Point = [Interval; 2];

/// Prove original-polynomial Euclidean clearance for every pair in the declared relation.
/// Certified boxes prune groups; remaining pairs use outward hull projections and finite
/// cached subdivision. Witnesses below the limit and incomplete bounds both refuse.
/// Example: `let proof = certify_cylindrical_clearance(query, || false)?;`.
pub fn certify_cylindrical_clearance(
    query: CylindricalClearanceQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalClearance, CylindricalClearanceError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalClearanceError {
        requested_clearance: query.requested_clearance,
        requested_options: query.options,
        source_count: query.sources.len(),
        cause,
    })
}

fn execute(
    query: CylindricalClearanceQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalClearance> {
    let excluded = admission::admit(query, cancelled)?;
    let count = query.domain.pair_count(query.sources.len());
    if count == 0 {
        return Ok(CertifiedCylindricalClearance::assemble(
            query,
            (0, report::Statistics::default(), 0),
        ));
    }
    let mut context = context::Context::capture(query, excluded, cancelled)?;
    let mut sources: Vec<_> = (0..query.sources.len()).collect();
    let root = hierarchy::build(&mut context, &mut sources)?;
    hierarchy::certify(&mut context, root)?;
    Ok(completed(query, count, context))
}

fn completed(
    query: CylindricalClearanceQuery<'_>,
    count: usize,
    context: context::Context<'_, '_>,
) -> CertifiedCylindricalClearance {
    CertifiedCylindricalClearance::assemble(
        query,
        (count, context.statistics, context.arithmetic.operations()),
    )
}
