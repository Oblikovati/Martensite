//! Analytic projections, constant interpolation and control-polygon repairs are independent controls.
use super::super::construction_corpus::{geometry_record_fixtures as build, print::Reader};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::{geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    adv_approx::{AdvApproxApproxAFunction, AdvApproxEvaluatorFunction},
    geom::{GeomBSplineCurve, GeomBSplineSurface, GeomCurve, GeomSurface},
    geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkgeombase::geom_lib::{
    CheckBSplineCurve, CheckCurveOnSurface, DenominatorMultiplier, Interpolate,
    InterpolationErrors, IsPlanarSurface, MakeCurvefromApprox, Tool, compute_deviation,
    compute_deviation_pso,
};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Point2, Point3},
};

fn curve3d() -> Handle<dyn GeomCurve> {
    build::curve(&mut Reader::new(&[1., 3., 4., 5., 1., 0., 0.])).unwrap()
}
fn curve2d() -> Handle<dyn Geom2dCurve> {
    build::curve2d(&mut Reader::new(&[1., 3., 4., 1., 0.])).unwrap()
}
fn surface(kind: f64, extra: &[f64]) -> Handle<dyn GeomSurface> {
    let mut fields = vec![kind, 0., 0., 5., 0., 0., 1., 1., 0., 0., 0.];
    fields.extend_from_slice(extra);
    build::surface(&mut Reader::new(&fields)).unwrap()
}
fn point_arrays(lower: i32, points: &[Point3]) -> (Array1<'static, Point3>, Array1<'static, f64>) {
    let mut poles = Array1::with_bounds(lower, lower + points.len() as i32 - 1).unwrap();
    let mut parameters = Array1::with_bounds(lower, lower + points.len() as i32 - 1).unwrap();
    for (i, p) in points.iter().enumerate() {
        poles[lower + i as i32] = *p;
        parameters[lower + i as i32] = i as f64;
    }
    (poles, parameters)
}
#[test]
fn line_projection_has_exact_parameter_and_inclusive_distance_limit() {
    let line = curve3d();
    assert_eq!(
        Tool::parameter(&line, Point3::new(5., 4., 5.25), 0.25).unwrap(),
        Some(2.)
    );
    assert_eq!(
        Tool::parameter(&line, Point3::new(5., 4., 5.25), 0.125).unwrap(),
        None
    );
    assert_eq!(
        Tool::parameter(&Handle::default(), Point3::default(), 1.).unwrap(),
        None
    );
}
#[test]
fn planar_projection_recovers_coordinates_and_rejects_excess_normal_distance() {
    let plane = surface(1., &[]);
    assert_eq!(
        Tool::parameters(&plane, Point3::new(2., 3., 5.25), 0.25).unwrap(),
        Some((2., 3.))
    );
    assert_eq!(
        Tool::parameters(&plane, Point3::new(2., 3., 5.25), 0.125).unwrap(),
        None
    );
}
#[test]
fn planar_line_projection_matches_the_scalar_orthogonal_projection() {
    assert_eq!(
        Tool::parameter_2d(&curve2d(), Point2::new(5., 4.25), 0.25).unwrap(),
        Some(2.)
    );
    assert_eq!(
        Tool::parameter_2d(&curve2d(), Point2::new(5., 4.25), 0.125).unwrap(),
        None
    );
}
#[test]
fn plane_detection_returns_the_actual_plane_and_rejects_curved_quadrics() {
    let planar = IsPlanarSurface::new(&surface(1., &[]), 1e-9).unwrap();
    assert!(planar.is_planar());
    assert_eq!(planar.plan().unwrap().distance(Point3::new(2., 3., 8.)), 3.);
    let cylinder = IsPlanarSurface::new(&surface(2., &[2.]), 1e-9).unwrap();
    assert!(!cylinder.is_planar());
    assert_eq!(
        cylinder.plan().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
}
#[test]
fn constant_interpolation_reproduces_all_given_positions() {
    let p = Point3::new(7., 8., 9.);
    let (points, parameters) = point_arrays(1, &[p; 3]);
    let result = Interpolate::new(3, 3, &points, &parameters).unwrap();
    assert!(result.is_done());
    for t in [0., 1., 2.] {
        assert_eq!(result.curve().value(t).unwrap(), p);
    }
}
#[test]
fn interpolation_failure_precedence_preserves_null_result() {
    let (points, parameters) = point_arrays(1, &[Point3::default(); 3]);
    for (degree, count, error) in [
        (4, 3, InterpolationErrors::NotEnoughtPoints),
        (2, 3, InterpolationErrors::DegreeSmallerThan3),
    ] {
        let result = Interpolate::new(degree, count, &points, &parameters).unwrap();
        assert!(!result.is_done());
        assert_eq!(result.error(), error);
        assert!(result.curve().is_null());
    }
}
#[test]
fn interpolation_requires_one_based_source_arrays() {
    let (points, parameters) = point_arrays(-2, &[Point3::default(); 3]);
    let result = Interpolate::new(3, 3, &points, &parameters).unwrap();
    assert!(!result.is_done());
    assert_eq!(result.error(), InterpolationErrors::NotEnoughtPoints);
}
fn reversed_control_polygon() -> Handle<GeomBSplineCurve> {
    build::curve(&mut Reader::new(&[
        7., 0., 0., 3., 4., 0., 0., 0., 1., 0., 0., -1., 0., 0., 2., 0., 0., 2., 0., 1., 4., 4.,
    ]))
    .unwrap()
    .cast()
}
#[test]
fn repaired_copy_redistributes_the_reversed_end_without_changing_the_source() {
    let original = reversed_control_polygon();
    let mut check = CheckBSplineCurve::new(&original, 1e-9, 1e-9).unwrap();
    assert!(!check.is_done());
    assert_eq!(check.need_tangent_fix(), (true, false));
    let repaired = check.fixed_tangent(true, false).unwrap();
    assert!(check.is_done());
    assert_eq!(repaired.pole(2).unwrap(), Point3::new(-0.5, 0., 0.));
    assert_eq!(original.pole(2).unwrap(), Point3::new(1., 0., 0.));
    check.fix_tangent(true, false).unwrap();
    assert_eq!(original.pole(2).unwrap(), Point3::new(-0.5, 0., 0.));
}
#[test]
fn unrequested_tangent_repair_returns_null_and_preserves_the_polygon() {
    let original = reversed_control_polygon();
    let mut check = CheckBSplineCurve::new(&original, 1e-9, 1e-9).unwrap();
    assert!(check.fixed_tangent(false, false).unwrap().is_null());
    assert_eq!(original.pole(2).unwrap(), Point3::new(1., 0., 0.));
    assert!(!check.is_done());
}
#[test]
fn empty_curve_on_surface_check_pins_status_and_result_reset_order() {
    let curve: Handle<dyn Adaptor3dCurve> =
        Handle::new(GeomAdaptorCurve::with_curve(&curve3d()).unwrap()).cast();
    let mut check = CheckCurveOnSurface::with_curve(&curve, 1e-9);
    check.set_parallel(true);
    check.perform(&Handle::default());
    assert!(!check.is_done());
    assert_eq!(check.error_status(), 1);
    check.init_empty();
    assert!(check.is_done());
    assert!(check.is_parallel());
    assert_eq!(
        (check.max_distance(), check.max_parameter()),
        (f64::MAX, 0.)
    );
    check.perform(&Handle::default());
    assert_eq!(check.error_status(), 1);
}
#[test]
fn straight_line_chord_has_zero_deviation_under_both_methods() {
    let curve = Geom2dAdaptorCurve::with_curve(&curve2d()).unwrap();
    let result = compute_deviation(&curve, 0., 2., 1., 20).unwrap();
    assert_eq!(result.deviation, 0.);
    assert_eq!(compute_deviation_pso(&curve, 0., 2., 4, 20).unwrap().0, 0.);
}
#[test]
fn invalid_newton_start_preserves_absent_optional_outputs() {
    let curve = Geom2dAdaptorCurve::with_curve(&curve2d()).unwrap();
    let result = compute_deviation(&curve, 0., 2., 3., 20).unwrap();
    assert_eq!(result.deviation, -1.);
    assert!(result.prm_on_curve.is_none());
    assert!(result.pt_on_curve.is_none());
    assert!(result.vec_curv_line.is_none());
    assert!(result.line.is_none());
}
struct ZeroCoordinateEvaluator;
impl AdvApproxEvaluatorFunction for ZeroCoordinateEvaluator {
    fn evaluate(
        &mut self,
        dimension: i32,
        _interval: [f64; 2],
        _parameter: f64,
        _derivative: i32,
        result: &mut [f64],
    ) -> i32 {
        result[..dimension as usize].fill(0.);
        0
    }
}
#[test]
fn zero_vector_approximation_yields_the_independent_zero_curve() {
    let mut tolerance = Array1::with_bounds(1, 1).unwrap();
    tolerance[1] = 1e-9;
    let approximation = AdvApproxApproxAFunction::new(
        0,
        0,
        1,
        None,
        None,
        Some(&tolerance),
        0.,
        1.,
        Shape::C1,
        3,
        2,
        &mut ZeroCoordinateEvaluator,
    )
    .unwrap();
    let maker = MakeCurvefromApprox::new(&approximation);
    assert!(maker.is_done());
    assert_eq!(
        (
            maker.nb_1d_spaces().unwrap(),
            maker.nb_2d_spaces().unwrap(),
            maker.nb_3d_spaces().unwrap()
        ),
        (0, 0, 1)
    );
    let curve = maker.curve(1).unwrap();
    for t in [0., 0.5, 1.] {
        assert_eq!(curve.value(t).unwrap(), Point3::default());
    }
}

#[test]
fn unit_weight_denominator_multiplier_is_the_constant_one_law() {
    let fields = [
        9., 1., 0., 0., 1., 1., 2., 2., 0., 0., 0., 0., 1., 0., 1., 0., 0., 1., 1., 0., 1., 1., 1.,
        1., 2., 0., 1., 2., 2., 2., 0., 1., 2., 2.,
    ];
    let patch: Handle<GeomBSplineSurface> =
        build::surface(&mut Reader::new(&fields)).unwrap().cast();
    let mut flat = Array1::with_bounds(1, 8).unwrap();
    for i in 1..=8 {
        flat[i] = if i <= 4 { 0. } else { 1. };
    }
    let law = DenominatorMultiplier::new(&patch, &flat).unwrap();
    for u in [0., 0.5, 1.] {
        for v in [0., 0.5, 1.] {
            assert_eq!(law.value(u, v).unwrap(), 1.);
        }
    }
}
