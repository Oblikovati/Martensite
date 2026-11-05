//! `bugs_modalg_1` bodies written by hand: their scripts compute their inputs.
use crate::support::commands::prism;
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    nbshapes::{Counting, nbshapes},
    one_step::bsection,
    properties::Prop,
    section_check::checksection,
    session::Session,
    surface_commands::{circle, circle_along, mkedge, polyline, prism_infinite},
};

/// Where `nbshapes` keeps the vertices and the edges.
const VERTEX_ROW: usize = 0;
const EDGE_ROW: usize = 1;

/// The scripts' `for` loop: `n` teeth of a zigzag in one plane of constant `y`.
fn zigzag(n: usize, dx: f64, sx: f64, sz: f64, y: f64) -> Vec<[f64; 3]> {
    let (mut x, mut z) = (0., 0.);
    let mut points = vec![[x, y, z]];
    for _ in 0..n {
        x += dx;
        points.push([x, y, z]);
        x += sx;
        z += sz;
        points.push([x, y, z]);
        x += dx;
        points.push([x, y, z]);
        x += sx;
        z -= sz;
        points.push([x, y, z]);
    }
    points
}

fn banner(name: &str) {
    println!("============");
    println!("{name}");
    println!("============");
}

fn section_checked(s: &mut dyn Session, length: f64) {
    checkprops(s, "result", Prop::Length, Expected::Mass(length));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60899_1(s: &mut dyn Session) {
    banner("BUC60899");
    let (n, dx, sx) = (2., 4., 1.);
    let xc = (2. * dx + 2. * sx) * n / 2.;
    let zc = -30.;
    let r = xc * 0.9;
    let y = -50.;
    polyline(s, "w", &zigzag(2, dx, sx, 10., y));
    mkedge(s, "c", &circle([xc, 0., zc], r));
    prism_infinite(s, "cyl", "c", [0., 0., -zc * 2.]);
    prism_infinite(s, "crg", "w", [0., -y * 2., 0.]);
    bsection(s, "result", "crg", "cyl");
    section_checked(s, 110.093);
}

pub fn buc60899_2(s: &mut dyn Session) {
    banner("BUC60899");
    let (n, dx, sx) = (2., 4., 1.);
    let xc = (2. * dx + 2. * sx) * n / 2.;
    let zc = -30.;
    let r = xc * 0.9;
    let y = -50.;
    polyline(s, "w", &zigzag(2, dx, sx, 10., y));
    let corners = [
        [xc - r, -r * 2., zc],
        [xc + r, -r * 2., zc],
        [xc + r, r * 2., zc],
        [xc - r, r * 2., zc],
        [xc - r, -r * 2., zc],
    ];
    polyline(s, "c", &corners);
    prism_infinite(s, "cyl", "c", [0., 0., -zc * 2.]);
    prism_infinite(s, "crg", "w", [0., -y * 2., 0.]);
    bsection(s, "result", "crg", "cyl");
    if nbshapes(s, "result", Counting::Located)[EDGE_ROW] == 1 {
        println!("Shape is empty, it is Faulty result");
    } else {
        println!("Shape is not empty, OK");
    }
    section_checked(s, 162.299);
}

pub fn buc60901(s: &mut dyn Session) {
    banner("BUC60901");
    let (n, dx, sx, sz) = (3., 1., 20., 10.);
    let xc = (2. * dx + 2. * sx) * n / 2.;
    let zc = sz * 10.;
    let r = xc * 0.9;
    let y = -r * 1.2;
    polyline(s, "w", &zigzag(3, dx, sx, sz, y));
    mkedge(s, "c", &circle_along([xc, 0., zc], [0.5, -0.3, 1.], r));
    prism(s, "cyl", "c", [0., 0., -zc * 2.]);
    prism(s, "crg", "w", [0., -y * 2., 0.]);
    bsection(s, "result", "crg", "cyl");
    let counts = nbshapes(s, "result", Counting::Located);
    if counts[VERTEX_ROW] != counts[EDGE_ROW] {
        println!("Section is opened: Faulty shape");
    } else {
        println!("Section is closed: OK");
    }
    section_checked(s, 350.404);
}
