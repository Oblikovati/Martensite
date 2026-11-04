//! Accepted OIT assets were independently reproduced on both proved Mesa software cohorts.
use super::ffp_reference_profile::{FfpNativeReferenceProfile, ffp_native_reference_profile};
use martensite_tkernel::standard::Handle;
use martensite_vulkan::Graphic3dRenderDriver;

/// Select independently proved software buffers using the actual device; `software_reference(&driver)`.
pub fn software_reference(driver: &Handle<Graphic3dRenderDriver>) -> bool {
    let identity = driver
        .driver_identity()
        .expect("expected selected Vulkan physical-device identity");
    matches!(
        ffp_native_reference_profile(&identity).unwrap(),
        FfpNativeReferenceProfile::SoftwareMesa25 | FfpNativeReferenceProfile::SoftwareMesa26
    )
}
