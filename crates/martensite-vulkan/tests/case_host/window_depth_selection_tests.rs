use super::*;
#[test]
fn default_and_explicit_operation_keep_projection_and_units_independent() {
    assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
    assert_eq!(
        selected(Some(OsStr::new("hardware-v1"))).unwrap(),
        Behavior::HardwareV1
    );
    let selected_v2 = selected(Some(OsStr::new("grid12-over8-v2"))).unwrap();
    let old = RenderBehaviorConfiguration::default()
        .with_projection(martensite_vulkan::RenderProjectionBehavior::FactorizedTrianglesV2)
        .with_depth_bias(martensite_vulkan::RenderDepthBiasBehavior::Native24UnitsV2);
    assert_eq!(
        capture_configuration(old, Some(OsStr::new("grid12-over8-v2"))).unwrap(),
        old.with_window_depth(selected_v2)
    );
    assert!(metadata(selected_v2).contains("\"reference_fractional_bits\":12"));
    assert!(metadata(Behavior::HardwareV1).contains("\"behavior_version\":1"));
}
#[test]
fn float64_metadata_preserves_case_arithmetic_and_raw_clip_contract() {
    let float64 = selected(Some(OsStr::new("float64-grid12-over8-v5"))).unwrap();
    assert!(metadata(float64).contains("Float64ReferenceWindowDepthV5"));
    assert!(metadata(float64).contains("\"behavior_version\":5"));
    assert_eq!(
        metadata(float64),
        concat!(
            "{\"window_depth_behavior\":\"Float64ReferenceWindowDepthV5\",\"behavior_version\":5,",
            "\"reference_fractional_bits\":12,\"raster_fractional_bits\":8,\"grid_rounding\":\"nearest-even\",",
            "\"arithmetic\":\"ordered-binary64-no-contraction; float-controls-unspecified\",",
            "\"input_contract\":\"actual-binary32-raw-GL-clip-before-remap\"}"
        )
    );
}
#[test]
fn malformed_or_case_specific_names_are_not_fallbacks() {
    for value in [
        "",
        "true",
        "amd",
        "dragg",
        "grid8-over12-v2",
        "grid12-over8-v1",
    ] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}
#[test]
fn direct_fragment_depth_is_an_explicit_successor_not_an_alias_for_geometry_replay() {
    let v2 = selected(Some(OsStr::new("grid12-over8-v2"))).unwrap();
    let v3 = selected(Some(OsStr::new("grid12-over8-v3"))).unwrap();
    assert_ne!(v2, v3);
    assert_eq!(v3.behavior_version(), 3);
    let v4 = selected(Some(OsStr::new("clipped-fan-grid8-v4"))).unwrap();
    assert_eq!(v4.behavior_version(), 4);
    assert!(metadata(v4).contains("unsnapped-framebuffer-origin"));
    assert!(metadata(v3).contains("ReferenceWindowDepthV3"));
    assert!(metadata(v3).contains("\"behavior_version\":3"));
}
#[test]
fn shared_actual_incidence_is_version_six_and_never_relabels_v4_replay() {
    let shared = selected(Some(OsStr::new("clipped-incidence-grid12-v6"))).unwrap();
    let old = selected(Some(OsStr::new("clipped-fan-grid8-v4"))).unwrap();
    assert_eq!(shared.behavior_version(), 6);
    assert_eq!(old.behavior_version(), 4);
    assert!(metadata(shared).contains("actual-vs-clip-distance"));
    assert!(selected(Some(OsStr::new("clipped-incidence-grid8-v6"))).is_err());
    assert_ne!(shared, old);
}
