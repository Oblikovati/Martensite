//! The commands of the volume and geometry-description scripts: surface variables, primitives
//! on a plane's frame, `compound`, `settolerance` and `mkvolume`.
use super::{commands::bound_shape, session::Session};
use crate::common_operation::report_result_alerts;
use martensite_tkbo::bop_algo::BOPAlgoMakerVolume;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::{ncollection::List, precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSphericalSurface, GeomSurface};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;

fn frame_along(at: [f64; 3], normal: [f64; 3]) -> Axis3 {
    let normal = Direction3::new(normal[0], normal[1], normal[2])
        .unwrap_or_else(|failure| panic!("normal {normal:?}: {failure:?}"));
    Axis3::from_normal(Point3::new(at[0], at[1], at[2]), normal)
        .unwrap_or_else(|failure| panic!("frame {at:?}: {failure:?}"))
}

/// `cylinder c x y z dx dy dz radius`; `cylinder([0.; 3], [0., 0., 1.], 10.)`.
pub fn cylinder(at: [f64; 3], normal: [f64; 3], radius: f64) -> Handle<dyn GeomSurface> {
    let surface = GeomCylindricalSurface::new(frame_along(at, normal), radius)
        .unwrap_or_else(|failure| panic!("cylinder {radius}: {failure:?}"));
    Handle::new(surface).cast()
}

/// `sphere c x y z dx dy dz radius`; `sphere([0.; 3], [0., 0., 1.], 550.)`.
pub fn sphere(at: [f64; 3], normal: [f64; 3], radius: f64) -> Handle<dyn GeomSurface> {
    let surface = GeomSphericalSurface::new(frame_along(at, normal), radius)
        .unwrap_or_else(|failure| panic!("sphere {radius}: {failure:?}"));
    Handle::new(surface).cast()
}

fn radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

/// `pcylinder name plane radius height degrees`; `pcylinder_on(s, "c", frame, 1., 2., 360.)`.
pub fn pcylinder_on(
    s: &mut dyn Session,
    name: &str,
    on: Axis2,
    radius: f64,
    height: f64,
    degrees: f64,
) {
    let mut maker = BRepPrimAPIMakeCylinder::with_axes_angle(on, radius, height, radians(degrees))
        .unwrap_or_else(|failure| panic!("pcylinder {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `pcone name plane radius1 radius2 height degrees`; `pcone_on(s, "c", frame, [2., 1., 3.], 360.)`.
pub fn pcone_on(s: &mut dyn Session, name: &str, on: Axis2, sizes: [f64; 3], degrees: f64) {
    let [radius1, radius2, height] = sizes;
    let mut maker =
        BRepPrimAPIMakeCone::with_axes_angle(on, radius1, radius2, height, radians(degrees))
            .unwrap_or_else(|failure| panic!("pcone {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `ptorus name plane radius1 radius2 degrees`; `ptorus_on(s, "t", frame, 4., 1., 360.)`.
pub fn ptorus_on(
    s: &mut dyn Session,
    name: &str,
    on: Axis2,
    radius1: f64,
    radius2: f64,
    degrees: f64,
) {
    let mut maker = BRepPrimAPIMakeTorus::with_axes_angle(on, radius1, radius2, radians(degrees))
        .unwrap_or_else(|failure| panic!("ptorus {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `psphere name plane radius degrees`; `psphere_turned(s, "b", frame, 4., 360.)`.
pub fn psphere_turned(s: &mut dyn Session, name: &str, on: Axis2, radius: f64, degrees: f64) {
    let mut maker = BRepPrimAPIMakeSphere::with_axis_angle(on, radius, radians(degrees))
        .unwrap_or_else(|failure| panic!("psphere {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `compound a b ... name`; a name that is not bound is left out, as the command does.
/// Example: `compound(s, &["a", "b"], "c")`.
pub fn compound(s: &mut dyn Session, parts: &[&str], name: &str) {
    let builder = BRepBuilder::default();
    let mut whole = Compound::default();
    builder.make_compound(&mut whole);
    for part in parts {
        if let Some(shape) = s.shape(part) {
            builder.add(&mut whole, shape).unwrap();
        }
    }
    s.bind(name, &whole.into_shape());
}

/// `settolerance name value`: forces every tolerance of the shape to the value.
/// Example: `settolerance(s, "a", 6.35e-7)`.
pub fn settolerance(s: &mut dyn Session, name: &str, value: f64) {
    println!(
        "Setting Tolerance to {}",
        martensite_tkernel::storage::format_general(value, 17)
    );
    let shape = bound_shape(s, name);
    ShapeFixShapeTolerance::new()
        .limit_tolerance(
            &shape,
            value,
            value,
            martensite_tkg3d::top_abs::ShapeEnum::Shape,
        )
        .unwrap_or_else(|failure| panic!("settolerance {name} {value}: {failure:?}"));
}

/// `mkvolume name a b ...`: the solids the arguments bound, with the default options.
/// Example: `mkvolume(s, "result", &["f1", "f2"])`.
pub fn mkvolume(s: &mut dyn Session, name: &str, parts: &[&str]) {
    let arguments: List<Shape> = parts.iter().map(|part| bound_shape(s, part)).collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.set_intersect(true);
    maker.set_avoid_internal_shapes(false);
    let options = maker.options_mut();
    options.set_run_parallel(false);
    options.set_fuzzy_value(precision::confusion());
    options.set_use_obb(false);
    maker.perform();
    report_result_alerts(maker.options());
    if !maker.options().has_errors() {
        s.bind(name, maker.shape());
    }
}
