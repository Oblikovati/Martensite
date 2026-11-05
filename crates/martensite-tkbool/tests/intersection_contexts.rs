//! Analytic face orientation and 2D edge projection contracts.
use martensite_tkbool::top_ope_brep::{TopOpeBRepHctxee2d, TopOpeBRepHctxff2d};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_adaptor::BRepAdaptorSurface,
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::{
    geom::{GeomCylindricalSurface, GeomSphericalSurface, GeomSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::gp::{Axis3, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

fn rectangle() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}
fn first_edge(face: &Face) -> Edge {
    let explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    topods::edge(explorer.current()).unwrap()
}
fn failure<T>(result: Result<T, Failure>, message: &str) {
    let Err(error) = result else {
        panic!("expected {message}")
    };
    assert_eq!(error.exception_type(), "Standard_Failure");
    assert_eq!(error.message(), message);
}
#[test]
fn face_context_defaults_tolerances_and_index_diagnostics_match_native() {
    let context = TopOpeBRepHctxff2d::new();
    assert_eq!(context.tolerances(), (0., 0.));
    assert!(!context.faces_same_oriented());
    assert!(!context.surfaces_same_oriented());
    for index in [1, 2] {
        assert!(context.face(index).unwrap().is_null());
        assert!(context.hsurface(index).unwrap().is_null());
        assert!(context.face_same_oriented_with_ref(index).unwrap());
    }
    for index in [-1, 0, 3] {
        failure(context.face(index), "TopOpeBRepHctxff2d::Face");
        failure(context.hsurface(index), "TopOpeBRepHctxff2d::HSurface");
        failure(
            context.face_same_oriented_with_ref(index),
            "TopOpeBRepHctxff2d::FSO",
        );
    }
}
#[test]
fn face_context_retains_surface_handles_and_resolves_reversed_orientation() {
    let first = rectangle();
    let mut second = first.clone();
    second.set_orientation(Orientation::Reversed);
    let mut context = TopOpeBRepHctxff2d::new();
    context.set_faces(&first, &second).unwrap();
    assert!(context.surfaces_same_oriented());
    assert!(!context.faces_same_oriented());
    assert!(context.face_same_oriented_with_ref(1).unwrap());
    assert!(!context.face_same_oriented_with_ref(2).unwrap());
    let handle = context.hsurface(1).unwrap();
    context.set_faces(&first, &second).unwrap();
    assert!(handle == context.hsurface(1).unwrap());
    context.set_faces(&first, &rectangle()).unwrap();
    assert!(context.faces_same_oriented());
    assert!(context.face(1).unwrap().is_equal(&first));
    assert!(handle == context.hsurface(1).unwrap());
}
#[test]
fn explicit_surface_replacement_is_skipped_when_only_handle_identity_changes() {
    let face = rectangle();
    let first = Handle::new(BRepAdaptorSurface::with_face(&face, false).unwrap());
    let second = Handle::new(BRepAdaptorSurface::with_face(&face, false).unwrap());
    let mut context = TopOpeBRepHctxff2d::new();
    context.set_hsurfaces(&first, &second).unwrap();
    assert!(context.hsurface(1).unwrap() == first);
    let replacement = Handle::new(BRepAdaptorSurface::with_face(&face, false).unwrap());
    context.set_hsurfaces(&replacement, &second).unwrap();
    assert!(context.hsurface(1).unwrap() == first);
    context
        .set_hsurfaces(&Handle::default(), &Handle::default())
        .unwrap();
    assert!(context.hsurface(2).unwrap() == second);
}
#[test]
fn max_tolerance_preserves_native_nan_ordering_and_unvalidated_signs() {
    let mut context = TopOpeBRepHctxff2d::new();
    for (first, second, expected) in [(1., 2., 2.), (-1., -2., -1.), (1., f64::NAN, 1.)] {
        context.set_tolerances(first, second);
        assert_eq!(context.max_tolerance(), expected);
        assert_eq!(context.tolerances().0, first);
    }
    context.set_tolerances(f64::NAN, 1.);
    assert!(context.max_tolerance().is_nan());
}
#[test]
fn edge_context_defaults_and_invalid_indices_retain_exact_failures() {
    let context = TopOpeBRepHctxee2d::new();
    for index in [1, 2] {
        assert!(context.edge(index).unwrap().is_null());
        assert!(!context.domain(index).unwrap().has_first_point());
        assert!(!context.domain(index).unwrap().has_last_point());
        assert!(context.curve(index).unwrap().curve().is_null());
    }
    for index in [-1, 0, 3] {
        failure(context.edge(index), "TopOpeBRepHctxee2d::Edge");
        failure(context.curve(index), "TopOpeBRepHctxee2d::Curve");
        failure(context.domain(index), "TopOpeBRepHctxee2d::Domain");
    }
}
#[test]
fn planar_edge_context_loads_full_pcurve_and_independent_edge_domain() {
    let face = rectangle();
    let edge = first_edge(&face);
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    let mut context = TopOpeBRepHctxee2d::new();
    context.set_edges(&edge, &edge, &surface, &surface).unwrap();
    for index in [1, 2] {
        assert_rectangle_edge_domain(&context, &edge, index);
    }
}
fn assert_rectangle_edge_domain(context: &TopOpeBRepHctxee2d, edge: &Edge, index: i32) {
    assert!(context.edge(index).unwrap().is_same(edge));
    let domain = context.domain(index).unwrap();
    assert_eq!(domain.first_parameter().unwrap(), -1.);
    assert_eq!(domain.last_parameter().unwrap(), 1.);
    assert_eq!(domain.first_point().unwrap(), Point2::new(-1., -1.));
    assert_eq!(domain.last_point().unwrap(), Point2::new(-1., 1.));
    assert_eq!(
        domain.first_tolerance().unwrap(),
        BRepTool::edge_tolerance(edge).unwrap()
    );
    assert_eq!(
        domain.last_tolerance().unwrap(),
        domain.first_tolerance().unwrap()
    );
    assert_eq!(
        context.curve(index).unwrap().value(0.).unwrap(),
        Point2::new(-1., 0.)
    );
}
fn cylindrical_projection_fixture() -> (Face, BRepAdaptorSurface, BRepAdaptorSurface, Edge) {
    let cylinder: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap()).cast();
    let face = BRepBuilderAPIMakeFace::from_surface_bounds(&cylinder, (0., 1., 0., 3.), 1e-7)
        .unwrap()
        .face()
        .unwrap();
    let first = BRepAdaptorSurface::with_face(&face, false).unwrap();
    let second = BRepAdaptorSurface::with_face(&rectangle(), false).unwrap();
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(2., 0., 0.), Point3::new(2., 0., 3.))
            .unwrap()
            .edge()
            .unwrap();
    (face, first, second, edge)
}
#[test]
fn distinct_surface_projection_reuses_general_cylinder_projection_provider() {
    let (face, first, second, edge) = cylindrical_projection_fixture();
    let mut context = TopOpeBRepHctxee2d::new();
    context
        .set_edges(&first_edge(&face), &edge, &first, &second)
        .unwrap();
    let domain = context.domain(2).unwrap();
    assert_eq!(domain.first_parameter().unwrap(), 0.);
    assert_eq!(domain.last_parameter().unwrap(), 3.);
    for (parameter, expected) in [
        (0., Point2::new(0., 0.)),
        (1.5, Point2::new(0., 1.5)),
        (3., Point2::new(0., 3.)),
    ] {
        assert!(
            context
                .curve(2)
                .unwrap()
                .value(parameter)
                .unwrap()
                .distance(expected)
                <= 1e-12
        );
    }
    assert!(domain.first_point().unwrap().distance(Point2::new(0., 0.)) <= 1e-12);
    assert!(domain.last_point().unwrap().distance(Point2::new(0., 3.)) <= 1e-12);
}
#[test]
fn context_types_expose_send_and_sync_bounds_without_claiming_provider_determinism() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepHctxff2d>();
    assert_send_sync::<TopOpeBRepHctxee2d>();
}

