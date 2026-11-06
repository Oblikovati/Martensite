//! The geometry and topology commands the viewer scripts build their inputs with, under the
//! arguments and defaults of the commands.
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Compound, Shape, Shell, Wire};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dLine, Geom2dTrimmedCurve,
};
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomCylindricalSurface, GeomLine, GeomPlane, GeomSurface,
    GeomTrimmedCurve,
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis3, Axis22d, Direction2, Direction3, Point2, Point3, Transform3,
    TransformForm, Vector3, dx, dz,
};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakePrism};
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeEdge2d, BRepBuilderAPIMakeFace,
    BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape as _, BRepBuilderAPIMakeVertex,
    BRepBuilderAPIMakeWire, BRepBuilderAPITransform,
};

pub type Curve = Handle<dyn GeomCurve>;
pub type Curve2d = Handle<dyn Geom2dCurve>;
pub type Surface = Handle<dyn GeomSurface>;

fn point(v: &[f64]) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}

fn direction(v: &[f64]) -> Direction3 {
    Direction3::from_vector(Vector3::new(v[0], v[1], v[2])).unwrap()
}

/// `[x y z [dx dy dz [ux uy uz]]]` of `plane`, `cylinder` and the others.
pub fn frame(v: &[f64]) -> Axis3 {
    match v.len() {
        0 => Axis3::new(Point3::new(0., 0., 0.), dz(), dx()).unwrap(),
        3 => Axis3::new(point(v), dz(), dx()).unwrap(),
        6 => Axis3::from_normal(point(v), direction(&v[3..])).unwrap(),
        9 => Axis3::new(point(v), direction(&v[3..]), direction(&v[6..])).unwrap(),
        count => panic!("frame of {count} values; expected 0, 3, 6 or 9"),
    }
}

/// `plane name [x y z [dx dy dz]]`; `plane(&[0., 0., 3.])`.
pub fn plane(v: &[f64]) -> Surface {
    Handle::new(GeomPlane::new(frame(v))).cast()
}

/// `cylinder name x y z dx dy dz ux uy uz radius`; `cylinder(&frame_values, 10.)`.
pub fn cylinder(v: &[f64], radius: f64) -> Surface {
    Handle::new(GeomCylindricalSurface::new(frame(v), radius).unwrap()).cast()
}

/// `circle name x y z [dx dy dz] radius`; `circle(&[0., 0., 0., 0., 0., 1.], 10.)`.
pub fn circle(v: &[f64], radius: f64) -> Curve {
    let axes = match v.len() {
        3 => Axis2::new(point(v), dz()).unwrap(),
        6 => Axis2::new(point(v), direction(&v[3..])).unwrap(),
        count => panic!("circle frame of {count} values; expected 3 or 6"),
    };
    Handle::new(GeomCircle::new(axes, radius).unwrap()).cast()
}

/// `line name x y z dx dy dz`; `line(&[-6., 0., 0.], &[1., 0., 0.])`.
pub fn line(at: &[f64], along: &[f64]) -> Curve {
    Handle::new(GeomLine::from_point(point(at), direction(along))).cast()
}

/// `trim name curve first last`; `trim(&arc, 0., 1.)`.
pub fn trim(curve: &Curve, first: f64, last: f64) -> Curve {
    Handle::new(GeomTrimmedCurve::new(curve, first, last, true, true).unwrap()).cast()
}

/// `line name x y dx dy`; `line_2d(0., 0., 1., 0.)`.
pub fn line_2d(x: f64, y: f64, dir_x: f64, dir_y: f64) -> Curve2d {
    let along = Direction2::new(dir_x, dir_y).unwrap();
    Handle::new(Geom2dLine::from_point(Point2::new(x, y), along)).cast()
}

fn planar_frame(x: f64, y: f64, along: (f64, f64)) -> Axis22d {
    let along = Direction2::new(along.0, along.1).unwrap();
    Axis22d::new(Point2::new(x, y), along, true).unwrap()
}

/// `circle name x y radius`; `circle_2d(33., 6.5, 4.)`.
pub fn circle_2d(x: f64, y: f64, radius: f64) -> Curve2d {
    Handle::new(Geom2dCircle::new(planar_frame(x, y, (1., 0.)), radius).unwrap()).cast()
}

/// `ellipse name x y dx dy major minor`, with `(1, 0)` for the form without a direction.
/// Example: `ellipse_2d(0., 0., (1., 0.), 0.75, 0.5)`.
pub fn ellipse_2d(x: f64, y: f64, along: (f64, f64), major: f64, minor: f64) -> Curve2d {
    Handle::new(Geom2dEllipse::new(planar_frame(x, y, along), major, minor).unwrap()).cast()
}

/// `trim name curve first last`, on a 2D curve; `trim_2d(&line, 0., 2.)`.
pub fn trim_2d(curve: &Curve2d, first: f64, last: f64) -> Curve2d {
    Handle::new(Geom2dTrimmedCurve::new(curve, first, last, true, true).unwrap()).cast()
}

