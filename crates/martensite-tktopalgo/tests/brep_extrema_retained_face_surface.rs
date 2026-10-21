//! Surface initialization retains calculated results; the next operation uses the new plane.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tktopalgo::brep_extrema::BRepExtremaExtFF;

fn horizontal_face(z: f64) -> Result<Face, Failure> {
    let plane = Plane3::new(Point3::new(0., 0., z), Direction3::new(0., 0., 1.)?)?;
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-10., 10., -10., 10.))?.face()
}

#[test]
fn reinitialization_keeps_prior_distance_until_the_next_perform() -> Result<(), Failure> {
    let (first, second) = (horizontal_face(0.)?, horizontal_face(3.)?);
    let mut extrema = BRepExtremaExtFF::with_faces(&first, &second)?;
    assert!(extrema.is_parallel()?);
    assert_eq!((extrema.nb_ext(), extrema.square_distance(1)?), (1, 9.));
    let plane: Handle<GeomPlane> = BRepTool::surface(&second)?.cast();
    plane.set_pln(Plane3::new(
        Point3::new(0., 0., 5.),
        Direction3::new(0., 0., 1.)?,
    )?);
    extrema.initialize(&second)?;
    assert!(extrema.is_done());
    assert!(!extrema.is_parallel()?);
    assert_eq!((extrema.nb_ext(), extrema.square_distance(1)?), (1, 9.));
    extrema.perform(&first, &second)?;
    assert!(extrema.is_done());
    assert!(extrema.is_parallel()?);
    assert_eq!((extrema.nb_ext(), extrema.square_distance(1)?), (1, 25.));
    Ok(())
}
