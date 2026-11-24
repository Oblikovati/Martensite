//! A cap-to-reference homotopy stays inside a source-bound tube of the authored boundary.
use super::*;
#[derive(Clone, Debug, PartialEq)]
pub(in super::super) struct CapHomotopyBounds {
    pub envelope: f64,
    pub required_clearance: f64,
}
pub(super) fn certify(family: &family::UnqualifiedFamily) -> Checked<CapHomotopyBounds> {
    let mut envelope: f64 = 0.;
    for prepared in &family.selection.prepared.prepared {
        let member = family
            .members
            .get(&prepared.region)
            .ok_or(BudgetFailure::Source(prepared.region))?;
        for proof in &member.geometry.storage {
            envelope = envelope.max(span_envelope(proof, prepared.radius, prepared.region)?);
        }
    }
    let required_clearance = strict_clearance(envelope)?;
    member::clearance(family, required_clearance)?;
    Ok(CapHomotopyBounds {
        envelope,
        required_clearance,
    })
}
fn span_envelope(
    proof: &storage::StoredProof,
    radius: f64,
    region: SurfaceRegionId,
) -> Checked<f64> {
    require_lift(proof, region)?;
    let cap = arithmetic::scale_upper(
        proof.cap_profile.developed_position_bound(),
        radius.max(1.),
        Quantity::CapHomotopy,
    )?;
    volume::add_upper(
        proof.profile.developed_position_bound(),
        cap,
        Quantity::CapHomotopy,
    )
}
fn require_lift(proof: &storage::StoredProof, region: SurfaceRegionId) -> Checked<()> {
    if !proof
        .cap_lift
        .matches_sources([&proof.profile, &proof.cap_profile])
    {
        return Err(BudgetFailure::Source(region));
    }
    Ok(())
}

fn strict_clearance(envelope: f64) -> Checked<f64> {
    let required = arithmetic::scale_upper(envelope, 2., Quantity::CapHomotopy)?.next_up();
    volume::valid_bound(required, Quantity::CapHomotopy)?;
    Ok(required)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clearance_threshold_is_strict_even_for_zero_or_exact_doubling() {
        assert!(strict_clearance(0.).unwrap() > 0.);
        assert!(strict_clearance(0.5).unwrap() > 1.);
    }
}
