//! Family-directed rows are finalized once before native construction or dependent proofs.
use super::*;
mod endpoints;
mod gamma;
mod inventory;
mod transition;
pub(in crate::surface_sketch::wrapped_profile) use endpoints::FinalEndpoints;
pub(in crate::surface_sketch::wrapped_profile) use inventory::Inventory;
pub(in crate::surface_sketch::wrapped_profile) struct DirectedFamily {
    pub gamma: std::sync::Arc<ConicalDirectedGammaEvidence>,
    pub inventory: Inventory,
}
pub(super) fn finalize(
    execution: WrappedConeExecutionSelection,
    originals: &[DevelopedRegionRequest],
    prepared: &mut [Prepared],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    if !matches!(
        execution,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1
    ) {
        return Ok(());
    }
    let inventory = Inventory::capture(prepared)?;
    let gamma = gamma::certify(originals, prepared, &inventory, work, cancelled)?;
    let endpoints = std::sync::Arc::new(FinalEndpoints::plan(prepared, &gamma)?);
    let plan = DirectedPlan {
        gamma: &gamma,
        inventory: &inventory,
        endpoints: &endpoints,
    };
    finalize_members(originals, prepared, &plan, work, cancelled)?;
    endpoints.validate(prepared)?;
    retain(prepared, DirectedFamily { gamma, inventory }, endpoints);
    Ok(())
}
struct DirectedPlan<'a> {
    gamma: &'a ConicalDirectedGammaEvidence,
    inventory: &'a Inventory,
    endpoints: &'a FinalEndpoints,
}
fn finalize_members(
    originals: &[DevelopedRegionRequest],
    prepared: &mut [Prepared],
    plan: &DirectedPlan<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    for member in prepared.iter_mut() {
        transition::member(
            original(originals, member.source.region)?,
            member,
            plan,
            work,
            cancelled,
        )?;
    }
    Ok(())
}
fn retain(
    prepared: &mut [Prepared],
    family: DirectedFamily,
    endpoints: std::sync::Arc<FinalEndpoints>,
) {
    let family = std::sync::Arc::new(family);
    for member in prepared {
        member.final_endpoints = Some(std::sync::Arc::clone(&endpoints));
        member.directed_family = Some(std::sync::Arc::clone(&family));
    }
}

fn original(
    originals: &[DevelopedRegionRequest],
    region: SurfaceRegionId,
) -> ConeChecked<&DevelopedRegionRequest> {
    originals
        .iter()
        .find(|source| source.region == region)
        .ok_or_else(|| missing(region, None, "directed original member owner"))
}
fn chart(prepared: &Prepared) -> ConeChecked<&ConicalChartDomainEvidence> {
    prepared.chart_domain.as_deref().ok_or_else(|| {
        missing(
            prepared.source.region,
            None,
            "directed chart domain authority",
        )
    })
}
fn fit<'a>(prepared: &'a Prepared, piece: &AuthoredPiece) -> ConeChecked<&'a SpanFit> {
    prepared.fits.get(&piece.span).ok_or_else(|| {
        missing(
            prepared.source.region,
            Some(piece.span),
            "directed nearest fit owner",
        )
    })
}
fn missing(region: SurfaceRegionId, span: Option<CurveSpan>, field: &'static str) -> ConeFailure {
    failure(
        Some(region),
        span,
        ConeCause::Binding {
            field,
            actual: vec![],
            expected: vec![1],
        },
    )
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn actual_failure(
    cause: ConicalActualPatchError,
    inventory: &Inventory,
) -> ConeFailure {
    let (region, span) = inventory.owner(&cause);
    failure(region, span, inventory.math_cause(cause))
}

pub(in super::super) fn admit(
    request: &WrappedConeNormalRequest,
    execution: WrappedConeExecutionSelection,
) -> ConeChecked<()> {
    use martensite_tkbo::profile_extrusion::PlanarProofVersion;
    if matches!(
        execution,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1
    ) && !matches!(
        request.proof.planar_proof,
        PlanarProofVersion::FilteredCompositeBernsteinV5 { .. }
            | PlanarProofVersion::GeometricCompositeV6 { .. }
    ) {
        return Err(ConeFailure::new(
            None,
            None,
            WrappedFailureClass::Unsupported,
            ConeCause::ExecutionProof(Box::new(request.proof.planar_proof)),
        )
        .at(WrappedStage::Admission));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
