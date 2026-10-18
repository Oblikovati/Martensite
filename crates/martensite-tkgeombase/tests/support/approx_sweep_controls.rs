//! Independently differentiated polynomial values and callback boundaries supplement retained requests.
use super::*;
fn function(rational: bool) -> AxSweep {
    AxSweep {
        parent: StandardTransient::default(),
        settings: Settings {
            nbpol: 3,
            nb2d: 2,
            rational,
            d1mode: 1,
            d2mode: 1,
            resolution: true,
            nbc2: 2,
            nbc3: 4,
            pfirst: -0.5,
            plast: 1.5,
        },
        record: Mutex::new(Record::default()),
    }
}
struct Derivatives {
    poles: Array1<'static, Point3>,
    first: Array1<'static, Vector3>,
    second: Array1<'static, Vector3>,
    points2: Array1<'static, Point2>,
    first2: Array1<'static, Vector2>,
    second2: Array1<'static, Vector2>,
    weights: Array1<'static, f64>,
    weights1: Array1<'static, f64>,
    weights2: Array1<'static, f64>,
}
impl Derivatives {
    fn new() -> Self {
        Self {
            poles: Array1::with_bounds(1, 3).unwrap(),
            first: Array1::with_bounds(1, 3).unwrap(),
            second: Array1::with_bounds(1, 3).unwrap(),
            points2: Array1::with_bounds(1, 2).unwrap(),
            first2: Array1::with_bounds(1, 2).unwrap(),
            second2: Array1::with_bounds(1, 2).unwrap(),
            weights: Array1::with_bounds(1, 3).unwrap(),
            weights1: Array1::with_bounds(1, 3).unwrap(),
            weights2: Array1::with_bounds(1, 3).unwrap(),
        }
    }
    fn first(&mut self) -> SweepD1<'_, 'static> {
        SweepD1 {
            poles: &mut self.poles,
            d_poles: &mut self.first,
            poles2d: &mut self.points2,
            d_poles2d: &mut self.first2,
            weights: &mut self.weights,
            d_weights: &mut self.weights1,
        }
    }
    fn second(&mut self) -> SweepD2<'_, 'static> {
        SweepD2 {
            d1: SweepD1 {
                poles: &mut self.poles,
                d_poles: &mut self.first,
                poles2d: &mut self.points2,
                d_poles2d: &mut self.first2,
                weights: &mut self.weights,
                d_weights: &mut self.weights1,
            },
            d2_poles: &mut self.second,
            d2_poles2d: &mut self.second2,
            d2_weights: &mut self.weights2,
        }
    }
}
#[test]
fn polynomial_values_and_two_derivatives_match_closed_forms() {
    let f = function(true);
    let mut d = Derivatives::new();
    assert!(f.d2(0.5, 0., 1., d.second()).unwrap());
    assert!(d.poles[2].distance(&Point3::new(1.1, 0.225, 0.3875)) < 1e-15);
    assert!((d.first[2] - Vector3::new(1., 0.7, -0.075)).magnitude() < 1e-15);
    assert!((d.second[2] - Vector3::new(0., 1., -0.3)).magnitude() < 1e-15);
    assert_eq!(d.points2[2], Point2::new(0.5, 1.25));
    assert_eq!(d.first2[2], Vector2::new(1., 1.));
    assert_eq!(d.second2[2], Vector2::new(0., 2.));
    assert_eq!(d.weights[2], 1.1);
    assert_eq!(d.weights1[2], 0.2);
    assert_eq!(d.weights2[2], 0.);
}
#[test]
fn missing_first_derivative_errors_and_false_keeps_evaluated_values() {
    let mut f = function(true);
    let mut d = Derivatives::new();
    f.settings.d1mode = 0;
    assert_eq!(
        f.d1(0.5, 0., 1., d.first()).unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
    f.settings.d1mode = 2;
    assert!(!f.d1(0.5, 0., 1., d.first()).unwrap());
    assert_eq!(d.weights[2], 1.1);
}
#[test]
fn missing_second_derivative_errors_and_false_keeps_evaluated_values() {
    let mut f = function(true);
    let mut d = Derivatives::new();
    f.settings.d2mode = 0;
    assert_eq!(
        f.d2(0.5, 0., 1., d.second()).unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
    f.settings.d2mode = 2;
    assert!(!f.d2(0.5, 0., 1., d.second()).unwrap());
    assert_eq!(d.weights2[2], 0.);
}
#[test]
fn interval_partitions_and_recorded_tolerances_remain_distinct() {
    let f = function(false);
    let mut c2 = Array1::with_bounds(1, 3).unwrap();
    let mut c3 = Array1::with_bounds(1, 5).unwrap();
    f.intervals(&mut c2, Shape::C2).unwrap();
    f.intervals(&mut c3, Shape::C3).unwrap();
    assert_eq!(c2.as_slice(), &[-0.5, 0.5, 1.5]);
    assert_eq!(c3.as_slice(), &[-0.5, 0., 0.5, 1., 1.5]);
}
#[test]
fn callback_records_and_resolution_keep_separate_tolerances() {
    let f = function(false);
    f.set_interval(0., 0.5).unwrap();
    f.set_interval(0.5, 1.).unwrap();
    f.set_tolerance(1e-4, 1e-6).unwrap();
    let r = *f.record.lock().unwrap();
    assert_eq!((r.first, r.last, r.nbinterval), (0.5, 1., 2));
    assert_eq!((r.tol3d, r.tol2d), (1e-4, 1e-6));
    assert_eq!(f.resolution(2, 0.125).unwrap(), (0.1875, 0.125));
}
#[test]
fn rational_weight_lower_bounds_and_unavailable_resolution_are_explicit() {
    let f = function(true);
    let mut weights = Array1::with_bounds(1, 3).unwrap();
    f.get_minimal_weight(&mut weights).unwrap();
    for (a, b) in weights.iter().zip([0.95, 0.9, 0.85]) {
        assert!((a - b).abs() < 1e-15);
    }
}
#[test]
fn unavailable_resolution_and_nonrational_extent_report_typed_errors() {
    let mut f = function(true);
    f.settings.resolution = false;
    assert_eq!(
        f.resolution(1, 0.1).unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
    f.settings.rational = false;
    assert_eq!(
        f.maximal_section().unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
}
fn approximation() -> (Handle<AxSweep>, SweepApproximation) {
    let mut f = function(false);
    f.settings.nb2d = 0;
    let owner = Handle::new(f);
    let mut a = SweepApproximation::new(&owner.clone().cast());
    a.perform(0., 1., SweepOptions::new(1e-4, 1e-3, 1e-5, 1e-2))
        .unwrap();
    assert!(a.is_done());
    (owner, a)
}
fn interval_count(owner: &Handle<AxSweep>) -> i32 {
    owner.get().unwrap().record.lock().unwrap().nbinterval
}
#[test]
fn approximation_eval_reuses_cached_interval_and_second_derivative() {
    let (owner, mut a) = approximation();
    let mut values = [-7.; 9];
    assert_eq!(a.eval(0.5, 2, 0., 1., &mut values).unwrap(), 0);
    assert_eq!(values, [0., 1., -0.3, 0., 1., -0.3, 0., 1., -0.3]);
    let before = interval_count(&owner);
    assert_eq!(a.eval(0.5, 2, 0., 1., &mut values).unwrap(), 0);
    assert_eq!(interval_count(&owner), before);
    assert_eq!(a.eval(0.5, 2, 0., 0.75, &mut values).unwrap(), 0);
    assert_eq!(interval_count(&owner), before + 1);
}
#[test]
fn unsupported_order_leaves_output_untouched() {
    let (_, mut a) = approximation();
    let mut values = [-7.; 9];
    assert_eq!(a.eval(0.5, 3, 0., 1., &mut values).unwrap(), 2);
    assert_eq!(values, [-7.; 9]);
}
#[test]
fn effective_callback_tolerances_follow_section_minimum_and_fixed_scale() {
    let (owner, _) = approximation();
    let r = *owner.get().unwrap().record.lock().unwrap();
    assert_eq!(r.tol3d, (1e-4 / 1.5) / 20.);
    assert_eq!(r.tol2d, 1e-5 / 20.);
}
#[test]
fn output_rejects_missing_and_changed_tokens() {
    let request = include_str!("../fixtures/approx_sweep_corpus/inputs-00.txt")
        .lines()
        .next()
        .unwrap();
    let expected = include_str!("../fixtures/approx_sweep_corpus/expected-00.txt")
        .split_inclusive('\n')
        .next()
        .unwrap();
    let values: Vec<f64> = request
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let output = Output::default();
    sweep_case(&output, &values);
    let actual = output.finish();
    assert!(super::super::policy::equivalent(expected, &actual));
    assert!(!super::super::policy::equivalent(
        &format!("{expected}0 "),
        &actual
    ));
    assert!(!super::super::policy::equivalent(
        &expected.replacen(' ', "9 ", 1),
        &actual
    ));
}
