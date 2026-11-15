//! An explicit generic worker option leaves every original source case unchanged.
use martensite_vulkan::RenderPhongInterpolationBehavior as Behavior;
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_PHONG_INTERPOLATION";
pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    match value {
        None => Ok(Behavior::default()),
        Some(v) if v == "hardware-v1" => Ok(Behavior::HardwareV1),
        Some(v) if v == "unsnapped-v2" => Ok(Behavior::UnsnappedV2),
        Some(v) if v == "unsnapped-v3" => Ok(Behavior::UnsnappedV3),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-v1, unsnapped-v2 or unsnapped-v3"
            ),
        )),
    }
}
pub(crate) fn metadata(behavior: Behavior) -> String {
    format!(
        "{{\"phong_interpolation\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}
type DriverPolicies = (
    martensite_vulkan::VulkanOptions,
    martensite_vulkan::RenderPipelinePolicies,
    martensite_vulkan::RenderCoordinateConvention,
    martensite_vulkan::RenderLineWidthPolicy,
    martensite_vulkan::RenderLineClippingBehavior,
);
pub(super) fn explicit_driver(
    (options, policies, coordinates, widths, clipping): DriverPolicies,
    (points, dots): (
        martensite_vulkan::RenderPointRasterBehavior,
        martensite_vulkan::RenderLightDotBehavior,
    ),
) -> Result<
    Option<martensite_tkernel::standard::Handle<martensite_vulkan::Graphic3dRenderDriver>>,
    Box<dyn std::error::Error>,
> {
    let explicit = std::env::var_os(SELECTION);
    let behavior = selected(explicit.as_deref())?;
    if explicit.is_none() {
        return Ok(None);
    }
    eprintln!("{}", metadata(behavior));
    eprintln!("{}", super::metadata(dots));
    eprintln!("{}", super::point_raster_selection::metadata(points));
    Ok(Some(
        martensite_vulkan::Graphic3dRenderDriver::with_phong_interpolation_configuration(
            options,
            policies,
            coordinates,
            widths,
            clipping,
            points,
            dots,
            behavior,
        )?,
    ))
}
pub(crate) fn admit_observation(behavior: Behavior) -> Result<(), martensite_vulkan::RenderError> {
    if behavior == Behavior::HardwareV1 {
        return Ok(());
    }
    Err(
        martensite_vulkan::RenderPhongInterpolationFailure::UnsupportedInterface {
            actual: "concurrent vertex-output observation with unsnapped interpolation".into(),
        }
        .into(),
    )
}
#[cfg(test)]
support_unit_tests! {
    #[path = "phong_interpolation_selection_tests.rs"]
    mod tests;
}
