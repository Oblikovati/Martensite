//! Independent intersection bookkeeping and exact legacy exception contracts.
use martensite_tkbool::{
    top_ope_brep::{
        TopOpeBRepHctxee2d, TopOpeBRepHctxff2d, TopOpeBRepP2DStatus, TopOpeBRepPoint2d,
    },
    top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSTransition},
};
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeomalgo::int_res2d::{IntersectionPoint, Transition};
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

fn failure<T>(result: Result<T, Failure>, message: &str) {
    let Err(error) = result else {
        panic!("expected {message}")
    };
    assert_eq!(error.exception_type(), "Standard_Failure");
    assert_eq!(error.message(), message);
}
#[test]
fn default_point_has_independent_zero_fields_keep_true_and_unknown_status() {
    for point in [TopOpeBRepPoint2d::new(), TopOpeBRepPoint2d::default()] {
        assert!(!point.has_pint());
        failure(point.pint(), "TopOpeBRepPoint2d::Pint");
        assert!(!point.is_point_of_segment());
        assert_eq!(point.segment_ancestors(), (false, 0, 0));
        assert_eq!(point.status(), TopOpeBRepP2DStatus::Unknown);
        assert_eq!(point.index(), 0);
        assert_eq!(*point.value(), Point3::default());
        assert_eq!(*point.value2d(), Point2::default());
        assert!(point.keep());
        assert_eq!(point.edges_config(), TopOpeBRepDSConfig::UnshGeometry);
        assert_eq!(point.tolerance(), 0.);
        assert!(point.hctxff2d().is_null() && point.hctxee2d().is_null());
    }
}
#[test]
fn copied_intersection_and_positions_do_not_change_other_bookkeeping() {
    let intersection = IntersectionPoint::new(
        Point2::new(2., 3.),
        4.,
        5.,
        Transition::default(),
        Transition::default(),
        false,
    );
    let mut point = TopOpeBRepPoint2d::new();
    point.set_pint(&intersection);
    assert!(point.has_pint());
    assert_eq!(*point.pint().unwrap(), intersection);
    point.set_value(&Point3::new(6., 7., 8.));
    point.set_value2d(&Point2::new(9., 10.));
    assert_eq!(*point.value(), Point3::new(6., 7., 8.));
    assert_eq!(*point.value2d(), Point2::new(9., 10.));
    assert_eq!(point.index(), 0);
    assert_eq!(point.parameter(1).unwrap(), 0.);
}
#[test]
fn ancestor_status_changes_only_when_both_indices_are_nonzero() {
    let mut point = TopOpeBRepPoint2d::new();
    for (first, second) in [(1, 2), (-1, -2), (i32::MIN, i32::MAX)] {
        point.set_segment_ancestors(first, second);
        assert_eq!(point.segment_ancestors(), (true, first, second));
        assert_eq!(point.status(), TopOpeBRepP2DStatus::New);
    }
    point.set_status(TopOpeBRepP2DStatus::SegmentLast);
    for (first, second) in [(0, 2), (1, 0), (0, 0)] {
        point.set_segment_ancestors(first, second);
        assert_eq!(point.segment_ancestors(), (false, first, second));
        assert_eq!(point.status(), TopOpeBRepP2DStatus::SegmentLast);
    }
}
#[test]
fn vertex_presence_flag_can_hide_and_reveal_the_retained_vertex() {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap();
    let mut point = TopOpeBRepPoint2d::new();
    for index in [1, 2] {
        failure(point.vertex(index), "TopOpeBRepPoint2d::Vertex");
        point.set_vertex(index, &vertex).unwrap();
        assert!(point.is_vertex(index).unwrap());
        assert!(point.vertex(index).unwrap().is_same(&vertex));
        point.set_is_vertex(index, false).unwrap();
        failure(point.vertex(index), "TopOpeBRepPoint2d::Vertex");
        point.set_is_vertex(index, true).unwrap();
        assert!(point.vertex(index).unwrap().is_same(&vertex));
    }
}
#[test]
fn transitions_and_parameters_are_independent_per_edge_and_mutably_borrowed() {
    let mut point = TopOpeBRepPoint2d::new();
    let forward = TopOpeBRepDSTransition::from_orientation(Orientation::Forward);
    let reversed = TopOpeBRepDSTransition::from_orientation(Orientation::Reversed);
    point.set_transition(1, &forward).unwrap();
    assert_eq!(*point.transition(1).unwrap(), forward);
    assert_eq!(
        *point.transition(2).unwrap(),
        TopOpeBRepDSTransition::default()
    );
    *point.change_transition(2).unwrap() = reversed;
    assert_eq!(*point.transition(2).unwrap(), reversed);
    point.set_parameter(1, -3.5).unwrap();
    point.set_parameter(2, f64::INFINITY).unwrap();
    assert_eq!(point.parameter(1).unwrap(), -3.5);
    assert_eq!(point.parameter(2).unwrap(), f64::INFINITY);
    assert!(!point.is_vertex(1).unwrap());
}
#[test]
fn indexed_errors_preserve_getter_setter_and_nested_vertex_literal_distinctions() {
    let mut point = TopOpeBRepPoint2d::new();
    for index in [-1, 0, 3] {
        failure(
            point.set_is_vertex(index, true),
            "TopOpeBRepPoint2d::SetIsVertex",
        );
        failure(point.is_vertex(index), "TopOpeBRepPoint2d::IsVertex");
        failure(
            point.set_vertex(index, &Vertex::default()),
            "TopOpeBRepPoint2d::SetVertex",
        );
        failure(point.vertex(index), "TopOpeBRepPoint2d::IsVertex");
        failure(
            point.set_transition(index, &TopOpeBRepDSTransition::default()),
            "TopOpeBRepPoint2d::SetTransition",
        );
        failure(point.transition(index), "TopOpeBRepPoint2d::Transition");
        failure(
            point.change_transition(index),
            "TopOpeBRepPoint2d::ChangeTransition",
        );
        failure(
            point.set_parameter(index, 1.),
            "TopOpeBRepPoint2d::SetParameter",
        );
        failure(point.parameter(index), "TopOpeBRepPoint2d::Parameter");
    }
}
#[test]
fn independent_flags_config_and_tolerance_do_not_impose_validation() {
    let mut point = TopOpeBRepPoint2d::new();
    point.set_index(-7);
    point.set_keep(false);
    point.set_is_point_of_segment(true);
    point.set_edges_config(TopOpeBRepDSConfig::DiffOriented);
    point.set_tolerance(-0.25);
    assert_eq!(point.index(), -7);
    assert!(!point.keep());
    assert!(point.is_point_of_segment());
    assert_eq!(point.edges_config(), TopOpeBRepDSConfig::DiffOriented);
    assert_eq!(point.tolerance(), -0.25);
    point.dump(1, 2);
}
#[test]
fn point_context_handles_retain_identity_and_lifetime_after_source_handles_drop() {
    let mut point = TopOpeBRepPoint2d::new();
    let ff = Handle::new(TopOpeBRepHctxff2d::new());
    let ee = Handle::new(TopOpeBRepHctxee2d::new());
    point.set_hctxff2d(&ff);
    point.set_hctxee2d(&ee);
    assert!(point.hctxff2d() == ff);
    assert!(point.hctxee2d() == ee);
    drop((ff, ee));
    assert!(!point.hctxff2d().is_null());
    assert!(!point.hctxee2d().is_null());
    point.set_hctxff2d(&Handle::default());
    point.set_hctxee2d(&Handle::default());
    assert!(point.hctxff2d().is_null() && point.hctxee2d().is_null());
}
#[test]
fn intersection_record_supports_send_and_sync_bounds() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepPoint2d>();
}
