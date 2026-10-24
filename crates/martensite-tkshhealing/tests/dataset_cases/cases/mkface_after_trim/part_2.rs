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

/// after trim — B8.
pub fn b8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_nr"));
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
        "B8",
    );
}

/// after trim — B9.
pub fn b9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_nr"));
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
        "B9",
    );
}

/// after trim — C1.
pub fn c1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_nr"));
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
        "C1",
    );
}

/// after trim — C2.
pub fn c2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_nr"));
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
        "C2",
    );
}

/// after trim — C3.
pub fn c3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_r"));
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
        "C3",
    );
}

/// after trim — C4.
pub fn c4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_r"));
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
        "C4",
    );
}
