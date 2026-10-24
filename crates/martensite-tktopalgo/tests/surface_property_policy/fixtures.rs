use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Face, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomEllipse, GeomSurface, GeomSurfaceOfLinearExtrusion};
use martensite_tkmath::{
    gp::{Axis2, Cylinder3, Direction3, Plane3, Point3},
    poly::{Triangle, Triangulation},
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};
use std::f64::consts::{FRAC_PI_2, TAU};

pub(super) fn cylinder_face() -> Face {
    BRepBuilderAPIMakeFace::from_cylinder_bounds(
        Cylinder3::new(Default::default(), 1.0).unwrap(),
        (0.0, FRAC_PI_2, 0.0, 2.0),
    )
    .unwrap()
    .face()
    .unwrap()
}

pub(super) fn cylinder_mesh() -> Handle<Triangulation> {
    let mesh = Handle::new(Triangulation::with_sizes(4, 2, false, false).unwrap());
    for (i, p) in [
        (1, Point3::new(1., 0., 0.)),
        (2, Point3::new(0., 1., 0.)),
        (3, Point3::new(0., 1., 2.)),
        (4, Point3::new(1., 0., 2.)),
    ] {
        mesh.set_node(i, p).unwrap();
    }
    mesh.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    mesh.set_triangle(2, Triangle::new(1, 3, 4)).unwrap();
    mesh
}

pub(super) fn meshed_cylinder() -> (Face, Handle<Triangulation>) {
    let face = cylinder_face();
    let mesh = cylinder_mesh();
    BRepBuilder::default()
        .update_face_triangulation(&face, &mesh, true)
        .unwrap();
    (face, mesh)
}

pub(super) fn triangle_face() -> (Face, Handle<Triangulation>) {
    let mesh = Handle::new(Triangulation::with_sizes(3, 1, false, false).unwrap());
    for (i, p) in [
        (1, Point3::new(0., 0., 0.)),
        (2, Point3::new(2., 0., 0.)),
        (3, Point3::new(0., 1., 0.)),
    ] {
        mesh.set_node(i, p).unwrap();
    }
    mesh.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_with_triangulation(&mut face, &mesh)
        .unwrap();
    (face, mesh)
}

pub(super) fn compound(faces: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut shape = Compound::default();
    builder.make_compound(&mut shape);
    for face in faces {
        builder.add(&mut shape, face).unwrap();
    }
    shape.into()
}

fn ellipse_curve(z: f64) -> Handle<dyn GeomCurve> {
    let axis = Axis2::new(Point3::new(0., 0., z), Direction3::new(0., 0., 1.).unwrap()).unwrap();
    Handle::new(GeomEllipse::new(axis, 2., 1.).unwrap()).cast()
}

fn ellipse_cap(z: f64) -> Face {
    let edge = BRepBuilderAPIMakeEdge::from_curve(&ellipse_curve(z))
        .unwrap()
        .edge()
        .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap();
    let plane = Plane3::new(Point3::new(0., 0., z), Direction3::new(0., 0., 1.).unwrap()).unwrap();
    BRepBuilderAPIMakeFace::from_plane_wire(plane, &wire, true)
        .unwrap()
        .face()
        .unwrap()
}

pub(super) fn elliptic_prism_faces() -> Shape {
    let surface: Handle<dyn GeomSurface> = Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&ellipse_curve(0.), Direction3::new(0., 0., 1.).unwrap())
            .unwrap(),
    )
    .cast();
    let side = BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., TAU, 0., 3.), 1e-7)
        .unwrap()
        .face()
        .unwrap();
    compound(&[ellipse_cap(0.).into(), side.into(), ellipse_cap(3.).into()])
}

pub(super) fn narrow_parameter_square() -> Face {
    let surface = narrow_parameter_surface();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface, 1e-7)
        .unwrap();
    face
}

fn narrow_parameter_surface() -> Handle<dyn GeomSurface> {
    use martensite_tkernel::ncollection::{Array1, Array2};
    use martensite_tkg3d::geom::GeomBSplineSurface;
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for (u, v) in [(1, 1), (1, 2), (2, 1), (2, 2)] {
        *poles.change_value(u, v).unwrap() = Point3::new(f64::from(u - 1), f64::from(v - 1), 0.);
    }
    let mut uk = Array1::with_bounds(1, 2).unwrap();
    uk[1] = 0.;
    uk[2] = 1e-13;
    let mut vk = uk.clone();
    vk[2] = 1.;
    let mut mults = Array1::with_bounds(1, 2).unwrap();
    mults.init(2);
    Handle::new(
        GeomBSplineSurface::new(&poles, (&uk, &mults), (&vk, &mults), (1, 1), (false, false))
            .unwrap(),
    )
    .cast()
}
