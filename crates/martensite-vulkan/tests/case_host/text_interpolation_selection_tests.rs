use super::*;

#[test]
fn absent_and_explicit_hardware_preserve_v1_while_grid_is_opt_in() {
    assert_eq!(
        selected(None).unwrap(),
        RenderTextInterpolationBehavior::HardwareV1
    );
    assert_eq!(
        selected(Some(OsStr::new("hardware-v1"))).unwrap(),
        RenderTextInterpolationBehavior::HardwareV1
    );
    assert_eq!(
        selected(Some(OsStr::new("projected-grid12-v2"))).unwrap(),
        RenderTextInterpolationBehavior::ProjectedGrid12V2
    );
}

#[test]
fn flat_core_transport_is_an_explicit_version_four_worker_choice() {
    let behavior = selected(Some(OsStr::new("projected-grid12-flat-corners-v4"))).unwrap();
    assert_eq!(
        behavior,
        RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4
    );
    assert!(metadata(behavior).contains("\"behavior_version\":4"));
}

#[test]
fn unknown_selection_reports_value_and_expected_values() {
    let error = selected(Some(OsStr::new("grid8"))).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("grid8"));
    assert!(error.to_string().contains("projected-grid12-v2"));
}
