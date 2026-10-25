use super::{fixed, fixtures};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Face, Shape},
};
use martensite_tkmath::{
    gp::{Point3, Transform3, Vector3},
    poly::Triangle,
    toploc::Location,
};
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfacePropertyOptions, SurfacePropertyResult, SurfaceRepresentation,
};

#[test]
fn owned_results_survive_source_location_and_mesh_mutation() {
    let (mut face, mesh) = fixtures::triangle_face();
    let report = mesh_report(&face);
    let retained = *report.properties();
    mesh.set_node(1, Point3::new(0., 0., 2.)).unwrap();
    face.set_location(&translation(), false).unwrap();
    let changed = mesh_report(&face);
    assert!((changed.properties().mass() - 6.0_f64.sqrt()).abs() < 1e-12);
    assert_eq!(*report.properties(), retained);
    assert_ne!(
        changed.properties().centre_of_mass(),
        retained.centre_of_mass()
    );
}

#[test]
fn later_face_failure_neither_mutates_inputs_nor_publishes_partial_properties() {
    let plain = fixtures::cylinder_face();
    let (broken, mesh) = fixtures::triangle_face();
    mesh.set_triangle(1, Triangle::new(1, 2, 99)).unwrap();
    let before = (mesh.node(1).unwrap(), mesh.triangle(1).unwrap());
    let surface = BRepTool::surface(&plain).unwrap();
    let shape = fixtures::compound(&[plain.clone().into(), broken.into()]);
    let failure = BRepGProp::surface_properties_with_options(
        &shape,
        fixed(SurfaceRepresentation::PreferExistingMesh),
    )
    .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!((mesh.node(1).unwrap(), mesh.triangle(1).unwrap()), before);
    assert!(BRepTool::surface(&plain).unwrap() == surface);
}

#[test]
fn unavailable_face_representation_is_an_error_instead_of_partial_publication() {
    let mut empty = Face::default();
    BRepBuilder::default().make_face(&mut empty);
    let shape = fixtures::compound(&[fixtures::cylinder_face().into(), empty.into()]);
    let failure =
        BRepGProp::surface_properties_with_options(&shape, SurfacePropertyOptions::default())
            .unwrap_err();
    assert!(failure.message().contains("0 usable representations"));
}

#[test]
fn located_mesh_area_and_centroid_use_the_input_location_without_changing_it() {
    let (mut face, mesh) = fixtures::triangle_face();
    let location = translation();
    face.set_location(&location, false).unwrap();
    let original = mesh.node(1).unwrap();
    let report = mesh_report(&face);
    assert!((report.properties().mass() - 1.0).abs() < 1e-12);
    assert!(
        report
            .properties()
            .centre_of_mass()
            .distance(&Point3::new(3. + 2. / 3., 5. + 1. / 3., 7.))
            < 1e-12
    );
    assert_eq!(face.location(), &location);
    assert_eq!(mesh.node(1).unwrap(), original);
}

#[test]
fn nonfinite_measurement_values_are_not_published_as_successful_properties() {
    let (face, mesh) = fixtures::triangle_face();
    mesh.set_node(1, Point3::new(f64::NAN, 0., 0.)).unwrap();
    let failure = BRepGProp::surface_properties_with_options(
        &face,
        fixed(SurfaceRepresentation::PreferExistingMesh),
    )
    .unwrap_err();
    assert!(failure.message().contains("NaN"));
    assert!(mesh.node(1).unwrap().x().is_nan());
}

fn translation() -> Location {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(3., 5., 7.));
    Location::new(transform)
}

fn mesh_report(face: &Shape) -> SurfacePropertyResult {
    BRepGProp::surface_properties_with_options(
        face,
        fixed(SurfaceRepresentation::PreferExistingMesh),
    )
    .unwrap()
}

#[test]
fn retained_properties_do_not_alias_mutable_source_geometry() {
    let face = fixtures::cylinder_face();
    let options = fixed(SurfaceRepresentation::PreferGeometry);
    let report = BRepGProp::surface_properties_with_options(&face, options).unwrap();
    let retained = *report.properties();
    let surface = BRepTool::surface(&face)
        .unwrap()
        .cast::<martensite_tkg3d::geom::GeomCylindricalSurface>();
    surface.set_radius(2.0).unwrap();
    let changed = BRepGProp::surface_properties_with_options(&face, options).unwrap();
    assert!((changed.properties().mass() - 2.0 * std::f64::consts::PI).abs() < 1e-12);
    assert_eq!(*report.properties(), retained);
    assert!((retained.mass() - std::f64::consts::PI).abs() < 1e-12);
}
