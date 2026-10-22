use super::{ShapeDivider, ShapeUpgradeShapeConvertToBezier};
use martensite_tkernel::standard::Failure;

pub(super) fn perform(
    tool: &ShapeUpgradeShapeConvertToBezier,
    new_context: bool,
) -> Result<bool, Failure> {
    perform_with_policy(tool, tool, new_context)
}

pub(super) fn perform_with_policy(
    tool: &ShapeUpgradeShapeConvertToBezier,
    policy: &(impl ShapeDivider + ?Sized),
    new_context: bool,
) -> Result<bool, Failure> {
    let level = tool.shape_divide_level() + 1;
    tool.set_shape_divide_level(level);
    let changed = run_at_level(tool, policy, new_context, level)?;
    // A caught nested failure can leave the live depth above the entry level.
    if tool.shape_divide_level() == 1 {
        super::postprocess::run(tool)?;
    }
    tool.set_shape_divide_level(tool.shape_divide_level() - 1);
    Ok(changed)
}

fn run_at_level(
    tool: &ShapeUpgradeShapeConvertToBezier,
    policy: &(impl ShapeDivider + ?Sized),
    new_context: bool,
    level: i32,
) -> Result<bool, Failure> {
    if level != 1 {
        return run_base_divide(policy, new_context);
    }
    run_first_level(tool, policy, new_context)
}

fn run_first_level(
    tool: &ShapeUpgradeShapeConvertToBezier,
    policy: &(impl ShapeDivider + ?Sized),
    new_context: bool,
) -> Result<bool, Failure> {
    let modes = tool.modes();
    let mut changed = false;
    let mut next_context = new_context;
    if modes.surfaces {
        changed = run_surface_phase(tool, policy, modes, next_context)?;
        next_context = false;
    }
    let current_modes = tool.modes();
    if current_modes.curves_2d || current_modes.curves_3d {
        changed = run_curve_phase(tool, policy, current_modes, next_context)?;
    }
    Ok(changed)
}

fn run_surface_phase(
    tool: &ShapeUpgradeShapeConvertToBezier,
    policy: &(impl ShapeDivider + ?Sized),
    modes: super::ConversionModes,
    new_context: bool,
) -> Result<bool, Failure> {
    tool.set_2d_conversion(false);
    tool.set_3d_conversion(false);
    let changed = run_base_divide(policy, new_context)?;
    tool.set_source_to_result();
    // Native restores only the two flags it masks, retaining callback policy edits.
    tool.set_2d_conversion(modes.curves_2d);
    tool.set_3d_conversion(modes.curves_3d);
    Ok(changed)
}

fn run_curve_phase(
    tool: &ShapeUpgradeShapeConvertToBezier,
    policy: &(impl ShapeDivider + ?Sized),
    modes: super::ConversionModes,
    new_context: bool,
) -> Result<bool, Failure> {
    tool.set_surface_conversion(false);
    let changed = run_base_divide(policy, new_context)?;
    tool.set_surface_conversion(modes.surfaces);
    Ok(changed)
}

fn run_base_divide(
    policy: &(impl ShapeDivider + ?Sized),
    new_context: bool,
) -> Result<bool, Failure> {
    // Qualified base Perform still calls the originating virtual policy hooks.
    super::super::shape_divide::perform_with_policy(policy, new_context)
}
