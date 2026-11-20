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
type OriginalMembers<'a> = BTreeMap<SurfaceRegionId, &'a DevelopedRegionRequest>;

pub(super) fn finalize(
    execution: WrappedConeExecutionSelection,
    originals: &[DevelopedRegionRequest],
    prepared: &mut [Prepared],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    if !execution.is_directed() {
        return Ok(());
    }
    finalize_rows(
        originals,
        prepared,
        domain::refinement(execution),
        work,
        cancelled,
    )
}

pub(super) fn finalize_rows(
    originals: &[DevelopedRegionRequest],
    prepared: &mut [Prepared],
    refinement: ConicalPrimitiveRefinement,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    let inventory = Inventory::capture(prepared)?;
    let members = operation::source::by_region(originals);
    let gamma = gamma::certify(
        originals, &members, prepared, &inventory, refinement, work, cancelled,
    )?;
    let endpoints = std::sync::Arc::new(FinalEndpoints::plan(prepared, &gamma)?);
    let plan = DirectedPlan {
        gamma: &gamma,
        inventory: &inventory,
        endpoints: &endpoints,
    };
    finalize_members(&members, prepared, &plan, work, cancelled)?;
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
    originals: &OriginalMembers<'_>,
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
fn original<'a>(
    originals: &OriginalMembers<'a>,
    region: SurfaceRegionId,
) -> ConeChecked<&'a DevelopedRegionRequest> {
    originals
        .get(&region)
        .copied()
        .ok_or_else(|| missing(region, None, "directed original member owner"))
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

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn chart<'a>(
    prepared: &'a Prepared,
    span: CurveSpan,
    fit: &SpanFit,
) -> ConeChecked<&'a ConicalChartDomainEvidence> {
    prepared
        .boundary_metric(span, fit.local_chart)?
        .chart
        .ok_or_else(|| {
            missing(
                prepared.source.region,
                Some(span),
                "directed chart domain authority",
            )
        })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn fit<'a>(
    prepared: &'a Prepared,
    piece: &AuthoredPiece,
) -> ConeChecked<&'a SpanFit> {
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
    request: &ConeRun<'_>,
    execution: WrappedConeExecutionSelection,
) -> ConeChecked<()> {
    use martensite_tkbo::profile_extrusion::PlanarProofVersion;
    if (execution.is_directed() || execution.is_connected_atlas())
        && !matches!(
            request.proof.planar_proof,
            PlanarProofVersion::FilteredCompositeBernsteinV5 { .. }
                | PlanarProofVersion::GeometricCompositeV6 { .. }
        )
    {
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
