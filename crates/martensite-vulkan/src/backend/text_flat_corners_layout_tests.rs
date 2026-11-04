use super::*;
use crate::RenderTextInterpolationBehavior as Behavior;

fn exact_limits() -> vk::PhysicalDeviceLimits {
    vk::PhysicalDeviceLimits {
        max_vertex_input_attributes: 9,
        max_vertex_input_binding_stride: 112,
        max_vertex_input_attribute_offset: 104,
        max_vertex_output_components: 40,
        max_fragment_input_components: 40,
        ..Default::default()
    }
}

#[test]
fn core_transport_accepts_exact_limits_without_a_barycentric_feature() {
    assert!(admit_limits(&exact_limits()).is_ok());
    assert_eq!(
        BINDINGS[0].stride as usize,
        super::super::text_flat_corners::STRIDE
    );
    assert_eq!(
        ATTRIBUTES.map(|attribute| attribute.offset),
        [0, 16, 32, 40, 56, 72, 88, 96, 104]
    );
    assert_eq!(
        ATTRIBUTES.map(|attribute| attribute.location),
        [0, 1, 2, 3, 4, 5, 6, 7, 8]
    );
}

#[test]
fn each_insufficient_resource_limit_reports_actual_and_required_values() {
    for component in 0..5 {
        let mut limits = exact_limits();
        let fields = [
            &mut limits.max_vertex_input_attributes,
            &mut limits.max_vertex_input_binding_stride,
            &mut limits.max_vertex_input_attribute_offset,
            &mut limits.max_vertex_output_components,
            &mut limits.max_fragment_input_components,
        ];
        *fields.into_iter().nth(component).unwrap() -= 1;
        let error = admit_limits(&limits).unwrap_err();
        assert!(matches!(
            error.text_interpolation_failure(),
            Some(Failure::InsufficientLimits { .. })
        ));
        assert!(error.to_string().contains("required"));
    }
}

fn default_raster() -> vk::PipelineRasterizationStateCreateInfo<'static> {
    vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL)
}

fn admitted(raster: &vk::PipelineRasterizationStateCreateInfo<'_>, stages: u32) -> bool {
    require_raster(Behavior::ProjectedGrid12FlatCornersV4, true, raster, stages).is_ok()
}

#[test]
fn selected_core_transport_refuses_nonfill_polygon_modes() {
    let mut raster = default_raster();
    assert!(admitted(&raster, 2));
    for polygon in [
        vk::PolygonMode::LINE,
        vk::PolygonMode::POINT,
        vk::PolygonMode::FILL_RECTANGLE_NV,
    ] {
        raster.polygon_mode = polygon;
        assert!(!admitted(&raster, 2));
    }
    assert!(require_raster(Behavior::ProjectedGrid12V2, true, &raster, 2).is_ok());
}

#[test]
fn selected_core_transport_refuses_discard_and_additional_stages() {
    let mut raster = default_raster();
    raster.rasterizer_discard_enable = vk::TRUE;
    assert!(!admitted(&raster, 2));
    raster.rasterizer_discard_enable = vk::FALSE;
    assert!(!admitted(&raster, 3));
}

#[test]
fn selected_core_transport_refuses_unreviewed_raster_extension_nodes() {
    let mut extension = vk::PipelineRasterizationConservativeStateCreateInfoEXT::default();
    let raster = default_raster().push_next(&mut extension);
    assert!(!admitted(&raster, 2));
    assert!(require_raster(Behavior::HardwareV1, true, &raster, 2).is_ok());
}
