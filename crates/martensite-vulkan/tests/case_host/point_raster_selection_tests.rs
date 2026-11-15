use super::*;
#[test]
fn default_and_explicit_v1_preserve_replay() {
    for value in [None, Some(OsStr::new("hardware-quad-v1"))] {
        assert_eq!(
            selected(value).unwrap(),
            RenderPointRasterBehavior::HardwareQuadV1
        );
    }
}
#[test]
fn fixed_grid_is_explicit_and_versioned() {
    let behavior = selected(Some(OsStr::new("fixed-grid-aliased-v2"))).unwrap();
    assert_eq!(behavior, RenderPointRasterBehavior::FixedGridAliasedV2);
    assert_eq!(
        metadata(behavior),
        "{\"point_raster_behavior\":\"FixedGridAliasedV2\",\"behavior_version\":2}"
    );
}
#[test]
fn unit12_grid_is_explicit_and_versioned() {
    let behavior = selected(Some(OsStr::new("fixed-grid-unit12-v3"))).unwrap();
    assert_eq!(behavior, RenderPointRasterBehavior::FixedGridUnit12V3);
    assert_eq!(behavior.behavior_version(), 3);
}
#[test]
fn malformed_selection_refuses_before_driver_creation() {
    for value in ["", "true", "sphere", "fixed-grid-aliased-v3"] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}
