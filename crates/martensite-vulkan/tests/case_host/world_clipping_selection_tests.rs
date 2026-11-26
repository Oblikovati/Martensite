use super::*;

#[test]
fn absent_and_explicit_fragment_path_replay_version_one() {
    for value in [None, Some(OsStr::new("fragment-v1"))] {
        assert_eq!(
            selected(value).unwrap(),
            RenderWorldClippingBehavior::FragmentV1
        );
    }
}

#[test]
fn raster_clipping_is_explicit_and_reports_its_version() {
    let behavior = selected(Some(OsStr::new("raster-clip-distance-v2"))).unwrap();
    assert_eq!(behavior, RenderWorldClippingBehavior::RasterClipDistanceV2);
    assert_eq!(
        metadata(behavior),
        "{\"world_clipping_behavior\":\"RasterClipDistanceV2\",\"behavior_version\":2}"
    );
}

#[test]
fn unknown_selectors_are_rejected_with_the_value() {
    for value in ["", "true", "raster-clip-distance-v4", "bug30695"] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}

#[test]
fn fixed_function_raster_clipping_has_an_explicit_versioned_selector() {
    let behavior = selected(Some(OsStr::new("raster-fixed-function-v4"))).unwrap();
    assert_eq!(behavior, RenderWorldClippingBehavior::RasterFixedFunctionV4);
    assert!(metadata(behavior).ends_with("\"behavior_version\":4}"));
}
#[test]
fn clip_space_raster_clipping_has_an_explicit_versioned_selector() {
    let behavior = selected(Some(OsStr::new("raster-fixed-function-v5"))).unwrap();
    assert_eq!(
        behavior,
        RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5
    );
    assert!(metadata(behavior).ends_with("\"behavior_version\":5}"));
}
#[test]
fn raster_clipping_v3_has_an_explicit_versioned_selector() {
    let behavior = selected(Some(OsStr::new("raster-clip-distance-v3"))).unwrap();
    assert_eq!(behavior, RenderWorldClippingBehavior::RasterClipDistanceV3);
    assert!(metadata(behavior).ends_with("\"behavior_version\":3}"));
}
