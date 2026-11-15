use super::*;
#[test]
fn defaults_and_combined_replay_are_unchanged() {
    for combined in [false, true] {
        assert_eq!(
            selected(combined, None, None).unwrap(),
            super::super::run_coordinate_policies(combined)
        );
    }
}
#[test]
fn frame_selection_preserves_default_provoking_policy() {
    let (policies, frame) = selected(false, Some(OsStr::new("legacy-bottom-up-v1")), None).unwrap();
    assert_eq!(frame, Frame::LegacyBottomUpV1);
    assert_eq!(policies.raster, Raster::LegacyFirstV1);
}
#[test]
fn raster_selection_preserves_default_frame() {
    let (policies, frame) =
        selected(false, None, Some(OsStr::new("last-provoking-vertex-v2"))).unwrap();
    assert_eq!(frame, Frame::TopLeftLogicalV2);
    assert_eq!(policies.raster, Raster::LastProvokingVertexV2);
}
#[test]
fn explicit_defaults_override_combined_replay() {
    let actual = selected(
        true,
        Some(OsStr::new("top-left-logical-v2")),
        Some(OsStr::new("legacy-first-v1")),
    )
    .unwrap();
    assert_eq!(actual, super::super::run_coordinate_policies(false));
}
#[test]
fn unknown_values_refuse_with_expected_shape() {
    for (frame, raster, name) in [
        (Some(OsStr::new("wrong")), None, FRAME_SELECTION),
        (None, Some(OsStr::new("wrong")), RASTER_SELECTION),
    ] {
        let error = selected(false, frame, raster).unwrap_err().to_string();
        assert!(
            error.contains(name) && error.contains("wrong") && error.contains("expected unset")
        );
    }
}
