//! Independent state tables, topology identity and source-borrow contracts.
use martensite_tkbool::top_ope_brep::TopOpeBRepVPointInter;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorCurve2d,
    topexp::Explorer,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkgeomalgo::{
    int_patch::IntPatchPoint,
    int_start::{TheArc, TheVertex},
    int_surf::{IntSurfTransition, IntSurfTypeTrans},
};
use martensite_tkmath::gp::{Plane3, Point2, Point3};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex},
    brep_top_adaptor::BRepTopAdaptorHVertex,
};

fn assert_domain<T>(result: Result<T, Failure>, message: &str) {
    let Err(error) = result else {
        panic!("expected domain error {message}")
    };
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert_eq!(error.message(), message);
}
fn restriction_fixture() -> (Face, Edge, TheArc, TheVertex) {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap();
    let explorer = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(explorer.current()).unwrap();
    let curve = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap());
    let arc: TheArc = curve.cast();
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap();
    let hvertex: TheVertex = Handle::new(BRepTopAdaptorHVertex::new(&vertex, &curve)).cast();
    (face, edge, arc, hvertex)
}
fn transition(kind: IntSurfTypeTrans) -> IntSurfTransition {
    let mut transition = IntSurfTransition::new();
    transition.set_type(false, kind);
    transition
}
fn source_point() -> IntPatchPoint {
    let mut point = IntPatchPoint::new();
    point.set_value_tangent(Point3::new(1., 2., 3.), 0.25, true);
    point.set_parameters(4., 5., 6., 7.);
    point.set_parameter(8.);
    point.set_multiple(true);
    point
}

