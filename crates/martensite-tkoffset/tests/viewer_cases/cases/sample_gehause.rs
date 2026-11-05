//! The housing part of the CAM-I modelling comparison: planar geometry, cylindrical holes and
//! boolean operations only.
use crate::commands::{check_shape, explode, unify_same_domain};
use crate::geometry::{
    Curve2d, bezier_2d, box_at, circle_2d, edge_of, edge_of_2d, ellipse_2d, face_on, line_2d,
    plane, plane_face, prism, rotated, translated, trim_2d, wire_of,
};
use crate::grading::{DEMO, run_alone};
use crate::modelling::{fixed, loft, to_3d};
use crate::scene::{SHADED, ViewScene};
use crate::support::one_step::{bcut, bfuse};
use crate::support::rules::Verdict;
use crate::variables::Variables;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::Color;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Plane3, Point3, dx, dz};

/// A piece of a contour: a trimmed line `(x, y, dx, dy, length)` or a Bezier of three poles.
enum Piece {
    Line(f64, f64, f64, f64, f64),
    Bezier([[f64; 2]; 3]),
}
use Piece::{Bezier, Line};

const CORNER: Piece = Bezier([[22., 10.], [20., 10.], [20., 8.]]);
const BASE_CONTOUR: [Piece; 19] = [
    Line(0., 0., 1., 0., 35.),
    Bezier([[35., 0.], [39., 0.], [39., 4.]]),
    Line(39., 4., 0., 1., 6.),
    Bezier([[39., 10.], [39., 13.], [35., 13.]]),
    Line(35., 13., -1., 0., 8.),
    Line(27., 13., 0., -1., 3.),
    Line(27., 10., -1., 0., 5.),
    CORNER,
    Line(20., 8., 0., -1., 4.),
    Line(20., 4., -1., 0., 9.),
    Line(11., 4., 0., -1., 1.),
    Line(11., 3., -1., 0., 6.),
    Line(5., 3., 0., 1., 10.),
    Line(5., 13., -1., 0., 5.),
    Line(0., 13., 0., -1., 4.),
    Line(0., 9., 1., 0., 4.),
    Line(4., 9., 0., -1., 5.),
    Line(4., 4., -1., 0., 4.),
    Line(0., 4., 0., -1., 4.),
];
const TOP_CONTOUR: [Piece; 13] = [
    Line(0., 0., 1., 0., 28.),
    Line(28., 0., 0., 1., 13.),
    Line(28., 13., -1., 0., 1.),
    Line(27., 13., 0., -1., 3.),
    Line(27., 10., -1., 0., 5.),
    CORNER,
    Line(20., 8., 0., -1., 4.),
    Line(20., 4., -1., 0., 9.),
    Line(11., 4., 0., -1., 1.),
    Line(11., 3., -1., 0., 6.),
    Line(5., 3., 0., 1., 10.),
    Line(5., 13., -1., 0., 5.),
    Line(0., 13., 0., -1., 13.),
];

fn piece_curve(piece: &Piece) -> Curve2d {
    match piece {
        Line(x, y, dir_x, dir_y, length) => trim_2d(&line_2d(*x, *y, *dir_x, *dir_y), 0., *length),
        Bezier(poles) => bezier_2d(poles),
    }
}

fn plane_at(z: f64, x: f64, y: f64) -> Plane3 {
    Plane3::from_position(Axis3::new(Point3::new(x, y, z), dz(), dx()).unwrap())
}

/// The base: the contour less the big hole, extruded by 3.
fn base_part() -> (Shape, Variables) {
    let edges: Vec<Shape> = BASE_CONTOUR
        .iter()
        .map(|p| edge_of_2d(&piece_curve(p)))
        .collect();
    let outer = wire_of(&edges);
    let hole = wire_of(&[edge_of_2d(&circle_2d(33., 6.5, 4.))]);
    let mut variables = Variables::default();
    variables.set("f", fixed(&plane_face(&outer)));
    variables.set("f1", fixed(&plane_face(&hole)));
    bcut(&mut variables, "f", "f", "f1");
    (prism(&variables.get("f"), [0., 0., 3.]), variables)
}

/// The contour on the top face, extruded by 1.5.
fn top_part() -> Shape {
    let top = plane_at(3., 0., 0.);
    let edge_on_top = |piece: &Piece| edge_of(&to_3d(&piece_curve(piece), &top));
    let edges: Vec<Shape> = TOP_CONTOUR.iter().map(edge_on_top).collect();
    let face = face_on(&plane(&[0., 0., 3.]), &wire_of(&edges));
    prism(&fixed(&face), [0., 0., 1.5])
}

