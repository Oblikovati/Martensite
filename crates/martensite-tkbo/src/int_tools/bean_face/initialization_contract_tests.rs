//! Initialization state remains observable when materializing transformed geometry fails.
use super::*;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface},
    geom_eval::GeomEvalParaboloidSurface,
};
use martensite_tkmath::{
    gp::{Axis1, Axis3, Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};

fn source_edge(direction: Direction3, tolerance: f64, last: f64) -> Edge {
    let line: Handle<dyn GeomCurve> =
        Handle::new(GeomLine::new(Axis1::new(Point3::default(), direction))).cast();
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder
        .make_edge_with_curve(&mut edge, &line, tolerance)
        .unwrap();
    builder.range(&edge, 0., last, true).unwrap();
    edge
}

fn refusing_surface_face() -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomEvalParaboloidSurface::new(Axis3::default(), 1.).unwrap()).cast();
    translated_face(&surface, 1e-7)
}

fn translated_face(surface: &Handle<dyn GeomSurface>, tolerance: f64) -> Face {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(0., 0., 2.));
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_located_surface(&mut face, surface, &Location::new(transform), tolerance)
        .unwrap();
    face
}

fn seeded_solver() -> BeanFaceIntersector {
    let mut bean = BeanFaceIntersector::new();
    bean.tolerances = [3e-7, 4e-7];
    bean.criteria = 7e-7;
    bean.curve_resolution = 0.125;
    bean.range = Range::new(-2., 2.);
    bean.bounds = [-3., 4., -5., 6.];
    bean.done = true;
    bean.minimum_square_distance = 37.;
    bean.results.append(Range::new(8., 9.));
    bean.transformed_surface =
        Handle::new(GeomPlane::from_point(Point3::new(0., 0., 3.), Direction3::Z).unwrap()).cast();
    bean
}

fn assert_old_solver_state(bean: &BeanFaceIntersector, old_surface: &Handle<dyn GeomSurface>) {
    assert_eq!(bean.tolerances, [3e-7, 4e-7]);
    assert_eq!(bean.criteria, 7e-7);
    assert_eq!(bean.curve_resolution, 0.125);
    assert_eq!(bean.range.range(), (-2., 2.));
    assert_eq!(bean.bounds, [-3., 4., -5., 6.]);
    assert!(bean.is_done());
    assert_eq!(bean.minimal_square_distance(), 37.);
    assert_eq!(bean.result().length(), 1);
    assert_eq!(bean.result().first().unwrap().range(), (8., 9.));
    assert_eq!(&bean.transformed_surface, old_surface);
}

fn assert_failed_initialization(mode: usize) {
    let edge = source_edge(Direction3::X, 1e-7, 2.);
    let face = refusing_surface_face();
    let mut bean = seeded_solver();
    let old_surface = bean.transformed_surface.clone();
    let failure = failing_reinitialization(&mut bean, &edge, &face, mode);
    assert_eq!(failure.exception_type(), "Standard_NotImplemented");
    assert_eq!(failure.message(), "GeomEval_ParaboloidSurface::Transform");
    assert_old_solver_state(&bean, &old_surface);
    assert!(bean.curve.get().unwrap().edge().is_same(&edge));
    assert!(bean.surface.get().unwrap().face().is_same(&face));
}

