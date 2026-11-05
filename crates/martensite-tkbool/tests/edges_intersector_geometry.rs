//! Analytic same-plane intersections cover the full edge coordinator pipeline.
use martensite_tkbool::top_ope_brep::{TopOpeBRepEdgesIntersector, TopOpeBRepP2DStatus};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
fn face() -> Result<Face, Failure> {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-5., 5., -5., 5.))?.face()
}
fn edge(a: (f64, f64), b: (f64, f64)) -> Result<Edge, Failure> {
    BRepBuilderAPIMakeEdge::from_points(Point3::new(a.0, a.1, 0.), Point3::new(b.0, b.1, 0.))?
        .edge()
}
#[test]
fn crossing_lines_produce_one_isolated_point_and_exhausted_public_cursor() -> Result<(), Failure> {
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    state.perform(
        &edge((-1., 0.), (1., 0.))?,
        &edge((0., -1.), (0., 1.))?,
        true,
    )?;
    assert_eq!((state.nb_points(), state.nb_segments()), (1, 0));
    assert!(!state.more_point());
    assert!(!state.is_empty());
    assert!(!state.same_domain());
    let point = state.point_at(1)?;
    assert!(point.value().distance(&Point3::default()) < 1e-12);
    assert_eq!(point.status(), TopOpeBRepP2DStatus::Intersection);
    assert!(!point.is_vertex(1)? && !point.is_vertex(2)?);
    state.init_point(true);
    assert!(state.more_point());
    state.next_point();
    assert!(!state.more_point());
    Ok(())
}
#[test]
fn overlapping_lines_keep_two_segment_endpoints_and_same_domain() -> Result<(), Failure> {
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    state.perform(
        &edge((-1., 0.), (1., 0.))?,
        &edge((0., 0.), (2., 0.))?,
        true,
    )?;
    assert_eq!((state.nb_points(), state.nb_segments()), (0, 1));
    assert!(state.same_domain());
    assert!(state.has_segment());
    assert_eq!(state.points().len(), 2);
    assert_eq!(
        state.point_at(1)?.status(),
        TopOpeBRepP2DStatus::SegmentFirst
    );
    assert_eq!(
        state.point_at(2)?.status(),
        TopOpeBRepP2DStatus::SegmentLast
    );
    assert!(state.point_at(1)?.value().distance(&Point3::default()) < 1e-12);
    assert!(
        state
            .point_at(2)?
            .value()
            .distance(&Point3::new(1., 0., 0.))
            < 1e-12
    );
    Ok(())
}
#[test]
fn reused_coordinator_replaces_prior_results_with_empty_disjoint_intersection()
-> Result<(), Failure> {
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    let a = edge((-1., 0.), (1., 0.))?;
    state.perform(&a, &a, false)?;
    assert!(state.has_segment());
    state.perform(&a, &edge((-1., 2.), (1., 2.))?, false)?;
    assert!(state.is_empty());
    assert!(!state.has_segment());
    assert!(!state.same_domain());
    assert_eq!((state.nb_points(), state.nb_segments()), (0, 0));
    Ok(())
}
#[test]
fn native_crossing_and_shared_endpoint_transition_baselines_match() -> Result<(), Failure> {
    use martensite_tkg3d::top_abs::{ShapeEnum, State};
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    state.perform(
        &edge((-1., 0.), (1., 0.))?,
        &edge((0., -1.), (0., 1.))?,
        true,
    )?;
    let p = state.point_at(1)?;
    assert_eq!(
        (p.transition(1)?.before(), p.transition(1)?.after()),
        (State::In, State::Out)
    );
    assert_eq!(
        (p.transition(2)?.before(), p.transition(2)?.after()),
        (State::Out, State::In)
    );
    assert_eq!(p.transition(1)?.shape_before(), ShapeEnum::Face);
    assert_eq!(state.status1(), TopOpeBRepP2DStatus::SegmentFirst);
    state.perform(
        &edge((-1., 0.), (1., 0.))?,
        &edge((1., 0.), (2., 0.))?,
        true,
    )?;
    let p = state.point_at(1)?;
    assert!(p.is_vertex(1)? && p.is_vertex(2)?);
    assert_eq!(
        (p.transition(1)?.before(), p.transition(1)?.after()),
        (State::Out, State::In)
    );
    assert_eq!(
        (p.transition(2)?.before(), p.transition(2)?.after()),
        (State::In, State::Out)
    );
    assert_eq!(p.transition(1)?.shape_before(), ShapeEnum::Edge);
    Ok(())
}
#[test]
fn same_domain_native_transitions_are_independent_of_crossed_edge_orientation()
-> Result<(), Failure> {
    use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
    for reverse in [false, true] {
        let mut state = TopOpeBRepEdgesIntersector::new();
        let face = face()?;
        state.set_faces(&face, &face)?;
        let mut second = edge((0., 0.), (2., 0.))?;
        if reverse {
            second.set_orientation(Orientation::Reversed);
        }
        state.perform(&edge((-1., 0.), (1., 0.))?, &second, true)?;
        let (first, last) = (state.point_at(1)?, state.point_at(2)?);
        assert_eq!(
            (first.transition(1)?.before(), first.transition(1)?.after()),
            (State::Out, State::In)
        );
        assert_eq!(
            (first.transition(2)?.before(), first.transition(2)?.after()),
            (State::In, State::In)
        );
        assert_eq!(
            (last.transition(1)?.before(), last.transition(1)?.after()),
            (State::In, State::In)
        );
        assert_eq!(
            (last.transition(2)?.before(), last.transition(2)?.after()),
            (State::In, State::Out)
        );
        assert_eq!(last.transition(2)?.shape_before(), ShapeEnum::Edge);
    }
    Ok(())
}
#[test]
fn overlapping_circle_arcs_are_same_domain_and_keep_arc_endpoints() -> Result<(), Failure> {
    use martensite_tkmath::gp::{Axis2, Circle3};
    let circle = Circle3::new(Axis2::default(), 2.)?;
    let first = BRepBuilderAPIMakeEdge::from_circ_params(circle, 0., 2.)?.edge()?;
    let second = BRepBuilderAPIMakeEdge::from_circ_params(circle, 1., 3.)?.edge()?;
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    state.perform(&first, &second, true)?;
    assert!(state.same_domain());
    assert_eq!((state.nb_points(), state.nb_segments()), (0, 1));
    assert_eq!(state.points().len(), 2);
    assert!((state.point_at(1)?.parameter(1)? - 1.).abs() < 1e-12);
    assert!((state.point_at(2)?.parameter(1)? - 2.).abs() < 1e-12);
    Ok(())
}
#[test]
fn spherical_face_meridian_self_overlap_preserves_segment_geometry() -> Result<(), Failure> {
    use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    let shape = BRepPrimAPIMakeSphere::new(2.)?.shape()?.clone();
    let face = topods::face(Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)?.current())?;
    let edges = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let meridian = edges
        .filter_map(|s| {
            let edge = topods::edge(&s).ok()?;
            (!BRepTool::degenerated(&edge).ok()?).then_some(edge)
        })
        .next()
        .expect("sphere meridian");
    let mut state = TopOpeBRepEdgesIntersector::new();
    state.set_faces(&face, &face)?;
    state.perform(&meridian, &meridian, false)?;
    assert_eq!(state.nb_segments(), 1);
    assert!(state.same_domain());
    for point in state.points() {
        assert!((point.value().distance(&Point3::default()) - 2.).abs() < 1e-7);
    }
    Ok(())
}
#[test]
fn working_dimension_and_internal_crossed_edge_control_transition_shapes() -> Result<(), Failure> {
    use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
    let mut state = TopOpeBRepEdgesIntersector::new();
    let face = face()?;
    state.set_faces(&face, &face)?;
    let first = edge((-1., 0.), (1., 0.))?;
    let mut second = edge((0., -1.), (0., 1.))?;
    state.set_dimension(1);
    state.perform(&first, &second, true)?;
    assert_eq!(
        state.point_at(1)?.transition(1)?.shape_before(),
        ShapeEnum::Edge
    );
    second.set_orientation(Orientation::Internal);
    state.set_dimension(2);
    state.perform(&first, &second, true)?;
    let transition = state.point_at(1)?.transition(1)?;
    assert_eq!(
        (transition.before(), transition.after()),
        (State::In, State::In)
    );
    assert_eq!(transition.shape_before(), ShapeEnum::Face);
    Ok(())
}
