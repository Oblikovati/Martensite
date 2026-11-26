use super::*;

#[test]
fn unset_and_v1_preserve_hardware_replay() {
    for value in [None, Some(OsStr::new("hardware-v1"))] {
        assert_eq!(selected(value).unwrap(), Coverage::HardwareV1);
    }
}

#[test]
fn diamond_exit_is_explicit_and_versioned() {
    let behavior = selected(Some(OsStr::new("diamond-exit-v2"))).unwrap();
    assert_eq!(behavior, Coverage::DiamondExitV2);
    assert_eq!(
        metadata(behavior),
        "{\"thin_line_coverage\":\"DiamondExitV2\",\"behavior_version\":2}"
    );
}

#[test]
fn malformed_selection_is_refused_with_its_value() {
    for value in ["", "sphere", "amd", "diamond-exit-v1"] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}

#[test]
fn major_axis_interpolation_is_explicit_and_versioned() {
    let behavior = selected(Some(OsStr::new("diamond-exit-major-axis-v3"))).unwrap();
    assert_eq!(behavior, Coverage::DiamondExitMajorAxisV3);
    assert_eq!(behavior.behavior_version(), 3);
    assert!(metadata(behavior).contains("DiamondExitMajorAxisV3"));
}
#[test]
fn fused_projection_is_explicit_and_versioned() {
    let behavior = selected(Some(OsStr::new("diamond-exit-fused-projection-v4"))).unwrap();
    assert_eq!(behavior, Coverage::DiamondExitFusedProjectionV4);
    assert_eq!(behavior.behavior_version(), 4);
}
