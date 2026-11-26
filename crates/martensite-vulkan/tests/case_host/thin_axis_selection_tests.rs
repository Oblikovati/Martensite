use super::*;
#[test]
fn explicit_grid_origin_and_legacy_selectors_are_exact() {
    assert_eq!(selected(None).unwrap(), RenderThinAxisCoverage::HardwareV1);
    assert_eq!(
        selected(Some(OsStr::new("hardware-v1"))).unwrap(),
        Default::default()
    );
    for value in [
        "fixed-grid8-bottom-up-v2",
        "fixed-grid12-bottom-up-v2",
        "fixed-grid8-top-down-v2",
        "fixed-grid12-top-down-v2",
    ] {
        assert_eq!(
            selected(Some(OsStr::new(value))).unwrap(),
            fixed_choice(OsStr::new(value)).unwrap()
        );
    }
    assert!(selected(Some(OsStr::new("amd"))).is_err());
    assert!(metadata(RenderThinAxisCoverage::HardwareV1).contains("HardwareV1"));
}
