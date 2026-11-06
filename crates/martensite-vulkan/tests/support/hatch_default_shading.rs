//! ViewerCommands::VShading has two real operations and no RAII view updater.
use martensite_tkernel::standard::{Failure, Handle, catch_native_failure};
use martensite_tkv3d::ais::{AisInteractiveContext, AisShape};

/// Preserve the bare command's default coefficient and explicit Redisplay.
/// Example: `source_default_shading(context, object)?`.
pub(super) fn source_default_shading(
    context: &AisInteractiveContext,
    object: &Handle<AisShape>,
) -> Result<(), Failure> {
    catch_native_failure(|| {
        let selected = object.cast();
        context.set_object_deviation_coefficient(&selected, 0.0008, true)?;
        context.redisplay(&selected, true, false)
    })
}