/// `vertex name x y z`; `vertex(10., 0., 0.)`.
pub fn vertex(x: f64, y: f64, z: f64) -> Shape {
    let maker = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z)).unwrap();
    maker.vertex().unwrap().into_shape()
}

/// `polyline name x y z ...`; `polyline(&[[0., 0., 0.], [0., 0., 20.]])`.
pub fn polyline(points: &[[f64; 3]]) -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for p in points {
        polygon.add_point(point(p)).unwrap();
    }
    polygon.wire().unwrap().into_shape()
}

/// `mkedge name curve`; `edge_of(&circle)`.
pub fn edge_of(curve: &Curve) -> Shape {
    let maker = BRepBuilderAPIMakeEdge::from_curve(curve).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `mkedge name curve2d`, an edge on the default plane; `edge_of_2d(&line)`.
pub fn edge_of_2d(curve: &Curve2d) -> Shape {
    let maker = BRepBuilderAPIMakeEdge2d::from_curve(curve).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `mkedge name curve v1 v2`; `edge_between(&arc, &v1, &v2)`.
pub fn edge_between(curve: &Curve, first: &Shape, last: &Shape) -> Shape {
    let (first, last) = (
        topods::vertex(first).unwrap(),
        topods::vertex(last).unwrap(),
    );
    let maker = BRepBuilderAPIMakeEdge::from_curve_vertices(curve, &first, &last).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `edge name v1 v2`; `edge_of_vertices(&v1, &v2)`.
pub fn edge_of_vertices(first: &Shape, last: &Shape) -> Shape {
    let (first, last) = (
        topods::vertex(first).unwrap(),
        topods::vertex(last).unwrap(),
    );
    let maker = BRepBuilderAPIMakeEdge::from_vertices(&first, &last).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `mkedge name curve2d surface v1 v2`; `edge_on_surface(&line, &cylinder, &v1, &v2)`.
pub fn edge_on_surface(curve: &Curve2d, surface: &Surface, first: &Shape, last: &Shape) -> Shape {
    let (first, last) = (
        topods::vertex(first).unwrap(),
        topods::vertex(last).unwrap(),
    );
    let maker =
        BRepBuilderAPIMakeEdge::from_pcurve_vertices(curve, surface, &first, &last).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `mkedge name curve2d surface first last`; `edge_on_surface_range(&arc, &cylinder, 0., 3.14)`.
pub fn edge_on_surface_range(curve: &Curve2d, surface: &Surface, first: f64, last: f64) -> Shape {
    let maker = BRepBuilderAPIMakeEdge::from_pcurve_params(curve, surface, first, last).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `wire name part ...`, each part an edge or a wire; `wire_of(&[edge])`.
pub fn wire_of(parts: &[Shape]) -> Shape {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for part in parts {
        match part.shape_type().unwrap() {
            martensite_tkg3d::top_abs::ShapeEnum::Edge => {
                maker.add_edge(&topods::edge(part).unwrap()).unwrap();
            }
            _ => maker.add_wire(&topods::wire(part).unwrap()).unwrap(),
        }
    }
    assert!(maker.is_done(), "wire of {} parts: not done", parts.len());
    maker.shape().unwrap().clone()
}

/// `mkface name surface wire`; `face_on(&plane, &wire)`.
pub fn face_on(surface: &Surface, wire: &Shape) -> Shape {
    let wire: Wire = topods::wire(wire).unwrap();
    let mut maker = BRepBuilderAPIMakeFace::from_surface_wire(surface, &wire, true).unwrap();
    maker.shape().unwrap().clone()
}

/// `mkface name surface`: the face of the whole surface; `whole_face(&plane)`.
pub fn whole_face(surface: &Surface) -> Shape {
    let tolerance = martensite_tkernel::precision::confusion();
    let mut maker = BRepBuilderAPIMakeFace::from_surface(surface, tolerance).unwrap();
    maker.shape().unwrap().clone()
}

/// `mkface name surface umin umax vmin vmax`; `bounded_face(&plane, (-4., 4., -4., 4.))`.
pub fn bounded_face(surface: &Surface, bounds: (f64, f64, f64, f64)) -> Shape {
    let tolerance = martensite_tkernel::precision::confusion();
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(surface, bounds, tolerance).unwrap();
    maker.shape().unwrap().clone()
}

/// `mkplane name wire`; `plane_face(&wire)`.
pub fn plane_face(wire: &Shape) -> Shape {
    let wire: Wire = topods::wire(wire).unwrap();
    let mut maker = BRepBuilderAPIMakeFace::from_wire(&wire, false).unwrap();
    maker.shape().unwrap().clone()
}

/// `prism result base dx dy dz`; `prism(&face, [0., 0., 3.])`.
pub fn prism(base: &Shape, by: [f64; 3]) -> Shape {
    let vector = Vector3::new(by[0], by[1], by[2]);
    let mut maker = BRepPrimAPIMakePrism::new(base, vector, false, true).unwrap();
    maker.shape().unwrap().clone()
}

/// `box name x y z dx dy dz`; `box_at([0., 0., 0.], [5., 3., 8.])`.
pub fn box_at(corner: [f64; 3], size: [f64; 3]) -> Shape {
    let mut maker =
        BRepPrimAPIMakeBox::at_point(point(&corner), size[0], size[1], size[2]).unwrap();
    maker.solid().unwrap().into_shape()
}

/// `2dbeziercurve name 3 pole 1 pole 1 pole 1`; `bezier_2d(&[[0., 0.], [1., 0.], [1., 1.]])`.
pub fn bezier_2d(poles: &[[f64; 2]; 3]) -> Curve2d {
    let points = poles.map(|pole| Point2::new(pole[0], pole[1]));
    let weights = [1.; 3];
    let points = Array1::from_shared(&points, 1);
    let weights = Array1::from_shared(&weights, 1);
    Handle::new(Geom2dBezierCurve::with_weights(&points, &weights).unwrap()).cast()
}

/// `orientation name R`; `reversed(&wire)`.
pub fn reversed(shape: &Shape) -> Shape {
    shape.oriented(Orientation::Reversed)
}

/// `shape name Sh`, then `add face name` for each; `shell_of(&faces)`.
pub fn shell_of(faces: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut made = Shell::default();
    builder.make_shell(&mut made);
    let mut shape = made.as_ref().clone();
    for face in faces {
        builder.add(&mut shape, face).unwrap();
    }
    shape
}

/// `bounding name -save ...`: the corners of the box, with the triangulation when there is one.
/// Example: `let (min, max) = bounded_extent(&shape);`.
pub fn bounded_extent(shape: &Shape) -> ([f64; 3], [f64; 3]) {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true).unwrap();
    let (min, max) = (bounds.corner_min().unwrap(), bounds.corner_max().unwrap());
    ([min.x(), min.y(), min.z()], [max.x(), max.y(), max.z()])
}

/// `compound name ...`; `compound_of(&[a, b])`.
pub fn compound_of(parts: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut made = Compound::default();
    builder.make_compound(&mut made);
    let mut shape = made.as_ref().clone();
    for part in parts {
        builder.add(&mut shape, part).unwrap();
    }
    shape
}

/// `tcopy from`: the shape with copied geometry; `copied(&shape)`.
pub fn copied(shape: &Shape) -> Shape {
    let mut maker = BRepBuilderAPICopy::new();
    maker.perform(shape, true, false).unwrap();
    maker.shape().unwrap().clone()
}

/// The `transform` command: `moved` for an identity, `BRepBuilderAPITransform` otherwise.
fn transformed(shape: &Shape, trsf: Transform3) -> Shape {
    if trsf.form() == TransformForm::Identity {
        return shape.moved(&Location::new(trsf), true).unwrap();
    }
    let mut transformer = BRepBuilderAPITransform::new(trsf);
    transformer.perform(shape, false, false).unwrap();
    assert!(transformer.is_done(), "transform of the shape: not done");
    transformer.shape().unwrap().clone()
}

/// `ttranslate shape dx dy dz`; `translated(&shape, [0., 0., 20.])`.
pub fn translated(shape: &Shape, by: [f64; 3]) -> Shape {
    let mut trsf = Transform3::default();
    trsf.set_translation(Vector3::new(by[0], by[1], by[2]));
    transformed(shape, trsf)
}

/// `trotate shape x y z dx dy dz degrees`; `rotated(&shape, [0.; 3], [0., 0., 1.], 90.)`.
pub fn rotated(shape: &Shape, at: [f64; 3], along: [f64; 3], degrees: f64) -> Shape {
    let mut trsf = Transform3::default();
    let axis = Axis1::new(point(&at), direction(&along));
    trsf.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))
        .unwrap();
    transformed(shape, trsf)
}

/// `tscale shape x y z factor`; `scaled(&shape, [0.; 3], 0.5)`.
pub fn scaled(shape: &Shape, at: [f64; 3], factor: f64) -> Shape {
    let mut trsf = Transform3::default();
    trsf.set_scale(point(&at), factor).unwrap();
    transformed(shape, trsf)
}

/// `tmirror shape x y z dx dy dz`; `mirrored(&shape, [0.; 3], [0., 1., 0.])`.
pub fn mirrored(shape: &Shape, at: [f64; 3], normal: [f64; 3]) -> Shape {
    let mut trsf = Transform3::default();
    let frame = Axis2::new(point(&at), direction(&normal)).unwrap();
    trsf.set_mirror_plane(frame);
    transformed(shape, trsf)
}

/// `mkcurve name edge`: the 3D curve of an edge over its range; `curve_of_edge(&edge)`.
pub fn curve_of_edge(edge: &Shape) -> Curve {
    let found = BRepTool::curve_with_location(&topods::edge(edge).unwrap()).unwrap();
    assert!(!found.curve.is_null(), "mkcurve: the edge has no 3d curve");
    let trimmed = GeomTrimmedCurve::new(&found.curve, found.first, found.last, true, true).unwrap();
    let trimmed: Curve = Handle::new(trimmed).cast();
    trimmed
        .transformed(found.location.transformation())
        .unwrap()
        .cast()
}
