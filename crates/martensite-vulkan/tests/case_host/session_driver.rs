//! Select the existing driver policies once before constructing the source session.
use super::*;
use martensite_vulkan::RenderLineClippingBehavior;
use std::io;
/// The validation-enabled driver of a run: the default frame, or with `BOTTOM_UP_CONTROL`
/// set, `LegacyBottomUpV1` with last-vertex provoking, the frame in which llvmpipe sets up
/// attributes as native OpenGL does (tests/support/native_dump_grading.rs `rounding_control`);
/// `run_driver(RenderLineWidthPolicy::default())?`.
pub(crate) fn run_driver(
    line_widths: RenderLineWidthPolicy,
) -> Result<Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
    let options = crate::device_options::validation_options();
    let (policies, frame) = coordinate_policy_selection::from_environment()?;
    let (line, world, world_explicit) = selected_clippings()?;
    light_dot_selection::driver(
        options,
        policies,
        frame,
        line_widths,
        line,
        world,
        world_explicit,
    )
}

fn selected_clippings() -> io::Result<(
    RenderLineClippingBehavior,
    martensite_vulkan::RenderWorldClippingBehavior,
    bool,
)> {
    let line_request = std::env::var_os(line_clipping_selection::LINE_CLIPPING_SELECTION);
    let line = line_clipping_selection::selected_behavior(line_request.as_deref())?;
    emit_line_selection(line_request.is_some(), line);
    let world_request = std::env::var_os(world_clipping_selection::SELECTION);
    let world = world_clipping_selection::selected(world_request.as_deref())?;
    emit_world_selection(world_request.is_some(), world);
    Ok((line, world, world_request.is_some()))
}

fn emit_line_selection(explicit: bool, behavior: RenderLineClippingBehavior) {
    if explicit {
        eprintln!("{}", line_clipping_selection::selection_metadata(behavior));
    }
}

fn emit_world_selection(explicit: bool, behavior: martensite_vulkan::RenderWorldClippingBehavior) {
    if explicit {
        eprintln!("{}", world_clipping_selection::metadata(behavior));
    }
}
