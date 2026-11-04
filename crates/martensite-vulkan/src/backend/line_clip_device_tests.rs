use super::*;
use crate::RenderLineClippingRequirement as Missing;

#[test]
fn legacy_and_other_drivers_require_no_new_feature() {
    let legacy =
        LineClipDevice::observed(RenderLineClippingBehavior::VulkanDriverV1, 13, false, 0, 0);
    let hardware = LineClipDevice::observed(
        RenderLineClippingBehavior::PlatformNativeV2,
        vk::DriverId::MESA_RADV.as_raw() as u32,
        false,
        0,
        0,
    );
    for device in [legacy, hardware] {
        assert_eq!(device.admit(false), Ok(false));
        assert_eq!(device.admit(true), Ok(false));
        assert!(!device.enabled());
    }
}
#[test]
fn software_clipping_admits_only_explicit_supported_stock_stages() {
    let device =
        LineClipDevice::observed(RenderLineClippingBehavior::PlatformNativeV2, 13, true, 4, 4);
    assert!(device.enabled());
    assert_eq!(device.admit(false), Ok(true));
    assert_eq!(device.admit(true), Err(Missing::RegisteredLineProgram));
}
#[test]
fn software_feature_and_limit_refusals_never_choose_legacy() {
    let absent = LineClipDevice::observed(
        RenderLineClippingBehavior::PlatformNativeV2,
        13,
        false,
        8,
        8,
    );
    let short =
        LineClipDevice::observed(RenderLineClippingBehavior::PlatformNativeV2, 13, true, 3, 8);
    assert_eq!(absent.admit(false), Err(Missing::ShaderClipDistance));
    assert_eq!(short.admit(false), Err(Missing::FourClipDistances));
    assert!(!absent.enabled());
    assert!(!short.enabled());
}

#[test]
fn typed_refusals_retain_each_actual_feature_and_limit() {
    for (supported, clip, combined, reason) in [
        (false, 8, 8, Missing::ShaderClipDistance),
        (true, 3, 8, Missing::FourClipDistances),
        (true, 8, 3, Missing::FourClipDistances),
    ] {
        let observed = LineClipDevice::observed(
            RenderLineClippingBehavior::PlatformNativeV2,
            13,
            supported,
            clip,
            combined,
        );
        assert_eq!(observed.admit(false), Err(reason));
        let failure = observed.refusal(
            RenderLineClippingBehavior::PlatformNativeV2,
            fixture_device_identity(),
            reason,
        );
        assert_eq!(
            (
                failure.shader_clip_distance,
                failure.max_clip_distances,
                failure.max_combined_clip_and_cull_distances
            ),
            (supported, clip, combined)
        );
        let message = failure.to_string();
        assert!(message.contains(&format!("shaderClipDistance={supported}")));
        assert!(message.contains(&format!("maxClipDistances={clip}")));
        assert!(message.contains(&format!("maxCombinedClipAndCullDistances={combined}")));
    }
}
fn fixture_device_identity() -> crate::RenderDriverIdentity {
    crate::RenderDriverIdentity {
        vendor_id: 99,
        device_id: 88,
        driver_id: 13,
        driver_version: 1,
        driver_name: "named missing-capability device".into(),
        driver_info: "independent CPU observation".into(),
        device_name: "fixture".into(),
    }
}
#[test]
fn source_clipping_version_requires_the_same_software_stage_only() {
    let software =
        LineClipDevice::observed(RenderLineClippingBehavior::PlatformNativeV3, 13, true, 4, 4);
    assert!(software.enabled());
    let radv = vk::DriverId::MESA_RADV.as_raw() as u32;
    let hardware = LineClipDevice::observed(
        RenderLineClippingBehavior::PlatformNativeV3,
        radv,
        true,
        8,
        8,
    );
    assert!(!hardware.enabled());
}
