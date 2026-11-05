//! `tests/mkface/edges_to_faces`: faces rebuilt from a compound of edges.
use crate::support::{
    checkshape::checkshape_named,
    commands::{compound, make_box},
    construction::nurbsconvert,
    heal_commands::{bopcheck_is_ok, compound_exploded_many, edgestofaces, mkedge_line},
    nbshapes::{Counting, NbCount, checknbshapes},
    session::Session,
    surface_commands::{circle_along, mkedge},
};

const Z: [f64; 3] = [0., 0., 1.];

fn banner() {
    println!("========");
    println!("OCC28426");
    println!("========");
    println!();
}

/// `edgestofaces result c`, then the script's checks on `result`.
fn rebuilt(s: &mut dyn Session, faces: usize, wires: usize) {
    edgestofaces(s, "result", "c");
    checkshape_named(s, "result");
    checked(s, "result", faces, wires);
}

fn checked(s: &mut dyn Session, name: &str, faces: usize, wires: usize) {
    let expected = [NbCount::Wires(wires), NbCount::Faces(faces)];
    checknbshapes(s, name, Counting::Located, &expected);
}

fn bopchecked(s: &mut dyn Session, name: &str) {
    if !bopcheck_is_ok(s, name) {
        println!("Error : The result is self-interfered");
    }
}

fn boxes_edges(s: &mut dyn Session, second: Option<([f64; 3], [f64; 3])>) {
    make_box(s, "b1", [0.; 3], [10., 10., 10.]);
    if let Some((at, size)) = second {
        make_box(s, "b2", at, size);
        compound_exploded_many(s, &["b1", "b2"], "c");
    } else {
        compound_exploded_many(s, &["b1"], "c");
    }
}

pub fn A1(s: &mut dyn Session) {
    banner();
    boxes_edges(s, None);
    rebuilt(s, 6, 6);
    bopchecked(s, "result");
}

pub fn A2(s: &mut dyn Session) {
    banner();
    boxes_edges(s, Some(([10., 0., 0.], [10., 10., 10.])));
    rebuilt(s, 11, 11);
    bopchecked(s, "result");
}

pub fn A3(s: &mut dyn Session) {
    banner();
    boxes_edges(s, Some(([10., 5., 5.], [10., 10., 10.])));
    rebuilt(s, 13, 13);
    bopchecked(s, "result");
}

pub fn A4(s: &mut dyn Session) {
    banner();
    boxes_edges(s, Some(([10., 0., 0.], [10., 10., 5.])));
    rebuilt(s, 12, 12);
    bopchecked(s, "result");
}

pub fn A5(s: &mut dyn Session) {
    banner();
    boxes_edges(s, Some(([2., 2., 10.], [6., 6., 6.])));
    rebuilt(s, 12, 13);
    bopchecked(s, "result");
}

fn circle_edge(s: &mut dyn Session, name: &str, at: [f64; 3], normal: [f64; 3], radius: f64) {
    let circle = circle_along(at, normal, radius);
    mkedge(s, name, &circle);
}

pub fn D6(s: &mut dyn Session) {
    banner();
    circle_edge(s, "e", [0.; 3], Z, 10.);
    edgestofaces(s, "result", "e");
    checkshape_named(s, "result");
    checked(s, "result", 1, 1);
    bopchecked(s, "result");
}

fn three_circles(s: &mut dyn Session, centers: [[f64; 3]; 3], radii: [f64; 3]) {
    circle_edge(s, "e1", centers[0], Z, radii[0]);
    circle_edge(s, "e2", centers[1], Z, radii[1]);
    circle_edge(s, "e3", centers[2], Z, radii[2]);
    compound(s, &["e1", "e2", "e3"], "c");
}

pub fn D7(s: &mut dyn Session) {
    banner();
    three_circles(s, [[0.; 3], [5., 0., 0.], [5., 5., 0.]], [10., 10., 10.]);
    rebuilt(s, 7, 7);
    bopchecked(s, "result");
}

pub fn D8(s: &mut dyn Session) {
    banner();
    three_circles(s, [[0.; 3], [-2., 0., 0.], [2., 0., 0.]], [10., 5., 5.]);
    rebuilt(s, 4, 5);
    bopchecked(s, "result");
}

pub fn D9(s: &mut dyn Session) {
    banner();
    boxes_edges(s, Some(([10., 0., 0.], [10., 10., 5.])));
    nurbsconvert(s, "c", "c");
    rebuilt(s, 12, 12);
    bopchecked(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    banner();
    circle_edge(s, "ec", [0.; 3], Z, 10.);
    mkedge_line(s, "el", [0.; 3], [1., 0., 0.], (-15., 15.));
    compound(s, &["el", "ec"], "c");
    edgestofaces(s, "result1", "c");
    compound(s, &["ec", "el"], "c");
    edgestofaces(s, "result", "c");
    checkshape_named(s, "result1");
    checkshape_named(s, "result");
    checked(s, "result1", 2, 2);
    checked(s, "result", 2, 2);
    bopchecked(s, "result1");
    bopchecked(s, "result");
}

pub fn E2(s: &mut dyn Session) {
    banner();
    circle_edge(s, "ex", [0.; 3], [1., 0., 0.], 10.);
    circle_edge(s, "ey", [0.; 3], [0., 1., 0.], 10.);
    circle_edge(s, "ez", [0.; 3], Z, 10.);
    mkedge_line(s, "elx", [0.; 3], [1., 0., 0.], (-11., 11.));
    mkedge_line(s, "ely", [0.; 3], [0., 1., 0.], (-11., 11.));
    mkedge_line(s, "elz", [0.; 3], Z, (-11., 11.));
    compound(s, &["ex", "ey", "ez", "elx", "ely", "elz"], "c");
    rebuilt(s, 12, 12);
    bopchecked(s, "result");
}
