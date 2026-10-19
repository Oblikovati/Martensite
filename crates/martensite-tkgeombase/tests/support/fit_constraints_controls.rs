//! Independent affine/zero-curve expectations and explicit failure-state controls.
use martensite_tkernel::{
    ncollection::{Array1, Array2, HArray1},
    standard::Handle,
};
use martensite_tkgeombase::{
    app_def::{
        Compute, LinearCriteria, MultiLine, SmoothCriterion, Variational, VariationalOptions,
    },
    app_par_curves::{Constraint, Constraints, MultiCurve, MultiCurveEval},
    approx::{ComputeLineOptions, ParametrizationType},
    fem_tool::Curve,
};
use martensite_tkmath::{geom_abs::Shape, gp::Point3, plib::HermitJacobi};

fn affine(t: f64) -> Point3 {
    Point3::new(-2.0 + 3.0 * t, 7.0 - 2.0 * t, 1.0 + t)
}

fn affine_line() -> MultiLine {
    let points: Vec<Point3> = (0..9).map(|i| affine(f64::from(i) / 8.0)).collect();
    MultiLine::from_points(&Array1::from_shared(&points, 1))
}

fn line_options() -> ComputeLineOptions {
    ComputeLineOptions {
        degree_min: 2,
        degree_max: 2,
        tolerance_3d: 1e-9,
        tolerance_2d: 1e-9,
        nb_iterations: 2,
        cutting: false,
        parametrization: ParametrizationType::IsoParametric,
        squares: true,
    }
}

fn samples(line: &MultiLine) -> Vec<Point3> {
    (1..=line.nb_multi_points().unwrap())
        .map(|i| line.value(i).unwrap().point(1).unwrap())
        .collect()
}

#[test]
fn affine_compute_has_exact_endpoint_poles_and_zero_residual() {
    let line = affine_line();
    let before = samples(&line);
    let mut curve = Compute::new(line_options());
    curve.set_constraints(Constraint::PassPoint, Constraint::PassPoint);
    curve.perform(&line).unwrap();
    assert!(curve.is_all_approximated() && curve.is_tolerance_reached());
    assert_eq!(curve.nb_multi_curves(), 1);
    let piece = curve.value(1).unwrap();
    assert_affine_poles(piece);
    assert!(curve.error(1).unwrap().0 < 1e-12);
    assert_eq!(samples(&line), before);
}

fn assert_affine_poles(piece: &MultiCurve) {
    assert_eq!(piece.nb_poles(), 3);
    for index in 1..=3 {
        let expected = affine(f64::from(index - 1) / 2.0);
        let actual = piece.value(index).unwrap().point(1).unwrap();
        assert!(actual.distance(&expected) < 1e-12);
    }
}

fn zero_curve() -> Handle<Curve> {
    let curve = Curve::new(3, 1, &HermitJacobi::new(3, Shape::C0).unwrap(), 1e-7).unwrap();
    curve.knots()[1] = 0.0;
    curve.knots()[2] = 1.0;
    let mut coefficients = Array2::with_bounds(0, 3, 1, 3).unwrap();
    coefficients.init(0.0);
    curve.set_element(1, &coefficients).unwrap();
    Handle::new(curve)
}

fn fixed_distance_criterion() -> LinearCriteria {
    let points = [Point3::new(3.0, 4.0, 0.0); 5];
    let line = MultiLine::from_points(&Array1::from_shared(&points, 1));
    let criterion = LinearCriteria::new(&line, 1, 5).unwrap();
    let parameters = Array1::from_shared(&[0.0, 0.25, 0.5, 0.75, 1.0], 1);
    criterion.set_parameters(&Handle::new(HArray1::from_array(&parameters)));
    criterion.set_curve(&zero_curve()).unwrap();
    criterion
}

#[test]
fn zero_curve_distances_match_five_identical_three_four_five_triangles() {
    let criterion = fixed_distance_criterion();
    let errors = criterion.error_values().unwrap();
    assert_eq!(errors.max_error, 5.0);
    assert_eq!(errors.quadratic_error, 125.0);
    assert_eq!(errors.average_error, 25.0);
}

#[test]
fn zero_curve_has_zero_tension_flexion_and_jerk_energy() {
    let criterion = fixed_distance_criterion();
    let quality = criterion.quality_values(1.0, 1.0, 1.0).unwrap();
    assert_eq!(quality.criteria, [0.0; 3]);
}

fn smoothing() -> Variational {
    let constraints: Constraints = Handle::new(HArray1::from_array(&Array1::from_shared(&[], 1)));
    let options = VariationalOptions {
        max_degree: 5,
        max_segment: 1,
        continuity: Shape::C1,
        with_min_max: false,
        with_cutting: false,
        tolerance: 1e-6,
        nb_iterations: 1,
    };
    Variational::new(&affine_line(), 1, 9, &constraints, options).unwrap()
}

#[test]
fn variational_weights_are_normalized_by_their_independent_sum() {
    let mut operation = smoothing();
    operation.set_criterium_weight(1.0, 1.0, 2.0).unwrap();
    assert_eq!(operation.criterium_weight(), [0.25, 0.25, 0.5]);
}

#[test]
fn rejected_variational_weight_preserves_existing_weights_and_completion_state() {
    let mut operation = smoothing();
    let before = operation.criterium_weight();
    let error = operation.set_criterium_weight(1.0, -1.0, 2.0).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert_eq!(operation.criterium_weight(), before);
    assert!(operation.is_created());
    assert!(!operation.is_done());
}

#[test]
fn variational_result_is_unavailable_before_approximation() {
    let operation = smoothing();
    assert_eq!(
        operation
            .value()
            .err()
            .expect("unfinished result must fail")
            .exception_type(),
        "StdFail_NotDone"
    );
    assert!(!operation.is_done());
}
