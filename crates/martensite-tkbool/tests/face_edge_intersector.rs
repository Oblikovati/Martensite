//! Closed-form face/edge intersections and preserved legacy traversal details.
use martensite_tkbool::top_ope_brep::TopOpeBRepFaceEdgeIntersector;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{Edge, Face, Vertex},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine},
    top_abs::{Orientation, ShapeEnum, State},
};
use martensite_tkmath::gp::{Direction3, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

fn face() -> Result<Face, Failure> {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))?.face()
}
fn edge(x: f64) -> Result<Edge, Failure> {
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::new(x, 0., 0.),
        Direction3::new(0., 0., 1.)?,
    ))
    .cast();
    BRepBuilderAPIMakeEdge::from_curve_params(&curve, -1., 1.)?.edge()
}

#[test]
fn forward_normal_crossing_has_exact_point_parameters_states_and_transitions() -> Result<(), Failure>
{
    let mut inter = TopOpeBRepFaceEdgeIntersector::new();
    let mut f = face()?;
    let mut e = edge(0.)?;
    f.set_orientation(Orientation::Reversed);
    e.set_orientation(Orientation::Reversed);
    inter.perform(f.as_ref(), e.as_ref())?;
    inter.init_point();
    assert_crossing_geometry(&inter)?;
    assert_crossing_transitions(&inter)?;
    assert_face_transitions(&inter)?;
    assert_eq!(inter.index(), 0);
    assert!(inter.transition(3, Orientation::Forward).is_err());
    inter.next_point();
    assert!(!inter.more_point());
    assert!(inter.value().is_err());
    Ok(())
}

#[test]
fn boundary_is_on_outside_is_empty_and_forced_tolerance_is_overwritten() -> Result<(), Failure> {
    let mut inter = TopOpeBRepFaceEdgeIntersector::new();
    inter.force_tolerance(0.25);
    assert_eq!(inter.tolerance(), 0.25);
    let edge = edge(2.)?;
    BRepBuilder::default().update_edge_tolerance(&edge, 2e-6)?;
    inter.perform(face()?.as_ref(), edge.as_ref())?;
    inter.init_point();
    assert_eq!(inter.state()?, State::On);
    assert_eq!(inter.tolerance(), 2e-6);
    inter.perform(face()?.as_ref(), crate::edge(3.)?.as_ref())?;
    inter.init_point();
    assert!(inter.is_empty());
    assert!(!inter.more_point());
    assert!(inter.shape(0).is_err());
    Ok(())
}

#[test]
fn vertex_search_retains_earlier_match_but_returns_last_visit_status() -> Result<(), Failure> {
    let inter = TopOpeBRepFaceEdgeIntersector::new();
    let edge = edge(0.)?;
    let vertices: Vec<_> =
        Explorer::new(edge.as_ref(), ShapeEnum::Vertex, ShapeEnum::Shape)?.collect();
    let first = martensite_tkbrep::topods::vertex(&vertices[0])?;
    let point = BRepTool::pnt(&first)?;
    let mut output = Vertex::default();
    assert!(!inter.is_vertex_on(edge.as_ref(), point, 1e-7, &mut output)?);
    assert!(output.is_same(&first));
    assert!(!inter.is_vertex_on(edge.as_ref(), point, 0., &mut output)?);
    assert!(output.is_null());
    Ok(())
}

#[test]
fn initialization_invalid_cursor_and_nonmatching_vertex_leave_checked_outputs()
-> Result<(), Failure> {
    let mut inter = TopOpeBRepFaceEdgeIntersector::default();
    assert!(inter.is_empty());
    assert_eq!(inter.nb_points(), 0);
    assert!(inter.value().is_err());
    inter.init_point();
    assert!(!inter.more_point());
    inter.perform(face()?.as_ref(), edge(0.)?.as_ref())?;
    inter.init_point();
    let mut output = Vertex::default();
    assert!(!inter.is_vertex(2, &mut output)?);
    assert!(output.is_null());
    assert!(!inter.is_vertex(1, &mut output)?);
    assert!(!inter.is_vertex(9, &mut output)?);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<TopOpeBRepFaceEdgeIntersector>();
    Ok(())
}

fn assert_crossing_geometry(inter: &TopOpeBRepFaceEdgeIntersector) -> Result<(), Failure> {
    assert_eq!(inter.nb_points(), 1);
    assert!(!inter.is_empty());
    assert!(inter.more_point());
    assert_eq!(inter.shape(1)?.orientation(), Orientation::Forward);
    assert_eq!(inter.shape(2)?.orientation(), Orientation::Forward);
    assert_eq!(inter.value()?, Point3::default());
    assert_eq!(inter.parameter()?, 0.);
    assert_eq!(inter.uv_point()?, Point2::default());
    assert_eq!(inter.state()?, State::In);
    Ok(())
}
fn assert_crossing_transitions(inter: &TopOpeBRepFaceEdgeIntersector) -> Result<(), Failure> {
    let forward = inter.transition(2, Orientation::Forward)?;
    assert_eq!((forward.before(), forward.after()), (State::In, State::Out));
    let reversed = inter.transition(2, Orientation::Reversed)?;
    assert_eq!(
        (reversed.before(), reversed.after()),
        (State::Out, State::In)
    );
    assert_eq!(
        inter.transition(2, Orientation::Internal)?.after(),
        State::In
    );
    assert_eq!(
        inter.transition(2, Orientation::External)?.before(),
        State::Out
    );
    Ok(())
}
fn assert_face_transitions(inter: &TopOpeBRepFaceEdgeIntersector) -> Result<(), Failure> {
    for orientation in [
        Orientation::Forward,
        Orientation::Reversed,
        Orientation::Internal,
        Orientation::External,
    ] {
        let transition = inter.transition(1, orientation)?;
        assert_eq!(
            (transition.before(), transition.after()),
            (State::In, State::In)
        );
    }
    Ok(())
}
