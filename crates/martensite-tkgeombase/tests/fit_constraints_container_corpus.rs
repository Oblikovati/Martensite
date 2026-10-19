//! Complete constraint-point and multi-line requests with owned independent state checks.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/fit_constraints_container_corpus/inputs.txt"),
    include_str!("fixtures/fit_constraints_container_corpus/native.txt"),
)];
#[test]
fn ad_mpc_complete_family() {
    super::replay_family(CHUNKS, "ad_mpc", 171);
}
#[test]
fn ad_line_complete_family() {
    super::replay_family(CHUNKS, "ad_line", 67);
}
#[test]
fn constraint_container_requests_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xf162bdd20bbb4734
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x6c1229e913f78896
    );
}
use super::construction_corpus::{
    common::Reader,
    constraint_line_fixtures::read_line,
    fit_constraints_line::read_ad_line,
    fit_constraints_points::{array, read_mpc},
};
use martensite_tkgeombase::{app_def::MultiPointConstraint, app_par_curves::LineTool};
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};
#[test]
fn constraint_arrays_keep_shifted_inputs_mixed_dimensions_and_shared_vectors() {
    let mut r = Reader::new(&[
        5., 3., 1., 1., 1., 1., 1., 1., 1., 2., 3., 4., 5., 1., 0., 0., 0., 1., 0., 2., 0., 3., 0.,
        99.,
    ]);
    let point = read_mpc(&mut r).unwrap();
    assert_eq!(point.point(1).unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(point.point2d(2).unwrap(), Point2::new(4., 5.));
    assert_eq!(point.tang(1).unwrap(), Vector3::new(1., 0., 0.));
    assert_eq!(point.tang2d(2).unwrap(), Vector2::new(0., 1.));
    assert_eq!(point.curv(1).unwrap(), Vector3::new(0., 2., 0.));
    assert_eq!(point.curv2d(2).unwrap(), Vector2::new(3., 0.));
    assert!(point.is_tangency_point() && point.is_curvature_point());
    let mut copy = point.clone();
    copy.set_tang(1, Vector3::new(7., 8., 9.)).unwrap();
    assert_eq!(point.tang(1).unwrap(), Vector3::new(7., 8., 9.));
    assert_eq!(r.real(), 99.);
}
#[test]
fn a_failing_setter_allocates_its_array_before_reporting_the_invalid_index() {
    let mut point = MultiPointConstraint::with_counts(1, 1).unwrap();
    assert!(!point.is_tangency_point());
    let error = point.set_tang(0, Vector3::new(1., 0., 0.)).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
    assert!(point.is_tangency_point());
    assert_eq!(point.tang(1).unwrap(), Vector3::new(0., 0., 0.));
    assert!(!point.is_curvature_point());
}
#[test]
fn allocation_on_a_copy_does_not_retroactively_attach_an_array_to_the() {
    let point = MultiPointConstraint::with_counts(1, 0).unwrap();
    let mut copy = point.clone();
    copy.set_tang(1, Vector3::new(2., 3., 4.)).unwrap();
    assert!(!point.is_tangency_point());
    assert!(copy.is_tangency_point());
    assert_eq!(copy.tang(1).unwrap(), Vector3::new(2., 3., 4.));
}
#[test]
fn line_constructor_reindexes_shifted_point_arrays_and_keeps_copy_sharing() {
    let mut r = Reader::new(&[3., -2., 2., 1., 2., 3., 4., 5., 6., 99.]);
    let line = read_ad_line(&mut r).unwrap();
    assert_eq!(line.nb_multi_points().unwrap(), 2);
    assert_eq!(
        line.value(1).unwrap().point(1).unwrap(),
        Point3::new(1., 2., 3.)
    );
    assert_eq!(
        line.value(2).unwrap().point(1).unwrap(),
        Point3::new(4., 5., 6.)
    );
    let copy = line.clone();
    let replacement = MultiPointConstraint::with_counts(1, 0).unwrap();
    replacement.set_point(1, Point3::new(7., 8., 9.)).unwrap();
    copy.set_value(2, &replacement).unwrap();
    assert_eq!(
        line.value(2).unwrap().point(1).unwrap(),
        Point3::new(7., 8., 9.)
    );
    assert_eq!(r.real(), 99.);
}
#[test]
fn line_tools_keep_unwritten_sentinels_and_absent_derivative_state() {
    let mut r = Reader::new(&[1., 0., 1., 0., 0., 7., 8., 9., 99.]);
    let line = read_line(&mut r);
    let mut points = array(1, &[Point3::new(-7., -7., -7.); 3]);
    line.value_3d(1, &mut points).unwrap();
    assert_eq!(points[1], Point3::new(7., 8., 9.));
    assert_eq!([points[2], points[3]], [Point3::new(-7., -7., -7.); 2]);
    let mut vectors = array(1, &[Vector3::new(-7., -7., -7.); 2]);
    assert!(!line.tangency_3d(1, &mut vectors).unwrap());
    assert_eq!([vectors[1], vectors[2]], [Vector3::new(-7., -7., -7.); 2]);
    assert_eq!(r.real(), 99.);
}
#[test]
fn shared_line_reader_keeps_flagged_tangent_curvature_order() {
    let mut r = Reader::new(&[
        1., 1., 1., 1., 1., 1., 2., 3., 4., 5., 1., 0., 0., 0., 1., 0., 2., 0., 3., 0., 99.,
    ]);
    let line = read_line(&mut r);
    let point = line.value(1).unwrap();
    assert_eq!(point.point(1).unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(point.point2d(2).unwrap(), Point2::new(4., 5.));
    assert_eq!(point.tang(1).unwrap(), Vector3::new(1., 0., 0.));
    assert_eq!(point.tang2d(2).unwrap(), Vector2::new(0., 1.));
    assert_eq!(point.curv(1).unwrap(), Vector3::new(0., 2., 0.));
    assert_eq!(point.curv2d(2).unwrap(), Vector2::new(3., 0.));
    assert_eq!(r.real(), 99.);
}
#[test]
fn shared_array_builder_preserves_bounds_and_copies_its_source() {
    let mut source = [2, 3];
    let a = array(-2, &source);
    source[0] = 99;
    assert_eq!((a.lower(), a.upper()), (-2, -1));
    assert_eq!(a.as_slice(), &[2, 3]);
    assert_eq!(source, [99, 3]);
}
