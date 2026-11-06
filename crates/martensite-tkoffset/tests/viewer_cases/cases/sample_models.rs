//! The models of the milling cutter and the twist drill: the `bugs` scripts and the samples build
//! them with the same commands.
use crate::commands::{check_shape, explode, unify_same_domain};
use crate::geometry::{
    bounded_face, circle, copied, curve_of_edge, cylinder, edge_between, edge_of, edge_of_vertices,
    edge_on_surface, face_on, frame, line_2d, plane, polyline, rotated, scaled, translated,
    trim_2d, vertex, wire_of,
};
use crate::support::one_step::{bcommon, bcut, bfuse, bsection};
use crate::sweeps::Sweep;
use crate::variables::Variables;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Axis2;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis as _,
};
use std::f64::consts::PI;

const ORIGIN: [f64; 3] = [0., 0., 0.];
const Z_AXIS: [f64; 3] = [0., 0., 1.];

/// `_curvetoface`: a whole curve as a planar face.
fn curve_face(curve: &crate::geometry::Curve, plane_surface: &crate::geometry::Surface) -> Shape {
    face_on(plane_surface, &wire_of(&[edge_of(curve)]))
}

fn cylinder_on(frame_values: &[f64], radius: f64, height: f64) -> Shape {
    let axes = frame(frame_values).ax2().unwrap();
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(axes, radius, height);
    maker.solid().unwrap().into_shape()
}

fn cone_on(frame_values: &[f64], radii: (f64, f64), height: f64) -> Shape {
    let axes: Axis2 = frame(frame_values).ax2().unwrap();
    let mut maker = BRepPrimAPIMakeCone::with_axes(axes, radii.0, radii.1, height).unwrap();
    maker.solid().unwrap().into_shape()
}

/// The toothed profile: an inner disk with 25 teeth, fused one by one and unified.
fn cutter_profile(variables: &mut Variables, outer_radius: f64) {
    let p0 = plane(&[]);
    let outer = curve_face(&circle(&[0., 0., 0., 0., 0., 1.], outer_radius), &p0);
    let inner = curve_face(&circle(&[0., 0., 0., 0., 0., 1.], outer_radius - 2.), &p0);
    let round = curve_face(&circle(&[3., 6., 0., 0., 0., 1.], 5.), &p0);
    let tip = outer_radius + 1.;
    let cut = polyline(&[
        [-2., 0., 0.],
        [-2., tip, 0.],
        [0.5, tip, 0.],
        [-1., 0., 0.],
        [-2., 0., 0.],
    ]);
    variables.set("outer", outer);
    variables.set("round", round);
    variables.set("cut", face_on(&p0, &cut));
    bcommon(variables, "teeth", "cut", "round");
    bcommon(variables, "teeth", "teeth", "outer");
    variables.set("profile", copied(&inner));
    let teeth_count = 25;
    for _ in 0..teeth_count {
        bfuse(variables, "profile", "profile", "teeth");
        let turned = rotated(
            &variables.get("teeth"),
            ORIGIN,
            Z_AXIS,
            360. / f64::from(teeth_count),
        );
        variables.set("teeth", turned);
    }
    let unified = unify_same_domain(&variables.get("profile"));
    variables.set("profile", unified);
}

/// `polyline sp ...` and the helical guide on a cylinder of radius `radius`: the guide's frame
/// starts where its X direction says, and turns by `angle` over `height`.
struct GuidedSpine {
    spine: Shape,
    guide: Shape,
}

fn guided_spine(
    radius: f64,
    height: f64,
    angle: f64,
    start: [f64; 3],
    x_direction: [f64; 3],
) -> GuidedSpine {
    let spine = polyline(&[[0., 0., 0.], [0., 0., height]]);
    let frame_values = [
        0.,
        0.,
        0.,
        0.,
        0.,
        1.,
        x_direction[0],
        x_direction[1],
        x_direction[2],
    ];
    let surface = cylinder(&frame_values, radius);
    let v1 = vertex(start[0], start[1], start[2]);
    let v2 = vertex(start[0], start[1], height);
    let helix = line_2d(0., 0., angle, height);
    let helix = trim_2d(&helix, 0., (angle * angle + height * height).sqrt());
    let v2 = rotated(&v2, ORIGIN, Z_AXIS, 180. * angle / PI);
    let edge = edge_on_surface(&helix, &surface, &v1, &v2);
    GuidedSpine {
        spine,
        guide: wire_of(&[edge]),
    }
}

