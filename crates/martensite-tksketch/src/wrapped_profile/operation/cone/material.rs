//! Filled-material evidence combines actual compact UV topology with the whole normal map.
use super::*;
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
mod binding;
mod domain;
mod reference;
pub(super) use reference::certify as certify_reference;
#[cfg(test)]
mod tests;
/// Every field is produced by an executed proof and remains bound to retained geometry.
pub(in crate::surface_sketch::wrapped_profile) struct MaterialEvidence {
    original: DevelopedRegionRequest,
    uv: planar::PlanarEvidence,
    normal: ConicalMaterialEvidence,
    carriers: [ConicalTrimCarrier; 2],
    options: ConeClearanceOptions,
    planar_proof: PlanarProofVersion,
    requested_planar_proof: PlanarProofVersion,
    pub(super) census: martensite_tkbo::profile_extrusion::RegionCertificate,
    pub(super) planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
}
impl MaterialEvidence {
    pub(super) fn proof_version(&self) -> PlanarProofVersion {
        self.planar_proof
    }

    pub(super) fn chart_admission_tolerance(&self) -> f64 {
        // The synthetic UV request preserves this scalar verbatim; it is not a world metric.
        self.original.tolerance
    }
    pub(super) fn spatial_error(&self) -> f64 {
        self.normal.spatial_error()
    }
}
pub(super) fn certify(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<MaterialEvidence> {
    let (uv, census, planar_work, planar_proof) =
        certify_uv(request, original, prepared, stored, work, cancelled)?;
    let options = work.remaining();
    let normal = certify_normal(original, prepared, stored, work, cancelled)?;
    Ok(MaterialEvidence {
        original: original.clone(),
        uv,
        normal,
        carriers: stored.carriers,
        options,
        planar_proof,
        requested_planar_proof: request.proof.planar_proof,
        census,
        planar_work,
    })
}
fn certify_uv(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(
    planar::PlanarEvidence,
    martensite_tkbo::profile_extrusion::RegionCertificate,
    martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    PlanarProofVersion,
)> {
    let uv_request = domain::request(prepared, &stored.spans)?;
    let captured = capture_uv(
        &uv_request,
        request.options.planar_work,
        request.proof.planar_proof,
        work,
        cancelled,
    )?;
    let version = selected_version(
        original.region,
        request.proof.planar_proof,
        captured.proof_version(),
    )?;
    let uv = planar::select(&uv_request, &captured)
        .map_err(|cause| planar_failure(original.region, cause))?;
    Ok((uv, captured.certificate(), captured.proof_work(), version))
}
fn capture_uv(
    source: &DevelopedRegionRequest,
    requested: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    version: PlanarProofVersion,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<martensite_tkbo::profile_extrusion::PlanarRegionSet> {
    let result = planar::capture_with_proof_and_cancel(
        std::slice::from_ref(source),
        source.tolerance,
        work.planar_remaining(requested),
        version,
        &mut *cancelled,
    );
    let receipt = match &result {
        Ok(captured) => Some(captured.proof_work()),
        Err(planar::PlanarFailure::WorkCapture(cause)) => cause.proof_work(),
        Err(_) => None,
    };
    if let Some(receipt) = receipt {
        work.charge_planar(source.region, receipt)?;
    }
    result.map_err(|cause| planar_failure(source.region, cause))
}

fn selected_version(
    region: SurfaceRegionId,
    requested: PlanarProofVersion,
    actual: PlanarProofVersion,
) -> ConeChecked<PlanarProofVersion> {
    if requested != actual {
        return Err(mismatch(
            region,
            None,
            "actual selected UV planar proof version",
        ));
    }
    Ok(actual)
}

fn certify_normal(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalMaterialEvidence> {
    let spans = binding::spans(original, prepared, &stored.spans)?;
    let query = binding::query(prepared, stored.carriers, work.remaining(), &spans);
    let result = certify_conical_material(query, &mut *cancelled);
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalMaterialEvidence::operations,
    );
    work.charge(original.region, None, operations)?;
    result.map_err(|cause| math_failure(original.region, &stored.spans, cause))
}

pub(super) fn validate_publication(family: &ConeBudgeted) -> ConeChecked<()> {
    for prepared in &family.family.selection.prepared {
        let region = prepared.source.region;
        let member = family
            .family
            .members
            .get(&region)
            .ok_or_else(|| storage::failure(region, None, ConeCause::Missing(region)))?;
        validate_member(prepared, &member.geometry, &family.family.selection)?;
    }
    reference::validate(family)
}
fn validate_member(
    prepared: &Prepared,
    geometry: &Geometry,
    selection: &Selection,
) -> ConeChecked<()> {
    let region = prepared.source.region;
    let material = &geometry.material;
    let original = selection
        .planar
        .get(&region)
        .ok_or_else(|| storage::failure(region, None, ConeCause::Missing(region)))?;
    if !original.matches_request(&material.original) {
        return Err(mismatch(region, None, "filled-material original binding"));
    }
    binding::validate(prepared, &geometry.spans, material)
}

fn planar_failure(region: SurfaceRegionId, cause: planar::PlanarFailure) -> ConeFailure {
    storage::failure(region, None, ConeCause::Planar(Box::new(cause)))
}
fn math_failure(
    region: SurfaceRegionId,
    spans: &[storage::SpanProof],
    cause: ConicalMaterialError,
) -> ConeFailure {
    let span = cause
        .span
        .and_then(|index| spans.get(index))
        .map(|proof| proof.span);
    storage::failure(region, span, ConeCause::Material(Box::new(cause)))
}
pub(super) fn mismatch(
    region: SurfaceRegionId,
    span: Option<CurveSpan>,
    field: &'static str,
) -> ConeFailure {
    storage::mismatch(region, span, field, vec![0], vec![1])
}
