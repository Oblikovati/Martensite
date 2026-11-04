#[path = "depth_bias_selection.rs"]
pub(crate) mod depth_bias_selection;
#[path = "phong_interpolation_selection.rs"]
pub(super) mod phong_interpolation_selection;
#[path = "point_raster_selection.rs"]
pub(crate) mod point_raster_selection;
#[path = "projection_selection.rs"]
pub(crate) mod projection_selection;
#[path = "text_interpolation_selection.rs"]
pub(crate) mod text_interpolation_selection;
#[path = "thin_axis_selection.rs"]
pub(crate) mod thin_axis_selection;
#[path = "thin_line_selection.rs"]
pub(crate) mod thin_line_selection;
#[path = "triangle_coverage_selection.rs"]
pub(crate) mod triangle_coverage_selection;
#[path = "window_depth_selection.rs"]
pub(crate) mod window_depth_selection;
#[path = "world_clip_interpolation_selection.rs"]
pub(crate) mod world_clip_interpolation_selection;
// A worker selects immutable numerical behavior once; every source case body stays unchanged.
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderCoordinateConvention, RenderLightDotBehavior,
    RenderLineClippingBehavior, RenderLineWidthPolicy, RenderPipelinePolicies,
    RenderWorldClippingBehavior, VulkanOptions,
};
use std::{error::Error, ffi::OsStr, io};
const SELECTION: &str = "MARTENSITE_CASE_LIGHT_DOT_BEHAVIOR";
pub(super) type BehaviorChoices = (
    martensite_vulkan::RenderPointRasterBehavior,
    RenderLightDotBehavior,
    martensite_vulkan::RenderThinAxisCoverage,
    martensite_vulkan::RenderThinLineCoverage,
    martensite_vulkan::RenderPhongInterpolationBehavior,
    martensite_vulkan::RenderTriangleCoverageBehavior,
    martensite_vulkan::RenderTextInterpolationBehavior,
);
struct WorkerSelection {
    projection: (martensite_vulkan::RenderProjectionBehavior, bool),
    window_depth: (martensite_vulkan::RenderWindowDepthBehavior, bool),
    depth_bias: (martensite_vulkan::RenderDepthBiasBehavior, bool),
    choices: BehaviorChoices,
    world_clip_interpolation: (
        martensite_vulkan::RenderWorldClipInterpolationBehavior,
        bool,
    ),
    explicit: [bool; 8],
}
pub(super) fn selected(value: Option<&OsStr>) -> io::Result<RenderLightDotBehavior> {
    match value {
        None => Ok(Default::default()),
        Some(value) if value == "driver-native-v1" => Ok(RenderLightDotBehavior::DriverNativeV1),
        Some(value) if value == "numeric-max-zero-v2" => {
            Ok(RenderLightDotBehavior::NumericMaxZeroV2)
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, driver-native-v1 or numeric-max-zero-v2"
            ),
        )),
    }
}
pub(super) fn driver(
    options: VulkanOptions,
    policies: RenderPipelinePolicies,
    coordinates: RenderCoordinateConvention,
    widths: RenderLineWidthPolicy,
    clipping: RenderLineClippingBehavior,
    world_clipping: RenderWorldClippingBehavior,
    world_explicit: bool,
) -> Result<Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
    let selection = worker_selection()?;
    if legacy_selection(&selection, world_explicit) {
        return legacy_driver(options, policies, coordinates, widths, clipping);
    }
    emit_selection_metadata(&selection);
    configured_driver(
        options,
        policies,
        coordinates,
        widths,
        clipping,
        selection,
        world_clipping,
    )
}
fn legacy_selection(selection: &WorkerSelection, world_explicit: bool) -> bool {
    !selection.explicit.iter().any(|selected| *selected)
        && !selection.depth_bias.1
        && !selection.world_clip_interpolation.1
        && !selection.projection.1
        && !selection.window_depth.1
        && !world_explicit
}
fn configured_driver(
    options: VulkanOptions,
    policies: RenderPipelinePolicies,
    coordinates: RenderCoordinateConvention,
    widths: RenderLineWidthPolicy,
    clipping: RenderLineClippingBehavior,
    selection: WorkerSelection,
    world_clipping: RenderWorldClippingBehavior,
) -> Result<Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
    let config = combined_driver_configuration(&selection, world_clipping);
    Ok(Graphic3dRenderDriver::with_render_behavior_configuration(
        options,
        policies,
        coordinates,
        widths,
        clipping,
        config,
    )?)
}
fn combined_driver_configuration(
    selection: &WorkerSelection,
    world_clipping: RenderWorldClippingBehavior,
) -> martensite_vulkan::RenderBehaviorConfiguration {
    configuration(selection.choices)
        .with_depth_bias(selection.depth_bias.0)
        .with_projection(selection.projection.0)
        .with_window_depth(selection.window_depth.0)
        .with_world_clip_interpolation(selection.world_clip_interpolation.0)
        .with_world_clipping(world_clipping)
}
fn worker_selection() -> io::Result<WorkerSelection> {
    let (points, dots, point_selected, dots_selected) = selected_points_and_dots()?;
    let (axes, lines, axes_selected, lines_selected) = selected_axes_and_lines()?;
    let (interpolation, triangles, text, phong_selected, triangle_selected, text_selected) =
        selected_phong_and_triangles()?;
    Ok(WorkerSelection {
        projection: selected_projection()?,
        window_depth: selected_window_depth()?,
        depth_bias: selected_depth_bias()?,
        choices: (points, dots, axes, lines, interpolation, triangles, text),
        world_clip_interpolation: selected_world_clip_interpolation()?,
        explicit: [
            point_selected,
            dots_selected,
            axes_selected,
            lines_selected,
            phong_selected,
            triangle_selected,
            text_selected,
            std::env::var_os(world_clip_interpolation_selection::SELECTION).is_some(),
        ],
    })
}
fn selected_world_clip_interpolation() -> io::Result<(
    martensite_vulkan::RenderWorldClipInterpolationBehavior,
    bool,
)> {
    let request = std::env::var_os(world_clip_interpolation_selection::SELECTION);
    Ok((
        world_clip_interpolation_selection::selected(request.as_deref())?,
        request.is_some(),
    ))
}
fn selected_projection() -> io::Result<(martensite_vulkan::RenderProjectionBehavior, bool)> {
    let request = std::env::var_os(projection_selection::SELECTION);
    Ok((
        projection_selection::selected(request.as_deref())?,
        request.is_some(),
    ))
}
fn selected_window_depth() -> io::Result<(martensite_vulkan::RenderWindowDepthBehavior, bool)> {
    let request = std::env::var_os(window_depth_selection::SELECTION);
    Ok((
        window_depth_selection::selected(request.as_deref())?,
        request.is_some(),
    ))
}
fn selected_depth_bias() -> io::Result<(martensite_vulkan::RenderDepthBiasBehavior, bool)> {
    let request = std::env::var_os(depth_bias_selection::SELECTION);
    Ok((
        depth_bias_selection::selected(request.as_deref())?,
        request.is_some(),
    ))
}
fn selected_points_and_dots() -> io::Result<(
    martensite_vulkan::RenderPointRasterBehavior,
    RenderLightDotBehavior,
    bool,
    bool,
)> {
    let point_request = std::env::var_os(point_raster_selection::SELECTION);
    let dot_request = std::env::var_os(SELECTION);
    let (points, dots) = selected_pair(point_request.as_deref(), dot_request.as_deref())?;
    Ok((points, dots, point_request.is_some(), dot_request.is_some()))
}
fn selected_axes_and_lines() -> io::Result<(
    martensite_vulkan::RenderThinAxisCoverage,
    martensite_vulkan::RenderThinLineCoverage,
    bool,
    bool,
)> {
    let axes = std::env::var_os(thin_axis_selection::SELECTION);
    let lines = std::env::var_os(thin_line_selection::SELECTION);
    Ok((
        thin_axis_selection::selected(axes.as_deref())?,
        thin_line_selection::selected(lines.as_deref())?,
        axes.is_some(),
        lines.is_some(),
    ))
}
fn selected_phong_and_triangles() -> io::Result<(
    martensite_vulkan::RenderPhongInterpolationBehavior,
    martensite_vulkan::RenderTriangleCoverageBehavior,
    martensite_vulkan::RenderTextInterpolationBehavior,
    bool,
    bool,
    bool,
)> {
    let phong = std::env::var_os(phong_interpolation_selection::SELECTION);
    let triangles = std::env::var_os(triangle_coverage_selection::SELECTION);
    let text = std::env::var_os(text_interpolation_selection::SELECTION);
    Ok((
        phong_interpolation_selection::selected(phong.as_deref())?,
        triangle_coverage_selection::selected(triangles.as_deref())?,
        text_interpolation_selection::selected(text.as_deref())?,
        phong.is_some(),
        triangles.is_some(),
        text.is_some(),
    ))
}
fn emit_selection_metadata(selection: &WorkerSelection) {
    if selection.window_depth.1 {
        eprintln!(
            "{}",
            window_depth_selection::metadata(selection.window_depth.0)
        );
    }
    if selection.projection.1 {
        eprintln!(
            "{{\"triangle_projection\":\"{:?}\"}}",
            selection.projection.0
        );
    }
    if selection.depth_bias.1 {
        eprintln!("{}", depth_bias_selection::metadata(selection.depth_bias.0));
    }
    let choices = selection.choices;
    let selected = selection.explicit;
    if selected[2] {
        eprintln!("{}", thin_axis_selection::metadata(choices.2));
    }
    eprintln!("{}", point_raster_selection::metadata(choices.0));
    if selected[1] {
        eprintln!("{}", metadata(choices.1));
    }
    if selected[4] {
        eprintln!("{}", phong_interpolation_selection::metadata(choices.4));
    }
    if selected[3] {
        eprintln!("{}", thin_line_selection::metadata(choices.3));
    }
    if selected[5] {
        eprintln!("{}", triangle_coverage_selection::metadata(choices.5));
    }
    if selected[6] {
        eprintln!("{}", text_interpolation_selection::metadata(choices.6));
    }
    if selected[7] {
        eprintln!(
            "{}",
            world_clip_interpolation_selection::metadata(selection.world_clip_interpolation.0)
        );
    }
}
pub(super) fn selected_point_behavior(
    value: Option<&OsStr>,
) -> io::Result<martensite_vulkan::RenderPointRasterBehavior> {
    point_raster_selection::selected(value)
}
fn selected_pair(
    points: Option<&OsStr>,
    dots: Option<&OsStr>,
) -> io::Result<(
    martensite_vulkan::RenderPointRasterBehavior,
    RenderLightDotBehavior,
)> {
    Ok((selected_point_behavior(points)?, selected(dots)?))
}
pub(super) fn selected_choices(
    points: Option<&OsStr>,
    dots: Option<&OsStr>,
    axes: Option<&OsStr>,
    lines: Option<&OsStr>,
    interpolation: Option<&OsStr>,
    triangles: Option<&OsStr>,
    text: Option<&OsStr>,
) -> io::Result<BehaviorChoices> {
    Ok((
        point_raster_selection::selected(points)?,
        selected(dots)?,
        thin_axis_selection::selected(axes)?,
        thin_line_selection::selected(lines)?,
        phong_interpolation_selection::selected(interpolation)?,
        triangle_coverage_selection::selected(triangles)?,
        text_interpolation_selection::selected(text)?,
    ))
}
pub(super) fn configuration(
    choices: BehaviorChoices,
) -> martensite_vulkan::RenderBehaviorConfiguration {
    martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_point_raster(choices.0)
        .with_light_dots(choices.1)
        .with_thin_axes(choices.2)
        .with_thin_lines(choices.3)
        .with_phong_interpolation(choices.4)
        .with_triangle_coverage(choices.5)
        .with_text_interpolation(choices.6)
}
fn legacy_driver(
    options: VulkanOptions,
    policies: RenderPipelinePolicies,
    coordinates: RenderCoordinateConvention,
    widths: RenderLineWidthPolicy,
    clipping: RenderLineClippingBehavior,
) -> Result<Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
    Ok(Graphic3dRenderDriver::with_line_clipping_configuration(
        options,
        policies,
        coordinates,
        widths,
        clipping,
    )?)
}
fn metadata(behavior: RenderLightDotBehavior) -> String {
    format!(
        "{{\"light_dot_behavior\":\"{behavior:?}\",\"behavior_version\":{},\"scope\":\"built-in-programmable-phong\",\"unapplied\":[\"custom\",\"pbr\",\"fixed\"]}}",
        behavior.behavior_version()
    )
}
#[cfg(test)]
#[path = "light_dot_selection_tests.rs"]
mod tests;
