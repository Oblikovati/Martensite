//! Face construction regression fixtures: each
//! builds its curve and surface with the GeomliteTest commands, makes the face (the grid
//! end makes it from `r` where the case does not), sets the expected maximum tolerances,
//! then runs the mkface group end ([`super::mkface_grid::mkface_end`]).
#![allow(unused_imports)] // Each grid uses the commands its scripts call.
use super::super::mkface_grid::{MaxTolerances, mkface_end};
use crate::support::{
    geom_create::mkface_bounds_checked,
    geom_create::*,
    log,
    restore::restore,
    session::Session,
    topology::{mkface, mkface_bounds},
    vars::Vars,
};

/// tests/mkface/after_offset/A1.
pub fn a1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", plane(&[0.0, 0.0, 0.0]));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A1",
    );
}

/// tests/mkface/after_offset/A2.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", plane(&[0.0, 0.0, 0.0]));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 30.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A2",
    );
}

/// tests/mkface/after_offset/A3.
pub fn a3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cylinder(&[10.0]));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A3",
    );
}

/// tests/mkface/after_offset/A4.
pub fn a4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cylinder(&[10.0]));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 30.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A4",
    );
}

/// tests/mkface/after_offset/A5.
pub fn a5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cone(&[30.0, 30.0]));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A5",
    );
}

/// tests/mkface/after_offset/A6.
pub fn a6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cone(&[30.0, 30.0]));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 30.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A6",
    );
}

/// tests/mkface/after_offset/A7.
pub fn a7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", sphere(&[30.0]));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A7",
    );
}

/// tests/mkface/after_offset/A8.
pub fn a8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", sphere(&[30.0]));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 30.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 30.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A8",
    );
}

/// tests/mkface/after_offset/A9.
pub fn a9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", torus(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 30.0, 10.0]));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "A9",
    );
}

/// tests/mkface/after_offset/B1.
pub fn b1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", torus(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 30.0, 10.0]));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 5.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1.0000000000000002e-07),
        },
        "B1",
    );
}

/// tests/mkface/after_offset/B2.
pub fn b2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bsurf",
        beziersurf(&[
            3.0, 4.0, 0.0, 0.0, 0.0, 1.0, 10.0, 0.0, 5.0, 1.0, 20.0, 0.0, 0.0, 1.0, 0.0, 10.0, 2.0,
            1.0, 10.0, 10.0, 3.0, 1.0, 20.0, 10.0, 2.0, 1.0, 0.0, 20.0, 10.0, 1.0, 10.0, 20.0,
            20.0, 1.0, 20.0, 20.0, 10.0, 1.0, 0.0, 30.0, 0.0, 1.0, 10.0, 30.0, 0.0, 1.0, 20.0,
            30.0, 0.0, 1.0,
        ]),
    );
    let basis = v.surface("bsurf");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.0146174092815342e-06),
            vertex: Some(1.0146174092815346e-06),
        },
        "B2",
    );
}

/// tests/mkface/after_offset/B3.
pub fn b3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bsurf",
        beziersurf(&[
            3.0, 4.0, 0.0, 0.0, 0.0, 1.0, 10.0, 0.0, 5.0, 1.0, 20.0, 0.0, 0.0, 1.0, 0.0, 10.0, 2.0,
            1.0, 10.0, 10.0, 3.0, 1.0, 20.0, 10.0, 2.0, 1.0, 0.0, 20.0, 10.0, 1.0, 10.0, 20.0,
            20.0, 1.0, 20.0, 20.0, 10.0, 1.0, 0.0, 30.0, 0.0, 1.0, 10.0, 30.0, 0.0, 1.0, 20.0,
            30.0, 0.0, 1.0,
        ]),
    );
    let basis = v.surface("bsurf");
    v.set("bsurf_of", offset(&basis, 5.0));
    let basis = v.surface("bsurf_of");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.083051088685794e-06),
            vertex: Some(1.0830510886857944e-06),
        },
        "B3",
    );
}

/// tests/mkface/after_offset/B4.
pub fn b4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bsurf",
        beziersurf(&[
            3.0, 4.0, 0.0, 0.0, 0.0, 2.0, 10.0, 0.0, 5.0, 1.0, 20.0, 0.0, 0.0, 2.0, 0.0, 10.0, 2.0,
            3.0, 10.0, 10.0, 3.0, 1.0, 20.0, 10.0, 2.0, 3.0, 0.0, 20.0, 10.0, 1.0, 10.0, 20.0,
            20.0, 1.0, 20.0, 20.0, 10.0, 1.0, 0.0, 30.0, 0.0, 2.0, 10.0, 30.0, 0.0, 1.0, 20.0,
            30.0, 0.0, 1.0,
        ]),
    );
    let basis = v.surface("bsurf");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.742217317954965e-07),
            vertex: Some(4.742217317954966e-07),
        },
        "B4",
    );
}

/// tests/mkface/after_offset/B5.
pub fn b5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bsurf",
        beziersurf(&[
            3.0, 4.0, 0.0, 0.0, 0.0, 2.0, 10.0, 0.0, 5.0, 1.0, 20.0, 0.0, 0.0, 2.0, 0.0, 10.0, 2.0,
            3.0, 10.0, 10.0, 3.0, 1.0, 20.0, 10.0, 2.0, 3.0, 0.0, 20.0, 10.0, 1.0, 10.0, 20.0,
            20.0, 1.0, 20.0, 20.0, 10.0, 1.0, 0.0, 30.0, 0.0, 2.0, 10.0, 30.0, 0.0, 1.0, 20.0,
            30.0, 0.0, 1.0,
        ]),
    );
    let basis = v.surface("bsurf");
    v.set("bsurf_of", offset(&basis, 5.0));
    let basis = v.surface("bsurf_of");
    v.set("r", offset(&basis, 5.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.015093970246548e-07),
            vertex: Some(6.01509397024655e-07),
        },
        "B5",
    );
}

/// tests/mkface/after_offset/B6.
pub fn b6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_r"));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 3.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.487218577037974e-07),
            vertex: Some(6.487218577037976e-07),
        },
        "B6",
    );
}

/// tests/mkface/after_offset/B7.
pub fn b7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_r"));
    let basis = v.surface("a");
    v.set("a_of", offset(&basis, 3.0));
    let basis = v.surface("a_of");
    v.set("r", offset(&basis, 3.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.60728449446334e-07),
            vertex: Some(5.607284494463342e-07),
        },
        "B7",
    );
}
