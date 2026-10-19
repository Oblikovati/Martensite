//! Complete foundational basis/container families and independently expected elementary contracts.
const CHUNKS: &[(&str, &str)] = &[(
    include_str!("fixtures/app_par_container_corpus/inputs.txt"),
    include_str!("fixtures/app_par_container_corpus/native.txt"),
)];
#[test]
fn apc_bernstein_complete_family() {
    super::replay_family(CHUNKS, "apc_bernstein", 63);
}
#[test]
fn apc_spline_complete_family() {
    super::replay_family(CHUNKS, "apc_spline", 28);
}
#[test]
fn apc_multipoint_complete_family() {
    super::replay_family(CHUNKS, "apc_multipoint", 14);
}
#[test]
fn apc_multicurve_complete_family() {
    super::replay_family(CHUNKS, "apc_multicurve", 58);
}
#[test]
fn basis_container_requests_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xe82e57d2184ea8
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xd9e7f739f3154383
    );
}
use super::construction_corpus::{
    app_par_containers::{read_point, read_points},
    common::Reader,
    linear_tokens,
};
use martensite_tkgeombase::app_par_curves::{
    self as apc, MultiBSpCurve, MultiCurve, MultiCurveEval,
};
use martensite_tkmath::{
    gp::{Point3, Vector3},
    math::{IntegerVector, Matrix, Vector},
};
#[test]
fn quadratic_bernstein_values_and_derivatives_match_expanded_polynomials() {
    let u = linear_tokens::read_vector(&mut Reader::new(&[-2., 3., 0., 0.5, 1.]));
    let mut a = Matrix::filled(-2, 0, 1, 3, -7.).unwrap();
    let mut da = a.clone();
    apc::bernstein(3, &u, &mut a, &mut da).unwrap();
    for i in -2..=0 {
        let t = u[i];
        let expected = [(1. - t) * (1. - t), 2. * t * (1. - t), t * t];
        let derivative = [-2. * (1. - t), 2. - 4. * t, 2. * t];
        for j in 1..=3 {
            assert_eq!(a[(i, j)], expected[j as usize - 1]);
            assert_eq!(da[(i, j)], derivative[j as usize - 1]);
        }
        let mut dda = Vector::filled(1, 3, -7.).unwrap();
        apc::second_derivative_bernstein(t, &mut dda).unwrap();
        assert_eq!([dda[1], dda[2], dda[3]], [2., -4., 2.]);
    }
}
#[test]
fn clamped_linear_spline_basis_has_constant_derivatives_and_one_span() {
    let u = linear_tokens::read_vector(&mut Reader::new(&[3., 4., 0., 0.25, 0.5, 1.]));
    let knots = linear_tokens::read_vector(&mut Reader::new(&[1., 4., 0., 0., 1., 1.]));
    let mut a = Matrix::filled(3, 6, 1, 2, -7.).unwrap();
    let mut da = a.clone();
    let mut index = IntegerVector::filled(3, 6, -7).unwrap();
    apc::spline_function(2, 1, &u, &knots, &mut a, &mut da, &mut index).unwrap();
    for i in 3..=6 {
        assert_eq!([a[(i, 1)], a[(i, 2)]], [1. - u[i], u[i]]);
        assert_eq!([da[(i, 1)], da[(i, 2)]], [-1., 1.]);
        assert_eq!(index[i], 0);
    }
}
#[test]
fn point_reader_keeps_mixed_order_shared_copy_and_cursor() {
    use martensite_tkmath::gp::Point2;
    let mut r = Reader::new(&[0., 1., 1., 1., 2., 3., 4., 5., 99.]);
    let point = read_point(&mut r).unwrap();
    assert_eq!(point.point(1).unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(point.point2d(2).unwrap(), Point2::new(4., 5.));
    let copy = point.clone();
    copy.set_point(1, Point3::new(7., 8., 9.)).unwrap();
    assert_eq!(point.point(1).unwrap(), Point3::new(7., 8., 9.));
    assert_eq!(r.real(), 99.);
}
#[test]
fn curve_readers_preserve_linear_bezier_and_nonunit_spline_parameterization() {
    use martensite_tkernel::ncollection::Array1;
    let mut r = Reader::new(&[2., 0., 1., 0., 0., 0., 0., 0., 1., 0., 2., 4., 6., 99.]);
    let poles = read_points(&mut r).unwrap();
    let curve = MultiCurve::from_multi_points(&poles);
    assert_eq!(curve.value_3d(1, 0.25).unwrap(), Point3::new(0.5, 1., 1.5));
    let (_, d1, d2) = curve.d2_3d(1, 0.25).unwrap();
    assert_eq!(d1, Vector3::new(2., 4., 6.));
    assert_eq!(d2, Vector3::new(0., 0., 0.));
    let knots = Array1::from_shared(&[2., 5.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    let spline = MultiBSpCurve::from_multi_points(&poles, &knots, &mults);
    let (_, d1, d2) = spline.d2_3d(1, 3.5).unwrap();
    assert_eq!(spline.value_3d(1, 3.5).unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(d1, Vector3::new(2. / 3., 4. / 3., 2.));
    assert_eq!(d2, Vector3::new(0., 0., 0.));
    assert_eq!(r.real(), 99.);
}
#[test]
fn vector_and_dump_tokens_keep_bounds_bits_integers_and_newlines() {
    use super::construction_corpus::print::Output;
    let mut r = Reader::new(&[-2., 2., -0., f64::INFINITY, 99.]);
    let v = linear_tokens::read_vector(&mut r);
    assert_eq!(r.real(), 99.);
    let output = Output::default();
    linear_tokens::vector(&output, &v);
    let i = IntegerVector::filled(4, 5, -7).unwrap();
    linear_tokens::ivector(&output, &i);
    linear_tokens::dump(&output, "a\nb");
    assert_eq!(
        output.finish(),
        "-2 -1 9223372036854775808 9218868437227405312 4 5 -7 -7 a|b "
    );
}
