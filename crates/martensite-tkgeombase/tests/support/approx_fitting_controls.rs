//! Closed-form line reconstruction and invalid-state controls supplement the retained corpus.
use martensite_tkernel::ncollection::Array1;
use martensite_tkgeombase::{
    app_cont::{Function, FunctionBase},
    app_def::MultiLine,
    app_par_curves::{MultiCurve, MultiCurveEval, MultiPoint},
    approx::{
        BSplComputeLine, ComputeCLine, ComputeCLineOptions, ComputeLine, ComputeLineOptions,
        MCurvesToBSpCurve,
    },
};
use martensite_tkmath::{
    gp::{Point2, Point3, Vector2, Vector3},
    math::Vector,
};
fn straight_point(t: f64) -> Point3 {
    Point3::new(t, 2.0 * t, 1.0)
}
fn samples() -> (MultiLine, Vector<'static>) {
    let points: Vec<Point3> = (0..9).map(|i| straight_point(f64::from(i) / 8.0)).collect();
    let mut parameters = Vector::with_bounds(1, 9).unwrap();
    for i in 1..=9 {
        parameters[i] = f64::from(i - 1) / 8.0;
    }
    (
        MultiLine::from_points(&Array1::from_shared(&points, 1)),
        parameters,
    )
}
fn options() -> ComputeLineOptions {
    ComputeLineOptions {
        degree_min: 3,
        degree_max: 3,
        tolerance_3d: 1e-10,
        tolerance_2d: 1e-10,
        squares: true,
        ..ComputeLineOptions::default()
    }
}
fn assert_line(curve: &impl MultiCurveEval) {
    // The line lies in every positive-degree fitting space; this supplemental
    // finite-arithmetic bound does not modify any original corpus comparison.
    for i in 0..=32 {
        let t = f64::from(i) / 32.0;
        assert!(curve.value_3d(1, t).unwrap().distance(&straight_point(t)) < 1e-12);
    }
}
#[test]
fn multiline_fit_reconstructs_an_affine_line() {
    let (line, parameters) = samples();
    let fit = ComputeLine::with_line_and_parameters(&line, &parameters, options()).unwrap();
    assert!(fit.is_all_approximated());
    assert!(fit.is_tolerance_reached());
    assert_eq!(fit.nb_multi_curves(), 1);
    assert_line(fit.value(1).unwrap());
}
#[test]
fn spline_multiline_fit_reconstructs_an_affine_line() {
    let (line, parameters) = samples();
    let fit = BSplComputeLine::with_line_and_parameters(&line, &parameters, options()).unwrap();
    assert!(fit.is_all_approximated());
    assert!(fit.is_tolerance_reached());
    assert_line(fit.value());
}
struct ExactLineFunction {
    base: FunctionBase,
}
impl ExactLineFunction {
    fn new() -> Self {
        let mut base = FunctionBase::new();
        base.set_number_of_points(1, 0);
        Self { base }
    }
}
impl Function for ExactLineFunction {
    fn base(&self) -> &FunctionBase {
        &self.base
    }
    fn first_parameter(&self) -> f64 {
        0.0
    }
    fn last_parameter(&self) -> f64 {
        1.0
    }
    fn value(&self, u: f64, _p2: &mut Array1<'_, Point2>, p3: &mut Array1<'_, Point3>) -> bool {
        p3[1] = straight_point(u);
        true
    }
    fn d1(&self, _u: f64, _v2: &mut Array1<'_, Vector2>, v3: &mut Array1<'_, Vector3>) -> bool {
        v3[1] = Vector3::new(1., 2., 0.);
        true
    }
}
#[test]
// the implementation,501 sets done only after solving free poles.
fn cubic_endpoint_tangencies_leave_no_free_poles_and_report_incomplete() {
    let fit = ComputeCLine::with_line(
        &ExactLineFunction::new(),
        ComputeCLineOptions {
            degree_min: 3,
            degree_max: 3,
            ..ComputeCLineOptions::default()
        },
    )
    .unwrap();
    assert!(!fit.is_all_approximated());
    assert!(!fit.is_tolerance_reached());
    assert_eq!(fit.nb_multi_curves(), 1);
    assert_eq!(fit.parameters(1).unwrap(), (0., 1.));
    assert_line(&fit.value(1).unwrap());
}
#[test]
fn quartic_fitting_with_one_free_pole_recovers_line_endpoints_and_tangents() {
    let fit = ComputeCLine::with_line(
        &ExactLineFunction::new(),
        ComputeCLineOptions {
            degree_min: 4,
            degree_max: 4,
            ..ComputeCLineOptions::default()
        },
    )
    .unwrap();
    assert!(fit.is_all_approximated());
    assert!(fit.is_tolerance_reached());
    assert_eq!(fit.nb_multi_curves(), 1);
    assert_eq!(fit.parameters(1).unwrap(), (0., 1.));
    assert_line(&fit.value(1).unwrap());
}
fn line_multicurve() -> MultiCurve {
    let points = [straight_point(0.), straight_point(1.)];
    let poles: Vec<MultiPoint> = points
        .iter()
        .map(|p| MultiPoint::from_points(&Array1::from_shared(&[*p], 1)))
        .collect();
    MultiCurve::from_multi_points(&Array1::from_shared(&poles, 1))
}
#[test]
fn single_bezier_join_has_exact_degree_knots_and_shape() {
    let mut join = MCurvesToBSpCurve::new();
    join.append(&line_multicurve());
    join.perform().unwrap();
    let spline = join.value();
    assert_eq!(MultiCurveEval::degree(spline).unwrap(), 1);
    assert_eq!(spline.knots().unwrap().as_slice(), &[0., 1.]);
    assert_eq!(spline.multiplicities().unwrap().as_slice(), &[2, 2]);
    assert_line(spline);
}
#[test]
fn empty_join_and_out_of_range_fit_are_typed_failures() {
    let mut join = MCurvesToBSpCurve::new();
    assert_eq!(
        join.perform().unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    let fit = ComputeLine::new(options());
    assert_eq!(
        fit.value(1).err().unwrap().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn each_family_rejects_changed_and_missing_output_tokens() {
    use super::super::construction_corpus::{policy, run_values};
    for family in ["ax_compute", "ax_bspl", "ax_fit", "ax_mcurves"] {
        let (request, expected) = super::CHUNKS
            .iter()
            .flat_map(|(r, a)| r.lines().zip(a.split_inclusive('\n')))
            .find(|(r, _)| r.starts_with(&format!("{family} ")))
            .unwrap();
        let values: Vec<f64> = request
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();
        let actual = run_values(family, &values).unwrap();
        assert!(policy::equivalent(expected, &actual));
        assert!(!policy::equivalent(&format!("{expected}0 "), &actual));
        assert!(!policy::equivalent(
            &expected.replacen(' ', "9 ", 1),
            &actual
        ));
    }
}
