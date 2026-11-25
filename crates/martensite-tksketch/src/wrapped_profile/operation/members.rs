//! Member-local builders feed one owned geometric and native receipt pair.
use super::{stages::*, *};
use family::{Control, FamilyStage};
use std::collections::BTreeMap;
type Members<G> = BTreeMap<SurfaceRegionId, QualifiedMember<G>>;
pub(super) fn qualify<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    selection: &S::Selection,
    views: Vec<coverage::MemberView>,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<Members<S::Geometry>, S::Error> {
    let prepared = S::prepared(selection);
    let stored = store(support, originals, prepared, views, context, control)?;
    control.run_for::<_, S::Error>(FamilyStage::Binding, None, |poll| {
        support.bind(originals, selection, &stored, context, poll)
    })?;
    if support.defer_native_checks() {
        return directed::finish(support, originals, selection, stored, context, control);
    }
    finish(support, originals, prepared, stored, context, control)
}
pub(in crate::surface_sketch::wrapped_profile) fn store<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    prepared: &[S::Prepared],
    views: Vec<coverage::MemberView>,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<BTreeMap<SurfaceRegionId, S::Stored>, S::Error> {
    let mut stored = BTreeMap::new();
    let originals = source::by_region(originals);
    let prepared = source::by_region(prepared);
    for view in views {
        let (original, retained) = sources::<S>(&originals, &prepared, view.region)?;
        let member =
            control.run_for::<_, S::Error>(FamilyStage::Storage, Some(view.region), |poll| {
                support.store(original, retained, view, context, poll)
            })?;
        stored.insert(original.as_ref().region, member);
    }
    Ok(stored)
}
fn finish<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    prepared: &[S::Prepared],
    mut stored: BTreeMap<SurfaceRegionId, S::Stored>,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<Members<S::Geometry>, S::Error> {
    let mut members = BTreeMap::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let region = original.as_ref().region;
        let member = stored.remove(&region).ok_or_else(|| S::missing(region))?;
        let member =
            control.run_for::<_, S::Error>(FamilyStage::MemberChecks, Some(region), |poll| {
                let (geometry, shape) =
                    support.geometry(original, retained, member, context, poll)?;
                let native = pipeline::qualify_native(original.as_ref(), &shape)
                    .map_err(|cause| S::native(region, cause))?;
                Ok(QualifiedMember { geometry, native })
            })?;
        members.insert(region, member);
    }
    Ok(members)
}
fn sources<'a, S: SupportStages>(
    originals: &BTreeMap<SurfaceRegionId, &'a S::Original>,
    prepared: &BTreeMap<SurfaceRegionId, &'a S::Prepared>,
    region: SurfaceRegionId,
) -> Result<(&'a S::Original, &'a S::Prepared), S::Error> {
    let original = originals
        .get(&region)
        .copied()
        .ok_or_else(|| S::missing(region))?;
    let retained = prepared
        .get(&region)
        .copied()
        .ok_or_else(|| S::missing(region))?;
    Ok((original, retained))
}

mod directed;
