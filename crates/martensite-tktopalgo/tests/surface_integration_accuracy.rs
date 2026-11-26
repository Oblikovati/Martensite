//! Independent analytic controls for general bounded-surface integration (#407).
#[path = "surface_integration_accuracy/fixtures.rs"]
mod fixtures;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfaceIntegration, SurfacePropertyOptions,
};
// Outward rational enclosure: docs/features/surface-integration-accuracy/reference-bounds.json.
const HALF_ELLIPSE_AREA: f64 = 5.105399772679626;

#[test]
fn adaptive_surface_area_matches_independent_elliptic_integral() {
    let face = fixtures::extrusion(1., 0., 2.);
    for target in [1e-5, 1e-9, 1e-12] {
        verify_area_estimate(&face, target);
    }
}
fn verify_area_estimate(face: &martensite_tkbrep::topods::Shape, target: f64) {
    let (props, achieved) = measurement(face, target);
    assert!(achieved <= target, "reported {achieved}, target {target}");
    let observed = (props.mass() - HALF_ELLIPSE_AREA).abs() / HALF_ELLIPSE_AREA;
    assert!(
        observed <= target,
        "area {}, error {observed}, reported {achieved}",
        props.mass()
    );
    let rounding = 16. * f64::EPSILON;
    assert!(
        observed <= achieved + rounding,
        "observed {observed}, reported {achieved}"
    );
    assert!((props.centre_of_mass().x() - 3.414590200008447).abs() <= target);
}
fn measurement(face: &martensite_tkbrep::topods::Shape, target: f64) -> (GProps, f64) {
    let result =
        BRepGProp::surface_properties_with_options(face, adaptive_options(target)).unwrap();
    let achieved = match result.error_evidence() {
        SurfaceErrorEvidence::EstimatedRelative {
            max_face_relative_error,
        } => max_face_relative_error,
        other => panic!("expected EstimatedRelative evidence, got {other:?}"),
    };
    (result.properties().clone(), achieved)
}

#[test]
fn equivalent_parameter_periods_orientations_and_model_scales_keep_relative_accuracy() {
    for scale in [1e-6, 1., 1e6] {
        for phase in [
            0.,
            std::f64::consts::TAU,
            -std::f64::consts::TAU,
            std::f64::consts::PI / 7.,
            std::f64::consts::FRAC_PI_2,
        ] {
            verify_period_and_orientation(scale, phase);
        }
    }
}
fn verify_period_and_orientation(scale: f64, phase: f64) {
    let face = fixtures::extrusion(scale, phase, 2.);
    let reference = HALF_ELLIPSE_AREA * scale;
    for shape in [&face, &face.reversed()] {
        let (props, achieved) = measurement(shape, 1e-12);
        let observed = (props.mass() - reference).abs() / reference;
        assert!(
            observed <= 1e-12,
            "scale {scale}, phase {phase}, error {observed}, reported {achieved}"
        );
    }
}

#[test]
fn small_nonzero_parameter_intervals_are_integrated_instead_of_discarded() {
    for height in [1e-13, 1e-9, 1., 1e9] {
        let face = fixtures::extrusion(1., 0., height);
        let (props, _) = measurement(&face, 1e-12);
        let reference = HALF_ELLIPSE_AREA * height / 2.;
        assert!(
            (props.mass() - reference).abs() <= reference * 1e-12,
            "height {height}, area {}",
            props.mass()
        );
    }
}

#[test]
fn exhausted_adaptive_refinement_is_a_named_failure_in_the_owned_api() {
    let face = fixtures::extrusion(1., 0., 2.);
    let error =
        BRepGProp::surface_properties_with_options(&face, adaptive_options(1e-30)).unwrap_err();
    assert_eq!(error.exception_type(), "StdFail_NotDone");
    assert!(error.message().contains("refinement"));
}
fn adaptive_options(target: f64) -> SurfacePropertyOptions {
    SurfacePropertyOptions {
        integration: SurfaceIntegration::AdaptiveEstimate {
            requested_relative_target: target,
        },
        ..Default::default()
    }
}
#[test]
fn area_and_centroid_match_independently_partitioned_occt_faces() {
    let reference = include_str!("data/surface_integration/occt-reference.csv");
    for row in reference
        .lines()
        .filter(|line| line.starts_with("partitioned,"))
    {
        verify_occt_row(row);
    }
}
fn verify_occt_row(row: &str) {
    let fields: Vec<_> = row.split(',').collect();
    let target: f64 = fields[1].parse().unwrap();
    let area: f64 = fields[3].parse().unwrap();
    let centroid: f64 = fields[4].parse().unwrap();
    assert!((area - HALF_ELLIPSE_AREA).abs() <= 16. * f64::EPSILON * area);
    let (props, _) = measurement(&fixtures::extrusion(1., 0., 2.), target);
    assert!((props.mass() - area).abs() <= target * area);
    assert!((props.centre_of_mass().x() - centroid).abs() <= target);
}

#[test]
fn generated_eccentricities_require_accurate_area_or_explicit_budget_failure() {
    let references = include_str!("data/surface_integration/eccentricity-controls.csv");
    for row in references.lines().filter(|line| !line.starts_with('#')) {
        let (ratio, area) = row.split_once(',').unwrap();
        let ratio: f64 = ratio.parse().unwrap();
        let area: f64 = area.parse().unwrap();
        verify_eccentricity(ratio, area);
    }
}
fn verify_eccentricity(ratio: f64, area: f64) {
    let shape = fixtures::extrusion_ratio(1., ratio, 0., 2.);
    let options = adaptive_options(1e-12);
    if ratio == 0.000001 {
        let error = BRepGProp::surface_properties_with_options(&shape, options).unwrap_err();
        assert_eq!(error.exception_type(), "StdFail_NotDone");
        assert!(error.message().contains("refinement"));
        return;
    }
    let result = BRepGProp::surface_properties_with_options(&shape, options).unwrap();
    assert!(
        (result.properties().mass() - area).abs() <= area * 1e-12,
        "ratio {ratio}, area {}",
        result.properties().mass()
    );
}
