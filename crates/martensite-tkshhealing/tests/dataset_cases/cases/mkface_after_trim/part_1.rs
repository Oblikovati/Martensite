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

/// tests/mkface/after_trim/A1.
pub fn a1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", plane(&[0.0, 0.0, 0.0]));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [50.0, 100.0, 50.0, 100.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A1",
    );
}

/// tests/mkface/after_trim/A2.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", plane(&[0.0, 0.0, 0.0]));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.0, 30.0, 0.0, 30.0]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.0, 10.0, 0.0, 10.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A2",
    );
}

/// tests/mkface/after_trim/A3.
pub fn a3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cylinder(&[10.0]));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [0.0, 1.0, 0.0, 30.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A3",
    );
}

/// tests/mkface/after_trim/A4.
pub fn a4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cylinder(&[10.0]));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.0, 1.0, 0.0, 30.0]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.0, 0.5, 0.0, 10.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A4",
    );
}

/// tests/mkface/after_trim/A5.
pub fn a5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cone(&[30.0, 30.0]));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [0.0, 1.0, 0.0, 30.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A5",
    );
}

/// tests/mkface/after_trim/A6.
pub fn a6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", cone(&[30.0, 30.0]));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.0, 1.0, 0.0, 30.0]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.0, 0.5, 0.0, 10.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A6",
    );
}

/// tests/mkface/after_trim/A7.
pub fn a7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", sphere(&[30.0]));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A7",
    );
}

/// tests/mkface/after_trim/A8.
pub fn a8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", sphere(&[30.0]));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.0, 0.5, 0.0, 0.5]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A8",
    );
}

/// tests/mkface/after_trim/A9.
pub fn a9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", torus(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 30.0, 10.0]));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "A9",
    );
}

/// tests/mkface/after_trim/B1.
pub fn b1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", torus(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 30.0, 10.0]));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.0, 0.5, 0.0, 0.5]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B1",
    );
}

/// tests/mkface/after_trim/B2.
pub fn b2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bsurf_r",
        beziersurf(&[
            3.0, 4.0, 0.0, 0.0, 0.0, 1.0, 10.0, 0.0, 5.0, 1.0, 20.0, 0.0, 0.0, 1.0, 0.0, 10.0, 2.0,
            1.0, 10.0, 10.0, 3.0, 1.0, 20.0, 10.0, 2.0, 1.0, 0.0, 20.0, 10.0, 1.0, 10.0, 20.0,
            20.0, 1.0, 20.0, 20.0, 10.0, 1.0, 0.0, 30.0, 0.0, 1.0, 10.0, 30.0, 0.0, 1.0, 20.0,
            30.0, 0.0, 1.0,
        ]),
    );
    let basis = v.surface("bsurf_r");
    v.set("r", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B2",
    );
}

/// tests/mkface/after_trim/B3.
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
    v.set("bsurf_tr", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let basis = v.surface("bsurf_tr");
    v.set("r", trim(&basis, [0.3, 0.5, 0.3, 0.5]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B3",
    );
}

/// tests/mkface/after_trim/B4.
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
    v.set("r", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B4",
    );
}

/// tests/mkface/after_trim/B5.
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
    v.set("bsurf_tr", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let basis = v.surface("bsurf_tr");
    v.set("r", trim(&basis, [0.3, 0.5, 0.3, 0.5]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B5",
    );
}

/// tests/mkface/after_trim/B6.
pub fn b6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_r"));
    let basis = v.surface("a");
    v.set("r", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B6",
    );
}

/// tests/mkface/after_trim/B7.
pub fn b7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_r"));
    let basis = v.surface("a");
    v.set("a_tr", trim(&basis, [0.5, 0.8, 0.5, 0.8]));
    let basis = v.surface("a_tr");
    v.set("r", trim(&basis, [0.3, 0.5, 0.3, 0.5]));
    let surface = v.surface("r");
    v.set("result", mkface(&surface));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "B7",
    );
}
