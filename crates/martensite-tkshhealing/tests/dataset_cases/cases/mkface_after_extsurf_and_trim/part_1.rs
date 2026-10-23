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

/// tests/mkface/after_extsurf_and_trim/A1.
pub fn a1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("es_line", extsurf(&basis, [1.0, 0.0, 0.0]));
    let basis = v.surface("es_line");
    v.set("es_line_tr", trim(&basis, [0.0, 30.0, 0.0, 50.0]));
    let surface = v.surface("es_line_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 20.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_trim/A2.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("es_line", extsurf(&basis, [1.0, 0.0, 0.0]));
    let basis = v.surface("es_line");
    v.set("es_line_tr", trim(&basis, [0.0, 30.0, 0.0, 50.0]));
    let basis = v.surface("es_line_tr");
    v.set("es_line_tr_tr", trim(&basis, [0.0, 20.0, 0.0, 40.0]));
    let surface = v.surface("es_line_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 10.0, 0.0, 30.0]));
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

/// tests/mkface/after_extsurf_and_trim/A3.
pub fn a3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0]));
    let basis = v.curve("circle");
    v.set("es_circle", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_circle");
    v.set("es_circle_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let surface = v.surface("es_circle_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 2.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_trim/A4.
pub fn a4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0]));
    let basis = v.curve("circle");
    v.set("es_circle", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_circle");
    v.set("es_circle_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let basis = v.surface("es_circle_tr");
    v.set("es_circle_tr_tr", trim(&basis, [0.0, 2.0, 0.0, 30.0]));
    let surface = v.surface("es_circle_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 20.0]));
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

/// tests/mkface/after_extsurf_and_trim/A5.
pub fn a5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set("es_ellipse", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_ellipse");
    v.set("es_ellipse_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let surface = v.surface("es_ellipse_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 2.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_trim/A6.
pub fn a6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set("es_ellipse", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_ellipse");
    v.set("es_ellipse_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let basis = v.surface("es_ellipse_tr");
    v.set("es_ellipse_tr_tr", trim(&basis, [0.0, 2.0, 0.0, 30.0]));
    let surface = v.surface("es_ellipse_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 20.0]));
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

/// tests/mkface/after_extsurf_and_trim/A7.
pub fn a7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set("es_hyperbola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_hyperbola");
    v.set("es_hyperbola_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let surface = v.surface("es_hyperbola_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 2.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_trim/A8.
pub fn a8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set("es_hyperbola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_hyperbola");
    v.set("es_hyperbola_tr", trim(&basis, [0.0, 3.0, 0.0, 50.0]));
    let basis = v.surface("es_hyperbola_tr");
    v.set("es_hyperbola_tr_tr", trim(&basis, [0.0, 2.0, 0.0, 30.0]));
    let surface = v.surface("es_hyperbola_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 20.0]));
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

/// tests/mkface/after_extsurf_and_trim/A9.
pub fn a9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set("es_parabola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_parabola");
    v.set("es_parabola_tr", trim(&basis, [0.0, 50.0, 0.0, 50.0]));
    let surface = v.surface("es_parabola_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 30.0, 0.0, 30.0]));
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

/// tests/mkface/after_extsurf_and_trim/B1.
pub fn b1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set("es_parabola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_parabola");
    v.set("es_parabola_tr", trim(&basis, [0.0, 50.0, 0.0, 50.0]));
    let basis = v.surface("es_parabola_tr");
    v.set("es_parabola_tr_tr", trim(&basis, [0.0, 40.0, 0.0, 40.0]));
    let surface = v.surface("es_parabola_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 30.0, 0.0, 30.0]));
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

/// tests/mkface/after_extsurf_and_trim/B2.
pub fn b2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set("es_beziercurve_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_nr");
    v.set("es_beziercurve_nr_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("es_beziercurve_nr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.7, 0.0, 0.7]));
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

/// tests/mkface/after_extsurf_and_trim/B3.
pub fn b3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set("es_beziercurve_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_nr");
    v.set("es_beziercurve_nr_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_nr_tr");
    v.set(
        "es_beziercurve_nr_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_beziercurve_nr_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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

/// tests/mkface/after_extsurf_and_trim/B4.
pub fn b4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_r",
        beziercurve(&[
            3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0, 1.0,
        ]),
    );
    let basis = v.curve("beziercurve_r");
    v.set("es_beziercurve_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_r");
    v.set("es_beziercurve_r_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("es_beziercurve_r_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
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

/// tests/mkface/after_extsurf_and_trim/B5.
pub fn b5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_r",
        beziercurve(&[
            3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0, 1.0,
        ]),
    );
    let basis = v.curve("beziercurve_r");
    v.set("es_beziercurve_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_r");
    v.set("es_beziercurve_r_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_r_tr");
    v.set("es_beziercurve_r_tr_tr", trim(&basis, [0.0, 0.5, 0.0, 0.5]));
    let surface = v.surface("es_beziercurve_r_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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
