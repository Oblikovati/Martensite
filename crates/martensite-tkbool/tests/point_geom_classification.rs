//! Closed-form point tolerance and planar-side classification expectations.
use martensite_tkbool::{
    top_ope_brep::{
        TopOpeBRepPoint2d, TopOpeBRepPointClassifier, TopOpeBRepPointGeomTool as Tool,
        TopOpeBRepVPointInter, TopOpeBRepVPointInterClassifier,
    },
    top_ope_brep_ds::TopOpeBRepDSPoint,
};
use martensite_tkg3d::top_abs::State;
use martensite_tkgeomalgo::int_patch::IntPatchPoint;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex};

#[test]
fn geometric_point_conversions_retain_positions_tolerances_and_equality() {
    let position = Point3::new(1., 2., 3.);
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(position, 0.125, false);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&source);
    let point = Tool::from_vpoint(&view).unwrap();
    assert_eq!(point.point(), position);
    assert_eq!(point.tolerance(), 0.125);
    let mut p2 = TopOpeBRepPoint2d::new();
    p2.set_value(&position);
    p2.set_tolerance(0.25);
    assert_eq!(Tool::from_point2d(&p2).tolerance(), 0.25);
    assert!(Tool::is_equal(
        &point,
        &TopOpeBRepDSPoint::new(Point3::new(1.1, 2., 3.), 0.0)
    ));
    assert!(!Tool::is_equal(
        &point,
        &TopOpeBRepDSPoint::new(Point3::new(1.2, 2., 3.), 0.0)
    ));
    let vertex = BRepBuilderAPIMakeVertex::new(position)
        .unwrap()
        .vertex()
        .unwrap();
    assert_eq!(Tool::from_shape(vertex.as_ref()).unwrap().point(), position);
}

#[test]
fn planar_classifier_preserves_sides_and_rejects_invalid_indices() {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(Point3::default(), 1e-7, false);
    source.set_parameters(0., 0., 2., 0.);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    let mut classifier = TopOpeBRepVPointInterClassifier::new();
    let mut cache = TopOpeBRepPointClassifier::new();
    assert!(classifier.edge().unwrap().is_null());
    assert_eq!(classifier.edge_parameter().unwrap(), 0.);
    assert_eq!(
        classifier
            .vpoint_position(face.as_ref(), &mut point, 1, &mut cache, false, 1e-7)
            .unwrap(),
        State::In
    );
    assert_eq!(
        classifier
            .vpoint_position(face.as_ref(), &mut point, 2, &mut cache, false, 1e-7)
            .unwrap(),
        State::Out
    );
    assert_eq!(point.state(1).unwrap(), State::In);
    assert_eq!(point.state(2).unwrap(), State::Out);
    let error = classifier
        .vpoint_position(face.as_ref(), &mut point, 0, &mut cache, false, 1e-7)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(error.message(), "VPointClassifier : wrong Index");
}

fn restricted_point(first: f64, second: f64) -> IntPatchPoint {
    use martensite_tkbrep::{
        brep::BRepBuilder, brep_adaptor::BRepAdaptorCurve2d, topexp::Explorer, topods,
    };
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tkgeomalgo::{int_start::TheArc, int_surf::IntSurfTransition};
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut edges = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let a = topods::edge(edges.current()).unwrap();
    edges.next().unwrap();
    let b = topods::edge(edges.current()).unwrap();
    BRepBuilder::default()
        .update_edge_tolerance(&a, first)
        .unwrap();
    BRepBuilder::default()
        .update_edge_tolerance(&b, second)
        .unwrap();
    let arc_a: TheArc =
        Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&a, &face).unwrap()).cast();
    let arc_b: TheArc =
        Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&b, &face).unwrap()).cast();
    let transition = IntSurfTransition::new();
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(Point3::default(), 0.1, false);
    point.set_arc(true, &arc_a, 0.5, &transition, &transition);
    point.set_arc(false, &arc_b, 0.25, &transition, &transition);
    point
}

