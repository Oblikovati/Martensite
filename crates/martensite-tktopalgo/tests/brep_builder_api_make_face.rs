//! Tests of `BRepBuilderAPI_MakeFace`.
#![allow(non_snake_case)]

use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomCylindricalSurface, GeomPlane, GeomRectangularTrimmedSurface, GeomSphericalSurface,
    GeomSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis3, Cone3, Cylinder3, Direction3, Plane3, Point3, Sphere3, Torus3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::f64::consts::PI;

/// `BRepCheck_Analyzer(aFace).IsValid()` (default flags).
fn is_valid(face: &Face) -> bool {
    let shape: Shape = face.clone().into();
    BRepCheckAnalyzer::new(&shape, true, false, false)
        .unwrap()
        .is_valid()
        .unwrap()
}

/// `BRepGProp::SurfaceProperties(aFace, aProps)` (default flags), then `Mass()`.
fn face_area(face: &Face) -> f64 {
    let mut a_props = GProps::new();
    BRepGProp::surface_properties(face, &mut a_props, false, false).unwrap();
    a_props.mass()
}

/// `BRepBuilderAPI_MakeEdge(P1, P2).Edge()`.
fn make_edge(p1: Point3, p2: Point3) -> martensite_tkbrep::topods::Edge {
    BRepBuilderAPIMakeEdge::from_points(p1, p2)
        .unwrap()
        .edge()
        .unwrap()
}

fn z_dir() -> Direction3 {
    Direction3::new(0.0, 0.0, 1.0).unwrap()
}

#[test]
fn BRepBuilderAPI_MakeFaceTest__FaceFromPlane() {
    let a_pln = Plane3::new(Point3::new(0.0, 0.0, 0.0), z_dir()).unwrap();
    let a_make_face = BRepBuilderAPIMakeFace::from_plane(a_pln).unwrap();
    assert!(a_make_face.is_done(), "Face from plane creation failed");

    let a_face = a_make_face.face().unwrap();
    assert!(!a_face.is_null(), "Resulting face is null");
}

#[test]
fn BRepBuilderAPI_MakeFaceTest__FaceFromWire() {
    // Create a rectangular wire in XY plane
    let a_p1 = Point3::new(0.0, 0.0, 0.0);
    let a_p2 = Point3::new(10.0, 0.0, 0.0);
    let a_p3 = Point3::new(10.0, 5.0, 0.0);
    let a_p4 = Point3::new(0.0, 5.0, 0.0);

    let an_e1 = make_edge(a_p1, a_p2);
    let an_e2 = make_edge(a_p2, a_p3);
    let an_e3 = make_edge(a_p3, a_p4);
    let an_e4 = make_edge(a_p4, a_p1);

    let mut a_make_wire = BRepBuilderAPIMakeWire::new();
    a_make_wire.add_edge(&an_e1).unwrap();
    a_make_wire.add_edge(&an_e2).unwrap();
    a_make_wire.add_edge(&an_e3).unwrap();
    a_make_wire.add_edge(&an_e4).unwrap();
    assert!(a_make_wire.is_done(), "Wire creation failed");

    let a_make_face =
        BRepBuilderAPIMakeFace::from_wire(&a_make_wire.wire().unwrap(), false).unwrap();
    assert!(a_make_face.is_done(), "Face from wire creation failed");

    let a_face = a_make_face.face().unwrap();
    assert!(!a_face.is_null());

    assert!(is_valid(&a_face), "Face from wire is not valid");

    // Area should be 10 * 5 = 50
    let mass = face_area(&a_face);
    assert!((mass - 50.0).abs() <= precision::confusion());
}

