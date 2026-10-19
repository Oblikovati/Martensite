//! Complete analytic-fit family plus independent function and formatter contracts.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-00.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-01.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-02.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-03.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-04.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-05.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-06.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-07.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-08.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-08.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-09.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-09.txt"),
    ),
    (
        include_str!("fixtures/continuity_least_squares_corpus/inputs-10.txt"),
        include_str!("fixtures/continuity_least_squares_corpus/expected-10.txt"),
    ),
];
#[test]
fn continuous_least_squares_complete_family() {
    super::replay_family(CHUNKS, "acc_ls", 2598);
}
#[test]
fn least_squares_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x7652255ae97b1d7a
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xf97158cb1e4f4fae
    );
}
use super::construction_corpus::{common::Reader, continuity_function::AccFunction};
use martensite_tkgeombase::app_cont::Function;
fn linear_function() -> AccFunction {
    AccFunction::read(&mut Reader::new(&[
        0., 1., 0., 1., 1., 0., 0., 0., 2., 3., 0., 0., 0., -1., 2., 0., 0., 0.,
    ]))
}
#[test]
fn fitted_linear_curve_matches_its_independent_closed_form() {
    use martensite_tkgeombase::{
        app_cont::LeastSquare,
        app_par_curves::{Constraint, MultiCurveEval},
    };
    let f = linear_function();
    let mut fit = LeastSquare::new(
        &f,
        0.,
        1.,
        Constraint::PassPoint,
        Constraint::PassPoint,
        2,
        10,
    )
    .unwrap();
    assert!(fit.is_done());
    let curve = fit.value().unwrap();
    for u in [0., 0.125, 0.5, 0.875, 1.] {
        let p = curve.value_2d(1, u).unwrap();
        assert!((p.x() - (2. + 3. * u)).abs() < 1e-12);
        assert!((p.y() - (-1. + 2. * u)).abs() < 1e-12);
    }
}
#[test]
fn analytic_function_keeps_jump_period_and_derivative_contracts() {
    use martensite_tkernel::ncollection::Array1;
    use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};
    let f = AccFunction::read(&mut Reader::new(&[
        0., 1., 0., 1., 0., 4., -2., 1., 1., 5., 2., 3., 0., 0., 0., -1., 2., 0., 0., 0.,
    ]));
    assert_eq!(f.get_number_of_points(), (0, 1));
    assert_eq!(f.period_information(1), (true, 5.));
    assert_eq!(f.period_information(2), (false, 0.));
    assert_eq!(
        (f.coord(1, 0.), f.coord(1, 0.5), f.coord(1, 1.)),
        (1., 3.5, 3.)
    );
    assert_eq!((f.deriv(1, 0.), f.deriv(2, 1.)), (3., 2.));
    let mut p2 = Array1::<Point2>::with_bounds(1, 1).unwrap();
    let mut p3 = Array1::<Point3>::with_bounds(1, 0).unwrap();
    assert!(f.value(0.5, &mut p2, &mut p3));
    assert_eq!(p2[1], Point2::new(3.5, 0.));
    let mut v2 = Array1::<Vector2>::with_bounds(1, 1).unwrap();
    let mut v3 = Array1::<Vector3>::with_bounds(1, 0).unwrap();
    assert!(!f.d1(0.5, &mut v2, &mut v3));
    assert_eq!(v2[1], Vector2::new(3., 2.));
}
#[test]
fn empty_curve_serialization_is_a_completed_empty_value() {
    use super::construction_corpus::{multi_curve_tokens::curve, print::Output};
    use martensite_tkgeombase::app_par_curves::MultiCurve;
    let output = Output::default();
    curve(&output, &MultiCurve::new()).unwrap();
    assert_eq!(output.finish(), "0 0 ");
}
#[test]
fn allocated_empty_curve_failure_keeps_the_written_pole_count() {
    use super::construction_corpus::{multi_curve_tokens::curve, print::Output};
    use martensite_tkgeombase::app_par_curves::MultiCurve;
    let output = Output::default();
    let curve_value = MultiCurve::with_nb_poles(0).unwrap();
    let error = curve(&output, &curve_value).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
    assert_eq!(output.finish(), "0 ");
}
#[test]
fn mixed_point_serializer_keeps_three_then_two_dimensional_coordinates() {
    use super::construction_corpus::{multi_curve_tokens::point, print::Output};
    use martensite_tkgeombase::app_par_curves::MultiPoint;
    use martensite_tkmath::gp::{Point2, Point3};
    let value = MultiPoint::with_counts(1, 1).unwrap();
    value.set_point(1, Point3::new(1., 2., 3.)).unwrap();
    value.set_point2d(2, Point2::new(-0., 4.)).unwrap();
    let output = Output::default();
    point(&output, &value).unwrap();
    let expected: String = [1.0f64, 2., 3., -0., 4.]
        .iter()
        .map(|v| format!("{} ", v.to_bits()))
        .collect();
    assert_eq!(output.finish(), format!("1 1 {expected}"));
}
#[test]
fn fit_with_no_free_poles_preserves_its_existing_not_done_status() {
    use martensite_tkgeombase::{
        app_cont::LeastSquare,
        app_par_curves::{Constraint, MultiCurveEval},
    };
    let f = linear_function();
    let mut fit = LeastSquare::new(
        &f,
        0.,
        1.,
        Constraint::PassPoint,
        Constraint::PassPoint,
        1,
        10,
    )
    .unwrap();
    assert!(!fit.is_done());
    let curve = fit.value().unwrap();
    assert_eq!(
        curve.value_2d(1, 0.).unwrap(),
        martensite_tkmath::gp::Point2::new(2., -1.)
    );
    assert_eq!(
        curve.value_2d(1, 1.).unwrap(),
        martensite_tkmath::gp::Point2::new(5., 1.)
    );
}
