//! Both reference representations produce certified volume bounds for the shared reports.
use super::*;

pub(in crate::surface_sketch::wrapped_profile) enum MemberReferenceVolume {
    Common(CertifiedConicalReferenceVolume),
    Decoded([f64; 2]),
}
impl MemberReferenceVolume {
    pub(in crate::surface_sketch::wrapped_profile) fn common(
        &self,
    ) -> Option<&CertifiedConicalReferenceVolume> {
        match self {
            Self::Common(proof) => Some(proof),
            Self::Decoded(_) => None,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn enclosure(
        &self,
    ) -> [f64; 2] {
        match self {
            Self::Common(proof) => proof.enclosure(),
            Self::Decoded(bounds) => *bounds,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn integration_subintervals(
        &self,
    ) -> u64 {
        self.common()
            .map_or(0, CertifiedConicalReferenceVolume::integration_subintervals)
    }
    pub(super) fn absolute_error_bound(&self) -> ConeChecked<f64> {
        match self {
            Self::Common(proof) => Ok(proof.absolute_error_bound()),
            Self::Decoded([lo, hi]) => comparison::distance([*lo; 2], [*hi; 2]),
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
) -> ConeChecked<(material::MaterialEvidence, MemberReferenceVolume)> {
    if prepared.executed_selection.is_connected_atlas() {
        let atlas = work.connected.as_mut().ok_or_else(|| {
            material::mismatch(original.region, None, "complete decoded atlas issuance")
        })?;
        return atlas.take_member(original.region);
    }
    let material = material::certify(request, original, prepared, stored, work, &mut *cancelled)?;
    let volume = reference::certify(request, prepared, work, cancelled)?;
    Ok((material, MemberReferenceVolume::Common(volume)))
}