#[test]
fn default_view_and_new_retain_native_initialized_fields() {
    for view in [
        TopOpeBRepVPointInter::new(),
        TopOpeBRepVPointInter::default(),
    ] {
        assert_eq!(view.shapes(), (0, 0));
        assert_eq!(view.index(), 0);
        assert_eq!(view.shape_index(), 0);
        assert!(!view.keep());
        assert!(!view.is_internal());
        assert!(view.point_of_intersection().is_none());
        for index in [1, 2] {
            assert_eq!(view.state(index).unwrap(), State::Unknown);
            assert!(view.edge_on(index).unwrap().is_null());
            assert_eq!(view.edge_on_parameter(index).unwrap(), 0.);
        }
    }
}
#[test]
fn point_metadata_and_parameters_delegate_without_reinterpretation() {
    let point = source_point();
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    view.set_shapes(-3, 9);
    view.set_index(-17);
    assert_eq!(view.shapes(), (-3, 9));
    assert_eq!(view.index(), -17);
    assert!(std::ptr::eq(view.point_of_intersection().unwrap(), &point));
    assert_eq!(view.value().unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(view.tolerance().unwrap(), 0.25);
    assert_eq!(view.parameter_on_line().unwrap(), 8.);
    assert_eq!(view.parameters_on_s1().unwrap(), (4., 5.));
    assert_eq!(view.parameters_on_s2().unwrap(), (6., 7.));
    assert_eq!(view.surface_parameters(1).unwrap(), Point2::new(4., 5.));
    assert_eq!(view.surface_parameters(2).unwrap(), Point2::new(6., 7.));
    assert!(view.is_multiple().unwrap());
    assert!(!view.is_on_dom_s1().unwrap());
    assert!(!view.is_on_dom_s2().unwrap());
}
#[test]
fn keep_truth_table_uses_domain_mask_and_state_with_no_implicit_mask_update() {
    for mask in [-1, 0, 1, 2, 3, 4] {
        for first in State::ALL {
            for second in State::ALL {
                let mut view = TopOpeBRepVPointInter::new();
                view.set_shape_index(mask);
                view.set_state(first, 1).unwrap();
                view.set_state(second, 2).unwrap();
                let inside_first = first == State::In || first == State::On;
                let inside_second = second == State::In || second == State::On;
                let expected = match mask {
                    1 => inside_second,
                    2 => inside_first,
                    0 | 3 => inside_first && inside_second,
                    _ => false,
                };
                assert_eq!(view.keep(), expected, "mask={mask}, {first:?}, {second:?}");
                view.change_keep(!expected);
                assert_eq!(view.keep(), !expected);
                view.update_keep();
                assert_eq!(view.keep(), expected);
            }
        }
    }
}
#[test]
fn setting_point_changes_only_domain_mask_and_rebinding_retains_other_fields() {
    let (_, _, arc, _) = restriction_fixture();
    let mut point = source_point();
    let t = transition(IntSurfTypeTrans::In);
    point.set_arc(true, &arc, 0.5, &t, &t);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_state(State::In, 1).unwrap();
    view.set_state(State::In, 2).unwrap();
    view.set_shapes(2, 3);
    view.set_point(&point);
    assert_eq!(view.shape_index(), 1);
    assert!(view.keep());
    view.change_keep(false);
    let mut second = source_point();
    second.set_arc(false, &arc, 1.5, &t, &t);
    view.set_point(&second);
    assert_eq!(view.shape_index(), 2);
    assert!(!view.keep());
    assert_eq!(view.shapes(), (2, 3));
    assert_eq!(view.state(1).unwrap(), State::In);
}
#[test]
fn valid_domain_delegates_arcs_vertices_parameters_and_transitions() {
    let (face, edge, arc, vertex) = restriction_fixture();
    let mut point = source_point();
    let incoming = transition(IntSurfTypeTrans::In);
    let outgoing = transition(IntSurfTypeTrans::Out);
    point.set_arc(true, &arc, 0.375, &incoming, &outgoing);
    point.set_arc(false, &arc, 0.625, &outgoing, &incoming);
    point.set_vertex(true, &vertex);
    point.set_vertex(false, &vertex);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    assert_eq!(view.shape_index(), 3);
    assert!(view.arc_on_s1().unwrap().is_same(&edge));
    assert!(view.arc_on_s2().unwrap().is_same(&edge));
    assert_eq!(view.parameter_on_arc1().unwrap(), 0.375);
    assert_eq!(view.parameter_on_arc2().unwrap(), 0.625);
    assert_eq!(
        view.transition_on_s1().unwrap().transition_type(),
        IntSurfTypeTrans::Out
    );
    assert_eq!(
        view.transition_on_s2().unwrap().transition_type(),
        IntSurfTypeTrans::In
    );
    assert_eq!(
        view.transition_line_arc1().unwrap().transition_type(),
        IntSurfTypeTrans::In
    );
    assert_eq!(
        view.transition_line_arc2().unwrap().transition_type(),
        IntSurfTypeTrans::Out
    );
    for index in [1, 2] {
        assert!(view.is_vertex(index).unwrap());
        assert!(view.edge(index).unwrap().is_same(&edge));
        assert!(!view.vertex(index).unwrap().is_null());
    }
    assert!(
        view.vertex_on_s1()
            .unwrap()
            .is_same(&view.vertex_on_s2().unwrap())
    );
    let mut output = Vec::new();
    view.dump_on_surface(1, &face, &mut output).unwrap();
    view.dump_on_surface(2, &face, &mut output).unwrap();
    view.dump(&face, &face, &mut output);
    assert!(output.is_empty());
}
#[test]
fn absent_domain_accessors_preserve_exact_native_exception_messages() {
    let point = source_point();
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    assert_domain(view.arc_on_s1(), "");
    assert_domain(view.arc_on_s2(), "");
    assert_domain(view.parameter_on_arc1(), "");
    assert_domain(view.parameter_on_arc2(), "");
    assert_domain(view.transition_on_s1(), "");
    assert_domain(view.transition_on_s2(), "");
    assert_domain(view.transition_line_arc1(), "");
    assert_domain(view.transition_line_arc2(), "");
    assert_domain(view.vertex_on_s1(), "TopOpeBRepVPointInter::VertexOnS1");
    assert_domain(view.vertex_on_s2(), "TopOpeBRepVPointInter::VertexOnS2");
    for index in [-1, 0, 1, 2, 3] {
        assert!(view.edge(index).unwrap().is_null());
        assert!(view.vertex(index).unwrap().is_null());
        assert!(!view.is_vertex(index).unwrap());
        assert_eq!(view.edge_parameter(index).unwrap(), 0.);
    }
    assert_eq!(view.surface_parameters(0).unwrap(), Point2::new(0., 0.));
    assert_eq!(view.surface_parameters(3).unwrap(), Point2::new(0., 0.));
}
#[test]
fn invalid_state_and_on_accessors_raise_but_invalid_on_assignment_is_ignored() {
    let mut view = TopOpeBRepVPointInter::new();
    view.change_keep(true);
    for index in [i32::MIN, -1, 0, 3, i32::MAX] {
        assert_domain(view.state(index), "TopOpeBRepVPointInter::State");
        assert_domain(
            view.set_state(State::Out, index),
            "TopOpeBRepVPointInter::State",
        );
        assert_domain(view.edge_on(index), "TopOpeBRepVPointInter::EdgeON");
        assert_domain(
            view.edge_on_parameter(index),
            "TopOpeBRepVPointInter::EdgeONParameter",
        );
        view.set_edge_on(&Shape::default(), 12., index);
        assert!(view.keep());
        assert_eq!(view.edge_on_parameter(1).unwrap(), 0.);
        assert_eq!(view.edge_on_parameter(2).unwrap(), 0.);
    }
}
#[test]
fn edge_search_prefers_first_then_second_domain_then_on_state_and_retains_miss_output() {
    let (_, edge, arc, _) = restriction_fixture();
    let mut point = source_point();
    let t = transition(IntSurfTypeTrans::In);
    point.set_arc(true, &arc, 0.375, &t, &t);
    point.set_arc(false, &arc, 0.625, &t, &t);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    view.set_edge_on(&edge, 9., 1);
    view.set_state(State::On, 1).unwrap();
    let mut parameter = 77.;
    assert!(view.par_on_edge(&edge, &mut parameter).unwrap());
    assert_eq!(parameter, 0.375);
    let second_only = {
        let mut p = source_point();
        p.set_arc(false, &arc, 0.625, &t, &t);
        p
    };
    view.set_point(&second_only);
    assert!(view.par_on_edge(&edge, &mut parameter).unwrap());
    assert_eq!(parameter, 0.625);
    let no_domains = source_point();
    view.set_point(&no_domains);
    assert!(view.par_on_edge(&edge, &mut parameter).unwrap());
    assert_eq!(parameter, 9.);
    view.set_state(State::Out, 1).unwrap();
    parameter = 77.;
    assert!(!view.par_on_edge(&edge, &mut parameter).unwrap());
    assert_eq!(parameter, 77.);
    view.set_edge_on(&edge, 11., 2);
    view.set_state(State::On, 2).unwrap();
    assert!(view.par_on_edge(&edge, &mut parameter).unwrap());
    assert_eq!(parameter, 11.);
}
#[test]
fn point_equality_is_strict_in_parameter_and_inclusive_in_spatial_tolerance() {
    let mut origin = source_point();
    origin.set_value(Point3::default());
    origin.set_parameter(0.);
    origin.set_tolerance(0.25);
    let mut first = TopOpeBRepVPointInter::new();
    first.set_point(&origin);
    for (distance, parameter, expected) in [
        (0.25, 0., true),
        (0.5, 0., false),
        (0., precision::p_confusion(), false),
        (0., precision::p_confusion() / 2., true),
    ] {
        let mut point = origin.clone();
        point.set_value(Point3::new(distance, 0., 0.));
        point.set_parameter(parameter);
        let mut second = TopOpeBRepVPointInter::new();
        second.set_point(&point);
        assert_eq!(first.equal_pp(&second).unwrap(), expected);
    }
}
#[test]
fn unbound_or_wrongly_typed_native_pointer_cases_are_safe_errors() {
    let view = TopOpeBRepVPointInter::new();
    let error = view.value().unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert!(error.message().contains("source point=None"));
    let (_, _, _, vertex) = restriction_fixture();
    let mut point = source_point();
    point.set_vertex(true, &vertex);
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    assert_domain(
        view.arc_on_s1(),
        "TopOpeBRepVPointInter::Arc: expected a non-null BRepAdaptor_Curve2d",
    );
}
#[test]
fn point_views_are_send_and_sync_for_their_borrow_lifetime() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepVPointInter<'static>>();
}

