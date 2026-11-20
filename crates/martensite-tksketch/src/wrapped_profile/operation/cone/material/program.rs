//! A V5 UV program borrows and charges the same finite operation integration allowance.
use super::*;
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn capture_program_uv(
    sources: &[impl AsRef<DevelopedRegionRequest>],
    requested: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    version: PlanarProofVersion,
    work: &mut storage::Work,
    integration: CylindricalFluxOptions,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<martensite_tkbo::profile_extrusion::PlanarRegionSet> {
    let source = sources.first().map(AsRef::as_ref).ok_or_else(|| {
        ConeFailure::new(
            None,
            None,
            WrappedFailureClass::Incomplete,
            ConeCause::Scalar {
                field: "UV program members",
                actual: 0.,
                expected: "at least one decoded member",
            },
        )
    })?;
    let remaining = u64::from(work.integration(integration).max_total_subintervals);
    let result = planar::capture_with_integration_budget_and_cancel(
        sources,
        source.tolerance,
        work.planar_remaining(requested),
        version,
        remaining,
        &mut *cancelled,
    );
    let (planar, cells) = observed(&result);
    let charged = charge(work, source.region, planar, cells, integration);
    let captured = result.map_err(|cause| planar_failure(source.region, cause))?;
    charged?;
    validate_requested_program(source.region, &captured, version)?;
    Ok(captured)
}
fn observed(
    result: &Result<martensite_tkbo::profile_extrusion::PlanarRegionSet, planar::PlanarFailure>,
) -> (
    Option<martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt>,
    u64,
) {
    match result {
        Ok(captured) => (
            Some(captured.proof_work()),
            captured
                .integral_program_evidence()
                .map_or(0, |proof| proof.integration_subintervals()),
        ),
        Err(planar::PlanarFailure::WorkCapture(cause)) => (
            cause.proof_work(),
            cause
                .integral_program_failure()
                .map_or(0, |proof| proof.integration_subintervals()),
        ),
        Err(_) => (None, 0),
    }
}
fn charge(
    work: &mut storage::Work,
    region: SurfaceRegionId,
    planar: Option<martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt>,
    cells: u64,
    integration: CylindricalFluxOptions,
) -> ConeChecked<()> {
    let integration_charge = work.charge_integration(region, cells, integration);
    let planar_charge = planar
        .map(|receipt| work.charge_planar(region, receipt))
        .transpose();
    integration_charge?;
    planar_charge?;
    Ok(())
}

pub(super) fn validate_family(family: &ConeBudgeted, limit: u64) -> ConeChecked<()> {
    if !has_program_members(family) {
        return Ok(());
    }
    let mut consumed = 0u64;
    for prepared in &family.family.selection.prepared {
        let region = prepared.source.region;
        consumed = validate_member_loan(family, region, limit, consumed)?;
    }
    if consumed != u64::from(family.budget.integration_subintervals) {
        return Err(mismatch(
            family.family.selection.prepared[0].source.region,
            None,
            "aggregate UV and flux integration conservation",
        ));
    }
    Ok(())
}
fn validate_entry(
    region: SurfaceRegionId,
    proof: &martensite_tkbo::profile_extrusion::PlanarIntegralEvidence,
    limit: u64,
    consumed: u64,
) -> ConeChecked<()> {
    if limit.checked_sub(consumed) != Some(proof.entry_allowance()) {
        return Err(mismatch(
            region,
            None,
            "ordered operation integral entry allowance",
        ));
    }
    Ok(())
}
fn member_cells(
    region: SurfaceRegionId,
    consumed: u64,
    uv: u64,
    geometry: &Geometry,
) -> ConeChecked<u64> {
    consumed
        .checked_add(uv)
        .and_then(|total| total.checked_add(geometry.reference_volume.integration_subintervals()))
        .and_then(|total| total.checked_add(geometry.flux.integration_subintervals()))
        .ok_or_else(|| mismatch(region, None, "aggregate integration cell overflow"))
}

fn validate_requested_program(
    region: SurfaceRegionId,
    captured: &martensite_tkbo::profile_extrusion::PlanarRegionSet,
    version: PlanarProofVersion,
) -> ConeChecked<()> {
    if captured.requested_proof_version() != version {
        return Err(mismatch(
            region,
            None,
            "original requested UV integral policy",
        ));
    }
    Ok(())
}

fn has_program_members(family: &ConeBudgeted) -> bool {
    family.family.members.values().next().is_some_and(|member| {
        member
            .geometry
            .material
            .uv
            .integral_program_evidence()
            .is_some()
    })
}
fn validate_member_loan(
    family: &ConeBudgeted,
    region: SurfaceRegionId,
    limit: u64,
    consumed: u64,
) -> ConeChecked<u64> {
    let member = family
        .family
        .members
        .get(&region)
        .ok_or_else(|| mismatch(region, None, "integral member owner"))?;
    let geometry = &member.geometry;
    let proof = geometry
        .material
        .uv
        .integral_program_evidence()
        .ok_or_else(|| mismatch(region, None, "V5 member integral evidence"))?;
    validate_entry(region, proof, limit, consumed)?;
    member_cells(region, consumed, proof.integration_subintervals(), geometry)
}
