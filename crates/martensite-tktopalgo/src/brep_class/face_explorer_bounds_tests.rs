use super::*;
use crate::brep_lib::BRepLibMakeFace;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomRectangularTrimmedSurface;
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3, Sphere3};
use martensite_tkmath::toploc::Location;

fn bounded_plane(origin: Point3, normal: Direction3) -> Face {
    let plane = Plane3::new(origin, normal).unwrap();
    BRepLibMakeFace::from_plane_bounds(plane, (-2., 3., -4., 5.))
        .unwrap()
        .face()
        .unwrap()
}

fn default_bounded_plane() -> Face {
    bounded_plane(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
}

fn boxed_explorer(face: &Face) -> BRepClassFaceExplorer {
    let mut explorer = BRepClassFaceExplorer::new(face).unwrap();
    explorer.set_use_bnd_box(true);
    explorer
}

fn published_bounds(explorer: &BRepClassFaceExplorer) -> (f64, f64, f64, f64) {
    (
        explorer.my_u_min,
        explorer.my_u_max,
        explorer.my_v_min,
        explorer.my_v_max,
    )
}

#[test]
fn operation_bounds_match_closed_forms_in_transformed_planes() {
    for normal in [
        Direction3::new(0., 0., 1.).unwrap(),
        Direction3::new(1., 2., 3.).unwrap(),
    ] {
        verify_planar_bounds_and_duplicates(&bounded_plane(Point3::new(3., -4., 5.), normal));
    }
}

fn verify_planar_bounds_and_duplicates(face: &Face) {
    let mut explorer = BRepClassFaceExplorer::new(face).unwrap();
    assert!(explorer.ready_planar_operation_bounds().is_none());
    explorer.set_use_bnd_box(true);
    explorer.reuse_planar_operation_bounds();
    assert_eq!(published_bounds(&explorer), (-2., 3., -4., 5.));
    assert_eq!(
        published_bounds(&explorer),
        brep_tools::uv_bounds(face).unwrap()
    );
    explorer.my_probe_edges.push(explorer.my_probe_edges[0]);
    explorer.reuse_planar_operation_bounds();
    assert_eq!(published_bounds(&explorer), (-2., 3., -4., 5.));
}

#[test]
fn unavailable_boxes_retain_general_bounds() {
    let mut explorer = boxed_explorer(&default_bounded_plane());
    explorer.my_edges[0].set_bounding_box_unavailable();
    assert!(explorer.ready_planar_operation_bounds().is_none());
}

#[test]
fn nonplanar_operations_retain_general_bounds() {
    let sphere = Sphere3::new(Axis3::default(), 2.).unwrap();
    let face = BRepLibMakeFace::from_sphere_bounds(sphere, (0., 1., -0.5, 0.5))
        .unwrap()
        .face()
        .unwrap();
    assert!(
        boxed_explorer(&face)
            .ready_planar_operation_bounds()
            .is_none()
    );
}

#[test]
fn null_faces_do_not_publish_void_operation_bounds() {
    assert!(
        boxed_explorer(&Face::default())
            .ready_planar_operation_bounds()
            .is_none()
    );
}

#[test]
fn flattened_boxes_preserve_individual_gap_coordinates() {
    let mut result = BoundingBox2::default();
    let natural = (-10., 10., -10., 10.);
    let first = expanded_box([-1., -2., 3., 4.], 2.);
    let second = expanded_box([0., -1., 6., 7.], 1.);
    append_natural_plane_box(&mut result, first, natural).unwrap();
    append_natural_plane_box(&mut result, second, natural).unwrap();
    assert_eq!(result.get().unwrap(), [-3., -4., 7., 8.]);
}

#[test]
fn nonfinite_and_out_of_domain_boxes_do_not_modify_aggregate_bounds() {
    let mut result = BoundingBox2::default();
    let natural = (-10., 10., -10., 10.);
    append_natural_plane_box(&mut result, [-3., -4., 7., 8.], natural).unwrap();
    for coordinates in [
        [f64::NAN, 0., 1., 1.],
        [-11., 0., 1., 1.],
        [0., 0., f64::INFINITY, 1.],
    ] {
        assert!(append_natural_plane_box(&mut result, coordinates, natural).is_none());
        assert_eq!(result.get().unwrap(), [-3., -4., 7., 8.]);
    }
}

#[test]
fn rectangular_trimmed_planes_keep_the_general_clamping_path() {
    let face = default_bounded_plane();
    let surface = BRepTool::surface(&face).unwrap();
    let trimmed =
        GeomRectangularTrimmedSurface::new(&surface, (-1., 1.), (-1., 1.), (true, true)).unwrap();
    let trimmed = Handle::new(trimmed).cast();
    BRepBuilder::default()
        .update_face(&face, &trimmed, &Location::default(), 1e-7)
        .unwrap();
    assert!(
        boxed_explorer(&face)
            .ready_planar_operation_bounds()
            .is_none()
    );
}

fn expanded_box(coordinates: [f64; 4], gap: f64) -> [f64; 4] {
    let mut bounds = BoundingBox2::default();
    bounds.update_bounds(
        [coordinates[0], coordinates[1]],
        [coordinates[2], coordinates[3]],
    );
    bounds.set_gap(gap);
    bounds.get().unwrap()
}

#[test]
fn empty_planar_faces_do_not_publish_void_operation_bounds() {
    let face = BRepLibMakeFace::from_plane(Plane3::default())
        .unwrap()
        .face()
        .unwrap();
    assert!(
        boxed_explorer(&face)
            .ready_planar_operation_bounds()
            .is_none()
    );
}
