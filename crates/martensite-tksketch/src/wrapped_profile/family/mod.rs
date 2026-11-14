//! One frozen transport table for all selected material components (#418).
use super::*;
use std::collections::BTreeMap;
mod control;
mod error;
use control::{Checkpoint, Control, FamilyStage};
mod freeze;
pub(super) use error::FamilyFailure;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod isolation_tests;
#[cfg(test)]
mod replay_tests;
#[cfg(test)]
mod tests;
type Checked<T> = Result<T, FamilyFailure>;
pub(super) struct UnqualifiedFamily {
    pub bytes: Vec<u8>,
    pub keys: Vec<BindingKey>,
    pub members: BTreeMap<SurfaceRegionId, pipeline::RetainedMember>,
    pub selection: selection::QualifiedSelection,
    pub coverage: coverage::FamilyCoverage,
    pub history: history::HistoryEvidence,
}
pub(super) fn construct(requests: &[Request]) -> Checked<UnqualifiedFamily> {
    construct_with_cancel(requests, |_| false)
}
pub(super) fn construct_with_cancel(
    requests: &[Request],
    mut cancelled: impl FnMut(Checkpoint) -> bool + Send,
) -> Checked<UnqualifiedFamily> {
    let mut control = Control::new(&mut cancelled);
    let selection = control.run(FamilyStage::Selection, None, |poll| {
        selection::certify_with_cancel(requests, Default::default(), poll)
            .map_err(|cause| FamilyFailure::Selection(Box::new(cause)))
    })?;
    let frozen = freeze::construct_with_control(&selection.prepared.prepared, &mut control)?;
    qualify_frozen(requests, selection, frozen, &mut control)
}
fn qualify_frozen(
    originals: &[Request],
    selection: selection::QualifiedSelection,
    frozen: freeze::Frozen,
    control: &mut Control<'_>,
) -> Checked<UnqualifiedFamily> {
    let (bytes, keys, root, bindings) = frozen;
    let (coverage, views) = bind_coverage(&selection.prepared.prepared, &root, &bindings, control)?;
    let history = history_stage(originals, &selection, &keys, control)?;
    let members = checked_members(originals, &selection, views, control)?;
    let result = UnqualifiedFamily {
        bytes,
        keys,
        members,
        selection,
        coverage,
        history,
    };
    control.run(FamilyStage::Completion, None, |_| Ok(result))
}

fn checked_members(
    originals: &[Request],
    selection: &selection::QualifiedSelection,
    views: Vec<coverage::MemberView>,
    control: &mut Control<'_>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::RetainedMember>> {
    let prepared = &selection.prepared.prepared;
    let stored = store_with_control(originals, prepared, views, control)?;
    control.run(FamilyStage::Binding, None, |_| {
        bind(originals, selection, &stored)
    })?;
    finish(originals, prepared, stored, control)
}

fn store(
    originals: &[Request],
    prepared: &[Request],
    views: Vec<coverage::MemberView>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::StoredMember>> {
    let mut cancelled = |_| false;
    store_with_control(
        originals,
        prepared,
        views,
        &mut Control::new(&mut cancelled),
    )
}
fn store_with_control(
    originals: &[Request],
    prepared: &[Request],
    views: Vec<coverage::MemberView>,
    control: &mut Control<'_>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::StoredMember>> {
    let mut stored = BTreeMap::new();
    for view in views {
        let (original, retained) = member_sources(originals, prepared, view.region)?;
        let member = control.run(FamilyStage::Storage, Some(view.region), |_| {
            pipeline::store_member(original, retained, view.shape, view.bindings)
                .map_err(|cause| FamilyFailure::Member(Box::new(cause)))
        })?;
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
    control: &mut Control<'_>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::RetainedMember>> {
    let mut members = BTreeMap::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let member = stored
            .remove(&original.region)
            .ok_or_else(|| membership(original.region))?;
        let member = control.run(FamilyStage::MemberChecks, Some(original.region), |_| {
            pipeline::finish_member(original, retained, member)
                .map_err(|cause| FamilyFailure::Member(Box::new(cause)))
        })?;
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

fn history_stage(
    originals: &[Request],
    selection: &selection::QualifiedSelection,
    keys: &[BindingKey],
    control: &mut Control<'_>,
) -> Checked<history::HistoryEvidence> {
    control.run(FamilyStage::History, None, |_| {
        history::certify(originals, selection, keys)
            .map_err(|cause| FamilyFailure::History(Box::new(cause)))
    })
}
fn bind_coverage(
    prepared: &[Request],
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
    control: &mut Control<'_>,
) -> Checked<(coverage::FamilyCoverage, Vec<coverage::MemberView>)> {
    control.run(FamilyStage::Coverage, None, |_| {
        coverage::certify_family(prepared, root, bindings)
            .map_err(|cause| FamilyFailure::Coverage(Box::new(cause)))
    })
}
fn member_sources<'a>(
    originals: &'a [Request],
    prepared: &'a [Request],
    region: SurfaceRegionId,
) -> Checked<(&'a Request, &'a Request)> {
    let original = originals
        .iter()
        .find(|r| r.region == region)
        .ok_or_else(|| membership(region))?;
    let retained = prepared
        .iter()
        .find(|r| r.region == region)
        .ok_or_else(|| membership(region))?;
    Ok((original, retained))
}
