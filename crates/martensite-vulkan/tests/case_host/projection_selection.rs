//! An explicit worker projection choice never changes authored original bodies.
use martensite_vulkan::RenderProjectionBehavior as Behavior;
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_TRIANGLE_PROJECTION";
pub(super) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    match value {
        None => Ok(Behavior::PrecomposedV1),
        Some(v) if v == "precomposed-v1" => Ok(Behavior::PrecomposedV1),
        Some(v) if v == "factorized-v2" => Ok(Behavior::FactorizedTrianglesV2),
        Some(v) if v == "factorized-lines-v3" => Ok(Behavior::FactorizedLinesV3),
        Some(v) if v == "fixed-model-view-v4" => Ok(Behavior::FixedFunctionModelViewV4),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, precomposed-v1, factorized-v2, factorized-lines-v3 or fixed-model-view-v4"
            ),
        )),
    }
}
pub(crate) fn capture_configuration(
    configuration: martensite_vulkan::RenderBehaviorConfiguration,
    value: Option<&OsStr>,
) -> io::Result<martensite_vulkan::RenderBehaviorConfiguration> {
    Ok(configuration.with_projection(selected(value)?))
}
#[cfg(test)]
#[path = "projection_selection_tests.rs"]
mod tests;
