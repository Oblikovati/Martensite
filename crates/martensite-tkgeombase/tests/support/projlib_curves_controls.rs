//! Linear projection and affine control-pole maps provide independent expectations.
use super::super::construction_corpus::{geometry_adaptor_fixtures as fixture, print::Reader};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d::Geom2dCurve};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkgeombase::proj_lib::{
    CompProjectedCurve, ComputeApprox, ComputeApproxOnPolarSurface, ProjectOnPlane, ProjectedCurve,
    make_pcurve_of_type,
};
use martensite_tkmath::{
    geom_abs::{CurveType, Shape},
    gp::{Axis3, Direction3, Point2, Point3, Vector3},
};

fn line(z: f64) -> Handle<dyn Adaptor3dCurve> {
    fixture::adapted_curve(&mut Reader::new(&[1., 0., 2., 1., 0., 2., z, 1., 0., 0.])).unwrap()
}
fn plane() -> Handle<dyn Adaptor3dSurface> {
    fixture::adapted_surface(&mut Reader::new(&[
        1., 0., 0., 0., 0., 0., 1., 1., 0., 0., 0.,
    ]))
    .unwrap()
}
#[test]
fn shared_shape_decoder_keeps_the_enumerator_order() {
    for (i, shape) in [
        Shape::C0,
        Shape::G1,
        Shape::C1,
        Shape::G2,
        Shape::C2,
        Shape::C3,
        Shape::CN,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(fixture::shape(i as i32), shape);
    }
}
#[test]
fn shared_adaptor_fixtures_preserve_whole_and_trimmed_curve_ranges() {
    let whole = fixture::adapted_curve(&mut Reader::new(&[0., 6., 0., 2., 0., 0., 0., 2., 0., 0.]))
        .unwrap();
    assert_eq!(
        (
            whole.first_parameter().unwrap(),
            whole.last_parameter().unwrap()
        ),
        (0., 1.)
    );
    let trimmed = line(3.);
    assert_eq!(
        (
            trimmed.first_parameter().unwrap(),
            trimmed.last_parameter().unwrap()
        ),
        (0., 2.)
    );
    assert_eq!(trimmed.value(1.).unwrap(), Point3::new(1., 2., 3.));
    let planar = fixture::adapted_curve2d(&mut Reader::new(&[1., 3., 4., 1., 0.])).unwrap();
    assert_eq!(planar.value(2.).unwrap(), Point2::new(5., 4.));
}
#[test]
fn normal_projection_preserves_linear_parameterization_and_derivatives() {
    let projection = ProjectOnPlane::with_plane(&Axis3::default());
    projection.load(&line(3.), 1e-9, true).unwrap();
    assert_eq!(projection.get_type().unwrap(), CurveType::Line);
    assert_eq!(
        (
            projection.first_parameter().unwrap(),
            projection.last_parameter().unwrap()
        ),
        (0., 2.)
    );
    for t in [0., 0.5, 2.] {
        assert_eq!(projection.value(t).unwrap(), Point3::new(t, 2., 0.));
    }
    let (p, d1, d2) = projection.d2(1.).unwrap();
    assert_eq!(p, Point3::new(1., 2., 0.));
    assert_eq!(d1, Vector3::new(1., 0., 0.));
    assert_eq!(d2, Vector3::default());
}
#[test]
fn oblique_projection_uses_direction_ratio_and_not_nearest_point_distance() {
    let direction = Direction3::new(1., 0., -1.).unwrap();
    let projection = ProjectOnPlane::with_direction(&Axis3::default(), direction);
    projection.load(&line(3.), 1e-9, true).unwrap();
    for t in [0., 1., 2.] {
        assert_eq!(projection.value(t).unwrap(), Point3::new(t + 3., 2., 0.));
    }
}
#[test]
fn projected_adaptor_trim_retains_the_selected_linear_subrange() {
    let projection = ProjectOnPlane::with_plane(&Axis3::default());
    projection.load(&line(3.), 1e-9, true).unwrap();
    let trimmed = projection.trim(0.5, 1.5, 1e-9).unwrap();
    assert_eq!(
        (
            trimmed.first_parameter().unwrap(),
            trimmed.last_parameter().unwrap()
        ),
        (0.5, 1.5)
    );
    assert_eq!(trimmed.value(1.).unwrap(), Point3::new(1., 2., 0.));
}
#[test]
fn projected_curve_exposes_exact_geometry_but_inherits_unimplemented_point_evaluation() {
    let projected = ProjectedCurve::with_tolerance(&plane(), &line(0.), 1e-9).unwrap();
    assert_eq!(projected.get_type().unwrap(), CurveType::Line);
    assert_eq!(
        projected.value(1.).unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
    let result = make_pcurve_of_type(&projected).unwrap();
    for t in [0., 1., 2.] {
        assert_eq!(result.value(t).unwrap(), Point2::new(t, 2.));
    }
}
#[test]
fn planar_bezier_projection_preserves_affine_control_point_geometry() {
    let curve = fixture::adapted_curve(&mut Reader::new(&[0., 6., 0., 2., 0., 2., 3., 2., 4., 5.]))
        .unwrap();
    let result = ComputeApprox::with_curve(&curve, &plane(), 1e-9).unwrap();
    assert!(result.bspline().is_null());
    assert!(!result.bezier().is_null());
    for t in [0., 0.5, 1.] {
        assert_eq!(
            result.bezier().value(t).unwrap(),
            Point2::new(2. * t, 2. + 2. * t)
        );
    }
    assert_eq!(curve.value(0.).unwrap(), Point3::new(0., 2., 3.));
}
#[test]
fn empty_approximation_and_polar_results_keep_defined_failure_state() {
    let mut approximation = ComputeApprox::new();
    assert!(approximation.bspline().is_null());
    assert!(approximation.bezier().is_null());
    approximation.set_tolerance(1e-6);
    assert_eq!(approximation.tolerance(), 1e-6);
    let polar = ComputeApproxOnPolarSurface::new();
    assert!(!polar.is_done());
    assert!(polar.bspline().is_null());
    assert_eq!(polar.tolerance(), -1.);
    assert_eq!(
        polar.curve2d().unwrap_err().exception_type(),
        "Standard_NoSuchObject"
    );
}
#[test]
fn composite_line_projection_preserves_parameter_and_constant_surface_coordinate() {
    let composite = CompProjectedCurve::with_tolerances(&plane(), &line(3.), 1e-9, 1e-9).unwrap();
    assert_eq!(composite.nb_curves(), 1);
    assert_eq!(composite.bounds(1).unwrap(), (0., 2.));
    assert_eq!(composite.is_v_iso(1).unwrap(), (true, 2.));
    for t in [0., 1., 2.] {
        assert_eq!(composite.value(t).unwrap(), Point2::new(t, 2.));
    }
}
