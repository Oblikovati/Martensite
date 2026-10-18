//! Closed-form metric controls supplement the retained stream without tolerance changes.
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::{extrema_pc as pc, extrema_pc2d as pc2};
use martensite_tkmath::gp::{Direction2, Direction3, Line2, Line3, Point2, Point3};
use pc2::SearchMode;

fn line2() -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dLine::from_point(Point2::new(0., 0.), Direction2::X)).cast()
}
fn line3() -> Handle<dyn GeomCurve> {
    Handle::new(GeomLine::from_point(Point3::new(0., 0., 0.), Direction3::X)).cast()
}
#[test]
fn uniform_grid_has_exact_equal_steps_and_endpoint_order() {
    let grid = pc2::GridEvaluator::build_uniform_params(-2., 2., 5).unwrap();
    assert_eq!(
        grid.array1().iter().copied().collect::<Vec<_>>(),
        [-2., -1., 0., 1., 2.]
    );
}
#[test]
fn planar_line_stationarity_is_u_minus_three_with_unit_derivative() {
    let curve = line2();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let function = pc2::DistanceFunction::new(&adaptor, Point2::new(3., 4.));
    for u in [-2., 0., 3., 5.] {
        assert_eq!(function.value(u).unwrap(), u - 3.);
        assert_eq!(function.values(u).unwrap(), (u - 3., 1.));
        assert_eq!(function.derivative(u).unwrap(), 1.);
    }
}
#[test]
fn spatial_line_stationarity_ignores_the_two_orthogonal_coordinates() {
    let curve = line3();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let function = pc::DistanceFunction::new(&adaptor, Point3::new(3., 4., 5.));
    for u in [-2., 0., 3., 5.] {
        assert_eq!(function.value(u).unwrap(), u - 3.);
        assert_eq!(function.values(u).unwrap(), (u - 3., 1.));
    }
}
#[test]
fn analytic_planar_line_projection_preserves_exact_minimum_and_bound_filter() {
    let line = Line2::new(Point2::new(0., 0.), Direction2::X);
    let mut evaluator = pc2::Line::with_domain(line, pc2::Domain1D::new(0., 10.));
    let result = evaluator.perform(Point2::new(3., 4.), 1e-9, SearchMode::MinMax);
    assert_eq!(result.nb_ext(), 1);
    assert_eq!(result.extrema[0].parameter, 3.);
    assert_eq!(result.extrema[0].square_distance, 16.);
    assert_eq!(result.extrema[0].point, Point2::new(3., 0.));
    assert!(result.extrema[0].is_minimum);
    let outside = evaluator.perform(Point2::new(12., 4.), 1e-9, SearchMode::MinMax);
    assert_eq!(outside.status, pc2::Status::NoSolution);
    assert_eq!(outside.nb_ext(), 0);
}
#[test]
fn analytic_spatial_line_projection_has_squared_orthogonal_remainder_41() {
    let line = Line3::new(Point3::new(0., 0., 0.), Direction3::X);
    let mut evaluator = pc::Line::new(line);
    let result = evaluator
        .perform(Point3::new(3., 4., 5.), 1e-9, SearchMode::MinMax)
        .unwrap();
    assert_eq!(result.nb_ext(), 1);
    assert_eq!(result.extrema[0].parameter, 3.);
    assert_eq!(result.extrema[0].square_distance, 41.);
    assert_eq!(result.extrema[0].point, Point3::new(3., 0., 0.));
}
#[test]
fn endpoint_extrema_follow_monotone_squared_distance_on_a_segment() {
    let line = Line2::new(Point2::new(0., 0.), Direction2::X);
    let mut evaluator = pc2::Line::with_domain(line, pc2::Domain1D::new(0., 2.));
    let result = evaluator.perform_with_endpoints(Point2::new(3., 4.), 1e-9, SearchMode::MinMax);
    let mut observed: Vec<_> = result
        .extrema
        .iter()
        .map(|e| (e.parameter, e.square_distance, e.is_minimum, e.is_maximum))
        .collect();
    observed.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert_eq!(observed, [(0., 25., false, true), (2., 17., true, false)]);
}
#[test]
fn derived_line_plane_contains_both_origin_and_all_line_points() {
    let line = Line3::new(Point3::new(2., 3., 4.), Direction3::X);
    let plane = pc::line_plane(&line).unwrap();
    for point in [
        Point3::new(2., 3., 4.),
        Point3::new(5., 3., 4.),
        Point3::new(-2., 3., 4.),
    ] {
        assert_eq!(plane.square_distance(point), 0.);
    }
}
#[test]
fn straight_planar_curve_sampling_keeps_exact_endpoints_on_the_line() {
    let curve = line2();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let (parameters, points) = martensite_tkgeombase::probe_support::tangential_deflection_2d(
        &adaptor,
        Some((0., 4.)),
        [0.1, 0.01, 2., 1e-9, 1e-7],
    )
    .unwrap();
    assert_eq!(parameters, [0., 4.]);
    assert_eq!(points, [Point3::new(0., 0., 0.), Point3::new(4., 0., 0.)]);
}
#[test]
fn straight_spatial_curve_sampling_keeps_exact_endpoints_on_the_line() {
    let curve = line3();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let (parameters, points) = martensite_tkgeombase::probe_support::tangential_deflection_3d(
        &adaptor,
        Some((0., 4.)),
        [0.1, 0.01, 2., 1e-9, 1e-7],
    )
    .unwrap();
    assert_eq!(parameters, [0., 4.]);
    assert_eq!(points, [Point3::new(0., 0., 0.), Point3::new(4., 0., 0.)]);
}

