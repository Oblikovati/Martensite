//! Public capacity queries are checked against the selected physical-device profile.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkservice::graphic3d::{Graphic3dGraphicDriver, Graphic3dTypeOfLimit};
use martensite_vulkan::Graphic3dRenderDriver;
#[test]
fn selected_device_texture_capacity_matches_the_independent_profile() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let maximum = driver.inquire_limit(Graphic3dTypeOfLimit::MaxTextureSize);
    assert!(
        maximum > 0,
        "received {maximum}; expected positive device capacity"
    );
    if let Some(expected) = expected_texture_dimension() {
        assert_eq!(maximum, expected);
    }
    record_capacity(&driver, maximum);
}
fn record_capacity(driver: &Graphic3dRenderDriver, maximum: i32) {
    let identity = driver.driver_identity().unwrap();
    println!(
        "{{\"kind\":\"texture-capacity\",\"vendor_id\":{},\"device_id\":{},\"driver_id\":{},\"driver_version\":{},\"maximum\":{maximum}}}",
        identity.vendor_id, identity.device_id, identity.driver_id, identity.driver_version
    );
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn expected_texture_dimension() -> Option<i32> {
    std::env::var("MARTENSITE_EXPECTED_TEXTURE_LIMIT")
        .ok()
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("received {value:?}; expected signed texture dimension"))
        })
}

struct UnavailableTextureDevice;
impl martensite_vulkan::RenderBackend for UnavailableTextureDevice {
    fn render(
        &mut self,
        _: &martensite_vulkan::RenderFrame,
    ) -> Result<martensite_vulkan::FrameCapture, martensite_vulkan::RenderError> {
        // A capability lookup must not submit frames to invent a device property.
        panic!("texture capability lookup submitted a frame; expected property-only lookup")
    }
}
#[test]
fn injected_backend_without_capabilities_remains_explicitly_unavailable() {
    let driver = Graphic3dRenderDriver::with_backend(UnavailableTextureDevice, 64).unwrap();
    let maximum = driver.inquire_limit(Graphic3dTypeOfLimit::MaxTextureSize);
    assert_eq!(
        maximum, 0,
        "injected device has no physical capability provider"
    );
}
