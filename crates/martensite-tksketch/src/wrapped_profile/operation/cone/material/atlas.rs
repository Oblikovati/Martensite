//! Bind periodic filled caps once, then loan one reference receipt to each decoded member.
use super::super::super::stages::OwnedFamily;
use super::super::prepare::selection::first_region;
use super::*;
use crate::surface_sketch::wrapped_profile::{planar_clearance, selection as periodic};
use measures::MemberReferenceVolume;

type Members = BTreeMap<SurfaceRegionId, (MaterialEvidence, MemberReferenceVolume)>;
pub(in crate::surface_sketch::wrapped_profile::operation::cone) struct Captured {
    pub reference: ConicalReferenceEvidence,
    pub binding: Binding,
    members: Members,
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) struct Binding {
    boundary: planar_clearance::OriginalClearance,
    material: periodic::PeriodicMaterialEvidence,
    pub operations: u32,
}
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum Refusal {
    Clearance(planar_clearance::ClearanceFailure),
    Material(periodic::MaterialFailure),
    Reference(ConicalDecodedFamilyError),
}
impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clearance(cause) => cause.fmt(f),
            Self::Material(cause) => cause.fmt(f),
            Self::Reference(cause) => cause.fmt(f),
        }
    }
}
impl std::error::Error for Refusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reference(cause) => Some(cause),
            _ => None,
        }
    }
}
impl Refusal {
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn cancelled(&self) -> bool {
        match self {
            Self::Clearance(cause) => cause.cancelled(),
            Self::Material(cause) => cause.cancelled(),
            Self::Reference(cause) => cause.is_cancelled(),
        }
    }
}
impl Captured {
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn take_member(
        &mut self,
        region: SurfaceRegionId,
    ) -> ConeChecked<(MaterialEvidence, MemberReferenceVolume)> {
        self.members
            .remove(&region)
            .ok_or_else(|| mismatch(region, None, "one issued atlas member"))
    }

    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn finish(
        self,
        region: SurfaceRegionId,
    ) -> ConeChecked<(ConicalReferenceEvidence, Binding)> {
        if !self.members.is_empty() {
            return Err(mismatch(
                region,
                None,
                "every decoded atlas member consumed exactly once",
            ));
        }
        Ok((self.reference, self.binding))
    }
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn capture(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    selection: &Selection,
    stored: &BTreeMap<SurfaceRegionId, storage::Stored>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Captured> {
    let region = first_region(originals)?;
    let started = work.consumed;
    let members = ordered_members(selection, stored)?;
    let profiles = projected(&members)?;
    let (planar, version) = capture_planar(request, &profiles, work, cancelled)?;
    let boundary = clearance(&profiles, region, work, cancelled)?;
    let material = winding(&profiles, &planar, &boundary, region, work, cancelled)?;
    let reference = reference(request, &members, &boundary, region, work, cancelled)?;
    let members = member_receipts(
        originals,
        &members,
        planar,
        version,
        request.proof.planar_proof,
        &reference,
    )?;
    Ok(Captured {
        reference,
        members,
        binding: Binding {
            boundary,
            material,
            operations: work.consumed - started,
        },
    })
}
type Member<'a> = (&'a Prepared, &'a storage::Stored);
fn ordered_members<'a>(
    selection: &'a Selection,
    stored: &'a BTreeMap<SurfaceRegionId, storage::Stored>,
) -> ConeChecked<Vec<Member<'a>>> {
    selection
        .prepared
        .iter()
        .map(|prepared| {
            let id = prepared.source.region;
            let stored = stored
                .get(&id)
                .ok_or_else(|| mismatch(id, None, "decoded atlas member"))?;
            Ok((prepared, stored))
        })
        .collect()
}
fn projected(members: &[Member<'_>]) -> ConeChecked<Vec<Request>> {
    let spans = members
        .iter()
        .map(|&(p, s)| (p, s.spans.as_slice()))
        .collect::<Vec<_>>();
    domain::periodic_requests(&spans)
}
fn capture_planar(
    request: &ConeRun<'_>,
    profiles: &[Request],
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(periodic::PlanarSelectionEvidence, PlanarProofVersion)> {
    let captured = capture_program_uv(
        profiles,
        request.options.planar_work,
        request.proof.planar_proof,
        work,
        request.proof.integration,
        cancelled,
    )?;
    let version = selected_version(
        profiles[0].region,
        request.proof.planar_proof,
        captured.proof_version(),
    )?;
    let members = planar::select_members(profiles, &captured)
        .map_err(|(region, cause)| planar_failure(region, cause))?;
    Ok((
        periodic::PlanarSelectionEvidence {
            members,
            certificate: captured.certificate(),
            proof_work: captured.proof_work(),
        },
        version,
    ))
}
fn clearance(
    profiles: &[Request],
    region: SurfaceRegionId,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<planar_clearance::OriginalClearance> {
    let policy = work.remaining();
    let result =
        planar_clearance::certify_many_with_limit(profiles, profiles, policy, &mut *cancelled);
    let spent = result.as_ref().map_or_else(
        |error| error.operations,
        |done| done.certificate.operations() + done.periodic.operations(),
    );
    let charged = work.charge(region, None, spent);
    let boundary = result.map_err(|error| refused(None, Refusal::Clearance(error.cause)))?;
    charged?;
    Ok(boundary)
}
fn winding(
    profiles: &[Request],
    planar: &periodic::PlanarSelectionEvidence,
    boundary: &planar_clearance::OriginalClearance,
    region: SurfaceRegionId,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<periodic::PeriodicMaterialEvidence> {
    let remaining = work.remaining();
    let mut options = periodic::MaterialOptions::default();
    options.probes.fractional_bits = remaining.fractional_bits;
    options.probes.max_operations = remaining.max_operations;
    let result = periodic::material::certify_projected_with_cancel(
        profiles,
        planar,
        boundary,
        options,
        &mut *cancelled,
    );
    let spent = result
        .as_ref()
        .map_or_else(|error| error.operations, |done| done.probes.operations());
    let charged = work.charge(region, None, spent);
    let material = result.map_err(|error| refused(None, Refusal::Material(error.cause)))?;
    charged?;
    Ok(material)
}
fn reference(
    request: &ConeRun<'_>,
    members: &[Member<'_>],
    boundary: &planar_clearance::OriginalClearance,
    region: SurfaceRegionId,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalReferenceEvidence> {
    let joins: Vec<_> = members.iter().map(|(_, stored)| &stored.joins).collect();
    let policy = ConeClearanceOptions {
        max_operations: work.requested.max_operations - work.consumed,
        ..work.requested
    };
    let result = certify_conical_decoded_family(
        &joins,
        &boundary.certificate,
        &boundary.periodic,
        request.options.spatial_error,
        policy,
        cancelled,
    );
    let spent = result.as_ref().map_or_else(
        |error| error.operations,
        ConicalReferenceEvidence::operations,
    );
    let charged = work.charge(region, None, spent);
    let reference = result.map_err(|error| {
        let owner = error
            .member
            .and_then(|index| members.get(index))
            .map(|(prepared, _)| prepared.source.region);
        refused(owner, Refusal::Reference(error))
    })?;
    charged?;
    Ok(reference)
}
fn member_receipts(
    originals: &[DevelopedRegionRequest],
    members: &[Member<'_>],
    mut planar: periodic::PlanarSelectionEvidence,
    version: PlanarProofVersion,
    requested: PlanarProofVersion,
    reference: &ConicalReferenceEvidence,
) -> ConeChecked<Members> {
    let region = first_region(originals)?;
    let caps = reference
        .decoded_caps()
        .filter(|caps| caps.len() == members.len() && originals.len() == members.len())
        .ok_or_else(|| mismatch(region, None, "complete decoded atlas cap receipts"))?;
    originals
        .iter()
        .zip(members)
        .zip(caps)
        .map(|((original, (prepared, stored)), cap)| {
            let region = original.region;
            if region != prepared.source.region {
                return Err(mismatch(region, None, "atlas receipt source order"));
            }
            let receipts = member_receipt(original, stored, cap, &mut planar, version, requested)?;
            Ok((region, receipts))
        })
        .collect()
}
fn member_receipt(
    original: &DevelopedRegionRequest,
    stored: &storage::Stored,
    cap: &ConicalDecodedCapEvidence,
    planar: &mut periodic::PlanarSelectionEvidence,
    version: PlanarProofVersion,
    requested: PlanarProofVersion,
) -> ConeChecked<(MaterialEvidence, MemberReferenceVolume)> {
    let uv = planar
        .members
        .remove(&original.region)
        .ok_or_else(|| mismatch(original.region, None, "selected decoded cap material"))?;
    let volume = volume_bounds(cap)?;
    if volume[0] <= 0. {
        return Err(mismatch(
            original.region,
            None,
            "positive signed decoded reference volume",
        ));
    }
    let material = MaterialEvidence {
        original: original.clone(),
        uv,
        selected_material: None,
        normal: MaterialRealization::Decoded(cap_error(cap)?),
        carriers: stored.carriers,
        options: cap.normal().options(),
        planar_proof: version,
        requested_planar_proof: requested,
        census: planar.certificate,
        planar_work: planar.proof_work,
    };
    Ok((material, MemberReferenceVolume::Decoded(volume)))
}
fn volume_bounds(cap: &ConicalDecodedCapEvidence) -> ConeChecked<[f64; 2]> {
    let range = measures::range(cap.normal().volume().clone())?;
    Ok([range.lower, range.upper])
}
fn cap_error(cap: &ConicalDecodedCapEvidence) -> ConeChecked<f64> {
    let [first, last] = cap.cap_errors();
    measures::upper(first.max(last))
}
fn refused(region: Option<SurfaceRegionId>, cause: Refusal) -> ConeFailure {
    ConeFailure::new(
        region,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Atlas(Box::new(cause)),
    )
}

pub(super) fn validate(family: &ConeBudgeted, binding: &Binding, limit: u64) -> ConeChecked<()> {
    let prepared = &family.family.selection.prepared;
    let region = prepared
        .first()
        .ok_or_else(|| {
            refused(
                None,
                Refusal::Clearance(planar_clearance::ClearanceFailure::EmptySelection),
            )
        })?
        .source
        .region;
    let reference = &family.budget.reference;
    let caps = reference
        .decoded_caps()
        .filter(|caps| caps.len() == prepared.len())
        .ok_or_else(|| mismatch(region, None, "complete published decoded atlas"))?;
    let joins = prepared.iter().filter_map(|prepared| {
        family
            .family
            .members
            .get(&prepared.source.region)
            .map(|member| &member.geometry.joins)
    });
    if !reference.shares_decoded_join_issuance(joins) {
        return Err(mismatch(
            region,
            None,
            "complete decoded atlas reference join issuance",
        ));
    }
    relations(reference, binding, region, caps.len())?;
    let mut cells = PublishedCells {
        integral: None,
        total: 0,
    };
    for (prepared, cap) in prepared.iter().zip(caps) {
        cells.member(&family.family, prepared, cap, limit)?;
    }
    if cells.total != u64::from(family.budget.integration_subintervals) {
        return Err(mismatch(
            region,
            None,
            "one decoded atlas UV program and retained flux cell total",
        ));
    }
    Ok(())
}
fn relations(
    reference: &ConicalReferenceEvidence,
    binding: &Binding,
    region: SurfaceRegionId,
    count: usize,
) -> ConeChecked<()> {
    if !binding
        .boundary
        .periodic
        .matches_boundary(&binding.boundary.certificate)
        || binding.material.probes.probes().len() != binding.material.relations.len()
        || (count > 1 && reference.decoded_solid_separation().is_none())
    {
        return Err(mismatch(
            region,
            None,
            "complete decoded atlas periodic and native solid relations",
        ));
    }
    Ok(())
}
struct PublishedCells<'a> {
    integral: Option<&'a martensite_tkbo::profile_extrusion::PlanarIntegralEvidence>,
    total: u64,
}
impl<'a> PublishedCells<'a> {
    fn member(
        &mut self,
        family: &'a OwnedFamily<Selection, Geometry>,
        prepared: &Prepared,
        cap: &ConicalDecodedCapEvidence,
        limit: u64,
    ) -> ConeChecked<()> {
        let region = prepared.source.region;
        let geometry = &family
            .members
            .get(&region)
            .ok_or_else(|| mismatch(region, None, "published atlas member"))?
            .geometry;
        let original =
            family.selection.planar.get(&region).ok_or_else(|| {
                mismatch(region, None, "published atlas original planar evidence")
            })?;
        let material = &geometry.material;
        if !original.matches_request(&material.original)
            || geometry.reference_volume.enclosure() != volume_bounds(cap)?
            || material.spatial_error().to_bits() != cap_error(cap)?.to_bits()
        {
            return Err(mismatch(
                region,
                None,
                "published atlas material and normal measures",
            ));
        }
        self.add(region, geometry, limit)
    }
    fn add(
        &mut self,
        region: SurfaceRegionId,
        geometry: &'a Geometry,
        limit: u64,
    ) -> ConeChecked<()> {
        let proof = geometry
            .material
            .integral_program_evidence()
            .ok_or_else(|| mismatch(region, None, "decoded atlas planar integral receipt"))?;
        if let Some(first) = self.integral {
            if !first.shares_issuance(proof) {
                return Err(mismatch(
                    region,
                    None,
                    "shared atlas planar integral issuance",
                ));
            }
        } else {
            program::validate_entry(region, proof, limit, 0)?;
            self.total = proof.integration_subintervals();
            self.integral = Some(proof);
        }
        self.total = program::member_cells(region, self.total, 0, geometry)?;
        Ok(())
    }
}