#[test]
fn zero_metric_tolerance_is_invalid_for_both_line_evaluators() {
    let mut planar = pc2::Line::new(Line2::new(Point2::new(0., 0.), Direction2::X));
    let result = planar.perform(Point2::new(3., 4.), 0., SearchMode::MinMax);
    assert_eq!(result.status, pc2::Status::InvalidInput);
    assert_eq!(result.nb_ext(), 0);
    let mut spatial = pc::Line::new(Line3::new(Point3::new(0., 0., 0.), Direction3::X));
    let result = spatial
        .perform(Point3::new(3., 4., 5.), 0., SearchMode::MinMax)
        .unwrap();
    assert_eq!(result.status, pc::Status::InvalidInput);
    assert_eq!(result.nb_ext(), 0);
}

#[test]
fn numerical_planar_grid_recovers_the_exact_line_stationary_point() {
    let curve = line2();
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    let parameters = pc2::GridEvaluator::build_uniform_params(0., 10., 3).unwrap();
    let mut grid = pc2::GridEvaluator::default();
    grid.set_params(&parameters);
    let point = Point2::new(3., 4.);
    let domain = pc2::Domain1D::new(0., 10.);
    let result = grid.perform(&adaptor, &point, &domain, 1e-9, SearchMode::MinMax);
    assert_eq!(result.nb_ext(), 1);
    assert_eq!(result.extrema[0].parameter, 3.);
    assert_eq!(result.extrema[0].square_distance, 16.);
}
#[test]
fn numerical_spatial_grid_recovers_the_exact_line_stationary_point() {
    let curve = line3();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let parameters = pc::GridEvaluator::build_uniform_params(0., 10., 3).unwrap();
    let mut grid = pc::GridEvaluator::default();
    grid.set_params(&parameters);
    let point = Point3::new(3., 4., 5.);
    let domain = pc::Domain1D::new(0., 10.);
    let result = grid
        .perform(&adaptor, &point, &domain, 1e-9, SearchMode::MinMax)
        .unwrap();
    assert_eq!(result.nb_ext(), 1);
    assert_eq!(result.extrema[0].parameter, 3.);
    assert_eq!(result.extrema[0].square_distance, 41.);
}