#[test]
fn double_restriction_caps_only_the_first_edge_and_sums_both_tolerances() {
    let source = restricted_point(1.2, 2.0);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&source);
    assert_eq!(Tool::from_vpoint(&view).unwrap().tolerance(), 7.25);
    view.set_shape_index(1);
    assert_eq!(Tool::from_vpoint(&view).unwrap().tolerance(), 1.2);
    view.set_shape_index(2);
    assert_eq!(Tool::from_vpoint(&view).unwrap().tolerance(), 2.0);
    view.set_shape_index(0);
    assert_eq!(Tool::from_vpoint(&view).unwrap().tolerance(), 0.1);
}

#[test]
fn domain_position_uses_the_recorded_edge_without_reclassifying_geometry() {
    let source = restricted_point(1e-7, 1e-7);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&source);
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut classifier = TopOpeBRepVPointInterClassifier::new();
    let mut cache = TopOpeBRepPointClassifier::new();
    for index in [1, 2] {
        assert_eq!(
            classifier
                .vpoint_position(face.as_ref(), &mut view, index, &mut cache, false, 1e-7)
                .unwrap(),
            State::On
        );
        assert!(
            view.edge_on(index)
                .unwrap()
                .is_same(&view.edge(index).unwrap())
        );
        assert_eq!(
            view.edge_on_parameter(index).unwrap(),
            if index == 1 { 0.5 } else { 0.25 }
        );
    }
}

#[test]
fn assumed_own_side_changes_slow_outside_to_inside() {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(Point3::new(2., 0., 0.), 1e-7, false);
    source.set_parameters(2., 0., 0., 0.);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_point(&source);
    point.set_shape_index(1);
    let mut classifier = TopOpeBRepVPointInterClassifier::new();
    let mut cache = TopOpeBRepPointClassifier::new();
    assert_eq!(
        classifier
            .vpoint_position(face.as_ref(), &mut point, 1, &mut cache, true, 1e-7)
            .unwrap(),
        State::In
    );
    assert_eq!(point.state(1).unwrap(), State::In);
}

#[test]
fn slow_boundary_validates_spatial_incidence_in_addition_to_uv() {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let mut classifier = TopOpeBRepVPointInterClassifier::new();
    let mut cache = TopOpeBRepPointClassifier::new();
    for (spatial, expected) in [
        (Point3::new(1., 0., 0.), State::On),
        (Point3::new(1., 0., 1.), State::Out),
        (Point3::new(1., 1., 0.), State::On),
    ] {
        let mut source = IntPatchPoint::new();
        source.set_value_tangent(spatial, 1e-7, false);
        source.set_parameters(spatial.x(), spatial.y(), 0., 0.);
        let mut point = TopOpeBRepVPointInter::new();
        point.set_point(&source);
        assert_eq!(
            classifier
                .vpoint_position(face.as_ref(), &mut point, 1, &mut cache, false, 1e-7)
                .unwrap(),
            expected
        );
        if expected == State::On {
            assert!(!classifier.edge().unwrap().is_null());
            assert!(!point.edge_on(1).unwrap().is_null());
        }
    }
}

#[test]
fn face_edge_conversion_copies_current_event_and_rejects_empty_cursor() {
    use martensite_tkbool::top_ope_brep::TopOpeBRepFaceEdgeIntersector;
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::geom::{GeomCurve, GeomLine};
    use martensite_tkmath::gp::Direction3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
    let mut inter = TopOpeBRepFaceEdgeIntersector::new();
    assert!(Tool::from_face_edge(&inter).is_err());
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::default(),
        Direction3::new(0., 0., 1.).unwrap(),
    ))
    .cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve_params(&curve, -1., 1.)
        .unwrap()
        .edge()
        .unwrap();
    inter.perform(face.as_ref(), edge.as_ref()).unwrap();
    inter.init_point();
    let point = Tool::from_face_edge(&inter).unwrap();
    assert_eq!(point.point(), Point3::default());
    assert_eq!(point.tolerance(), inter.tolerance());
}

#[test]
fn point_services_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<Tool>();
    require::<TopOpeBRepVPointInterClassifier>();
}