/// `tscale`, `ttranslate` and `tscale` of the three sections; the first stays at the bottom, as
/// the script leaves it for its display.
fn cutter_sections(variables: &mut Variables, radius: f64, height: f64) {
    explode(variables, "profile", ShapeEnum::Wire);
    let profile = variables.get("profile_1");
    let second = copied(&profile);
    let third = copied(&profile);
    let inner = radius - 2.;
    variables.set("profile_1", scaled(&profile, ORIGIN, 1. / inner));
    let second = translated(&second, [0., 0., height / 2.]);
    variables.set(
        "profile_2",
        scaled(&second, [0., 0., height / 2.], 4. / inner),
    );
    let third = translated(&third, [0., 0., height]);
    variables.set("profile_3", scaled(&third, [0., 0., height], 3. / inner));
}

/// Everything before the script's first `checkshape cutter`.
pub fn cutter_model(variables: &mut Variables) {
    let (radius, height, angle) = (10., 20., 0.5 * PI);
    cutter_profile(variables, radius);
    let guided = guided_spine(radius, height, angle, [radius, 0., 0.], [1., 0., 0.]);
    cutter_sections(variables, radius, height);
    let mut sweep = Sweep::start(&guided.spine);
    sweep.guide(&guided.guide);
    for name in ["profile_1", "profile_2", "profile_3"] {
        sweep.add(&variables.get(name));
    }
    variables.set("base", sweep.build_solid(false));
    variables.set(
        "shank",
        cylinder_on(&[0., 0., height, 0., 0., 1.], 1.4, height),
    );
    variables.set(
        "sh2",
        cylinder_on(&[0., 0., height + 1., 0., 0., 1.], 1.5, height - 1.),
    );
    bfuse(variables, "shank", "shank", "sh2");
    bfuse(variables, "cutter", "base", "shank");
    check_shape(variables, "cutter");
}

/// The drill's dimensions.
struct Drill {
    radius: f64,
    chisel: f64,
    half_web: f64,
    height: f64,
    angle: f64,
    point_degrees: f64,
}

const DRILL: Drill = Drill {
    radius: 4.,
    chisel: 3.5,
    half_web: 0.5,
    height: 80.,
    angle: 3. * PI,
    point_degrees: 118.,
};

/// The section before its flutes: the web rectangle within the outer circle, fused with the chisel.
fn drill_section(variables: &mut Variables, p0: &crate::geometry::Surface) {
    let Drill {
        radius: r,
        half_web: d,
        chisel,
        ..
    } = DRILL;
    let rectangle = polyline(&[
        [d, -r, 0.],
        [r, -r, 0.],
        [-d, r, 0.],
        [-r, r, 0.],
        [d, -r, 0.],
    ]);
    variables.set("rectangle1", face_on(p0, &rectangle));
    variables.set(
        "circle1",
        curve_face(&circle(&[0., 0., 0., 0., 0., 1.], r), p0),
    );
    variables.set(
        "circle2",
        curve_face(&circle(&[0., 0., 0., 0., 0., 1.], chisel), p0),
    );
    bcommon(variables, "sec", "rectangle1", "circle1");
    bfuse(variables, "sec", "sec", "circle2");
    let unified = unify_same_domain(&variables.get("sec"));
    variables.set("sec", unified);
}

/// The section of the helical sweep of the lip with the plane: the edge of the flute that gives a
/// straight cutting lip after sharpening; its two end points.
fn flute_edge(
    variables: &mut Variables,
    p0: &crate::geometry::Surface,
    guided: &GuidedSpine,
) -> ([f64; 3], [f64; 3]) {
    let Drill {
        radius: r,
        half_web: d,
        point_degrees: sigma,
        ..
    } = DRILL;
    let lip_end = -r / martensite_cmath::tan(sigma / 2. * PI / 180.);
    let lip = polyline(&[[d, -d / 2., 0.], [d, -r, lip_end]]);
    let mut sweep = Sweep::start(&guided.spine);
    sweep.guide(&guided.guide);
    sweep.add(&lip);
    variables.set("spiral", sweep.build_solid(false));
    variables.set("f0", bounded_face(p0, (-r, r, -r, r)));
    bsection(variables, "sflute", "spiral", "f0");
    explode(variables, "sflute", ShapeEnum::Edge);
    let curve = curve_of_edge(&variables.get("sflute_1"));
    let at = |parameter: f64| {
        let point = curve.value(parameter).unwrap();
        [point.x(), point.y(), point.z()]
    };
    (at(0.), at(1.))
}

