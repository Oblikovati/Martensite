use super::*;

#[test]
fn unset_and_versions_are_explicit() {
    assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
    assert_eq!(
        selected(Some(OsStr::new("fixed-grid-12-v2-top-down"))).unwrap(),
        fixed_grid(Origin::TopDown)
    );
    assert_eq!(
        selected(Some(OsStr::new("fixed-grid-12-v2-bottom-up"))).unwrap(),
        fixed_grid(Origin::BottomUp)
    );
    assert!(metadata(fixed_grid(Origin::TopDown)).contains("\"behavior_version\":2"));
    for (name, origin) in [
        ("fixed-grid-12-v3-top-down", Origin::TopDown),
        ("fixed-grid-12-v3-bottom-up", Origin::BottomUp),
    ] {
        let selected = selected(Some(OsStr::new(name))).unwrap();
        assert_eq!(selected, Behavior::FixedGrid12V3 { origin });
        assert!(metadata(selected).contains("\"behavior_version\":3"));
    }
}

#[test]
fn native_grid_v4_selectors_are_explicit_and_versioned() {
    for (name, origin) in [
        ("native-grid-12-v4-top-down", Origin::TopDown),
        ("native-grid-12-v4-bottom-up", Origin::BottomUp),
    ] {
        let behavior = selected(Some(OsStr::new(name))).unwrap();
        assert_eq!(behavior, Behavior::NativeGrid12V4 { origin });
        assert_eq!(behavior.behavior_version(), 4);
    }
}

#[test]
fn invalid_selector_reports_its_value_and_shape() {
    let error = selected(Some(OsStr::new("vendor"))).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("vendor"));
    assert!(error.to_string().contains("declared origin"));
}
