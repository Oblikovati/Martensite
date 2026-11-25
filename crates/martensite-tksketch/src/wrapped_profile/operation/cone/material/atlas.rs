//! Bind periodic filled caps once, then loan one reference receipt to each decoded member.
use super::super::prepare::selection::first_region;
use super::*;
use crate::surface_sketch::wrapped_profile::{planar_clearance, selection as periodic};
use measures::MemberReferenceVolume;
mod native_material;
mod publication;
pub(super) use publication::validate;

type Members = BTreeMap<SurfaceRegionId, (MaterialEvidence, MemberReferenceVolume)>;
pub(in crate::surface_sketch::wrapped_profile::operation::cone) struct Captured {
    pub reference: ConicalReferenceEvidence,
    pub binding: Binding,
    members: Members,
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) struct Binding {
    boundary: planar_clearance::OriginalClearance,
    material: Option<periodic::PeriodicMaterialEvidence>,
    pub operations: u32,
}
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum Refusal {
    Clearance(planar_clearance::ClearanceFailure),
    Material(periodic::MaterialFailure),
    Reference {
        cause: ConicalDecodedFamilyError,
        overlap: Option<[SurfaceRegionId; 2]>,
    },
}
impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clearance(cause) => cause.fmt(f),
            Self::Material(cause) => cause.fmt(f),
            Self::Reference { cause, .. } => cause.fmt(f),
        }
    }
}
impl std::error::Error for Refusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reference { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
impl Refusal {
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn cancelled(&self) -> bool {
        match self {
            Self::Clearance(cause) => cause.cancelled(),
            Self::Material(cause) => cause.cancelled(),
            Self::Reference { cause, .. } => cause.is_cancelled(),
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
    let native = selection
        .executed_selection
        .is_directed()
        .then_some(members.as_slice());
    let captured = capture_planar(request, &profiles, native, work, cancelled)?;
    let boundary = clearance(&profiles, region, work, cancelled)?;
    let material = native
        .is_none()
        .then(|| {
            winding(
                &profiles,
                &captured.planar,
                &boundary,
                region,
                work,
                cancelled,
            )
        })
        .transpose()?;
    let reference = reference(
        request,
        &members,
        &boundary,
        native.is_some(),
        region,
        work,
        cancelled,
    )?;
    let members = member_receipts(
        originals,
        &members,
        captured,
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
    native: Option<&[Member<'_>]>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<PlanarCapture> {
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
    let planar = periodic::PlanarSelectionEvidence {
        members,
        certificate: captured.certificate(),
        proof_work: captured.proof_work(),
    };
    let native = native
        .map(|members| native_material::select(members, profiles, &planar, &captured))
        .transpose()?
        .unwrap_or_default();
    Ok(PlanarCapture {
        planar,
        version,
        native,
    })
}
struct PlanarCapture {
    planar: periodic::PlanarSelectionEvidence,
    version: PlanarProofVersion,
    native: native_material::Selected,
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
    check_fill: bool,
    region: SurfaceRegionId,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalReferenceEvidence> {
    let joins: Vec<_> = members.iter().map(|(_, stored)| &stored.joins).collect();
    let policy = ConeClearanceOptions {
        max_operations: work.requested.max_operations - work.consumed,
        ..work.requested
    };
    let result = if check_fill {
        certify_conical_decoded_family_with_fill(
            &joins,
            &boundary.certificate,
            &boundary.periodic,
            request.options.spatial_error,
            policy,
            cancelled,
        )
    } else {
        certify_conical_decoded_family(
            &joins,
            &boundary.certificate,
            &boundary.periodic,
            request.options.spatial_error,
            policy,
            cancelled,
        )
    };
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
        let overlap = fill_overlap(&error, members);
        refused(
            owner,
            Refusal::Reference {
                cause: error,
                overlap,
            },
        )
    })?;
    charged?;
    Ok(reference)
}
fn fill_overlap(
    error: &ConicalDecodedFamilyError,
    members: &[Member<'_>],
) -> Option<[SurfaceRegionId; 2]> {
    let ConicalDecodedFamilyFailure::Fill(ConicalDecodedFillFailure::Overlap {
        source,
        target,
        ..
    }) = &error.cause
    else {
        return None;
    };
    let region = |index: usize| {
        members
            .get(index)
            .map(|(prepared, _)| prepared.source.region)
    };
    Some([region(*source)?, region(*target)?])
}
fn member_receipts(
    originals: &[DevelopedRegionRequest],
    members: &[Member<'_>],
    mut captured: PlanarCapture,
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
            let receipts = member_receipt(original, stored, cap, &mut captured, requested)?;
            Ok((region, receipts))
        })
        .collect()
}
fn member_receipt(
    original: &DevelopedRegionRequest,
    stored: &storage::Stored,
    cap: &ConicalDecodedCapEvidence,
    captured: &mut PlanarCapture,
    requested: PlanarProofVersion,
) -> ConeChecked<(MaterialEvidence, MemberReferenceVolume)> {
    let uv = captured
        .planar
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
    let (selected_material, periodic_curves) = captured
        .native
        .remove(&original.region)
        .map_or((None, None), |(material, curves)| {
            (Some(material), Some(curves))
        });
    let material = MaterialEvidence {
        original: original.clone(),
        uv: UvMaterial::Region(uv),
        selected_material,
        periodic_curves,
        normal: MaterialRealization::Decoded(cap_error(cap)?),
        carriers: stored.carriers,
        options: cap.normal().options(),
        planar_proof: captured.version,
        requested_planar_proof: requested,
        census: captured.planar.certificate,
        planar_work: captured.planar.proof_work,
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
