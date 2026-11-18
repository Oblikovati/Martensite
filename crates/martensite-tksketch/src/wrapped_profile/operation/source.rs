//! Resolve one owned profile index and directed selections for either analytic support.
use super::*;
pub(in crate::surface_sketch::wrapped_profile) fn read(
    profile: &martensite_tkbo::profile_extrusion::SketchProfile,
    selections: &[WrappedRegionSelection],
    cancelled: &mut CancelPoll<'_>,
    mut push: impl FnMut(&WrappedRegionSelection, &profile_input::ProfileReader<'_>) -> Checked<()>,
) -> Checked<()> {
    let Some(first) = selections.first() else {
        return Ok(());
    };
    input::poll(cancelled, WrappedStage::Input)?;
    let reader = profile_input::ProfileReader::new(profile)
        .map_err(|cause| input::profile_failure(cause, None))?;
    push(first, &reader)?;
    for selection in selections.iter().skip(1) {
        input::poll(cancelled, WrappedStage::Input)?;
        push(selection, &reader)?;
    }
    Ok(())
}