/// The closed flute: the section curve, a rounding of half the chisel radius and two lines.
fn flute_face(
    variables: &Variables,
    p0: &crate::geometry::Surface,
    ends: ([f64; 3], [f64; 3]),
) -> Shape {
    let Drill {
        radius: r, chisel, ..
    } = DRILL;
    let ([x0, y0, z0], [x1, y1, z1]) = ends;
    let vf0 = vertex(x0, y0, z0);
    let vf1 = vertex(x1, y1, z1);
    let rounding = circle(&[x0 + chisel / 2., y0, 0., 0., 0., 1.], chisel / 2.);
    let vf3 = vertex(x0 + chisel, y0, 0.);
    let round_edge = edge_between(&rounding, &vf3, &vf0);
    let vf2 = vertex(r, -r, 0.);
    let third = edge_of_vertices(&vf3, &vf2);
    let fourth = edge_of_vertices(&vf2, &vf1);
    let wire = wire_of(&[variables.get("sflute_1"), round_edge, third, fourth]);
    face_on(p0, &wire)
}

/// The sharpening tool: a box less the common of two turned cones.
fn sharpener(variables: &mut Variables) {
    let Drill {
        radius: r,
        half_web: d,
        height,
        angle,
        point_degrees: sigma,
        ..
    } = DRILL;
    let diameter = 2. * r;
    let theta = angle * r / height * martensite_cmath::sin((90. - sigma / 2.) * PI / 180.);
    let apex = height + 1.9 * diameter / martensite_cmath::tan(PI / 180. * sigma / 2.);
    let cone = cone_on(
        &[d, 1.9 * diameter, apex, 0., -1., -1.],
        (
            0.,
            100. * martensite_cmath::sin((sigma - 90.) / 2. * PI / 180.),
        ),
        100.,
    );
    let sh1 = rotated(&cone, ORIGIN, Z_AXIS, -theta * 180. / PI);
    let sh2 = rotated(&copied(&sh1), ORIGIN, Z_AXIS, 180.);
    let corner = martensite_tkmath::gp::Point3::new(-diameter / 2., -diameter / 2., 72.);
    let mut block = BRepPrimAPIMakeBox::at_point(corner, diameter, diameter, 20.).unwrap();
    variables.set("sh", block.solid().unwrap().into_shape());
    variables.set("sh1", sh1);
    variables.set("sh2", sh2);
    bcommon(variables, "qq", "sh1", "sh2");
    bcut(variables, "sharpener", "sh", "qq");
}

/// The shank: a cylinder, a tail cone and the transition cone to the body.
fn drill_shank(variables: &mut Variables) {
    let r = DRILL.radius;
    variables.set("shank", cylinder_on(&[0., 0., -40., 0., 0., 1.], 4., 40.));
    let mut transit = BRepPrimAPIMakeCone::new(r, 0., r).unwrap();
    variables.set("transit", transit.solid().unwrap().into_shape());
    variables.set("tail", cone_on(&[0., 0., -40., 0., 0., -0.5], (r, 0.), 0.5));
    bfuse(variables, "shank", "shank", "tail");
    bfuse(variables, "shank", "shank", "transit");
}

/// Everything before the script's first `checkshape drill`; the guide `gg` and the section
/// `sec_1` stay in the variables for the display.
pub fn drill_model(variables: &mut Variables) {
    println!("Creating the drill section profile...");
    let Drill {
        radius: r,
        height,
        angle,
        ..
    } = DRILL;
    let p0 = plane(&[]);
    drill_section(variables, &p0);
    let guided = guided_spine(r, height, angle, [0., -r, 0.], [0., -4., 0.]);
    let ends = flute_edge(variables, &p0, &guided);
    variables.set("flute", flute_face(variables, &p0, ends));
    bcut(variables, "sec", "sec", "flute");
    let turned = rotated(&variables.get("flute"), ORIGIN, Z_AXIS, 180.);
    variables.set("flute", turned);
    bcut(variables, "sec", "sec", "flute");
    println!("Sweeping the profile...");
    explode(variables, "sec", ShapeEnum::Wire);
    let mut sweep = Sweep::start(&guided.spine);
    sweep.guide(&guided.guide);
    sweep.add(&variables.get("sec_1"));
    variables.set("base", sweep.build_solid(false));
    println!("Sharpening...");
    sharpener(variables);
    bcut(variables, "body", "base", "sharpener");
    println!("Making a shank...");
    drill_shank(variables);
    bfuse(variables, "drill", "body", "shank");
    check_shape(variables, "drill");
    variables.set("gg", guided.guide);
}

/// `ttranslate sec_1 0 0 H; trotate sec_1 0 0 0 0 0 1 a*180/pi`: the section where the sweep ends.
pub fn drill_end_section(variables: &Variables) -> Shape {
    let Drill { height, angle, .. } = DRILL;
    let section = translated(&variables.get("sec_1"), [0., 0., height]);
    rotated(&section, ORIGIN, Z_AXIS, angle * 180. / PI)
}
