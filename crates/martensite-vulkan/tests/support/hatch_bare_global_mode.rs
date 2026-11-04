//! Bare ViewerCommands mode commands have two distinct update boundaries.
use martensite_tkernel::standard::{Failure, catch_native_failure};
use martensite_tkv3d::ais::AisInteractiveContext;

/// Preserve ViewerCommands's argc-two route without its unrelated RAII updater.
/// Example: `source_bare_global_mode(context, 1)?`.
pub(super) fn source_bare_global_mode(
    context: &AisInteractiveContext,
    mode: i32,
) -> Result<(), Failure> {
    catch_native_failure(|| {
        context.set_default_display_mode(mode, true)?;
        context.update_current_viewer();
        Ok(())
    })
}
