//! Renderer validation gates run on the pinned layer, never on whichever package the host installed.
use martensite_vulkan::{
    RenderValidationAdmissionFailure, VERIFIED_VALIDATION_LAYER, VulkanOptions, VulkanRenderer,
    active_validation_layer,
};

// Regression for the toolkit SIGSEGV (2026-10-15): with Ubuntu's layer 1.3.275 the parallel toolkit
// target crashed inside libVkLayer_khronos_validation.so in about one of ten runs on lavapipe and AMD,
// and reported a false MSAA depth-resolve hazard; the pinned 1.4.313 layer passed 80 parallel repeats.
#[test]
fn toolkit_validation_uses_the_verified_layer_dependency() {
    let active = active_validation_layer().unwrap();
    assert!(
        active >= Some(VERIFIED_VALIDATION_LAYER),
        "active validation layer {active:?}; expected at least {VERIFIED_VALIDATION_LAYER:?} \
         selected by the cargo runner the recording tool (.cargo/config.toml)"
    );
}

// The real loader decides, so the same test covers admission on the pinned layer and refusal on an
// older one.
#[test]
fn validated_startup_refuses_exactly_the_layers_below_the_verified_one() {
    let active = active_validation_layer().unwrap();
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE")
        .ok()
        .map(|text| text.parse().unwrap());
    let started = VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index,
    });
    match active {
        Some(active) if active < VERIFIED_VALIDATION_LAYER => {
            let error = started.err().unwrap();
            assert_eq!(
                error.validation_admission_failure(),
                Some(&RenderValidationAdmissionFailure::UnverifiedLayer {
                    layer: "VK_LAYER_KHRONOS_validation".to_owned(),
                    active,
                    required: VERIFIED_VALIDATION_LAYER,
                })
            );
        }
        Some(_) => assert!(
            started.is_ok(),
            "{:?}",
            started.err().map(|e| e.to_string())
        ),
        None => assert!(started.is_err()),
    }
}
