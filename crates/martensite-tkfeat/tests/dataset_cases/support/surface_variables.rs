//! The commands that fill a curve or surface variable, and those that read one by name.
use super::{
    feat_script::{FeatScript, checked, command_failed},
    restore::input_bytes,
    surface_commands,
};
use martensite_tkernel::{precision, standard::Handle, storage::IoStream};
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomPlane, GeomRectangularTrimmedSurface, GeomSurface,
};
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};

/// The first word of a file saved from a surface variable, before the surface's text.
const SURFACE_HEADER: &[u8] = b"DrawTrSurf_Surface";

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn direction(along: [f64; 3]) -> Direction3 {
    checked(Direction3::new(along[0], along[1], along[2]))
}

/// `plane name x y z nx ny nz`: the frame the command builds from a location and a normal.
/// Example: `plane(s, "p", [2., 0., 0.], [1., 0., 0.])`.
pub fn plane(s: &mut FeatScript<'_>, name: &str, at: [f64; 3], normal: [f64; 3]) {
    let frame = checked(Axis3::from_normal(point(at), direction(normal)));
    s.set_surface(name, Handle::new(GeomPlane::new(frame)).cast());
}

/// `plane name`: the XY plane at the origin; `plane_xoy(s, "test_plane")`.
pub fn plane_xoy(s: &mut FeatScript<'_>, name: &str) {
    plane(s, name, [0.0; 3], [0.0, 0.0, 1.0]);
}

/// `plane name x y z nx ny nz ux uy uz`; `plane_with_x(s, "p", [0.; 3], [0., 0., 1.], [1., 0., 0.])`.
pub fn plane_with_x(
    s: &mut FeatScript<'_>,
    name: &str,
    at: [f64; 3],
    normal: [f64; 3],
    x_axis: [f64; 3],
) {
    let frame = checked(Axis2::with_x_direction(
        point(at),
        direction(normal),
        direction(x_axis),
    ));
    s.set_surface(
        name,
        Handle::new(GeomPlane::new(Axis3::from_axis2(frame))).cast(),
    );
}

/// `restore [locate_data_file file] name` of a file that holds a surface; `restore_surface(s, "a.rle", "plan")`.
pub fn restore_surface(s: &mut FeatScript<'_>, file: &str, name: &str) {
    let bytes = input_bytes(file);
    let text = bytes
        .strip_prefix(SURFACE_HEADER)
        .unwrap_or_else(|| panic!("restore {file}: expected a saved surface"));
    let mut stream = IoStream::from_bytes(text.to_vec());
    s.set_surface(name, geom_tools::read_surface(&mut stream));
}

/// `mksurface name face`; `mksurface(s, "sf", "E_9")`.
pub fn mksurface(s: &mut FeatScript<'_>, name: &str, face: &str) {
    let surface = surface_commands::mksurface(s, face);
    s.set_surface(name, surface);
}

fn surface_named(s: &FeatScript<'_>, command: &str, name: &str) -> Handle<dyn GeomSurface> {
    s.surface(name)
        .unwrap_or_else(|| command_failed(&format!("{command} : {name} is not a surface")))
}

/// `trim name base u1 u2 v1 v2`; `trim(s, "pl2", "pl2", [100., 200., -200., -100.])`.
pub fn trim(s: &mut FeatScript<'_>, name: &str, base: &str, [u1, u2, v1, v2]: [f64; 4]) {
    let basis = surface_named(s, "trim", base);
    let trimmed = checked(GeomRectangularTrimmedSurface::new(
        &basis,
        (u1, u2),
        (v1, v2),
        (true, true),
    ));
    s.set_surface(name, Handle::new(trimmed).cast());
}

/// `mkface name surface`; `mkface(s, "ff", "sf")`.
pub fn mkface(s: &mut FeatScript<'_>, name: &str, surface: &str) {
    let surface = surface_named(s, "mkface", surface);
    let mut maker = checked(BRepBuilderAPIMakeFace::from_surface(
        &surface,
        precision::confusion(),
    ));
    let shape = checked(maker.shape()).clone();
    super::session::Session::bind(s, name, &shape);
}

/// `mkface name surface u1 u2 v1 v2`; `mkface_bounded(s, "f", "p", (-100., 100., -100., 100.))`.
pub fn mkface_bounded(
    s: &mut FeatScript<'_>,
    name: &str,
    surface: &str,
    bounds: (f64, f64, f64, f64),
) {
    let surface = surface_named(s, "mkface", surface);
    let mut maker = checked(BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        bounds,
        precision::confusion(),
    ));
    let shape = checked(maker.shape()).clone();
    super::session::Session::bind(s, name, &shape);
}

/// `circle name x y z nx ny nz ux uy uz radius`.
/// Example: `circle(s, "c", [0.; 3], [0., 0., 1.], [1., 0., 0.], 10.)`.
pub fn circle(
    s: &mut FeatScript<'_>,
    name: &str,
    at: [f64; 3],
    normal: [f64; 3],
    x_axis: [f64; 3],
    radius: f64,
) {
    let frame = checked(Axis2::with_x_direction(
        point(at),
        direction(normal),
        direction(x_axis),
    ));
    let curve: Handle<dyn GeomCurve> = Handle::new(checked(GeomCircle::new(frame, radius))).cast();
    s.set_curve(name, curve);
}

/// `mkedge name curve`; `mkedge(s, "e", "c")`.
pub fn mkedge(s: &mut FeatScript<'_>, name: &str, curve: &str) {
    let curve = s
        .curve(curve)
        .unwrap_or_else(|| command_failed(&format!("mkedge : {curve} is not a curve")));
    let edge = checked(checked(BRepBuilderAPIMakeEdge::from_curve(&curve)).edge()).into_shape();
    super::session::Session::bind(s, name, &edge);
}
