use super::{fixed, fixtures, support::property_bits};
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfaceIntegration, SurfacePropertyOptions,
    SurfaceRepresentation,
};

#[test]
fn established_fixed_and_adaptive_entry_points_replay_all_property_bits() {
    let shape = fixtures::elliptic_prism_faces();
    for epsilon in [1.0, 1e-5, 1e-9] {
        check_epsilon_replay(&shape, epsilon);
    }
}

#[test]
fn default_mesh_and_geometry_preferences_replay_established_full_properties() {
    let (face, _) = fixtures::meshed_cylinder();
    for (prefer_mesh, preference) in [
        (false, SurfaceRepresentation::PreferGeometry),
        (true, SurfaceRepresentation::PreferExistingMesh),
    ] {
        let mut previous = GProps::new();
        BRepGProp::surface_properties(&face, &mut previous, false, prefer_mesh).unwrap();
        let options = SurfacePropertyOptions {
            representation: preference,
            ..Default::default()
        };
        let result = BRepGProp::surface_properties_with_options(&face, options).unwrap();
        assert_eq!(property_bits(result.properties()), property_bits(&previous));
    }
}

#[test]
fn fixed_preference_for_missing_meshes_is_selectable_without_changing_defaults() {
    let shape = fixtures::elliptic_prism_faces();
    let fixed = BRepGProp::surface_properties_with_options(
        &shape,
        fixed(SurfaceRepresentation::PreferExistingMesh),
    )
    .unwrap();
    let adaptive =
        BRepGProp::surface_properties_with_options(&shape, SurfacePropertyOptions::default())
            .unwrap();
    let reference_enclosure = (41.6317152760022_f64, 41.631715276002204_f64);
    assert!((fixed.properties().mass() - reference_enclosure.0).abs() < 0.002);
    assert!(adaptive.properties().mass() < reference_enclosure.0);
    assert_eq!(fixed.error_evidence(), SurfaceErrorEvidence::Unknown);
    assert!(matches!(
        adaptive.error_evidence(),
        SurfaceErrorEvidence::EstimatedRelative { .. }
    ));
    assert_eq!(adaptive.effective_relative_target(), None);
}

fn check_epsilon_replay(shape: &Shape, epsilon: f64) {
    let mut previous = GProps::new();
    let error = BRepGProp::surface_properties_eps(shape, &mut previous, epsilon, false).unwrap();
    let options = epsilon_options(epsilon);
    let result = BRepGProp::surface_properties_with_options(shape, options).unwrap();
    assert_eq!(property_bits(result.properties()), property_bits(&previous));
    let evidence = if epsilon == 1.0 {
        SurfaceErrorEvidence::Unknown
    } else {
        SurfaceErrorEvidence::EstimatedRelative {
            max_face_relative_error: error,
        }
    };
    assert_eq!(result.error_evidence(), evidence);
}

fn epsilon_options(epsilon: f64) -> SurfacePropertyOptions {
    let integration = if epsilon == 1.0 {
        SurfaceIntegration::FixedOrder
    } else {
        SurfaceIntegration::AdaptiveEstimate {
            requested_relative_target: epsilon,
        }
    };
    SurfacePropertyOptions {
        integration,
        ..Default::default()
    }
}