/// A whole 2D curve on the plane at `at`, as a wire.
fn wire_on(curve: &Curve2d, at: [f64; 3]) -> Shape {
    wire_of(&[edge_of(&to_3d(curve, &plane_at(at[2], at[0], at[1])))])
}

/// The hole through the middle.
fn cut_middle_hole(variables: &mut Variables) {
    let small = circle_2d(23., 6.5, 2.);
    let sections = [
        wire_on(&small, [0., 0., 10.]),
        wire_on(&small, [0., 0., -10.]),
    ];
    variables.set("tool", loft(&sections, true, true, true));
    bcut(variables, "p", "p", "tool");
}

/// The tube around that hole.
fn fuse_tube(variables: &mut Variables) {
    let on_top = plane(&[0., 0., 4.5]);
    let face_of = |radius: f64| {
        let wire = wire_on(&circle_2d(23., 6.5, radius), [0., 0., 4.5]);
        prism(&fixed(&face_on(&on_top, &wire)), [0., 0., 3.])
    };
    variables.set("ptube_outer", face_of(3.));
    variables.set("ptube_inner", face_of(2.));
    bcut(variables, "ptube", "ptube_outer", "ptube_inner");
    bfuse(variables, "p", "p", "ptube");
}

/// The four teeth.
fn fuse_teeth(variables: &mut Variables) {
    let teeth = [
        ([26., 0., 0.], [3., 2.5, 7.]),
        ([26., 13., 0.], [3., -3., 7.]),
        ([0., 0., 0.], [5., 3., 8.]),
        ([0., 13., 0.], [5., -3., 8.]),
    ];
    for (corner, size) in teeth {
        variables.set("t", box_at(corner, size));
        bfuse(variables, "p", "p", "t");
    }
}

/// A box that the script names twice in `trotate` and `ttranslate`, so each moves it twice: the
/// commands transform every name they get, in turn.
fn doubly_placed_box(corner: [f64; 3], size: [f64; 3], degrees: f64, by: [f64; 3]) -> Shape {
    let mut block = box_at(corner, size);
    for _ in 0..2 {
        block = rotated(&block, [0., 0., 0.], [0., 1., 0.], degrees);
    }
    for _ in 0..2 {
        block = translated(&block, by);
    }
    block
}

/// The material off the tail's teeth and the teeth near the big hole.
fn cut_blends(variables: &mut Variables) {
    let (corner, size) = ([0., -5., 0.], [5., 25., 5.]);
    variables.set(
        "blend_box",
        doubly_placed_box(corner, size, -10., [2.5, 0., 2.25]),
    );
    bcut(variables, "p", "p", "blend_box");
    variables.set(
        "blend_box",
        doubly_placed_box(corner, size, -9., [14.5, 0., 1.5]),
    );
    bcut(variables, "p", "p", "blend_box");
}

/// The elliptic hole at the tail and its chamfer.
fn cut_tail(variables: &mut Variables) {
    let ellipse = ellipse_2d(0., 0., (1., 0.), 0.75, 0.5);
    let sections = [
        wire_on(&ellipse, [2., 6.5, 0.]),
        wire_on(&ellipse, [2., 6.5, 20.]),
    ];
    variables.set("tail_tube", loft(&sections, true, true, true));
    bcut(variables, "p", "p", "tail_tube");
    let chamfer = doubly_placed_box([0., 3., 0.], [5., 7., 5.], -5., [2., 0., 1.5]);
    variables.set("blend_box", chamfer);
    bcut(variables, "p", "p", "blend_box");
}

/// The script: the base and top fused, the features, then the only solid unified.
fn gehause_model(variables: &mut Variables) {
    let (base, _) = base_part();
    variables.set("p", base);
    variables.set("ptop", top_part());
    bfuse(variables, "p", "p", "ptop");
    cut_middle_hole(variables);
    fuse_tube(variables);
    fuse_teeth(variables);
    cut_blends(variables);
    cut_tail(variables);
    explode(variables, "p", ShapeEnum::Solid);
    variables.set("result", unify_same_domain(&variables.get("p_1")));
}

#[test]
fn gehause() {
    run_alone("cases::sample_gehause::gehause", &DEMO, Verdict::Ok, || {
        let mut variables = Variables::default();
        gehause_model(&mut variables);
        let mut scene = ViewScene::open("gehause");
        let background = Color::from_hex("C8C8FF").expect("the hexadecimal color");
        scene.set_background(background);
        scene.display_in_mode("result", &variables.get("result"), SHADED);
        scene.fit();
        scene.show_face_boundaries("result");
        check_shape(&variables, "result");
        let rgb = [background.red(), background.green(), background.blue()];
        assert!(scene.dump(rgb) > 0, "the view shows nothing");
        scene.close();
        println!("TEST COMPLETED");
    });
}
