//! `geometry_preview` case bodies: `box ... -preview` gives a vertex, an edge, a
//! rectangle or a box (0031336), each displayed alone, fitted and captured.
use crate::support::{
    curves::shape_set,
    log,
    prim::{box_at, box_min_max, box_preview_at, box_preview_min_max},
    session::{Session, ViewStep},
};
use martensite_tkbrep::topods::Shape;

fn banner(what: &str) {
    log::puts("==================================");
    log::puts("0031336: Modeling data - extend BRepPrimAPI_MakeBox with planar shape creation");
    log::puts(what);
    log::puts("==================================");
}

/// `donly name; fit; checkview -screenshot -2d -path image`.
fn capture(s: &mut dyn Session, name: &str, shape: Shape, image: &str) {
    shape_set(s, name, shape);
    s.step(ViewStep::Donly(&[name]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd(image));
}

/// The preview of a 10-unit box, of a 10x20x30 box by corners, the boxes themselves and
/// the 20-unit box (the case `box`; `box` is a reserved word in Rust).
pub fn box_(s: &mut dyn Session) {
    banner("Check a creation preview if it is a valid box and creation a box");
    s.step(ViewStep::SmallView("AXON"));
    capture(s, "b1", box_preview_at([0.; 3], [10.; 3]), "box_1");
    capture(
        s,
        "b2",
        box_preview_min_max([0.; 3], [10., 20., 30.]),
        "box_2",
    );
    capture(s, "b3", box_at([0.; 3], [10.; 3]), "box_3");
    capture(s, "b4", box_min_max([0.; 3], [10., 20., 30.]), "box_4");
    capture(s, "b5", box_at([0.; 3], [20.; 3]), "box_5");
}

/// The previews of two boxes thin in two directions: edges.
pub fn edge(s: &mut dyn Session) {
    banner("Check a creation preview if it is an edge");
    s.step(ViewStep::SmallView("+X+Y"));
    capture(s, "edge1", box_preview_at([0.; 3], [10., 0., 0.]), "edge_1");
    capture(
        s,
        "edge2",
        box_preview_min_max([30., 0., 0.], [40., 0., 0.]),
        "edge_2",
    );
}

/// The previews of two boxes thin in one direction: rectangles.
pub fn rectangle(s: &mut dyn Session) {
    banner("Check a creation preview if it is a rectangle");
    s.step(ViewStep::SmallView("+X+Y"));
    capture(
        s,
        "rect1",
        box_preview_at([0.; 3], [10., 10., 0.]),
        "rectangle_1",
    );
    capture(
        s,
        "rect2",
        box_preview_min_max([30., 40., 0.], [50., 60., 0.]),
        "rectangle_2",
    );
}

/// The previews of two boxes thin in every direction: vertices.
pub fn vertex(s: &mut dyn Session) {
    banner("Check a creation preview if it is a vertex");
    s.step(ViewStep::SmallView("+X+Y"));
    capture(s, "vertex1", box_preview_at([0.; 3], [0.; 3]), "vertex_1");
    capture(
        s,
        "vertex2",
        box_preview_min_max([30., 0., 0.], [30., 0., 0.]),
        "vertex_2",
    );
}
