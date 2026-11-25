//! Compare every retained spatial bound with its own allocated share.
use super::*;
pub(super) fn certify(
    original: &Request,
    family: &family::UnqualifiedFamily,
    limits: BudgetLimits,
) -> Checked<MemberBounds> {
    source(original, family, limits.spatial)?;
    let retained = family
        .members
        .get(&original.region)
        .ok_or(BudgetFailure::Source(original.region))?;
    let prepared = prepared(family, original.region)?;
    let expected = prepared.loops.iter().map(Vec::len).sum();
    inventory(original.region, retained, expected)?;
    cone_binding(original, prepared, retained)?;
    native::certify(original.region, &retained.native.report, limits.spatial)?;
    bounds(retained, original, limits)
}
fn cone_binding(
    original: &Request,
    prepared: &Request,
    retained: &pipeline::RetainedMember,
) -> Checked<()> {
    planar_cones::bind(
        original,
        prepared,
        &retained.geometry.storage,
        &retained.geometry.cones,
    )
    .map_err(|cause| BudgetFailure::Cones {
        region: original.region,
        cause: Box::new(cause),
    })
}

fn prepared(family: &family::UnqualifiedFamily, region: SurfaceRegionId) -> Checked<&Request> {
    family
        .selection
        .prepared
        .prepared
        .iter()
        .find(|r| r.region == region)
        .ok_or(BudgetFailure::Source(region))
}
fn context(r: &Request) -> [u64; 5] {
    [
        r.radius.to_bits(),
        r.depths[0].to_bits(),
        r.depths[1].to_bits(),
        r.tolerance.to_bits(),
        u64::from(r.max_fit_level),
    ]
}
fn source(original: &Request, family: &family::UnqualifiedFamily, tolerance: f64) -> Checked<()> {
    let evidence = family
        .selection
        .prepared
        .planar
        .members
        .get(&original.region)
        .ok_or(BudgetFailure::Source(original.region))?;
    let retained = prepared(family, original.region)?;
    if !evidence.matches_member(original)
        || context(original) != context(retained)
        || original.tolerance.to_bits() != tolerance.to_bits()
    {
        return Err(BudgetFailure::Source(original.region));
    }
    Ok(())
}
fn count(
    region: SurfaceRegionId,
    field: &'static str,
    actual: usize,
    expected: usize,
) -> Checked<()> {
    if actual != expected {
        return Err(BudgetFailure::Coverage {
            region,
            field,
            actual,
            expected,
        });
    }
    Ok(())
}
fn inventory(
    region: SurfaceRegionId,
    retained: &pipeline::RetainedMember,
    expected: usize,
) -> Checked<()> {
    for (field, actual) in [
        ("storage spans", retained.geometry.storage.len()),
        ("regularity cones", retained.geometry.cones.pieces.len()),
        ("join cones", retained.geometry.cones.joins.len()),
    ] {
        count(region, field, actual, expected)?;
    }
    matching_measure_spans(region, retained)
}
fn matching_measure_spans(
    region: SurfaceRegionId,
    retained: &pipeline::RetainedMember,
) -> Checked<()> {
    let spans: Vec<_> = retained.geometry.storage.iter().map(|p| p.span).collect();
    if spans != retained.geometry.measures.spans {
        return Err(span_failure(
            region,
            retained.geometry.measures.spans.clone(),
            spans,
        ));
    }
    Ok(())
}
fn span_failure(
    region: SurfaceRegionId,
    actual: Vec<CurveSpan>,
    expected: Vec<CurveSpan>,
) -> BudgetFailure {
    BudgetFailure::Spans {
        region,
        actual,
        expected,
    }
}
pub(super) fn clearance(family: &family::UnqualifiedFamily, required: f64) -> Checked<()> {
    let proof = &family.selection.prepared.boundary_clearance;
    if proof.certificate.clearance_lower_bound() < required
        || !proof
            .periodic
            .separation_lower_bound()
            .compare_f64(required)
            .is_ok_and(|order| order.is_ge())
    {
        return Err(clearance_failure(proof, required));
    }
    Ok(())
}
fn clearance_failure(proof: &planar_clearance::OriginalClearance, required: f64) -> BudgetFailure {
    BudgetFailure::Clearance {
        required,
        within: proof.certificate.clearance_lower_bound(),
        periodic: Box::new(proof.periodic.separation_lower_bound().clone()),
    }
}
fn bounds(
    retained: &pipeline::RetainedMember,
    original: &Request,
    limits: BudgetLimits,
) -> Checked<MemberBounds> {
    let region = original.region;
    let mut bounds = MemberBounds::new(retained);
    for proof in &retained.geometry.storage {
        if !proof.profile.matches_reference_curve(&proof.cap_profile) {
            return Err(BudgetFailure::Source(region));
        }
        bounds.add(proof, region, limits)?;
    }
    bounds.cap_realization = cap_realization::certify(original, retained, limits.spatial)?;
    Ok(bounds)
}
impl MemberBounds {
    fn new(retained: &pipeline::RetainedMember) -> Self {
        Self {
            profile: 0.,
            storage: 0.,
            trace: 0.,
            trace_tolerance: 0.,
            cap_realization: 0.,
            native_modes: retained
                .native
                .report
                .stages
                .each_ref()
                .map(|stage| stage.mode),
        }
    }

    fn add(
        &mut self,
        proof: &storage::StoredProof,
        region: SurfaceRegionId,
        limits: BudgetLimits,
    ) -> Checked<()> {
        self.positions(proof, region, limits)?;
        self.traces(proof, region, limits)
    }
    fn positions(
        &mut self,
        proof: &storage::StoredProof,
        region: SurfaceRegionId,
        limits: BudgetLimits,
    ) -> Checked<()> {
        let span = Some(proof.span);
        let profile = proof.profile.developed_position_bound();
        let storage = proof.depths.normal_sweep_error_bound();
        let profile = upper(profile, limits.profile, Quantity::Profile, region, span)?;
        let storage = upper(storage, limits.storage, Quantity::Storage, region, span)?;
        self.profile = self.profile.max(profile);
        self.storage = self.storage.max(storage);
        Ok(())
    }
    fn traces(
        &mut self,
        proof: &storage::StoredProof,
        region: SurfaceRegionId,
        limits: BudgetLimits,
    ) -> Checked<()> {
        for cap in &proof.trims.cap_residuals {
            self.trace = self
                .trace
                .max(cap_bound(cap, region, proof.span, limits.trace)?);
        }
        let actual = proof.trims.maximum_tolerance;
        let span = Some(proof.span);
        let tolerance = upper(actual, limits.trace, Quantity::TraceTolerance, region, span)?;
        self.trace_tolerance = self.trace_tolerance.max(tolerance);
        Ok(())
    }
}
fn cap_bound(
    cap: &martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalTrim,
    region: SurfaceRegionId,
    span: CurveSpan,
    limit: f64,
) -> Checked<f64> {
    let requested = cap.requested_spatial_limit();
    upper(requested, limit, Quantity::Trace, region, Some(span))?;
    upper(
        cap.spatial_residual_bound(),
        requested,
        Quantity::Trace,
        region,
        Some(span),
    )
}
