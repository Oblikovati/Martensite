use super::{fixed, fixtures, support};
use martensite_tkmath::{
    gp::{Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfacePropertyOptions, SurfaceRepresentation,
};

#[test]
fn empty_compound_completes_with_zero_measure_and_the_origin_centroid_convention() {
    let empty = fixtures::compound(&[]);
    let result =
        BRepGProp::surface_properties_with_options(&empty, SurfacePropertyOptions::default())
            .unwrap();
    assert!(result.is_empty());
    assert_eq!(result.properties().mass(), 0.0);
    assert_eq!(
        result.properties().centre_of_mass(),
        Point3::new(0., 0., 0.)
    );
    support::assert_inertia(result.properties(), [[0.; 3]; 3]);
    assert_eq!(result.error_evidence(), SurfaceErrorEvidence::Unknown);
}

#[test]
fn empty_query_centroid_is_the_located_origin_not_a_physical_center() {
    let mut empty = fixtures::compound(&[]);
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(3., 5., 7.));
    empty
        .set_location(&Location::new(transform), false)
        .unwrap();
    let result =
        BRepGProp::surface_properties_with_options(&empty, SurfacePropertyOptions::default())
            .unwrap();
    assert!(result.is_empty());
    assert_eq!(
        result.properties().centre_of_mass(),
        Point3::new(3., 5., 7.)
    );
}

#[test]
fn exhausted_nonempty_geometry_is_not_published_as_completed_empty() {
    let face = fixtures::narrow_parameter_square();
    let fixed = BRepGProp::surface_properties_with_options(
        &face,
        fixed(SurfaceRepresentation::PreferGeometry),
    )
    .unwrap();
    assert!(!fixed.is_empty());
    assert!((fixed.properties().mass() - 1.0).abs() < 1e-12);
    let error =
        BRepGProp::surface_properties_with_options(&face, SurfacePropertyOptions::default())
            .unwrap_err();
    assert_eq!(error.exception_type(), "StdFail_NotDone");
    assert!(error.message().contains("refinement"));
}
