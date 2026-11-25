//! Resolve one owned profile index and directed selections for either analytic support.
use super::*;
use std::collections::BTreeMap;

pub(super) fn by_region<T: AsRef<DevelopedRegionRequest>>(
    sources: &[T],
) -> BTreeMap<SurfaceRegionId, &T> {
    let mut indexed = BTreeMap::new();
    for source in sources {
        indexed.entry(source.as_ref().region).or_insert(source);
    }
    indexed
}
pub(in crate::surface_sketch::wrapped_profile) fn read(
    profile: &martensite_tkbo::profile_extrusion::SketchProfile,
    selections: &[WrappedRegionSelection],
    cancelled: &mut CancelPoll<'_>,
    mut push: impl FnMut(
        &WrappedRegionSelection,
        &profile_input::ProfileReader<'_>,
        &mut CancelPoll<'_>,
    ) -> Checked<()>,
) -> Checked<()> {
    let Some(first) = selections.first() else {
        return Ok(());
    };
    input::poll(cancelled, WrappedStage::Input)?;
    let reader = profile_input::ProfileReader::new(profile)
        .map_err(|cause| input::profile_failure(cause, None))?;
    push(first, &reader, cancelled)?;
    for selection in selections.iter().skip(1) {
        input::poll(cancelled, WrappedStage::Input)?;
        push(selection, &reader, cancelled)?;
    }
    Ok(())
}
