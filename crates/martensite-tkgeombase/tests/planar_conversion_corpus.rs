//! Complete original planar curve conversion families and independent geometry checks.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/planar_conversion_corpus/inputs-00.txt"),
        include_str!("fixtures/planar_conversion_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/planar_conversion_corpus/inputs-01.txt"),
        include_str!("fixtures/planar_conversion_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/planar_conversion_corpus/inputs-02.txt"),
        include_str!("fixtures/planar_conversion_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/planar_conversion_corpus/inputs-03.txt"),
        include_str!("fixtures/planar_conversion_corpus/expected-03.txt"),
    ),
];
#[test]
fn g2c_to_bspline_complete_family() {
    super::replay_family(CHUNKS, "g2c_to_bspline", 306);
}
#[test]
fn g2c_split_complete_family() {
    super::replay_family(CHUNKS, "g2c_split", 120);
}
#[test]
fn g2c_knot_split_complete_family() {
    super::replay_family(CHUNKS, "g2c_knot_split", 42);
}
#[test]
fn g2c_bezier_complete_family() {
    super::replay_family(CHUNKS, "g2c_bezier", 40);
}
#[test]
fn g2c_comp_complete_family() {
    super::replay_family(CHUNKS, "g2c_comp", 56);
}
#[test]
fn g2c_concat_complete_family() {
    super::replay_family(CHUNKS, "g2c_concat", 145);
}
#[test]
fn g2c_c0c1_complete_family() {
    super::replay_family(CHUNKS, "g2c_c0c1", 63);
}
#[test]
fn g2c_approx_complete_family() {
    super::replay_family(CHUNKS, "g2c_approx", 51);
}
#[test]
fn g2c_arcs_complete_family() {
    super::replay_family(CHUNKS, "g2c_arcs", 76);
}
#[test]
fn g2c_ppoint_complete_family() {
    super::replay_family(CHUNKS, "g2c_ppoint", 9);
}
#[test]
fn planar_conversion_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x83fb0bc5120b78ee
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x0a5e5fbf606e17c0
    );
}
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dBoundedCurve, Geom2dCurve,
};
use martensite_tkgeombase::geom2d_convert::{self as convert, BSplineCurveToBezierCurve, PPoint};
use martensite_tkmath::{
    convert::ParameterisationType,
    gp::{Point2, Xy},
};
fn array<T: Copy + Default>(values: &[T]) -> Array1<'static, T> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    array.as_mut_slice().copy_from_slice(values);
    array
}
fn corner_polyline() -> Handle<Geom2dBSplineCurve> {
    let poles = array(&[
        Point2::new(0., 0.),
        Point2::new(2., 0.),
        Point2::new(2., 3.),
    ]);
    Handle::new(
        Geom2dBSplineCurve::new(&poles, &array(&[0., 1., 2.]), &array(&[2, 1, 2]), 1, false)
            .unwrap(),
    )
}
#[test]
fn straight_quadratic_conversion_has_closed_form_values_and_derivatives() {
    let poles = array(&[
        Point2::new(0., 0.),
        Point2::new(2., 0.),
        Point2::new(4., 0.),
    ]);
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dBezierCurve::new(&poles).unwrap()).cast();
    let converted =
        convert::curve_to_bspline_curve(&curve, ParameterisationType::TgtThetaOver2).unwrap();
    assert_eq!(
        (
            converted.degree(),
            converted.nb_poles(),
            converted.is_rational()
        ),
        (2, 3, false)
    );
    for t in [0., 0.25, 0.5, 0.75, 1.] {
        let (p, d) = converted.d1(t).unwrap();
        assert_eq!(p, Point2::new(4. * t, 0.));
        assert_eq!((d.x(), d.y()), (4., 0.));
    }
    assert_eq!(curve.value(0.5).unwrap(), Point2::new(2., 0.));
}
#[test]
fn parameter_split_preserves_corner_geometry_and_source_domain() {
    let source = corner_polyline();
    let split = convert::split_bspline_curve(&source, 0.5, 1.5, 1e-9, true).unwrap();
    for (u, p) in [
        (0.5, Point2::new(1., 0.)),
        (1., Point2::new(2., 0.)),
        (1.5, Point2::new(2., 1.5)),
    ] {
        assert_eq!(split.value(u).unwrap(), p);
    }
    assert_eq!(
        (
            source.first_parameter(),
            source.last_parameter(),
            source.nb_poles()
        ),
        (0., 2., 3)
    );
    assert_eq!(source.end_point().unwrap(), Point2::new(2., 3.));
}
#[test]
fn bezier_decomposition_has_independent_segment_control_polygons() {
    let source = corner_polyline();
    let decomposition = BSplineCurveToBezierCurve::new(&source).unwrap();
    assert_eq!(decomposition.nb_arcs().unwrap(), 2);
    let first = decomposition.arc(1).unwrap();
    let second = decomposition.arc(2).unwrap();
    assert_eq!((first.degree(), second.degree()), (1, 1));
    assert_eq!(
        (first.pole(1).unwrap(), first.pole(2).unwrap()),
        (Point2::new(0., 0.), Point2::new(2., 0.))
    );
    assert_eq!(
        (second.pole(1).unwrap(), second.pole(2).unwrap()),
        (Point2::new(2., 0.), Point2::new(2., 3.))
    );
    assert_eq!(
        decomposition.arc(0).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn parameter_point_distance_is_euclidean_and_tangent_update_preserves_location() {
    let mut a = PPoint::new(1., Xy::new(0., 0.), Xy::new(1., 0.));
    let b = PPoint::new(2., Xy::new(3., 4.), Xy::new(0., 1.));
    assert_eq!(a.dist(&b), 5.);
    a.set_d1(Xy::new(-2., 3.));
    assert_eq!(
        (a.parameter(), a.point(), a.d1()),
        (1., Xy::new(0., 0.), Xy::new(-2., 3.))
    );
    // Equality is a parameter contract, independent of distance and derivative.
    assert_eq!(a, PPoint::new(1., Xy::new(99., 99.), Xy::new(0., 0.)));
}
#[test]
fn null_curve_serializers_preserve_sentinel_and_do_not_add_a_newline() {
    use super::construction_corpus::{geom2d_convert, print::Output};
    let output = Output::default();
    geom2d_convert::bspline(&output, &Handle::default());
    geom2d_convert::bezier(&output, &Handle::default());
    assert_eq!(output.finish(), "-1 -1 ");
}
