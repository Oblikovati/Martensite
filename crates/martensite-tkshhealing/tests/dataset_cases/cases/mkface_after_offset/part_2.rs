//! Part of the tests/mkface/after_offset case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871: each
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

/// tests/mkface/after_offset/B8.
pub fn b8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_nr"));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 3.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.087704510617566e-07),
            vertex: Some(6.087704510617568e-07),
        },
        "B8",
    );
}

/// tests/mkface/after_offset/B9.
pub fn b9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_np_nr"));
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
            edge: Some(9.463945673648274e-07),
            vertex: Some(9.463945673648276e-07),
        },
        "B9",
    );
}

/// tests/mkface/after_offset/C1.
pub fn c1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_nr"));
    let basis = v.surface("a");
    v.set("r", offset(&basis, 3.0));
    let surface = v.surface("r");
    v.set("result", mkface_bounds(&surface, [0.5, 0.8, 0.5, 0.8]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.313478182679421e-07),
            vertex: Some(6.313478182679423e-07),
        },
        "C1",
    );
}

/// tests/mkface/after_offset/C2.
pub fn c2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("a", restore("bs_s_p_nr"));
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
            edge: Some(8.281447473936777e-07),
            vertex: Some(8.281447473936779e-07),
        },
        "C2",
    );
}
