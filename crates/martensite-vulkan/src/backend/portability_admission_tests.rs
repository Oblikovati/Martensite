//! Pure advertised-capability cases; no loader, platform or GPU support claim.
use super::*;

use super::fixture::PortabilityCatalogFake;

#[test]
fn ordinary_catalog_does_not_enable_portability_or_require_subset_properties() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::surface::NAME]);
    let instance = instance_plan(&catalog.extensions);
    assert_eq!(instance.extension, None);
    assert_eq!(instance.flags, vk::InstanceCreateFlags::empty());
    let device = subset_plan(&catalog.extensions, None).unwrap();
    assert_eq!(device.extension, None);
    assert!(device.requested_features().is_none());
}

#[test]
fn advertised_enumeration_requires_both_extension_name_and_instance_flag() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_enumeration::NAME]);
    let instance = instance_plan(&catalog.extensions);
    assert_eq!(
        instance.extension,
        Some(ash::khr::portability_enumeration::NAME)
    );
    assert_eq!(
        instance.flags,
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    );
}

#[test]
fn subset_extension_cannot_be_enabled_without_queried_properties() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_subset::NAME]);
    let result = subset_plan(&catalog.extensions, None);
    assert_eq!(
        result,
        Err(PortabilityAdmissionFailure::MissingSubsetProperties)
    );
    assert!(result.unwrap_err().to_string().contains("absent"));
}

#[test]
fn compatible_subset_enables_extension_without_requesting_unused_triangle_fans() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_subset::NAME]);
    for alignment in [1, 2, 4] {
        let device = subset_plan(&catalog.extensions, Some(alignment)).unwrap();
        assert_eq!(device.extension, Some(ash::khr::portability_subset::NAME));
        let requested = device.requested_features().unwrap();
        assert_eq!(requested.triangle_fans, vk::FALSE);
        assert_eq!(requested.vertex_attribute_access_beyond_stride, vk::FALSE);
        assert!(requested.p_next.is_null());
    }
}

#[test]
fn incompatible_or_malformed_alignment_is_typed_and_never_repacked() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_subset::NAME]);
    for observed in [0, 3, 8, 16, u32::MAX] {
        let failure = subset_plan(&catalog.extensions, Some(observed)).unwrap_err();
        assert_eq!(
            failure,
            PortabilityAdmissionFailure::IncompatibleVertexStrideAlignment {
                observed,
                required_divisor: 4,
            }
        );
        assert!(failure.to_string().contains(&observed.to_string()));
        assert!(failure.to_string().contains("divisor of 4"));
    }
}

#[test]
fn similarly_named_extensions_do_not_enable_either_portability_contract() {
    let catalog = PortabilityCatalogFake::named(&[c"VK_KHR_portability_subset_extra"]);
    assert_eq!(instance_plan(&catalog.extensions).extension, None);
    assert_eq!(
        subset_plan(&catalog.extensions, Some(0)).unwrap().extension,
        None
    );
    assert!(!advertises(
        &catalog.extensions,
        ash::khr::portability_subset::NAME
    ));
}

#[test]
fn unqueried_subset_plan_leaves_all_features_disabled() {
    let catalog = PortabilityCatalogFake::named(&[ash::khr::portability_subset::NAME]);
    let device = subset_plan(&catalog.extensions, Some(4)).unwrap();
    let features = device.requested_features().unwrap();
    assert_eq!(feature_bits(&features), [vk::FALSE; 15]);
}

fn feature_bits(features: &vk::PhysicalDevicePortabilitySubsetFeaturesKHR<'_>) -> [vk::Bool32; 15] {
    [
        features.constant_alpha_color_blend_factors,
        features.events,
        features.image_view_format_reinterpretation,
        features.image_view_format_swizzle,
        features.image_view2_d_on3_d_image,
        features.multisample_array_image,
        features.mutable_comparison_samplers,
        features.point_polygons,
        features.sampler_mip_lod_bias,
        features.separate_stencil_mask_ref,
        features.shader_sample_rate_interpolation_functions,
        features.tessellation_isolines,
        features.tessellation_point_mode,
        features.triangle_fans,
        features.vertex_attribute_access_beyond_stride,
    ]
}
