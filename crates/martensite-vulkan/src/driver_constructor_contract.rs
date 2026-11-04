//! Owned constructor snapshots pin the existing independent policy selections before refactoring.
use super::*;
use crate::{
    RenderGuiSurfacePolicy, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline, RenderUnlitFaceColorPolicy,
};
type LegacySelection = (
    RenderTextureCoordinatePipeline,
    RenderMeshEdgePipeline,
    RenderLightingPipeline,
    RenderUnlitFaceColorPolicy,
    RenderGuiSurfacePolicy,
);

#[test]
fn legacy_vulkan_constructor_preserves_all_default_selections() {
    let driver = Graphic3dRenderDriver::new(constructor_options()).unwrap();
    assert_selection(&driver, Default::default());
    assert!(driver.driver_identity().is_some());
    assert!(matches!(&*driver.device.read(), DriverBackend::Vulkan(_)));
}

#[test]
fn legacy_vulkan_composition_preserves_each_selected_policy() {
    let expected = selected_legacy_policies();
    let driver = Graphic3dRenderDriver::with_pipeline_policies(
        constructor_options(),
        expected.0,
        expected.1,
        expected.2,
        expected.3,
        expected.4,
    )
    .unwrap();
    assert_selection(&driver, expected);
    assert!(matches!(&*driver.device.read(), DriverBackend::Vulkan(_)));
}

/// Reuse the frozen device selection; `constructor_options()` enables validation.
pub(super) fn constructor_options() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}

/// Reuse the independent legacy selections; `selected_legacy_policies()` supplies the expected tuple.
pub(super) fn selected_legacy_policies() -> LegacySelection {
    (
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::PolygonCompatibility,
        RenderLightingPipeline::FixedFunction,
        RenderUnlitFaceColorPolicy::LegacyV1,
        RenderGuiSurfacePolicy::CheckedV2,
    )
}

/// Apply the frozen constructor assertions; `assert_selection(&driver, expected)` checks every field.
pub(super) fn assert_selection(driver: &Graphic3dRenderDriver, expected: LegacySelection) {
    let actual = (
        driver.texture_images.coordinate_pipeline,
        driver.mesh_edge_pipeline,
        driver.lighting_pipeline(),
        driver.unlit_face_color_policy,
        driver.gui_surface,
    );
    assert_eq!(actual, expected);
    println!("{{\"constructor_policies\":{:?}}}", format!("{actual:?}"));
}

/// Named constructor-only I/O refusal shared by policy contracts.
pub(super) struct ConstructorUnusedBackend;
impl RenderBackend for ConstructorUnusedBackend {
    fn render(&mut self, frame: &crate::RenderFrame) -> Result<crate::FrameCapture, RenderError> {
        let expected = "constructor-only policy inspection without device I/O";
        panic!("frame {frame:?}, expected {expected}")
    }
}

#[test]
fn legacy_injected_constructor_keeps_its_backend_and_independent_policies() {
    let driver = Graphic3dRenderDriver::with_injected_policies(
        ConstructorUnusedBackend,
        128,
        Arc::new(crate::FreeTypeTextSource::default()),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderUnlitFaceColorPolicy::SideColorsV2,
        RenderGuiSurfacePolicy::CheckedV2,
    )
    .unwrap();
    assert_selection(&driver, injected_legacy_selection());
    assert!(matches!(&*driver.device.read(), DriverBackend::Injected(_)));
    assert!(driver.driver_identity().is_none());
}

#[test]
fn legacy_policy_refusal_precedes_an_invalid_physical_device_selection() {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(usize::MAX),
    };
    let failure = Graphic3dRenderDriver::with_pipeline_policies(
        options,
        Default::default(),
        Default::default(),
        RenderLightingPipeline::FixedFunction,
        RenderUnlitFaceColorPolicy::SideColorsV2,
        Default::default(),
    )
    .unwrap_err();
    assert!(
        failure.to_string().contains("SideColorsV2, FixedFunction"),
        "failure {failure:?}, expected incompatible policy before physical-device selection"
    );
}

fn injected_legacy_selection() -> LegacySelection {
    (
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        Default::default(),
        Default::default(),
        RenderUnlitFaceColorPolicy::SideColorsV2,
        RenderGuiSurfacePolicy::CheckedV2,
    )
}
