//! Independent polynomial fits and deliberate output corruption supplement exact retained requests.
use martensite_tkernel::{
    ncollection::{Array1, HArray1},
    standard::Handle,
};
use martensite_tkgeombase::{
    app_def::MultiLine,
    app_par_curves::{
        BSpFunction, BSpGradient, BSpGradientInput, Constraint, ConstraintCouple, Constraints,
        Function, Gradient, LeastSquare, MultiCurveEval, ResolConstraint,
    },
};
use martensite_tkmath::{gp::Point3, math::Vector};
fn polynomial(u: f64) -> Point3 {
    Point3::new(u, u * u, 2.0 * u + 1.0)
}
fn samples() -> (MultiLine, Vector<'static>, Constraints) {
    let points: Vec<Point3> = (0..9).map(|i| polynomial(f64::from(i) / 8.0)).collect();
    let line = MultiLine::from_points(&Array1::from_shared(&points, 1));
    let mut u = Vector::with_bounds(1, 9).unwrap();
    for i in 1..=9 {
        u[i] = f64::from(i - 1) / 8.0;
    }
    let ends = [
        ConstraintCouple::new(1, Constraint::PassPoint),
        ConstraintCouple::new(9, Constraint::PassPoint),
    ];
    (
        line,
        u,
        Handle::new(HArray1::from_array(&Array1::from_shared(&ends, 1))),
    )
}
fn assert_polynomial(curve: &impl MultiCurveEval) {
    // A quadratic lies exactly in this degree-three space. This independent bound
    // allows finite arithmetic in the normal equations, not a changed corpus tolerance.
    for i in 0..=32 {
        let u = f64::from(i) / 32.0;
        assert!(curve.value_3d(1, u).unwrap().distance(&polynomial(u)) < 1e-12);
    }
}
#[test]
fn least_squares_recovers_quadratic_and_cubic_bernstein_poles() {
    let (line, u, _) = samples();
    let mut fit = LeastSquare::new(
        &line,
        1,
        9,
        Constraint::PassPoint,
        Constraint::PassPoint,
        &u,
        4,
    )
    .unwrap();
    assert!(fit.is_done());
    let curve = fit.bezier_value().unwrap();
    assert_polynomial(&curve);
    let expected = [
        Point3::new(0., 0., 1.),
        Point3::new(1. / 3., 0., 5. / 3.),
        Point3::new(2. / 3., 1. / 3., 7. / 3.),
        Point3::new(1., 1., 3.),
    ];
    for (i, p) in expected.iter().enumerate() {
        assert!(
            curve
                .value(i as i32 + 1)
                .unwrap()
                .point(1)
                .unwrap()
                .distance(p)
                < 1e-12
        );
    }
}
#[test]
fn constraint_resolution_preserves_exact_endpoint_polynomial() {
    let (line, u, cons) = samples();
    let mut fit = LeastSquare::new(
        &line,
        1,
        9,
        Constraint::PassPoint,
        Constraint::PassPoint,
        &u,
        4,
    )
    .unwrap();
    let mut curve = fit.bezier_value().unwrap();
    let a = fit.function_matrix().unwrap().clone();
    let da = fit.derivative_function_matrix().unwrap().clone();
    let resol = ResolConstraint::new(&line, &mut curve, 1, 9, &cons, &a, &da, 1e-10).unwrap();
    assert!(resol.is_done());
    assert_eq!(resol.constraint_matrix().row_number(), 6);
    assert_polynomial(&curve);
}
#[test]
fn function_reports_zero_polynomial_residual() {
    let (line, u, cons) = samples();
    let mut fit = Function::new(&line, 1, 9, &cons, &u, 3).unwrap();
    let mut residual = -1.;
    assert!(fit.evaluate(&u, &mut residual).unwrap());
    assert!(residual.abs() < 1e-24);
    assert_polynomial(fit.curve_value().unwrap());
}
#[test]
fn spline_function_reports_zero_polynomial_residual() {
    let (line, u, cons) = samples();
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[4, 4], 1);
    let mut fit = BSpFunction::new(&line, 1, 9, &cons, &u, &knots, &mults, 4).unwrap();
    let mut residual = -1.;
    assert!(fit.evaluate(&u, &mut residual).unwrap());
    assert!(residual.abs() < 1e-24);
    assert_polynomial(&fit.curve_value().unwrap());
}
#[test]
fn gradient_preserves_exact_polynomial_parameterization() {
    let (line, mut u, cons) = samples();
    let fit = Gradient::new(&line, 1, 9, &cons, &mut u, 3, 1e-9, 1e-9, 10).unwrap();
    assert!(fit.is_done());
    assert_polynomial(&fit.value());
    assert!(fit.max_error_3d() < 1e-12);
}
#[test]
fn spline_gradient_preserves_exact_polynomial_parameterization() {
    let (line, mut u, cons) = samples();
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[4, 4], 1);
    let input = BSpGradientInput {
        ssp: &line,
        points: [1, 9],
        constraints: &cons,
        knots: &knots,
        mults: &mults,
        deg: 3,
        tolerances: [1e-9, 1e-9],
        nb_iterations: 10,
    };
    let fit = BSpGradient::new(&input, &mut u).unwrap();
    assert!(fit.is_done());
    assert_polynomial(&fit.value());
    assert!(fit.max_error_3d() < 1e-12);
}
#[test]
fn each_family_rejects_missing_and_changed_output_tokens() {
    use super::super::construction_corpus::{policy, run_values};
    for family in [
        "apc_ls",
        "apc_resol",
        "apc_function",
        "apc_bspfunction",
        "apc_gradient",
        "apc_bspgradient",
    ] {
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

fn replay_row(index: usize, row: &str) -> (usize, String) {
    use super::super::construction_corpus::run_values;
    let mut fields = row.split_whitespace();
    let family = fields.next().unwrap();
    let values: Vec<f64> = fields.map(|v| v.parse().unwrap()).collect();
    (index, run_values(family, &values).unwrap())
}
fn replay_partition(requests: &[&str], worker: usize, workers: usize) -> Vec<(usize, String)> {
    requests
        .iter()
        .enumerate()
        .skip(worker)
        .step_by(workers)
        .map(|(i, row)| replay_row(i, row))
        .collect()
}
fn ordered_replay(workers: usize) -> Vec<(usize, String)> {
    let requests: Vec<&str> = super::CHUNKS.iter().flat_map(|(r, _)| r.lines()).collect();
    let mut records = std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..workers)
            .map(|worker| {
                let requests = &requests;
                scope.spawn(move || replay_partition(requests, worker, workers))
            })
            .collect();
        jobs.into_iter()
            .flat_map(|job| job.join().unwrap())
            .collect::<Vec<_>>()
    });
    records.sort_by_key(|(i, _)| *i);
    records
}

#[test]
fn all_requests_match_byte_for_byte_with_one_and_four_workers() {
    let serial = ordered_replay(1);
    assert_eq!(serial, ordered_replay(4));
    let expected: String = super::CHUNKS.iter().map(|(_, rows)| *rows).collect();
    let actual: String = serial.into_iter().map(|(_, row)| row).collect();
    assert_eq!(actual, expected);
}
