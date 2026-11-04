use super::*;

#[test]
fn explicit_window_depth_survives_driver_configuration_and_legacy_route_check() {
    let window = window_depth_selection::selected(Some(OsStr::new("grid12-over8-v2"))).unwrap();
    let selection = WorkerSelection {
        world_clip_interpolation: (Default::default(), false),
        window_depth: (window, true),
        projection: (
            martensite_vulkan::RenderProjectionBehavior::FactorizedTrianglesV2,
            true,
        ),
        depth_bias: (
            martensite_vulkan::RenderDepthBiasBehavior::Native24UnitsV2,
            true,
        ),
        choices: selected_choices(None, None, None, None, None, None, None).unwrap(),
        explicit: [false; 8],
    };
    assert!(!legacy_selection(&selection, false));
    let expected = martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_projection(selection.projection.0)
        .with_depth_bias(selection.depth_bias.0)
        .with_window_depth(window);
    assert_eq!(
        combined_driver_configuration(&selection, RenderWorldClippingBehavior::FragmentV1),
        expected
    );
}
#[test]
fn point_and_light_dot_choices_compose_without_overwriting_either() {
    use martensite_vulkan::RenderPointRasterBehavior as Points;
    for (point, expected_point) in [
        ("hardware-quad-v1", Points::HardwareQuadV1),
        ("fixed-grid-aliased-v2", Points::FixedGridAliasedV2),
    ] {
        for (dot, expected_dot) in [
            ("driver-native-v1", RenderLightDotBehavior::DriverNativeV1),
            (
                "numeric-max-zero-v2",
                RenderLightDotBehavior::NumericMaxZeroV2,
            ),
        ] {
            assert_eq!(
                selected_pair(Some(OsStr::new(point)), Some(OsStr::new(dot))).unwrap(),
                (expected_point, expected_dot)
            );
        }
    }
    assert_eq!(
        selected_pair(None, None).unwrap(),
        (Points::HardwareQuadV1, Default::default())
    );
    assert!(selected_pair(Some(OsStr::new("invalid")), None).is_err());
    assert!(selected_pair(None, Some(OsStr::new("invalid"))).is_err());
}
#[test]
fn absent_and_explicit_legacy_replay_version_one() {
    for value in [None, Some(OsStr::new("driver-native-v1"))] {
        assert_eq!(
            selected(value).unwrap(),
            RenderLightDotBehavior::DriverNativeV1
        );
    }
}
#[test]
fn mixed_behavior_selection_preserves_all_seven_choices() {
    let selected = selected_choices(
        Some(OsStr::new("fixed-grid-aliased-v2")),
        Some(OsStr::new("numeric-max-zero-v2")),
        Some(OsStr::new("fixed-grid8-bottom-up-v2")),
        Some(OsStr::new("diamond-exit-v2")),
        Some(OsStr::new("unsnapped-v2")),
        Some(OsStr::new("fixed-grid-12-v2-top-down")),
        Some(OsStr::new("projected-grid12-v2")),
    )
    .unwrap();
    assert_eq!(
        selected.0,
        martensite_vulkan::RenderPointRasterBehavior::FixedGridAliasedV2
    );
    assert_eq!(selected.1, RenderLightDotBehavior::NumericMaxZeroV2);
    assert_eq!(
        selected.3,
        martensite_vulkan::RenderThinLineCoverage::DiamondExitV2
    );
    assert_eq!(
        selected.4,
        martensite_vulkan::RenderPhongInterpolationBehavior::UnsnappedV2
    );
    assert_eq!(
        selected.5,
        martensite_vulkan::RenderTriangleCoverageBehavior::FixedGrid12V2 {
            origin: martensite_vulkan::RenderTriangleGridOrigin::TopDown,
        }
    );
    assert_eq!(
        selected.6,
        martensite_vulkan::RenderTextInterpolationBehavior::ProjectedGrid12V2
    );
}
#[test]
fn point_unit12_composes_with_diamond_exit_and_unset_choices_replay_hardware() {
    let selected = selected_choices(
        Some(OsStr::new("fixed-grid-unit12-v3")),
        None,
        None,
        Some(OsStr::new("diamond-exit-v2")),
        None,
        None,
        None,
    )
    .unwrap();
    assert_eq!(
        selected.0,
        martensite_vulkan::RenderPointRasterBehavior::FixedGridUnit12V3
    );
    assert_eq!(
        selected.3,
        martensite_vulkan::RenderThinLineCoverage::DiamondExitV2
    );
    assert_eq!(selected.1, RenderLightDotBehavior::DriverNativeV1);
    assert_eq!(
        selected.2,
        martensite_vulkan::RenderThinAxisCoverage::HardwareV1
    );
    assert_eq!(
        selected.4,
        martensite_vulkan::RenderPhongInterpolationBehavior::HardwareV1
    );
    assert_eq!(
        selected_choices(None, None, None, None, None, None, None)
            .unwrap()
            .3,
        martensite_vulkan::RenderThinLineCoverage::HardwareV1
    );
    let config = configuration(selected);
    let expected = martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_point_raster(martensite_vulkan::RenderPointRasterBehavior::FixedGridUnit12V3)
        .with_thin_lines(martensite_vulkan::RenderThinLineCoverage::DiamondExitV2)
        .with_triangle_coverage(martensite_vulkan::RenderTriangleCoverageBehavior::HardwareV1);
    assert_eq!(config, expected);
}
#[test]
fn world_clipping_composes_with_existing_choices() {
    let choices = selected_choices(
        Some(OsStr::new("fixed-grid-unit12-v3")),
        None,
        None,
        Some(OsStr::new("diamond-exit-v2")),
        None,
        None,
        None,
    )
    .unwrap();
    let config = configuration(choices)
        .with_world_clipping(martensite_vulkan::RenderWorldClippingBehavior::RasterClipDistanceV2);
    let expected = martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_point_raster(martensite_vulkan::RenderPointRasterBehavior::FixedGridUnit12V3)
        .with_thin_lines(martensite_vulkan::RenderThinLineCoverage::DiamondExitV2)
        .with_world_clipping(martensite_vulkan::RenderWorldClippingBehavior::RasterClipDistanceV2);
    assert_eq!(config, expected);
}
#[test]
fn absent_world_selector_keeps_legacy_driver_route() {
    let selection = WorkerSelection {
        world_clip_interpolation: (Default::default(), false),
        window_depth: (Default::default(), false),
        projection: (Default::default(), false),
        depth_bias: (Default::default(), false),
        choices: selected_choices(None, None, None, None, None, None, None).unwrap(),
        explicit: [false; 8],
    };
    assert!(legacy_selection(&selection, false));
    assert!(!legacy_selection(&selection, true));
}
#[test]
fn projection_and_world_choices_survive_combined_driver_configuration() {
    use martensite_vulkan::{RenderBehaviorConfiguration, RenderProjectionBehavior};
    let projection = RenderProjectionBehavior::FactorizedTrianglesV2;
    let world = RenderWorldClippingBehavior::RasterClipDistanceV2;
    let selection = WorkerSelection {
        world_clip_interpolation: (Default::default(), false),
        window_depth: (Default::default(), false),
        projection: (projection, true),
        depth_bias: (Default::default(), false),
        choices: Default::default(),
        explicit: [false; 8],
    };
    assert!(!legacy_selection(&selection, false));
    assert_eq!(
        combined_driver_configuration(&selection, world),
        RenderBehaviorConfiguration::default()
            .with_projection(projection)
            .with_world_clipping(world)
    );
}
#[test]
fn version_two_selection_and_metadata_are_explicit() {
    let behavior = selected(Some(OsStr::new("numeric-max-zero-v2"))).unwrap();
    assert_eq!(behavior, RenderLightDotBehavior::NumericMaxZeroV2);
    assert_eq!(
        metadata(behavior),
        "{\"light_dot_behavior\":\"NumericMaxZeroV2\",\"behavior_version\":2,\"scope\":\"built-in-programmable-phong\",\"unapplied\":[\"custom\",\"pbr\",\"fixed\"]}"
    );
}
#[test]
fn invalid_versions_refuse_before_any_driver_is_constructed() {
    for value in ["", "true", "numeric-max-zero-v3", "bug27700"] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}

#[test]
fn world_interpolation_composes_with_projection_and_disables_legacy_route() {
    let selection = WorkerSelection {
        world_clip_interpolation: (
            martensite_vulkan::RenderWorldClipInterpolationBehavior::ProjectedGrid12V1,
            true,
        ),
        window_depth: (Default::default(), false),
        projection: (
            martensite_vulkan::RenderProjectionBehavior::FactorizedTrianglesV2,
            true,
        ),
        depth_bias: (Default::default(), false),
        choices: selected_choices(None, None, None, None, None, None, None).unwrap(),
        explicit: [false; 8],
    };
    assert!(!legacy_selection(&selection, false));
    let expected = configuration(selection.choices)
        .with_projection(selection.projection.0)
        .with_world_clip_interpolation(selection.world_clip_interpolation.0);
    assert_eq!(
        combined_driver_configuration(&selection, RenderWorldClippingBehavior::FragmentV1),
        expected
    );
}
