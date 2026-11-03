//! Bounded exact sufficient embedding proofs on owned low-degree rational query charts.
mod budget;
mod cycle;
mod hulls;
mod polynomial;
mod prepare;
mod projection;
mod types;
use super::certificate_arithmetic::ExactRatio as Ratio;
use budget::Budget;
pub use types::*;

/// Prove only the supplied canonical map, not trims or floating evaluation; `certify_surface_query(input, SurfaceQueryPolicy::bounded(), || false)`.
pub fn certify_surface_query(
    input: SurfaceQueryInput,
    policy: SurfaceQueryPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> SurfaceQueryReport {
    let mut budget = Budget::new(policy, &mut cancelled);
    let outcome = prove(&input, &mut budget);
    SurfaceQueryReport {
        input,
        policy,
        work: budget.work,
        outcome,
    }
}
fn prove(
    input: &SurfaceQueryInput,
    budget: &mut Budget<'_>,
) -> Result<SurfaceQueryTheorem, SurfaceQueryCause> {
    budget.step()?;
    if budget.policy.version != 1 {
        return Err(SurfaceQueryCause::Version(budget.policy.version));
    }
    prepare::validate(input)?;
    let patches = input
        .patches
        .iter()
        .map(|patch| prepare::homogeneous(patch, budget))
        .collect::<Result<Vec<_>, _>>()?;
    if input.periodic_u {
        return cycle::prove(&patches, budget);
    }
    projection::prove(&patches[0], budget)
}
#[cfg(test)]
mod tests;
