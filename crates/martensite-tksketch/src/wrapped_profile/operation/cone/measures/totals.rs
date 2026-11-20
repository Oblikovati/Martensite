//! The family gate publishes one q/L reference and separately labelled native proxies.
use super::comparison::add;
use super::*;
mod native_validation;
pub(super) fn certify(
    request: &ConeRun<'_>,
    family: &OwnedFamily<Selection, Geometry>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Budget> {
    native_validation::require(family)?;
    if family.selection.executed_selection.is_connected_atlas() {
        return connected(request, family, work);
    }
    let reference = material::certify_reference(request, family, work, cancelled)?;
    let mut bounds = accumulated(family)?;
    verify_work(request, family, work, reference.operations())?;
    enforce(request.options.volume, &bounds.volume)?;
    let cap_reference_proxy_error = cap_bridge(&reference, family)?;
    bounds.spatial_error = bounds
        .spatial_error
        .max(reference.filled_error())
        .max(cap_reference_proxy_error);
    enforce_spatial(request, bounds.spatial_error)?;
    completed(bounds, reference, cap_reference_proxy_error, None, work)
}
fn connected(
    request: &ConeRun<'_>,
    family: &OwnedFamily<Selection, Geometry>,
    work: &mut storage::Work,
) -> ConeChecked<Budget> {
    let region = family
        .selection
        .prepared
        .first()
        .map(|p| p.source.region)
        .ok_or_else(|| {
            scalar(
                "decoded atlas member count",
                0.,
                "at least one certified member",
            )
        })?;
    let captured = work
        .connected
        .take()
        .ok_or_else(|| material::mismatch(region, None, "completed decoded atlas binding"))?;
    let (reference, binding) = captured.finish(region)?;
    let mut bounds = accumulated(family)?;
    verify_work(request, family, work, binding.operations)?;
    enforce(request.options.volume, &bounds.volume)?;
    let [composed, cap] = connected_errors(&reference, region)?;
    bounds.spatial_error = bounds.spatial_error.max(composed);
    enforce_spatial(request, bounds.spatial_error)?;
    completed(bounds, reference, cap, Some(Box::new(binding)), work)
}
fn connected_errors(
    reference: &ConicalReferenceEvidence,
    region: SurfaceRegionId,
) -> ConeChecked<[f64; 2]> {
    let caps = reference
        .decoded_caps()
        .ok_or_else(|| material::mismatch(region, None, "decoded cap family reference"))?;
    let mut bounds = [0_f64; 2];
    for cap in caps {
        bounds[0] = bounds[0].max(upper(cap.composed_error())?);
        let [first, last] = cap.cap_errors();
        bounds[1] = bounds[1].max(upper(first.max(last))?);
    }
    Ok(bounds)
}
fn completed(
    bounds: FamilyBounds,
    reference: ConicalReferenceEvidence,
    cap_reference_proxy_error: f64,
    connected: Option<Box<material::atlas::Binding>>,
    work: &storage::Work,
) -> ConeChecked<Budget> {
    let planar_work = work.planar_receipt().ok_or_else(|| {
        scalar(
            "aggregate planar work",
            0.,
            "one source census and complete decoded domain receipts",
        )
    })?;
    Ok(Budget {
        actual_certificate: work.actual_certificate.clone(),
        spatial_error: bounds.spatial_error,
        volume: bounds.volume,
        arithmetic_operations: work.consumed,
        integration_subintervals: work.integration_cells,
        planar_work,
        reference,
        cap_reference_proxy_error,
        connected,
    })
}
struct FamilyBounds {
    spatial_error: f64,
    volume: WrappedConeVolumeBounds,
}
fn accumulated(family: &OwnedFamily<Selection, Geometry>) -> ConeChecked<FamilyBounds> {
    let mut bounds = FamilyBounds {
        spatial_error: 0.,
        volume: empty_volume(),
    };
    for member in family.members.values() {
        let geometry = &member.geometry;
        bounds.spatial_error = bounds
            .spatial_error
            .max(spatial(&geometry.spans)?)
            .max(geometry.material.spatial_error());
        let volume = member_volume(geometry)?;
        legacy_volume(&mut bounds.volume, &volume)?;
        reference_volume(&mut bounds.volume, &volume)?;
    }
    Ok(bounds)
}
fn empty_volume() -> WrappedConeVolumeBounds {
    WrappedConeVolumeBounds {
        discrepancy: bound(0.),
        integration_error: bound(0.),
        authored_reference_discrepancy: bound(0.),
        proxy_reference_discrepancy: bound(0.),
        reference_integration_error: bound(0.),
    }
}
fn legacy_volume(
    total: &mut WrappedConeVolumeBounds,
    actual: &WrappedConeVolumeBounds,
) -> ConeChecked<()> {
    total.discrepancy.upper = add(total.discrepancy.upper, actual.discrepancy.upper)?;
    total.integration_error.upper = add(
        total.integration_error.upper,
        actual.integration_error.upper,
    )?;
    Ok(())
}
fn reference_volume(
    total: &mut WrappedConeVolumeBounds,
    actual: &WrappedConeVolumeBounds,
) -> ConeChecked<()> {
    total.authored_reference_discrepancy.upper = add(
        total.authored_reference_discrepancy.upper,
        actual.authored_reference_discrepancy.upper,
    )?;
    total.proxy_reference_discrepancy.upper = add(
        total.proxy_reference_discrepancy.upper,
        actual.proxy_reference_discrepancy.upper,
    )?;
    total.reference_integration_error.upper = add(
        total.reference_integration_error.upper,
        actual.reference_integration_error.upper,
    )?;
    Ok(())
}
fn bound(upper: f64) -> WrappedCertifiedBound {
    WrappedCertifiedBound { upper }
}
fn enforce(policy: WrappedVolumePolicy, volume: &WrappedConeVolumeBounds) -> ConeChecked<()> {
    let WrappedVolumePolicy::Refuse(limit) = policy else {
        return Ok(());
    };
    if !limit.is_finite() || limit < 0. {
        return Err(scalar(
            "volume refusal allowance",
            limit,
            "finite nonnegative allowance",
        ));
    }
    let actual = volume.authored_reference_discrepancy.upper;
    if actual > limit {
        return Err(volume_limit(actual, limit));
    }

    Ok(())
}
fn cap_bridge(
    reference: &ConicalReferenceEvidence,
    family: &OwnedFamily<Selection, Geometry>,
) -> ConeChecked<f64> {
    let mut maximum: f64 = 0.;
    for member in family.members.values() {
        // Both independently certified filled domains are compared through the same authored material.
        maximum = maximum.max(add(
            reference.filled_error(),
            member.geometry.material.spatial_error(),
        )?);
    }
    Ok(maximum)
}
fn enforce_spatial(request: &ConeRun<'_>, actual: f64) -> ConeChecked<()> {
    if actual > request.options.spatial_error {
        return Err(scalar(
            "achieved cone spatial bound",
            actual,
            "no greater than original requested spatial error",
        ));
    }
    Ok(())
}
fn verify_work(
    request: &ConeRun<'_>,
    family: &OwnedFamily<Selection, Geometry>,
    work: &storage::Work,
    reference_operations: u32,
) -> ConeChecked<()> {
    let members = member_operations(family)?;
    let total = members
        .checked_add(reference_operations)
        .and_then(|total| total.checked_add(work.actual_operations))
        .ok_or_else(work_overflow)?;
    if work.requested != request.proof.arithmetic || work.consumed != total {
        return Err(scalar(
            "aggregate arithmetic work",
            f64::from(work.consumed),
            "selection plus member and global reference work under original requested policy",
        ));
    }
    Ok(())
}
fn member_operations(family: &OwnedFamily<Selection, Geometry>) -> ConeChecked<u32> {
    family
        .members
        .values()
        .try_fold(family.selection.arithmetic_operations, |total, member| {
            total.checked_add(member.geometry.arithmetic_operations)
        })
        .ok_or_else(work_overflow)
}
fn work_overflow() -> ConeFailure {
    scalar(
        "aggregate arithmetic work",
        f64::INFINITY,
        "u32 operation count without overflow",
    )
}
fn volume_limit(actual: f64, limit: f64) -> ConeFailure {
    let cause = publication_budget::BudgetFailure::Limit {
        quantity: publication_budget::Quantity::AuthoredIdeal,
        actual,
        limit,
        region: None,
        span: None,
    };
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Budget(Box::new(cause)),
    )
    .at(WrappedStage::Publication)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn volume_policy_uses_reference_body_instead_of_conditional_proxy() {
        let mut volume = empty_volume();
        volume.discrepancy.upper = 100.;
        volume.proxy_reference_discrepancy.upper = 100.;
        volume.authored_reference_discrepancy.upper = 0.1;
        assert!(enforce(WrappedVolumePolicy::Refuse(0.15), &volume).is_ok());
        assert!(enforce(WrappedVolumePolicy::Refuse(0.05), &volume).is_err());
        assert!(enforce(WrappedVolumePolicy::Refuse(f64::NAN), &volume).is_err());
    }
}
