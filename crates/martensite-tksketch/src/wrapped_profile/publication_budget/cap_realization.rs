//! Physical cap-domain distance after closure, cone, lift and global homotopy gates pass.
use super::*;
pub(super) fn certify(
    original: &Request,
    retained: &pipeline::RetainedMember,
    limit: f64,
) -> Checked<f64> {
    let mut maximum: f64 = 0.;
    for proof in &retained.geometry.storage {
        source(original, proof)?;
        for depth in 0..2 {
            let actual = endpoint_bound(proof, depth)?;
            admit_bound(actual, limit, original.region, proof.span)?;
            maximum = maximum.max(actual);
        }
    }
    Ok(maximum)
}
fn admit_bound(actual: f64, limit: f64, region: SurfaceRegionId, span: CurveSpan) -> Checked<f64> {
    upper(actual, limit, Quantity::CapDomain, region, Some(span))
}

fn source(original: &Request, proof: &storage::StoredProof) -> Checked<()> {
    let depth = &proof.depths;
    let actual = [
        depth.support_radius(),
        depth.requested_depths()[0],
        depth.requested_depths()[1],
    ];
    let expected = [original.radius, original.depths[0], original.depths[1]];
    if actual.map(f64::to_bits) != expected.map(f64::to_bits)
        || depth.stored_radii().map(f64::to_bits)
            != proof.certificate.stored_radii().map(f64::to_bits)
    {
        return Err(BudgetFailure::Source(original.region));
    }
    Ok(())
}
fn endpoint_bound(proof: &storage::StoredProof, depth: usize) -> Checked<f64> {
    let error = proof.depths.radius_rounding_bounds()[depth];
    let native = proof.depths.stored_radii()[depth];
    physical_bound(native, error, proof.cap_profile.developed_position_bound())
}
fn physical_bound(native: f64, error: f64, profile: f64) -> Checked<f64> {
    let radius = volume::add_upper(native, error, Quantity::CapDomain)?;
    let displacement = arithmetic::scale_upper(profile, radius.max(1.), Quantity::CapDomain)?;
    volume::add_upper(displacement, error, Quantity::CapDomain)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_cap_bound_encloses_closed_form_radius_and_profile_errors() {
        assert!(physical_bound(4., 0.25, 0.125).unwrap() >= 0.78125);
        assert!(physical_bound(0.5, 0.125, 0.25).unwrap() >= 0.375);
        assert_eq!(physical_bound(4., 0., 0.).unwrap(), 0.);
        assert!(physical_bound(f64::MAX, f64::MAX, 1.).is_err());
    }
    #[test]
    fn cap_domain_checks_depth_sources_and_refuses_an_unmet_limit() {
        let inputs = family::tests::requests();
        let family = family::construct(&inputs).unwrap();
        let input = &inputs[0];
        let retained = &family.members[&input.region];
        assert!(certify(input, retained, input.tolerance).unwrap() > 0.);
        assert!(matches!(
            certify(input, retained, 0.),
            Err(BudgetFailure::Limit {
                quantity: Quantity::CapDomain,
                ..
            })
        ));
        let mut changed = input.clone();
        changed.depths[0] += 0.125;
        assert!(matches!(
            certify(&changed, retained, input.tolerance),
            Err(BudgetFailure::Source(_))
        ));
    }
}
