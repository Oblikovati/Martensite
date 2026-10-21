//! Curve-surface initialization and analytic snapshots retain calculated state.
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve, GeomLine};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
use martensite_tktopalgo::brep_extrema::BRepExtremaExtCF;

fn planar_face() -> Result<Face, Failure> {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-10., 10., -10., 10.))?.face()
}

fn parabolic_edge() -> Result<Edge, Failure> {
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    poles[1] = Point3::new(-1., 0., 4.);
    poles[2] = Point3::new(0., 0., 2.);
    poles[3] = Point3::new(1., 0., 4.);
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomBezierCurve::new(&poles)?).cast();
    BRepBuilderAPIMakeEdge::from_curve_params(&curve, 0., 1.)?.edge()
}

fn parallel_line_edge() -> Result<(Handle<GeomLine>, Edge), Failure> {
    let line = Handle::new(GeomLine::from_point(
        Point3::new(0., 0., 3.),
        Direction3::new(1., 0., 0.)?,
    ));
    let curve: Handle<dyn GeomCurve> = line.clone().cast();
    let first = BRepBuilderAPIMakeEdge::from_curve_params(&curve, -1., 1.)?.edge()?;
    Ok((line, first))
}

#[test]
fn analytic_snapshot_keeps_parallel_history_before_generic_perform() -> Result<(), Failure> {
    let (line, first) = parallel_line_edge()?;
    let face = planar_face()?;
    let mut extrema = BRepExtremaExtCF::with_edge_face(&first, &face)?;
    assert!(extrema.is_parallel()?);
    assert_eq!(extrema.square_distance(1)?, 9.);
    line.set_direction(Direction3::new(0., 0., 1.)?);
    extrema.perform(&parabolic_edge()?, &face)?;
    assert!(extrema.is_done());
    assert!(extrema.is_parallel()?);
    assert_eq!(extrema.nb_ext(), 1);
    assert!((extrema.square_distance(1)? - 9.).abs() <= 1e-10);
    Ok(())
}

fn closable_bezier() -> Result<Handle<GeomBezierCurve>, Failure> {
    let mut poles = Array1::with_bounds(1, 4).unwrap();
    poles[1] = Point3::new(-1., 0., 4.);
    poles[2] = Point3::new(-0.5, 1., 2.);
    poles[3] = Point3::new(0.5, 1., 2.);
    poles[4] = Point3::new(1., 0., 4.);
    Ok(Handle::new(GeomBezierCurve::new(&poles)?))
}

#[test]
fn initialize_retains_completion_after_prior_bezier_becomes_closed() -> Result<(), Failure> {
    let bezier = closable_bezier()?;
    let curve: Handle<dyn GeomCurve> = bezier.clone().cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve_params(&curve, 0., 1.)?.edge()?;
    let face = planar_face()?;
    let mut extrema = BRepExtremaExtCF::with_edge_face(&edge, &face)?;
    assert!(extrema.is_done());
    assert_eq!(extrema.nb_ext(), 1);
    assert!((extrema.square_distance(1)? - 6.25).abs() <= 1e-10);
    let previous_point = extrema.point_on_edge(1)?;
    bezier.set_pole(4, Point3::new(-1., 0., 4.))?;
    extrema.initialize(&edge, &face)?;
    assert!(extrema.is_done());
    assert!(!extrema.is_parallel()?);
    assert_eq!(extrema.nb_ext(), 1);
    assert!((extrema.square_distance(1)? - 6.25).abs() <= 1e-10);
    assert_eq!(extrema.point_on_edge(1)?, previous_point);
    Ok(())
}
