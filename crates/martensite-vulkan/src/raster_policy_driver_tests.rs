//! Real factories admit one effective policy while preserving independent caller selections.
use super::constructor_contract::{
    ConstructorUnusedBackend, assert_selection, constructor_options, selected_legacy_policies,
};
use super::*;

#[test]
fn configured_driver_keeps_all_existing_selections_and_the_admitted_policy() {
    let selection = last_driver_configuration();
    let driver =
        Graphic3dRenderDriver::with_pipeline_configuration(constructor_options(), selection)
            .unwrap();
    assert_selection(&driver, selected_legacy_policies());
    assert_eq!(
        driver.raster_policy(),
        Some(RenderRasterPolicy::LastProvokingVertexV2)
    );
    assert!(driver.driver_identity().is_some());
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn last_driver_configuration() -> RenderPipelinePolicies {
    let expected = selected_legacy_policies();
    RenderPipelinePolicies {
        texture_coordinates: expected.0,
        mesh_edges: expected.1,
        lighting: expected.2,
        unlit_faces: expected.3,
        gui_surface: expected.4,
        raster: RenderRasterPolicy::LastProvokingVertexV2,
    }
}

#[test]
fn additive_raster_constructor_keeps_the_other_policy_defaults() {
    let driver = Graphic3dRenderDriver::with_raster_policy(
        constructor_options(),
        RenderRasterPolicy::LastProvokingVertexV2,
    )
    .unwrap();
    assert_selection(&driver, Default::default());
    assert_eq!(
        driver.raster_policy(),
        Some(RenderRasterPolicy::LastProvokingVertexV2)
    );
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}

#[test]
fn existing_renderer_and_driver_constructors_still_record_legacy_behavior() {
    let renderer = VulkanRenderer::new(constructor_options()).unwrap();
    let driver = Graphic3dRenderDriver::new(constructor_options()).unwrap();
    assert_eq!(renderer.raster_policy(), RenderRasterPolicy::LegacyFirstV1);
    assert_eq!(
        driver.raster_policy(),
        Some(RenderRasterPolicy::LegacyFirstV1)
    );
    assert_selection(&driver, Default::default());
}

#[test]
fn injected_backends_do_not_claim_an_admitted_vulkan_raster_policy() {
    let driver = Graphic3dRenderDriver::with_backend(ConstructorUnusedBackend, 128).unwrap();
    assert!(driver.raster_policy().is_none());
    assert!(driver.coordinate_convention().is_none());
    assert!(driver.driver_identity().is_none());
}

#[test]
fn composition_refusal_still_precedes_device_and_raster_admission() {
    let options = VulkanOptions {
        validation: true,
        device_index: Some(usize::MAX),
    };
    let selection = RenderPipelinePolicies {
        lighting: crate::RenderLightingPipeline::FixedFunction,
        unlit_faces: crate::RenderUnlitFaceColorPolicy::SideColorsV2,
        raster: RenderRasterPolicy::LastProvokingVertexV2,
        ..Default::default()
    };
    let error = Graphic3dRenderDriver::with_pipeline_configuration(options, selection).unwrap_err();
    assert!(
        error.to_string().contains("SideColorsV2, FixedFunction"),
        "error {error:?}, expected composition refusal before selecting a physical device"
    );
    assert!(error.raster_policy_failure().is_none());
}