fn degenerated_point_edge(point: Point3) -> Edge {
    let builder = BRepBuilder::default();
    let mut vertex = topods::Vertex::default();
    builder.make_vertex_at(&mut vertex, point, 1e-7).unwrap();
    let mut edge = Edge::default();
    builder.make_edge(&mut edge).unwrap();
    builder.add(&mut edge, &vertex).unwrap();
    builder.degenerated(&edge, true).unwrap();
    edge
}
#[test]
fn nonapex_degenerate_projection_retains_prior_curve_and_domain_but_replaces_edge() {
    let (face, first, second, edge) = cylindrical_projection_fixture();
    let mut context = TopOpeBRepHctxee2d::new();
    context
        .set_edges(&first_edge(&face), &edge, &first, &second)
        .unwrap();
    let prior_domain = *context.domain(2).unwrap();
    let degenerated = degenerated_point_edge(Point3::new(2., 0., 1.));
    context
        .set_edges(&first_edge(&face), &degenerated, &first, &second)
        .unwrap();
    assert!(context.edge(2).unwrap().is_same(&degenerated));
    assert_eq!(*context.domain(2).unwrap(), prior_domain);
    assert!(
        context
            .curve(2)
            .unwrap()
            .value(1.5)
            .unwrap()
            .distance(Point2::new(0., 1.5))
            <= 1e-12
    );
}
fn spherical_patch() -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(Axis3::default(), 2.).unwrap()).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0., 1., 0., std::f64::consts::FRAC_PI_2),
        1e-7,
    )
    .unwrap()
    .face()
    .unwrap()
}
fn degenerated_boundary(face: &Face) -> Edge {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let edge = topods::edge(explorer.current()).unwrap();
        if BRepTool::degenerated(&edge).unwrap() {
            return edge;
        }
        explorer.next().unwrap();
    }
    panic!("spherical patch: no degenerate edge; expected a pole boundary")
}
#[test]
fn spherical_apex_uses_first_face_degenerate_ancestor_curve() {
    let first = spherical_patch();
    let second = spherical_patch();
    let s1 = BRepAdaptorSurface::with_face(&first, false).unwrap();
    let s2 = BRepAdaptorSurface::with_face(&second, false).unwrap();
    let mut context = TopOpeBRepHctxee2d::new();
    context
        .set_edges(
            &first_edge(&first),
            &degenerated_boundary(&second),
            &s1,
            &s2,
        )
        .unwrap();
    let domain = context.domain(2).unwrap();
    assert_eq!(domain.first_parameter().unwrap(), 0.);
    assert_eq!(domain.last_parameter().unwrap(), 1.);
    assert_eq!(
        domain.first_point().unwrap(),
        Point2::new(0., std::f64::consts::FRAC_PI_2)
    );
    assert_eq!(
        domain.last_point().unwrap(),
        Point2::new(1., std::f64::consts::FRAC_PI_2)
    );
}
#[test]
fn invalid_null_surface_pair_is_checked_instead_of_dereferenced() {
    let face = rectangle();
    let surface = Handle::new(BRepAdaptorSurface::with_face(&face, false).unwrap());
    let mut context = TopOpeBRepHctxff2d::new();
    let error = context
        .set_hsurfaces(&surface, &Handle::default())
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert!(error.message().contains("surface=None"));
}
#[test]
fn missing_projected_3d_curve_is_a_checked_native_undefined_domain() {
    let (face, first, second, _) = cylindrical_projection_fixture();
    let mut empty = Edge::default();
    BRepBuilder::default().make_edge(&mut empty).unwrap();
    let mut context = TopOpeBRepHctxee2d::new();
    let error = context
        .set_edges(&first_edge(&face), &empty, &first, &second)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert!(error.message().contains("curve=None"));
}

