//! Exact geometry bounds distinguish location and subshape tolerance policy without meshes.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis1, Direction3, Plane3, Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

fn located_rectangle() -> Result<Shape, Failure> {
    let face: Face =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 1.))?.face()?;
    let builder = BRepBuilder::default();
    builder.update_face_tolerance(&face, 0.25)?;
    for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)? {
        builder.update_edge_tolerance(&topods::edge(&edge)?, 0.0625)?;
    }
    face.moved(&rectangle_location()?, false)
}

fn rectangle_location() -> Result<Location, Failure> {
    let mut transform = Transform3::default();
    transform.set_rotation(
        Axis1::new(Point3::new(0., 0., 0.), Direction3::Z),
        martensite_cmath::atan(0.75_f64),
    )?;
    transform.set_translation_part(Vector3::new(4., -3., 2.));
    Ok(Location::new(transform))
}

fn assert_rectangle_bounds(bounds: &BoundingBox3, gap: f64) -> Result<(), Failure> {
    let exact = [3.4, -3., 2., 5.6, -1., 2.];
    for (index, (value, base)) in bounds.get()?.into_iter().zip(exact).enumerate() {
        let expected = if index < 3 { base - gap } else { base + gap };
        assert!(
            (value - expected).abs() <= 1e-12,
            "bound {index}: {value}, expected {expected}"
        );
    }
    Ok(())
}

#[test]
fn add_uses_face_tolerance_after_bounding_located_plane_edges() -> Result<(), Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&located_rectangle()?, &mut bounds, false)?;
    assert_rectangle_bounds(&bounds, 0.25)
}

#[test]
fn add_close_preserves_exact_located_rectangle_extents() -> Result<(), Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add_close(&located_rectangle()?, &mut bounds)?;
    assert_rectangle_bounds(&bounds, 0.)
}

#[test]
fn optimal_plane_edges_use_edge_tolerance_without_face_enlargement() -> Result<(), Failure> {
    let shape = located_rectangle()?;
    for (use_tolerance, expected_gap) in [(false, 0.), (true, 0.0625)] {
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add_optimal(&shape, &mut bounds, false, use_tolerance)?;
        assert_rectangle_bounds(&bounds, expected_gap)?;
    }
    Ok(())
}
