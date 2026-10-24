use super::{fixed, fixtures, support};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_gprop::{BRepGProp, SurfaceErrorEvidence, SurfaceRepresentation};
use std::f64::consts::PI;

#[test]
fn fixed_cylinder_geometry_has_closed_form_centroid_and_inertia() {
    let report = BRepGProp::surface_properties_with_options(
        &fixtures::cylinder_face(),
        fixed(SurfaceRepresentation::PreferGeometry),
    )
    .unwrap();
    let props = report.properties();
    assert_cylinder_properties(props);
}

#[test]
fn triangle_properties_use_exact_polynomial_moment_formulas() {
    let (face, _) = fixtures::triangle_face();
    let report = BRepGProp::surface_properties_with_options(
        &face,
        fixed(SurfaceRepresentation::PreferGeometry),
    )
    .unwrap();
    let props = report.properties();
    assert_triangle_properties(props);
    assert_eq!(report.error_evidence(), SurfaceErrorEvidence::Unknown);
}

#[test]
fn coarse_cylinder_triangles_have_rectangle_moments_in_their_own_plane() {
    let (face, _) = fixtures::meshed_cylinder();
    let report = BRepGProp::surface_properties_with_options(
        &face,
        fixed(SurfaceRepresentation::PreferExistingMesh),
    )
    .unwrap();
    let props = report.properties();
    let area = 2. * 2.0_f64.sqrt();
    assert!((props.mass() - area).abs() < 1e-12);
    assert!(props.centre_of_mass().distance(&Point3::new(0.5, 0.5, 1.)) < 1e-12);
    support::assert_inertia(
        props,
        [
            [5. * area / 12., area / 12., 0.],
            [area / 12., 5. * area / 12., 0.],
            [0., 0., area / 6.],
        ],
    );
}

fn assert_cylinder_properties(props: &GProps) {
    assert!((props.mass() - PI).abs() < 1e-12);
    assert!(
        props
            .centre_of_mass()
            .distance(&Point3::new(2. / PI, 2. / PI, 1.))
            < 1e-12
    );
    let diagonal = 5. * PI / 6. - 4. / PI;
    let cross = 4. / PI - 1.;
    support::assert_inertia(
        props,
        [
            [diagonal, cross, 0.],
            [cross, diagonal, 0.],
            [0., 0., PI - 8. / PI],
        ],
    );
}

fn assert_triangle_properties(props: &GProps) {
    assert!((props.mass() - 1.).abs() < 1e-12);
    assert!(
        props
            .centre_of_mass()
            .distance(&Point3::new(2. / 3., 1. / 3., 0.))
            < 1e-12
    );
    support::assert_inertia(
        props,
        [
            [1. / 18., 1. / 18., 0.],
            [1. / 18., 2. / 9., 0.],
            [0., 0., 5. / 18.],
        ],
    );
}