fn failing_reinitialization(
    bean: &mut BeanFaceIntersector,
    edge: &Edge,
    face: &Face,
    mode: usize,
) -> Failure {
    if mode == 0 {
        return bean.init_shapes(edge, face).unwrap_err();
    }
    let curve = BRepAdaptorCurve::with_edge(edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(face, false).unwrap();
    if mode == 1 {
        return bean
            .init_adaptors(&curve, &surface, 11e-7, 13e-7)
            .unwrap_err();
    }
    fail_bounded_adaptors(bean, &curve, &surface)
}

fn fail_bounded_adaptors(
    bean: &mut BeanFaceIntersector,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) -> Failure {
    bean.init_adaptors_with_bounds(
        curve,
        surface,
        Range::new(4., -3.),
        [-4., 3., 7., -2.],
        [11e-7, 13e-7],
    )
    .unwrap_err()
}

#[test]
fn failed_adaptor_init_retains_old_scalars_results_and_transformed_surface() {
    assert_failed_initialization(1);
}

#[test]
fn failed_topology_init_retains_old_scalars_results_and_transformed_surface() {
    assert_failed_initialization(0);
}

fn planar_sources() -> (Edge, Face) {
    let edge = source_edge(Direction3::Z, 0.125, 4.);
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    (edge, translated_face(&plane, 0.25))
}

fn assert_initialized_geometry(bean: &BeanFaceIntersector, expected_criteria: f64) {
    let curve = bean.curve.get().unwrap();
    assert_eq!(curve.d0(1.).unwrap(), Point3::new(0., 0., 1.));
    assert_eq!(
        bean.transformed_surface.value(0.5, 0.25).unwrap(),
        Point3::new(0.5, 0.25, 2.)
    );
    assert_eq!(
        bean.surface.get().unwrap().d0(0.5, 0.25).unwrap(),
        Point3::new(0.5, 0.25, 2.)
    );
    assert_eq!(bean.tolerances, [0.125, 0.25]);
    assert_eq!(bean.criteria, expected_criteria);
    assert_eq!(bean.curve_resolution, expected_criteria);
    assert!(bean.result().is_empty());
}

fn natural_plane_bounds() -> [f64; 4] {
    let infinite = precision::infinite();
    [-infinite, infinite, -infinite, infinite]
}

fn constructed_solver(
    mode: usize,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) -> BeanFaceIntersector {
    match mode {
        0 => BeanFaceIntersector::from_shapes(&curve.edge(), &surface.face()).unwrap(),
        1 => BeanFaceIntersector::from_adaptors(curve, surface, 0.125, 0.25).unwrap(),
        _ => bounded_planar_solver(curve, surface, Range::new(0., 4.)),
    }
}

#[test]
fn geometry_constructors_materialize_locations_and_run_closed_form_intersection() {
    let (edge, face) = planar_sources();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    for mode in 0..3 {
        let mut bean = constructed_solver(mode, &curve, &surface);
        assert_initialized_geometry(&bean, initialization_criteria(mode));
        assert!(!bean.is_done());
        assert_eq!(bean.minimal_square_distance(), f64::MAX);
        assert_eq!(bean.context().is_null(), mode != 0);
        assert_constructor_bounds(&bean, mode);
        bean.set_bean_parameters(0., 4.);
        bean.perform().unwrap();
        assert!(bean.is_done());
        assert_eq!(bean.result().first().unwrap().range(), (1.75, 2.25));
        assert_eq!(bean.result().length(), 1);
    }
}

fn assert_constructor_bounds(bean: &BeanFaceIntersector, mode: usize) {
    if mode == 2 {
        assert_eq!(bean.range.range(), (0., 4.));
        assert_eq!(bean.bounds, [-1., 1., -1., 1.]);
        return;
    }
    assert_eq!(bean.range.range(), (0., 0.));
    assert_eq!(bean.bounds, natural_plane_bounds());
}

#[test]
fn successful_initializers_clear_results_and_retain_completion_distance_and_context() {
    let (edge, face) = planar_sources();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    for mode in 0..3 {
        let mut bean = seeded_solver();
        let context = Handle::new(Context::default());
        bean.set_context(&context);
        reinitialize_solver(&mut bean, &curve, &surface, mode);
        assert_initialized_geometry(&bean, initialization_criteria(mode));
        assert!(bean.is_done());
        assert_eq!(bean.minimal_square_distance(), 37.);
        assert_eq!(bean.context(), &context);
        assert_reinitialized_bounds(&bean, mode);
    }
}

fn reinitialize_solver(
    bean: &mut BeanFaceIntersector,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
    mode: usize,
) {
    match mode {
        0 => bean.init_shapes(&curve.edge(), &surface.face()).unwrap(),
        1 => bean.init_adaptors(curve, surface, 0.125, 0.25).unwrap(),
        _ => bean
            .init_adaptors_with_bounds(
                curve,
                surface,
                Range::new(4., -3.),
                [-4., 3., 7., -2.],
                [0.125, 0.25],
            )
            .unwrap(),
    }
}

fn assert_reinitialized_bounds(bean: &BeanFaceIntersector, mode: usize) {
    if mode == 2 {
        assert_eq!(bean.range.range(), (4., -3.));
        assert_eq!(bean.bounds, [-4., 3., 7., -2.]);
        return;
    }
    assert_eq!(bean.range.range(), (-2., 2.));
    assert_eq!(bean.bounds, natural_plane_bounds());
}

#[test]
fn constructor_copies_adaptor_values_before_source_adaptors_are_reinitialized() {
    let (edge, face) = planar_sources();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    let bean = constructed_solver(2, &curve, &surface);
    curve
        .initialize(&source_edge(Direction3::X, 1e-7, 2.))
        .unwrap();
    surface.initialize(&refusing_surface_face(), false).unwrap();
    assert_initialized_geometry(&bean, 0.375);
    assert!(bean.curve.get().unwrap().edge().is_same(&edge));
    assert!(bean.surface.get().unwrap().face().is_same(&face));
}

#[test]
fn bounded_constructor_propagates_exact_transform_failure() {
    let edge = source_edge(Direction3::X, 1e-7, 2.);
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(&refusing_surface_face(), false).unwrap();
    let failure = BeanFaceIntersector::from_adaptors_with_bounds(
        &curve,
        &surface,
        Range::new(4., -3.),
        [-4., 3., 7., -2.],
        [0.125, 0.25],
    )
    .err()
    .expect("translated eval surface refuses transformation");
    assert_eq!(failure.exception_type(), "Standard_NotImplemented");
    assert_eq!(failure.message(), "GeomEval_ParaboloidSurface::Transform");
}

#[test]
fn failed_bounded_init_does_not_assign_explicit_bounds_after_transform_failure() {
    assert_failed_initialization(2);
}

fn initialization_criteria(mode: usize) -> f64 {
    if mode == 0 {
        return 0.375 + precision::confusion();
    }
    0.375
}

#[test]
fn bounded_constructor_preserves_point_intervals_and_reversed_domain_behavior() {
    let (edge, face) = planar_sources();
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    for (range, count) in [
        (Range::new(2., 2.), 1),
        (Range::new(1., 1.), 0),
        (Range::new(4., -3.), 0),
    ] {
        assert_degenerate_range(&curve, &surface, range, count);
    }
}

fn assert_degenerate_range(
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
    range: Range,
    count: i32,
) {
    let mut bean = bounded_planar_solver(curve, surface, range);
    bean.perform().unwrap();
    assert!(bean.is_done());
    assert_eq!(bean.result().length(), count);
    if count == 1 {
        assert_eq!(bean.result().first().unwrap().range(), (2., 2.));
    }
}

fn bounded_planar_solver(
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
    range: Range,
) -> BeanFaceIntersector {
    BeanFaceIntersector::from_adaptors_with_bounds(
        curve,
        surface,
        range,
        [-1., 1., -1., 1.],
        [0.125, 0.25],
    )
    .unwrap()
}

#[test]
fn geometryless_topology_init_keeps_partial_curve_assignment_and_previous_geometry() {
    let mut bean = seeded_solver();
    let edge = source_edge(Direction3::X, 1e-7, 2.);
    bean.curve = Handle::new(BRepAdaptorCurve::with_edge(&edge).unwrap());
    let old_surface = bean.transformed_surface.clone();
    let mut empty = Edge::default();
    BRepBuilder::default().make_edge(&mut empty).unwrap();
    let failure = bean.init_shapes(&empty, &Face::default()).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert_eq!(failure.message(), "BRepAdaptor_Curve::No geometry");
    assert_old_solver_state(&bean, &old_surface);
    assert_partial_curve_assignment(&bean, &empty);
}

fn assert_partial_curve_assignment(bean: &BeanFaceIntersector, empty: &Edge) {
    assert!(bean.curve.get().unwrap().edge().is_same(empty));
    assert_eq!(
        bean.curve.get().unwrap().d0(1.).unwrap(),
        Point3::new(1., 0., 0.)
    );
    assert!(bean.surface.get().unwrap().face().is_null());
    assert!(!bean.context().is_null());
}