#[test]
fn equality_preserves_native_asymmetric_nan_tolerance_choice() {
    for (first_tolerance, second_tolerance, expected) in [
        (f64::NAN, 0.25, true),
        (0.25, f64::NAN, false),
        (-1., -2., false),
    ] {
        let mut first = source_point();
        first.set_tolerance(first_tolerance);
        let mut second = first.clone();
        second.set_tolerance(second_tolerance);
        let mut a = TopOpeBRepVPointInter::new();
        let mut b = TopOpeBRepVPointInter::new();
        a.set_point(&first);
        b.set_point(&second);
        assert_eq!(a.equal_pp(&b).unwrap(), expected);
    }
}

#[test]
fn null_vertex_handle_is_a_checked_error_instead_of_native_dereference_ub() {
    let mut point = source_point();
    point.set_vertex(true, &TheVertex::default());
    let mut view = TopOpeBRepVPointInter::new();
    view.set_point(&point);
    assert_domain(
        view.vertex_on_s1(),
        "TopOpeBRepVPointInter::Vertex: expected a non-null BRepTopAdaptor_HVertex",
    );
}

#[test]
fn owned_point_survives_source_scope_and_rebinding_keeps_native_state() {
    let mut view = TopOpeBRepVPointInter::new();
    view.set_shapes(7, 11);
    view.set_index(13);
    view.change_keep(true);
    {
        let point = source_point();
        view.set_owned_point(point.clone());
    }
    assert_eq!(view.value().unwrap(), Point3::new(1., 2., 3.));
    assert_eq!(view.parameter_on_line().unwrap(), 8.);
    assert_eq!(view.shapes(), (7, 11));
    assert_eq!(view.index(), 13);
    assert!(view.keep());
    assert_eq!(
        view.point_of_intersection().unwrap().parameters_on_s2(),
        (6., 7.)
    );
}
