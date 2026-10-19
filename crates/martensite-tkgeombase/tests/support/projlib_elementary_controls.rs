//! Orthogonal plane algebra and analytic cylinder parameters independently pin projection contracts.
use super::super::construction_corpus::{geometry_record_fixtures as build, print::Reader};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkgeombase::proj_lib::{
    self as pl, Cylinder, Plane, PrjFunc, PrjResolve, PrjResolveBox, Project, Projector,
};
use martensite_tkmath::{
    geom_abs::CurveType,
    gp::{Axis3, Cylinder3, Direction3, Line3, Plane3, Point2, Point3},
    math::{Matrix, Vector},
};

fn curve() -> GeomAdaptorCurve {
    let line = build::curve(&mut Reader::new(&[1., 0., 0., 1., 1., 0., 0.])).unwrap();
    GeomAdaptorCurve::with_curve(&line).unwrap()
}
fn surface() -> GeomAdaptorSurface {
    let plane = build::surface(&mut Reader::new(&[
        1., 0., 0., 0., 0., 0., 1., 1., 0., 0., 0.,
    ]))
    .unwrap();
    GeomAdaptorSurface::with_surface(&plane).unwrap()
}
fn vector(a: f64, b: f64) -> Vector<'static> {
    let mut v = Vector::with_bounds(1, 2).unwrap();
    v[1] = a;
    v[2] = b;
    v
}
fn bounds() -> PrjResolveBox {
    PrjResolveBox {
        tol: Point2::new(1e-9, 1e-9),
        inf: Point2::new(-10., -10.),
        sup: Point2::new(10., 10.),
    }
}
#[test]
fn base_projection_marks_other_curve_without_completing() {
    let mut projector = Projector::new();
    assert!(!projector.is_done());
    assert!(!projector.is_periodic());
    assert_eq!(projector.get_type(), CurveType::BSplineCurve);
    assert!(projector.bspline().is_null());
    assert!(projector.bezier().is_null());
    assert_eq!(
        projector.line().unwrap_err().exception_type(),
        "Standard_NoSuchObject"
    );
    let line = Line3::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap());
    projector.project_line(&line).unwrap();
    assert_eq!(projector.get_type(), CurveType::OtherCurve);
    assert!(!projector.is_done());
}
#[test]
fn plane_projector_recovers_coordinates_and_resets_completion_on_init() {
    let plane = Plane3::default();
    let line = Line3::new(
        Point3::new(3., 4., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    let mut projection = Plane::with_line(&plane, &line).unwrap();
    assert!(projection.is_done());
    assert_eq!(projection.get_type(), CurveType::Line);
    assert!(!projection.is_periodic());
    assert_eq!(projection.line().unwrap().location(), Point2::new(3., 4.));
    assert_eq!(projection.line().unwrap().direction().x(), 1.);
    projection.init(&plane);
    assert!(!projection.is_done());
    assert_eq!(projection.get_type(), CurveType::OtherCurve);
}
#[test]
fn plane_point_projection_is_independent_of_normal_displacement() {
    for z in [-7., 0., 11.] {
        assert_eq!(
            pl::project_point_on_plane(&Plane3::default(), Point3::new(3., 4., z)),
            Point2::new(3., 4.)
        );
    }
}
#[test]
fn cylinder_generatrix_projects_to_constant_angle_and_linear_height() {
    let cylinder = Cylinder3::new(Axis3::default(), 2.).unwrap();
    let line = Line3::new(
        Point3::new(2., 0., 3.),
        Direction3::new(0., 0., 1.).unwrap(),
    );
    let projection = Cylinder::with_line(&cylinder, &line);
    assert!(projection.is_done());
    let result = projection.line().unwrap();
    assert_eq!(result.location(), Point2::new(0., 3.));
    assert_eq!((result.direction().x(), result.direction().y()), (0., 1.));
    assert_eq!(
        pl::project_point_on_cylinder(&cylinder, Point3::new(2., 0., 5.)),
        Point2::new(0., 5.)
    );
}
#[test]
fn projection_function_all_fixed_coordinate_jacobians_follow_plane_algebra() {
    let (c, s) = (curve(), surface());
    for (fix, residual, jacobian) in [
        (1, [1., 4.], [1., 0., 0., 1.]),
        (2, [-1., 4.], [-1., 0., 0., 1.]),
        (3, [1., 2.], [-1., 1., 0., 0.]),
    ] {
        let mut function = PrjFunc::new(&c, 2., &s, fix).unwrap();
        assert_eq!(function.solution(), Point2::default());
        let mut f = vector(7.5, 7.5);
        let mut d = Matrix::filled(1, 2, 1, 2, 7.5).unwrap();
        assert!(function.values(&vector(3., 4.), &mut f, &mut d).unwrap());
        assert_eq!([f[1], f[2]], residual);
        assert_eq!([d[(1, 1)], d[(1, 2)], d[(2, 1)], d[(2, 2)]], jacobian);
        assert_eq!(function.solution(), Point2::new(3., 4.));
    }
}
#[test]
fn derivative_only_evaluation_updates_the_remembered_solution() {
    let (c, s) = (curve(), surface());
    let mut function = PrjFunc::new(&c, 2., &s, 1).unwrap();
    let mut d = Matrix::filled(1, 2, 1, 2, 7.5).unwrap();
    assert!(function.derivatives(&vector(5., 6.), &mut d).unwrap());
    assert_eq!(function.solution(), Point2::new(5., 6.));
    assert_eq!(
        [d[(1, 1)], d[(1, 2)], d[(2, 1)], d[(2, 2)]],
        [1., 0., 0., 1.]
    );
}
#[test]
fn function_and_solver_reject_fixed_coordinate_indices_outside_one_to_three() {
    let (c, s) = (curve(), surface());
    for fix in [0, 4] {
        assert_eq!(
            PrjFunc::new(&c, 0., &s, fix)
                .err()
                .unwrap()
                .exception_type(),
            "Standard_ConstructionError"
        );
        assert_eq!(
            PrjResolve::new(&c, &s, fix).err().unwrap().exception_type(),
            "Standard_ConstructionError"
        );
    }
}
#[test]
fn resolver_reports_not_done_before_perform_then_exact_plane_foot() {
    let (c, s) = (curve(), surface());
    let mut solver = PrjResolve::new(&c, &s, 1).unwrap();
    assert!(!solver.is_done());
    assert_eq!(
        solver.solution().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    solver.perform(2., 0., 0., bounds(), 1e-9, false).unwrap();
    assert!(solver.is_done());
    assert_eq!(solver.solution().unwrap(), Point2::new(2., 0.));
}
#[test]
fn strict_inside_option_is_unused_in_the_source_solver() {
    let (c, s) = (curve(), surface());
    for strict in [false, true] {
        let mut solver = PrjResolve::new(&c, &s, 1).unwrap();
        solver.perform(2., 0., 0., bounds(), 1e-9, strict).unwrap();
        assert_eq!(solver.solution().unwrap(), Point2::new(2., 0.));
    }
}
#[test]
fn repeated_resolve_reads_the_new_fixed_parameter() {
    let (c, s) = (curve(), surface());
    let mut solver = PrjResolve::new(&c, &s, 1).unwrap();
    for t in [2., -3.] {
        solver.perform(t, 0., 0., bounds(), 1e-9, false).unwrap();
        assert_eq!(solver.solution().unwrap(), Point2::new(t, 0.));
    }
}
