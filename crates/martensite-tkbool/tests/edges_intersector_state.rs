//! Source-backed state and segment reduction for the edge intersection coordinator.
use martensite_tkbool::top_ope_brep::{
    TopOpeBRepEdgesIntersector, TopOpeBRepP2DStatus, TopOpeBRepPoint2d,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
#[test]
fn constructor_indices_and_dimension_follow_native_state() {
    let mut state = TopOpeBRepEdgesIntersector::new();
    assert_eq!(state.dimension(), 2);
    for rejected in [-3, 0, 3] {
        state.set_dimension(rejected);
        assert_eq!(state.dimension(), 2);
    }
    state.set_dimension(1);
    assert_eq!(state.dimension(), 1);
    assert_eq!(state.tolerances(), (0., 0.));
    assert!(state.is_empty());
    assert!(!state.has_segment());
    assert!(!state.same_domain());
    assert_eq!((state.nb_points(), state.nb_segments()), (0, 0));
    assert!(state.more_point());
    state.init_point(true);
    assert!(!state.more_point());
    state.next_point();
    assert!(state.point().is_err());
    assert!(state.point_at(1).is_err());
    for index in [-1, 0, 3] {
        assert!(state.edge(index).is_err());
        assert!(state.face(index).is_err());
        assert!(state.curve(index).is_err());
        assert!(state.surface(index).is_err());
    }
    assert!(state.edge(1).unwrap().is_null());
    assert!(state.face(2).unwrap().is_null());
    state.dump("empty", 1, 2);
    assert!(state.points().is_empty());
}
#[test]
fn forced_tolerances_preserve_native_nan_ordering() {
    let mut state = TopOpeBRepEdgesIntersector::new();
    state.force_tolerances(-1., 4.);
    assert_eq!(state.tolerance_max(), 4.);
    state.force_tolerances(f64::NAN, 2.);
    assert!(state.tolerance_max().is_nan());
    state.force_tolerances(2., f64::NAN);
    assert_eq!(state.tolerance_max(), 2.);
}
fn endpoint(
    status: TopOpeBRepP2DStatus,
    index: i32,
    value: f64,
) -> Result<TopOpeBRepPoint2d, Failure> {
    let mut point = TopOpeBRepPoint2d::new();
    point.set_status(status);
    point.set_index(index);
    point.set_is_point_of_segment(true);
    point.set_value(&Point3::new(value, value * 2., value * 3.));
    point.set_value2d(&Point2::new(value, value * 2.));
    point.set_parameter(1, value)?;
    point.set_parameter(2, value * 2.)?;
    point.set_tolerance(value / 100.);
    Ok(point)
}
#[test]
fn segment_reduction_preserves_ancestry_midpoint_and_endpoint_flags() -> Result<(), Failure> {
    let state = TopOpeBRepEdgesIntersector::new();
    let mut first = endpoint(TopOpeBRepP2DStatus::SegmentFirst, 3, 2.)?;
    let mut last = endpoint(TopOpeBRepP2DStatus::SegmentLast, 4, 6.)?;
    let mut midpoint = TopOpeBRepPoint2d::new();
    assert!(state.reduce_segment(&mut first, &mut last, &mut midpoint)?);
    assert_eq!(*midpoint.value(), Point3::new(4., 8., 12.));
    assert_eq!(*midpoint.value2d(), Point2::new(4., 8.));
    assert_eq!(midpoint.parameter(1)?, 4.);
    assert_eq!(midpoint.parameter(2)?, 8.);
    assert!((midpoint.tolerance() - 0.12).abs() < 1e-15);
    assert_eq!(midpoint.segment_ancestors(), (true, 3, 4));
    assert_eq!(midpoint.status(), TopOpeBRepP2DStatus::New);
    assert!(!first.keep());
    assert!(!last.keep());
    assert!(!midpoint.is_point_of_segment());
    last.set_status(TopOpeBRepP2DStatus::Intersection);
    assert!(!state.reduce_segment(&mut first, &mut last, &mut midpoint)?);
    Ok(())
}
#[test]
fn face_loading_preserves_forced_tolerances_and_oriented_normals() -> Result<(), Failure> {
    use martensite_tkg3d::top_abs::Orientation;
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let first =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))?.face()?;
    let mut second = first.clone();
    second.set_orientation(Orientation::Reversed);
    let mut state = TopOpeBRepEdgesIntersector::new();
    state.set_faces(&first, &second)?;
    assert!(state.surfaces_same_oriented());
    assert!(!state.faces_same_oriented());
    assert!(state.face(1)?.is_equal(&first));
    assert!(state.face(2)?.is_equal(&second));
    let (a, b) = state.tolerances();
    assert!(a <= 1e-4 && b <= 1e-4);
    state.force_tolerances(0.25, 0.5);
    state.set_faces(&first, &first)?;
    assert_eq!(state.tolerances(), (0.25, 0.5));
    assert!(state.faces_same_oriented());
    Ok(())
}
#[test]
fn segment_reduction_composes_transitions_and_prefers_last_endpoint_vertex() -> Result<(), Failure>
{
    use martensite_tkbool::top_ope_brep_ds::TopOpeBRepDSTransition;
    use martensite_tkg3d::top_abs::{ShapeEnum, State};
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let state = TopOpeBRepEdgesIntersector::new();
    let mut first = endpoint(TopOpeBRepP2DStatus::SegmentFirst, 1, 0.)?;
    let mut last = endpoint(TopOpeBRepP2DStatus::SegmentLast, 2, 2.)?;
    let (a, b) = (
        BRepBuilderAPIMakeVertex::new(Point3::default())?.vertex()?,
        BRepBuilderAPIMakeVertex::new(Point3::new(2., 0., 0.))?.vertex()?,
    );
    first.set_vertex(1, &a)?;
    last.set_vertex(1, &b)?;
    let mut transition = TopOpeBRepDSTransition::default();
    transition.set_before(State::In, ShapeEnum::Face);
    first.set_transition(1, &transition)?;
    transition.set_after(State::Out, ShapeEnum::Edge);
    last.set_transition(1, &transition)?;
    let mut result = TopOpeBRepPoint2d::new();
    assert!(state.reduce_segment(&mut first, &mut last, &mut result)?);
    assert!(result.vertex(1)?.is_same(&b));
    assert!(!result.is_vertex(2)?);
    let transition = result.transition(1)?;
    assert_eq!(
        (transition.before(), transition.after()),
        (State::In, State::Out)
    );
    assert_eq!(
        (transition.shape_before(), transition.shape_after()),
        (ShapeEnum::Face, ShapeEnum::Edge)
    );
    Ok(())
}
