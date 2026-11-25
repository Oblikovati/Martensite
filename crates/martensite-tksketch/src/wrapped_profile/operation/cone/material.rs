//! Filled-material evidence combines actual compact UV topology with the whole normal map.
use super::*;
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
pub(super) mod atlas;
pub(super) type NativeCurveMap = Vec<(
    martensite_tkbo::profile_extrusion::ProfileCurveId,
    ConicalActualSideId,
)>;
mod binding;
mod directed;
mod domain;
pub(super) use domain::curve_ids as source_curve_ids;
#[cfg(test)]
pub(super) use domain::periodic_requests;
mod reference;
pub(super) use reference::certify as certify_reference;
#[cfg(test)]
mod tests;
/// Every field is produced by an executed proof and remains bound to retained geometry.
pub(in crate::surface_sketch::wrapped_profile) struct MaterialEvidence {
    original: DevelopedRegionRequest,
    uv: UvMaterial,
    pub(super) selected_material: Option<martensite_tkbo::body_check::CertifiedPlanarMaterial>,
    pub(super) periodic_curves: Option<NativeCurveMap>,
    normal: MaterialRealization,
    carriers: [ConicalTrimCarrier; 2],
    options: ConeClearanceOptions,
    planar_proof: PlanarProofVersion,
    requested_planar_proof: PlanarProofVersion,
    pub(super) census: martensite_tkbo::profile_extrusion::RegionCertificate,
    pub(super) planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
}
enum UvMaterial {
    Region(planar::PlanarEvidence),
    Directed(martensite_tkbo::body_check::DirectedCapWork),
}
impl UvMaterial {
    fn integral_program_evidence(
        &self,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarIntegralEvidence> {
        match self {
            Self::Region(region) => region.integral_program_evidence(),
            Self::Directed(_) => None,
        }
    }
}
enum MaterialRealization {
    Common(ConicalMaterialEvidence),
    Decoded(f64),
}
impl MaterialEvidence {
    pub(in crate::surface_sketch::wrapped_profile) fn carriers(&self) -> [ConicalTrimCarrier; 2] {
        self.carriers
    }

