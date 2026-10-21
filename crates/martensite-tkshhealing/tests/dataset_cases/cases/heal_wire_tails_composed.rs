//! tests/heal/wire_tails_composed case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871:
//! each builds planar faces from polygons of vertices (`polyvertex`, `mkplane`; A19 sews
//! four of them), runs `fixshape r s -maxtaila 10 -maxtailw w` (ShapeFix_Wire::FixTails)
//! and checks the counts and the length of `r`. tests/heal/end then shows `r` in the 3D
//! viewer (`checkview -display r -3d -vdispmode 0`), which these crate tests do not
//! reproduce.
use crate::support::{
    build::polyvertex,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops},
    session::{Session, checkview_display, set},
    sewing::sewing,
    swdraw::fixshape,
    topology::{mkplane, vertex},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;

/// `vertex name x y z`.
fn vertex_at(s: &mut dyn Session, name: &str, [x, y, z]: [f64; 3]) -> Shape {
    set(s, name, vertex(Point3::new(x, y, z)))
}

/// `polyvertex w ...` then `mkplane s w`.
fn plane_of(s: &mut dyn Session, names: (&str, &str), vertices: &[&Shape]) -> Shape {
    let w = set(s, names.0, polyvertex(vertices));
    set(
        s,
        names.1,
        mkplane(&w).unwrap_or_else(|error| panic!("{error}")),
    )
}

/// `fixshape r s -maxtaila 10 -maxtailw width`, `checknbshapes r ...` and
/// `checkprops r -l length`.
fn fix_tails(s: &mut dyn Session, face: &Shape, width: &str, counts: [usize; 8], length: f64) {
    let r = set(
        s,
        "r",
        fixshape(face, &["-maxtaila", "10", "-maxtailw", width]),
    );
    let mut expected = [None; 9];
    for (slot, count) in expected.iter_mut().zip(counts) {
        *slot = Some(count);
    }
    checknbshapes_some(&r, expected);
    checkprops(&r, Prop::Length, length);
    // tests/heal/end: `isdraw result` is false and `isdraw r` true, so it displays
    // r in the AXON 3D view, fits it, and captures the result.
    checkview_display(s, Some("r"), "wire-tails-composed.xwd");
}

/// tests/heal/wire_tails_composed/A1.
pub fn a1(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v2, &v3, &v1]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.054743851305652,
    );
}

/// tests/heal/wire_tails_composed/A2.
pub fn a2(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v3, &v1, &v2]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.054743851305652,
    );
}

/// tests/heal/wire_tails_composed/A3.
pub fn a3(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v1, &v2, &v3]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.054743851305652,
    );
}

/// tests/heal/wire_tails_composed/A4.
pub fn a4(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v2, &v1, &v3]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.0547440573886373,
    );
}

/// tests/heal/wire_tails_composed/A5.
pub fn a5(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v1, &v3, &v2]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.0547440573886373,
    );
}

/// tests/heal/wire_tails_composed/A6.
pub fn a6(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v3, &v2, &v1]);
    fix_tails(
        s,
        &s_face,
        "0.7",
        [3, 3, 1, 1, 0, 0, 0, 0],
        9.0547440573886373,
    );
}

/// tests/heal/wire_tails_composed/A7.
pub fn a7(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v2, &v3, &v1]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A8.
pub fn a8(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v3, &v1, &v2]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A9.
pub fn a9(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v1, &v2, &v3]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A10.
pub fn a10(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v2, &v1, &v3]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A11.
pub fn a11(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v1, &v3, &v2]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A12.
pub fn a12(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-2.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v3, &v2, &v1]);
    fix_tails(
        s,
        &s_face,
        "1",
        [2, 2, 1, 1, 0, 0, 0, 0],
        6.1622776601683791,
    );
}

/// tests/heal/wire_tails_composed/A13.
pub fn a13(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v2, &v3, &v1]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A14.
pub fn a14(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v3, &v1, &v2]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A15.
pub fn a15(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v1, &v2, &v3]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A16.
pub fn a16(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v3, &v2, &v1, &v3]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A17.
pub fn a17(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v2, &v1, &v3, &v2]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A18.
pub fn a18(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [0.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let s_face = plane_of(s, ("w", "s"), &[&v1, &v3, &v2, &v1]);
    fix_tails(
        s,
        &s_face,
        "1.5",
        [1, 1, 1, 1, 0, 0, 0, 0],
        1.4142135623730949,
    );
}

/// tests/heal/wire_tails_composed/A19.
pub fn a19(s: &mut dyn Session) {
    let v1 = vertex_at(s, "v1", [-1.0, 0.0, 0.0]);
    let v2 = vertex_at(s, "v2", [9.0, 0.0, 0.0]);
    let v3 = vertex_at(s, "v3", [1.0, 1.0, 0.0]);
    let v4 = vertex_at(s, "v4", [1.0, 1.0, 5.0]);
    let s1 = plane_of(s, ("w1", "s1"), &[&v1, &v3, &v2, &v1]);
    let s2 = plane_of(s, ("w2", "s2"), &[&v1, &v2, &v4, &v1]);
    let s3 = plane_of(s, ("w3", "s3"), &[&v2, &v3, &v4, &v2]);
    let s4 = plane_of(s, ("w4", "s4"), &[&v1, &v4, &v3, &v1]);
    let s_face = set(
        s,
        "s",
        sewing(1.0e-6, &[&s1, &s2, &s3, &s4]).expect("sewing"),
    );
    fix_tails(
        s,
        &s_face,
        "0.7",
        [5, 7, 4, 4, 1, 0, 0, 0],
        69.237606465667483,
    );
}
