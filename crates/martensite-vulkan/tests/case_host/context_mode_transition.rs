//! Share only the operation order pinned by the pre-extraction context-mode fixture.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject};

/// Preserve erase/display activation and typed failures; lookup remains the caller's policy.
/// Example: `redisplay_selection_mode_on_context(context, &object, 3)?`.
pub(crate) fn redisplay_selection_mode_on_context(
    context: &Handle<AisInteractiveContext>,
    object: &Handle<dyn AisInteractiveObject>,
    mode: i32,
) -> Result<(), Failure> {
    context.erase(object, true)?;
    context.update_current_viewer();
    context.set_auto_activate_selection(false);
    context.display(object, true)?;
    context.activate(object, mode, false)?;
    context.update_current_viewer();
    Ok(())
}