    pub(in crate::surface_sketch::wrapped_profile) fn integral_program_evidence(
        &self,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarIntegralEvidence> {
        self.uv.integral_program_evidence()
    }
    pub(in crate::surface_sketch::wrapped_profile) fn directed_cap_work(
        &self,
    ) -> Option<martensite_tkbo::body_check::DirectedCapWork> {
        match &self.uv {
            UvMaterial::Directed(work) => Some(*work),
            UvMaterial::Region(_) => None,
        }
    }

    pub(super) fn proof_version(&self) -> PlanarProofVersion {
        self.planar_proof
    }

    pub(super) fn chart_admission_tolerance(&self) -> f64 {
        // The synthetic UV request preserves this scalar verbatim; it is not a world metric.
        self.original.tolerance
    }
    pub(super) fn spatial_error(&self) -> f64 {
        match &self.normal {
            MaterialRealization::Common(proof) => proof.spatial_error(),
            MaterialRealization::Decoded(error) => *error,
        }
    }
}
pub(super) fn certify(
    request: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<MaterialEvidence> {
    let captured = certify_uv(request, original, prepared, stored, work, cancelled)?;
    let options = work.remaining();
    let normal = certify_normal(original, prepared, stored, work, cancelled)?;
    Ok(MaterialEvidence {
        original: original.clone(),
        uv: captured.uv,
        selected_material: captured.selected,
        periodic_curves: None,
        normal: MaterialRealization::Common(normal),
        carriers: stored.carriers,
        options,
        planar_proof: captured.version,
        requested_planar_proof: request.proof.planar_proof,
        census: captured.census,
        planar_work: captured.work,
    })
}
pub(super) fn certify_attachment_uv(
    request: &ConeRun<'_>,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(
    martensite_tkbo::body_check::CertifiedPlanarMaterial,
    martensite_tkbo::body_check::DirectedCapWork,
)> {
    let uv_request = domain::request(prepared, &stored.spans)?;
    let captured = directed::certify(request, &uv_request, work, cancelled)?;
    let (Some(selected), UvMaterial::Directed(observed)) = (captured.selected, captured.uv) else {
        return Err(mismatch(
            prepared.source.region,
            None,
            "directed attachment cap authority",
        ));
    };
    Ok((selected, observed))
}

fn certify_uv(
    request: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CapturedUvMaterial> {
    let uv_request = domain::request(prepared, &stored.spans)?;
    if prepared.executed_selection.uses_directed_cap_cycles() {
        return directed::certify(request, &uv_request, work, cancelled);
    }
    let captured = capture_uv(
        &uv_request,
        request.options.planar_work,
        request.proof.planar_proof,
        work,
        request.proof.integration,
        cancelled,
    )?;
    finish_uv(request, original, prepared, &uv_request, captured)
}
fn finish_uv(
    request: &ConeRun<'_>,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    uv_request: &DevelopedRegionRequest,
    captured: martensite_tkbo::profile_extrusion::PlanarRegionSet,
) -> ConeChecked<CapturedUvMaterial> {
    let version = selected_version(
        original.region,
        request.proof.planar_proof,
        captured.proof_version(),
    )?;
    let uv = planar::select(uv_request, &captured)
        .map_err(|cause| planar_failure(original.region, cause))?;
    let selected = retain_selected_material(prepared, &uv, &captured)?;
    Ok(CapturedUvMaterial {
        uv: UvMaterial::Region(uv),
        selected,
        census: captured.certificate(),
        work: captured.proof_work(),
        version,
    })
}
fn capture_uv(
    source: &DevelopedRegionRequest,
    requested: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    version: PlanarProofVersion,
    work: &mut storage::Work,
    integration: CylindricalFluxOptions,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<martensite_tkbo::profile_extrusion::PlanarRegionSet> {
    if matches!(
        version,
        PlanarProofVersion::FilteredCompositeBernsteinV5 { .. }
            | PlanarProofVersion::GeometricCompositeV6 { .. }
    ) {
        return capture_program_uv(
            std::slice::from_ref(source),
            requested,
            version,
            work,
            integration,
            cancelled,
        );
    }
    capture_legacy_uv(source, requested, version, work, cancelled)
}
fn capture_legacy_uv(
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
    let compatible = compatible_planar_modes(requested, actual);
    if !compatible {
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
    if prepared.chart_domain.is_some() {
        return developed::certify(query, original, prepared, &stored.spans, work, cancelled);
    }
    let result = certify_conical_material(query, &mut *cancelled);
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalMaterialEvidence::operations,
    );
    work.charge(original.region, None, operations)?;
    result.map_err(|cause| math_failure(original.region, &stored.spans, cause))
}

pub(super) fn validate_publication(
    family: &ConeBudgeted,
    integration_limit: u64,
) -> ConeChecked<()> {
    if let Some(atlas) = &family.budget.connected {
        return atlas::validate(family, atlas, integration_limit);
    }
    program::validate_family(family, integration_limit)?;
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
    if !selection.authored_member_matches(original, &material.original) {
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

mod developed;
mod program;
pub(super) use program::capture_planar_program;
#[cfg(test)]
pub(super) use program::capture_program_uv as capture_periodic_program;
use program::capture_program_uv;

fn compatible_planar_modes(requested: PlanarProofVersion, actual: PlanarProofVersion) -> bool {
    match (composite_options(requested), composite_options(actual)) {
        (Some(requested_options), Some(actual_options)) => {
            std::mem::discriminant(&requested) == std::mem::discriminant(&actual)
                && requested_options.0 == actual_options.0
                && actual_options.1 <= requested_options.1
        }
        _ => requested == actual,
    }
}

fn composite_options(version: PlanarProofVersion) -> Option<(u16, u64)> {
    match version {
        PlanarProofVersion::FilteredCompositeBernsteinV5 {
            max_components,
            max_total_subintervals,
        }
        | PlanarProofVersion::GeometricCompositeV6 {
            max_components,
            max_total_subintervals,
        } => Some((max_components, max_total_subintervals)),
        _ => None,
    }
}

struct CapturedUvMaterial {
    uv: UvMaterial,
    selected: Option<martensite_tkbo::body_check::CertifiedPlanarMaterial>,
    census: martensite_tkbo::profile_extrusion::RegionCertificate,
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    version: PlanarProofVersion,
}
fn retain_selected_material(
    prepared: &Prepared,
    uv: &planar::PlanarEvidence,
    captured: &martensite_tkbo::profile_extrusion::PlanarRegionSet,
) -> ConeChecked<Option<martensite_tkbo::body_check::CertifiedPlanarMaterial>> {
    if !super::domain::directed(prepared) {
        return Ok(None);
    }
    martensite_tkbo::body_check::CertifiedPlanarMaterial::from_region(captured, uv.selected_region)
        .map(Some)
        .map_err(|cause| {
            storage::failure(
                prepared.source.region,
                None,
                ConeCause::Native(Box::new(cause)),
            )
        })
}
