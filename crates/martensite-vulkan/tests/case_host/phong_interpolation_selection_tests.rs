use super::*;
#[test]
fn only_explicit_versions_are_accepted_before_construction() {
    assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
    assert_eq!(
        selected(Some(OsStr::new("hardware-v1"))).unwrap(),
        Behavior::HardwareV1
    );
    assert_eq!(
        selected(Some(OsStr::new("unsnapped-v2"))).unwrap(),
        Behavior::UnsnappedV2
    );
    assert_eq!(
        selected(Some(OsStr::new("unsnapped-v3"))).unwrap(),
        Behavior::UnsnappedV3
    );
    assert!(metadata(Behavior::UnsnappedV3).contains("\"behavior_version\":3"));
    assert!(admit_observation(Behavior::UnsnappedV3).is_err());
    assert!(selected(Some(OsStr::new("bug30008"))).is_err());
    assert!(admit_observation(Behavior::HardwareV1).is_ok());
    assert!(
        admit_observation(Behavior::UnsnappedV2)
            .unwrap_err()
            .phong_interpolation_failure()
            .is_some()
    );
    assert!(metadata(Behavior::UnsnappedV2).contains("\"behavior_version\":2"));
}