#[test]
fn BRepBuilderAPI_MakeFaceTest__FaceFromGeomPlane_WithBounds() {
    let a_geom_plane: Handle<dyn GeomSurface> =
        Handle::new(GeomPlane::from_point(Point3::new(0.0, 0.0, 0.0), z_dir()).unwrap()).cast();
    let a_make_face = BRepBuilderAPIMakeFace::from_surface_bounds(
        &a_geom_plane,
        (0.0, 10.0, 0.0, 5.0),
        precision::confusion(),
    )
    .unwrap();
    assert!(
        a_make_face.is_done(),
        "Face from Geom_Plane with bounds failed"
    );

    let a_face = a_make_face.face().unwrap();
    assert!(!a_face.is_null());

    assert!(is_valid(&a_face), "Bounded face is not valid");

    let mass = face_area(&a_face);
    assert!((mass - 50.0).abs() <= precision::confusion());
}

#[test]
fn BRepBuilderAPI_MakeFaceTest__FaceFromCylindricalSurface() {
    let an_axis = Axis3::from_normal(Point3::new(0.0, 0.0, 0.0), z_dir()).unwrap();
    let a_cyl_surf: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(an_axis, 5.0).unwrap()).cast();

    // Create a bounded face: U from 0 to 2*PI, V from 0 to 10
    let a_make_face = BRepBuilderAPIMakeFace::from_surface_bounds(
        &a_cyl_surf,
        (0.0, 2.0 * PI, 0.0, 10.0),
        precision::confusion(),
    )
    .unwrap();
    assert!(
        a_make_face.is_done(),
        "Face from cylindrical surface failed"
    );

    let a_face = a_make_face.face().unwrap();
    assert!(!a_face.is_null());

    assert!(is_valid(&a_face), "Cylindrical face is not valid");

    // Area of cylinder: 2*PI*R*H = 2*PI*5*10 = 100*PI
    let mass = face_area(&a_face);
    assert!((mass - 2.0 * PI * 5.0 * 10.0).abs() <= 0.01);
}

fn unique_shape_count(face: &Face, kind: ShapeEnum) -> usize {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&face.clone().into_shape(), kind, &mut shapes).unwrap();
    shapes.extent()
}

fn assert_full_period_face(face: Face, expected_edges: usize, expected_vertices: usize) {
    assert!(is_valid(&face));
    assert_eq!(unique_shape_count(&face, ShapeEnum::Edge), expected_edges);
    assert_eq!(
        unique_shape_count(&face, ShapeEnum::Vertex),
        expected_vertices
    );
}

#[test]
fn shifted_full_period_bounds_share_periodic_seams() {
    let axis = Axis3::from_normal(Point3::new(0.0, 0.0, 0.0), z_dir()).unwrap();
    let cylinder = Cylinder3::new(axis, 10.0).unwrap();
    let cone = Cone3::new(axis, 30.0_f64.to_radians(), 0.0).unwrap();
    let sphere = Sphere3::new(axis, 10.0).unwrap();
    let torus = Torus3::new(axis, 20.0, 5.0).unwrap();

    let cylinder =
        BRepBuilderAPIMakeFace::from_cylinder_bounds(cylinder, (PI, 3.0 * PI, -10.0, 10.0))
            .unwrap()
            .face()
            .unwrap();
    assert_full_period_face(cylinder, 3, 2);

    let sphere_basis: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::from_sphere(sphere)).cast();
    let trimmed_sphere: Handle<dyn GeomSurface> = Handle::new(
        GeomRectangularTrimmedSurface::new(
            &sphere_basis,
            (PI, 3.0 * PI),
            (-0.5 * PI, 0.5 * PI),
            (true, true),
        )
        .unwrap(),
    )
    .cast();
    let sphere = BRepBuilderAPIMakeFace::from_surface(&trimmed_sphere, precision::confusion())
        .unwrap()
        .face()
        .unwrap();
    assert_full_period_face(sphere, 3, 2);

    let cone = BRepBuilderAPIMakeFace::from_cone_bounds(cone, (PI, 3.0 * PI, 0.0, 20.0))
        .unwrap()
        .face()
        .unwrap();
    assert_full_period_face(cone, 3, 2);

    let torus = BRepBuilderAPIMakeFace::from_torus_bounds(torus, (PI, 3.0 * PI, 0.0, 2.0 * PI))
        .unwrap()
        .face()
        .unwrap();
    assert_full_period_face(torus, 2, 1);
}
