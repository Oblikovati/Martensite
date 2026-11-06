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
mod tests {
    use super::*;
    #[test]
    fn only_explicit_versions_are_accepted_before_construction() {
        assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
        assert_eq!(
            selected(Some(OsStr::new("hardware-v1"))).unwrap(),
            Behavior::HardwareV1
        );
        assert_eq!(
            selected(Some(OsStr::new("unsnapped-v2"))).unwrap(),
            Behavior::UnsnappedV2
        );
        assert_eq!(
            selected(Some(OsStr::new("unsnapped-v3"))).unwrap(),
            Behavior::UnsnappedV3
        );
        assert!(metadata(Behavior::UnsnappedV3).contains("\"behavior_version\":3"));
        assert!(admit_observation(Behavior::UnsnappedV3).is_err());
        assert!(selected(Some(OsStr::new("bug30008"))).is_err());
        assert!(admit_observation(Behavior::HardwareV1).is_ok());
        assert!(
            admit_observation(Behavior::UnsnappedV2)
                .unwrap_err()
                .phong_interpolation_failure()
                .is_some()
        );
        assert!(metadata(Behavior::UnsnappedV2).contains("\"behavior_version\":2"));
    }
}
