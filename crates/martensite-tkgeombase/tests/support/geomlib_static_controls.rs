//! Affine geometry, periodicity and exact line distances supply independent expectations.
use super::super::construction_corpus::{
    geometry_record_fixtures as build, geometry_utilities_elementary::read_reals,
    geometry_utilities_statics::curve_on_surface, print::Reader,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d, geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    geom::{GeomBSplineSurface, GeomBoundedCurve, GeomBoundedSurface, GeomCurve, GeomSurface},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
};
use martensite_tkgeombase::geom_lib as gl;
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2, Direction3, GeneralTransform2, Point2, Point3, Vector3},
};

fn line2d(direction: [f64; 2]) -> Handle<dyn Geom2dCurve> {
    build::curve2d(&mut Reader::new(&[1., 3., 4., direction[0], direction[1]])).unwrap()
}
fn planar_surface(kind: f64, extra: &[f64]) -> Handle<dyn GeomSurface> {
    let mut fields = vec![kind, 0., 0., 0., 0., 0., 1., 1., 0., 0., 0.];
    fields.extend_from_slice(extra);
    build::surface(&mut Reader::new(&fields)).unwrap()
}
fn segment() -> Handle<dyn GeomBoundedCurve> {
    build::curve(&mut Reader::new(&[6., 0., 2., 0., 0., 0., 1., 0., 0.]))
        .unwrap()
        .cast()
}
fn assert_point2(curve: &Handle<dyn Geom2dCurve>, parameter: f64, expected: [f64; 2]) {
    assert_eq!(
        curve.value(parameter).unwrap(),
        Point2::new(expected[0], expected[1])
    );
}
#[test]
fn planar_placement_obeys_frame_axes_and_translation() {
    let frame = Axis2::with_x_direction(
        Point3::new(10., 20., 30.),
        Direction3::new(0., 0., 1.).unwrap(),
        Direction3::new(0., 1., 0.).unwrap(),
    )
    .unwrap();
    let curve = gl::to_3d(frame, &line2d([1., 0.])).unwrap();
    for t in [0., 2.] {
        assert_eq!(curve.value(t).unwrap(), Point3::new(6., 23. + t, 30.));
    }
}
#[test]
fn general_line_transform_normalizes_parameter_direction() {
    let mut transform = GeneralTransform2::default();
    for (i, j, value) in [(1, 1, 2.), (2, 2, 3.), (1, 3, 5.), (2, 3, 7.)] {
        transform.set_value(i, j, value).unwrap();
    }
    let curve = gl::gtransform(&line2d([1., 0.]), &transform).unwrap();
    assert_point2(&curve, 0., [11., 19.]);
    assert_point2(&curve, 2., [13., 19.]);
}
#[test]
fn general_bezier_transform_preserves_affine_parameterization() {
    let curve = build::curve2d(&mut Reader::new(&[6., 0., 2., 0., 0., 2., 2.])).unwrap();
    let mut transform = GeneralTransform2::default();
    for (i, j, value) in [(1, 1, 2.), (2, 2, 3.), (1, 3, 5.), (2, 3, 7.)] {
        transform.set_value(i, j, value).unwrap();
    }
    assert_point2(&gl::gtransform(&curve, &transform).unwrap(), 0.5, [7., 10.]);
}
#[test]
fn identical_range_returns_the_existing_handle() {
    let source = line2d([1., 0.]);
    let mut output = Handle::default();
    gl::same_range(1e-9, &source, (0., 1.), (0., 1.), &mut output).unwrap();
    assert_eq!(source, output);
}
#[test]
fn translated_range_preserves_endpoint_geometry() {
    let source = line2d([1., 0.]);
    let mut output = Handle::default();
    gl::same_range(1e-9, &source, (0., 1.), (2., 3.), &mut output).unwrap();
    assert_point2(&output, 2., [3., 4.]);
    assert_point2(&output, 3., [4., 4.]);
    assert_point2(&source, 0., [3., 4.]);
}
#[test]
fn scaled_bezier_range_has_exact_linear_midpoint() {
    let source = build::curve2d(&mut Reader::new(&[6., 0., 2., 0., 0., 2., 2.])).unwrap();
    let mut output = Handle::default();
    gl::same_range(1e-9, &source, (0., 1.), (2., 4.), &mut output).unwrap();
    for (t, p) in [(2., [0., 0.]), (3., [1., 1.]), (4., [2., 2.])] {
        assert_point2(&output, t, p);
    }
}
#[test]
fn plane_normal_is_the_oriented_parameter_cross_product() {
    let surface = planar_surface(1., &[]);
    let mut normal = Direction3::new(1., 0., 0.).unwrap();
    assert_eq!(
        gl::norm_estim(&surface, Point2::new(2., 3.), 1e-9, &mut normal).unwrap(),
        0
    );
    assert_eq!(normal, Direction3::new(0., 0., 1.).unwrap());
}
#[test]
fn analytic_surface_closedness_follows_periodic_parameters() {
    for (kind, extra, expected) in [
        (1., vec![], (false, false)),
        (2., vec![2.], (true, false)),
        (5., vec![4., 1.], (true, true)),
    ] {
        assert_eq!(
            gl::is_closed(&planar_surface(kind, &extra), 1e-9).unwrap(),
            expected
        );
    }
}
#[test]
fn isoline_classification_distinguishes_constant_coordinate_and_orientation() {
    for (direction, expected) in [
        ([1., 0.], Some((false, 4., true))),
        ([0., -1.], Some((true, 3., false))),
        ([1., 1.], None),
    ] {
        let curve: Handle<dyn Adaptor2dCurve2d> =
            Handle::new(Geom2dAdaptorCurve::with_range(&line2d(direction), 0., 1.).unwrap()).cast();
        let actual = gl::is_iso_line(&curve)
            .unwrap()
            .map(|iso| (iso.is_u, iso.param, iso.is_forward));
        assert_eq!(actual, expected);
    }
}
#[test]
fn plane_isoline_and_general_builder_agree_with_the_affine_map() {
    let pcurve = line2d([1., 0.]);
    let surface = planar_surface(1., &[]);
    let c2d: Handle<dyn Adaptor2dCurve2d> =
        Handle::new(Geom2dAdaptorCurve::with_range(&pcurve, 0., 1.).unwrap()).cast();
    let surf: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&surface).unwrap()).cast();
    let iso = gl::is_iso_line(&c2d).unwrap().unwrap();
    let exact = gl::build_c3d_on_iso_line(&c2d, &surf, (0., 1.), 1e-9, iso).unwrap();
    let mut general: Handle<dyn GeomCurve> = Handle::default();
    assert_eq!(
        gl::build_curve3d(
            1e-9,
            &curve_on_surface(&pcurve, &surface).unwrap(),
            (0., 1.),
            &mut general,
            Shape::C1,
            8,
            10
        )
        .unwrap(),
        (0., 0.)
    );
    for t in [0., 0.5, 1.] {
        for curve in [&exact, &general] {
            assert_eq!(curve.value(t).unwrap(), Point3::new(3. + t, 4., 0.));
        }
    }
}
#[test]
fn identical_curve_distance_reports_the_source_tolerance_floor_separately() {
    let curve = build::curve(&mut Reader::new(&[1., 0., 0., 0., 1., 0., 0.])).unwrap();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let parameters = read_reals(&mut Reader::new(&[-2., 3., -1., 0., 1.]));
    assert_eq!(
        gl::eval_max_parametric_distance(&adaptor, &adaptor, 1e-9, &parameters).unwrap(),
        0.
    );
    assert_eq!(
        gl::eval_max_distance_along_parameter(&adaptor, &adaptor, 1e-9, &parameters).unwrap(),
        1e-9
    );
}
#[test]
fn adjusted_segment_matches_independent_cubic_hermite_end_constraints() {
    let mut curve = segment();
    gl::adjust_extremity(
        &mut curve,
        Point3::new(0., 1., 0.),
        Point3::new(2., 1., 0.),
        Vector3::new(1., 0., 0.),
        Vector3::new(1., 0., 0.),
    )
    .unwrap();
    for t in [0., 0.5, 1.] {
        let actual = curve.value(t).unwrap();
        let expected_x = -2. * t * t * t + 3. * t * t + t;
        // Cubic Bernstein conversion contains rounded thirds. Twenty unit-roundoff
        // contributions at coordinate scale two bound this finite test's evaluation.
        let arithmetic_bound = 20. * f64::EPSILON;
        assert!((actual.x() - expected_x).abs() <= arithmetic_bound);
        assert_eq!((actual.y(), actual.z()), (1., 0.));
    }
    for t in [0., 1.] {
        let tangent = curve.eval_d1(t).unwrap().d1;
        assert!(tangent.x() > 0.);
        assert_eq!((tangent.y(), tangent.z()), (0., 0.));
    }
}
#[test]
fn invalid_extension_continuity_is_a_source_defined_noop() {
    let original = segment();
    let mut curve = original.clone();
    gl::extend_curve_to_point(&mut curve, Point3::new(2., 0., 0.), 0, true).unwrap();
    assert_eq!(curve, original);
    let mut surface: Handle<dyn GeomBoundedSurface> = Handle::default();
    gl::extend_surf_by_length(&mut surface, 2., 4, true, true).unwrap();
    assert!(surface.is_null());
}
#[test]
fn cancelling_no_denominator_directions_does_not_access_the_surface() {
    let mut surface: Handle<GeomBSplineSurface> = Handle::default();
    gl::cancel_denominator_derivative(&mut surface, false, false).unwrap();
    assert!(surface.is_null());
}
