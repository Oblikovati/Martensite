use super::fixtures;
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfaceIntegration, SurfacePropertyOptions,
    SurfacePropertyResult, SurfaceRepresentation,
};

#[test]
fn defaults_keep_adaptive_geometry_and_expose_unknown_effective_target() {
    let options = SurfacePropertyOptions::default();
    assert_eq!(
        options.integration,
        SurfaceIntegration::AdaptiveEstimate {
            requested_relative_target: 1e-5
        }
    );
    assert_eq!(
        options.representation,
        SurfaceRepresentation::PreferGeometry
    );
    assert!(!options.skip_shared);
    let result =
        BRepGProp::surface_properties_with_options(&fixtures::cylinder_face(), options).unwrap();
    check_default_report(&result, options);
}

fn check_default_report(result: &SurfacePropertyResult, options: SurfacePropertyOptions) {
    assert_eq!(result.options(), options);
    assert_eq!(result.effective_relative_target(), None);
    assert_eq!(result.geometric_faces(), 1);
    assert_eq!(result.triangulated_faces(), 0);
    assert!(
        matches!(result.error_evidence(), SurfaceErrorEvidence::EstimatedRelative { max_face_relative_error } if max_face_relative_error.is_finite())
    );
}

#[test]
fn invalid_requested_targets_identify_the_value_and_open_interval() {
    for target in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -1.,
        -0.,
        0.,
        0.001,
        1.,
    ] {
        assert_invalid_target(target);
    }
}

fn assert_invalid_target(target: f64) {
    let options = SurfacePropertyOptions {
        integration: SurfaceIntegration::AdaptiveEstimate {
            requested_relative_target: target,
        },
        ..Default::default()
    };
    let failure =
        BRepGProp::surface_properties_with_options(&Shape::default(), options).unwrap_err();
    assert!(
        failure.message().contains(&target.to_string()),
        "{}",
        failure.message()
    );
    assert!(
        failure.message().contains("0 < target < 0.001"),
        "{}",
        failure.message()
    );
}

#[test]
fn empty_shape_completes_with_zero_properties_and_unknown_error() {
    let result = BRepGProp::surface_properties_with_options(
        &Shape::default(),
        SurfacePropertyOptions::default(),
    )
    .unwrap();
    assert_eq!(result.properties().mass(), 0.0);
    assert_eq!(result.geometric_faces(), 0);
    assert_eq!(result.triangulated_faces(), 0);
    assert_eq!(result.error_evidence(), SurfaceErrorEvidence::Unknown);
}

#[test]
fn public_policy_and_result_types_have_structural_thread_bounds() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<SurfaceIntegration>();
    send_sync::<SurfaceRepresentation>();
    send_sync::<SurfacePropertyOptions>();
    send_sync::<SurfaceErrorEvidence>();
    send_sync::<SurfacePropertyResult>();
}

#[test]
fn finite_positive_targets_keep_their_exact_requested_bits_without_interface_clamping() {
    for target in [
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        f64::from_bits(0.001_f64.to_bits() - 1),
    ] {
        let options = SurfacePropertyOptions {
            integration: SurfaceIntegration::AdaptiveEstimate {
                requested_relative_target: target,
            },
            ..Default::default()
        };
        let result =
            BRepGProp::surface_properties_with_options(&Shape::default(), options).unwrap();
        assert_eq!(result.options(), options);
        assert_eq!(result.effective_relative_target(), None);
    }
}
