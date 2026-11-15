use super::*;
#[test]
fn explicit_selection_preserves_strict_default_and_metadata() {
    for v in [None, Some(OsStr::new("hardware-format-v1"))] {
        assert_eq!(selected(v).unwrap(), Behavior::HardwareFormatV1);
        assert!(require_capture_default(v).is_ok());
    }
    let v = Some(OsStr::new("native24-units-v2"));
    assert_eq!(selected(v).unwrap(), Behavior::Native24UnitsV2);
    assert!(require_capture_default(v).is_err());
    assert!(metadata(selected(v).unwrap()).contains("\"behavior_version\":2"));
}
#[test]
fn malformed_selections_never_fall_back() {
    for v in ["", "true", "identic", "native24-units-v1"] {
        let e = selected(Some(OsStr::new(v))).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::InvalidInput);
        assert!(e.to_string().contains(v));
    }
}
