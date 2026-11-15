use super::*;
#[test]
fn default_and_explicit_versions_are_independent_of_other_selectors() {
    assert_eq!(selected(None).unwrap(), Behavior::PrecomposedV1);
    assert_eq!(
        selected(Some(OsStr::new("precomposed-v1"))).unwrap(),
        Behavior::PrecomposedV1
    );
    assert_eq!(
        selected(Some(OsStr::new("factorized-v2"))).unwrap(),
        Behavior::FactorizedTrianglesV2
    );
    assert_eq!(
        selected(Some(OsStr::new("factorized-lines-v3"))).unwrap(),
        Behavior::FactorizedLinesV3
    );
    assert_eq!(
        selected(Some(OsStr::new("fixed-model-view-v4"))).unwrap(),
        Behavior::FixedFunctionModelViewV4
    );
}
#[test]
fn malformed_selection_is_not_a_fallback() {
    for value in ["", "true", "amd", "dragg", "factorized-v1"] {
        let error = selected(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}
#[test]
fn capture_composes_the_same_projection_without_dropping_other_behaviors() {
    let configuration = martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_light_dots(martensite_vulkan::RenderLightDotBehavior::NumericMaxZeroV2);
    assert_eq!(
        capture_configuration(configuration, None).unwrap(),
        configuration
    );
    assert_eq!(
        capture_configuration(configuration, Some(OsStr::new("factorized-v2"))).unwrap(),
        configuration.with_projection(Behavior::FactorizedTrianglesV2)
    );
    assert!(capture_configuration(configuration, Some(OsStr::new("wrong"))).is_err());
}