#[test]
fn missing_first_pcurve_raises_exact_failure_after_edge_identity_assignment() {
    let face = rectangle();
    martensite_tkbool::top_ope_brep_tool::fc2d_prepare(&face, &face);
    let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
    let mut empty = Edge::default();
    BRepBuilder::default().make_edge(&mut empty).unwrap();
    let second = first_edge(&face);
    let mut context = TopOpeBRepHctxee2d::new();
    failure(
        context.set_edges(&empty, &second, &surface, &surface),
        "TopOpeBRepHctxee2d::SetEdges : no 2d curve",
    );
    assert!(context.edge(1).unwrap().is_same(&empty));
    assert!(context.edge(2).unwrap().is_same(&second));
    assert!(!context.domain(1).unwrap().has_first_point());
}
#[test]
fn separately_built_nonplanar_faces_on_same_surface_reuse_stored_pcurves() {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap()).cast();
    let face = BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 1., 0., 3.), 1e-7)
        .unwrap()
        .face()
        .unwrap();
    let other = BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 1., 0., 3.), 1e-7)
        .unwrap()
        .face()
        .unwrap();
    assert!(!face.is_same(&other));
    let first = BRepAdaptorSurface::with_face(&face, false).unwrap();
    let second = BRepAdaptorSurface::with_face(&other, false).unwrap();
    let mut context = TopOpeBRepHctxee2d::new();
    context
        .set_edges(&first_edge(&face), &first_edge(&other), &first, &second)
        .unwrap();
    assert_eq!(context.domain(2).unwrap().first_parameter().unwrap(), 0.);
    assert_eq!(context.domain(2).unwrap().last_parameter().unwrap(), 3.);
    assert_eq!(
        context.domain(2).unwrap().first_point().unwrap(),
        Point2::new(0., 0.)
    );
    assert_eq!(
        context.domain(2).unwrap().last_point().unwrap(),
        Point2::new(0., 3.)
    );
}

#[test]
fn context_handles_preserve_runtime_type_and_base_downcast_identity() {
    use martensite_tkernel::standard::{RuntimeTyped, StandardTransient};
    let face = Handle::new(TopOpeBRepHctxff2d::new());
    let base: Handle<StandardTransient> = face.cast();
    assert_eq!(base.dynamic_type().unwrap().name(), "TopOpeBRepHctxff2d");
    assert!(Handle::<TopOpeBRepHctxff2d>::down_cast(&base) == face);
    assert!(Handle::<TopOpeBRepHctxee2d>::down_cast(&base).is_null());
    let edge = Handle::new(TopOpeBRepHctxee2d::new());
    let base: Handle<StandardTransient> = edge.cast();
    assert_eq!(base.dynamic_type().unwrap().name(), "TopOpeBRepHctxee2d");
    assert!(Handle::<TopOpeBRepHctxee2d>::down_cast(&base) == edge);
    assert_eq!(
        TopOpeBRepHctxee2d::static_type().name(),
        "TopOpeBRepHctxee2d"
    );
}
