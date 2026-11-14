//! One frozen transport table for all selected material components (#418).
use super::*;
use std::collections::BTreeMap;
mod error;
mod freeze;
pub(super) use error::FamilyFailure;
#[cfg(test)]
mod tests;
type Checked<T> = Result<T, FamilyFailure>;
pub(super) struct UnqualifiedFamily {
    pub bytes: Vec<u8>,
    pub keys: Vec<BindingKey>,
    pub members: BTreeMap<SurfaceRegionId, pipeline::RetainedMember>,
    pub selection: selection::QualifiedSelection,
    pub coverage: coverage::FamilyCoverage,
}
pub(super) fn construct(requests: &[Request]) -> Checked<UnqualifiedFamily> {
    let selection = selection::certify(requests, Default::default())
        .map_err(|cause| FamilyFailure::Selection(Box::new(cause)))?;
    let prepared = &selection.prepared.prepared;
    let (bytes, keys, root, bindings) = freeze::construct(prepared)?;
    let (coverage, views) = coverage::certify_family(prepared, &root, &bindings)
        .map_err(|cause| FamilyFailure::Coverage(Box::new(cause)))?;
    let stored = store(requests, prepared, views)?;
    bind(requests, &selection, &stored)?;
    let members = finish(requests, prepared, stored)?;
    Ok(UnqualifiedFamily {
        bytes,
        keys,
        members,
        selection,
        coverage,
    })
}
fn store(
    originals: &[Request],
    prepared: &[Request],
    views: Vec<coverage::MemberView>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::StoredMember>> {
    let mut stored = BTreeMap::new();
    for view in views {
        let original = originals.iter().find(|r| r.region == view.region);
        let retained = prepared.iter().find(|r| r.region == view.region);
        let (Some(original), Some(retained)) = (original, retained) else {
            return Err(membership(view.region));
        };
        let member = pipeline::store_member(original, retained, view.shape, view.bindings)
            .map_err(|cause| FamilyFailure::Member(Box::new(cause)))?;
        stored.insert(view.region, member);
    }
    Ok(stored)
}
fn bind(
    originals: &[Request],
    selection: &selection::QualifiedSelection,
    stored: &BTreeMap<SurfaceRegionId, pipeline::StoredMember>,
) -> Checked<()> {
    let proofs = stored
        .iter()
        .map(|(id, member)| (*id, member.storage.as_slice()))
        .collect();
    planar_clearance::bind_many(
        &selection.prepared.boundary_clearance,
        originals,
        &selection.prepared.prepared,
        &proofs,
    )
    .map_err(|cause| FamilyFailure::Binding(Box::new(cause)))
}
fn finish(
    originals: &[Request],
    prepared: &[Request],
    mut stored: BTreeMap<SurfaceRegionId, pipeline::StoredMember>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::RetainedMember>> {
    let mut members = BTreeMap::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let member = stored
            .remove(&original.region)
            .ok_or_else(|| membership(original.region))?;
        let member = pipeline::finish_member(original, retained, member)
            .map_err(|cause| FamilyFailure::Member(Box::new(cause)))?;
        members.insert(original.region, member);
    }
    Ok(members)
}
fn membership(region: SurfaceRegionId) -> FamilyFailure {
    FamilyFailure::Native {
        region: Some(region),
        cause: Box::new(Failure::new(&format!(
            "family member={region:?}; expected one original, prepared and decoded member"
        ))),
    }
}
