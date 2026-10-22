//! Part of the tests/mkface/after_revsurf_and_offset case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871: each
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

/// tests/mkface/after_revsurf_and_offset/A1.
pub fn a1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("rs_line", revsurf(&basis, [0.0, 10.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.surface("rs_line");
    v.set("rs_line_of", offset(&basis, 5.0));
    let surface = v.surface("rs_line_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 10.0, 50.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.38279297262825e-07),
            vertex: Some(5.382792972628252e-07),
        },
        "A1",
    );
}

/// tests/mkface/after_revsurf_and_offset/A2.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("rs_line", revsurf(&basis, [0.0, 10.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.surface("rs_line");
    v.set("rs_line_of", offset(&basis, 5.0));
    let basis = v.surface("rs_line_of");
    v.set("rs_line_of_of", offset(&basis, 5.0));
    let surface = v.surface("rs_line_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 10.0, 50.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(7.176782083002101e-07),
            vertex: Some(7.176782083002104e-07),
        },
        "A2",
    );
}

/// tests/mkface/after_revsurf_and_offset/A3.
pub fn a3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0]));
    let basis = v.curve("circle");
    v.set(
        "rs_circle",
        revsurf(&basis, [0.0, 40.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_circle");
    v.set("rs_circle_of", offset(&basis, 5.0));
    let surface = v.surface("rs_circle_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 1.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(9.824483649014614e-07),
            vertex: Some(9.824483649014618e-07),
        },
        "A3",
    );
}

/// tests/mkface/after_revsurf_and_offset/A4.
pub fn a4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0]));
    let basis = v.curve("circle");
    v.set(
        "rs_circle",
        revsurf(&basis, [0.0, 40.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_circle");
    v.set("rs_circle_of", offset(&basis, 5.0));
    let basis = v.surface("rs_circle_of");
    v.set("rs_circle_of_of", offset(&basis, 5.0));
    let surface = v.surface("rs_circle_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 1.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(8.314584890359682e-07),
            vertex: Some(8.314584890359684e-07),
        },
        "A4",
    );
}

/// tests/mkface/after_revsurf_and_offset/A5.
pub fn a5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set(
        "rs_ellipse",
        revsurf(&basis, [0.0, 40.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_ellipse");
    v.set("rs_ellipse_of", offset(&basis, 5.0));
    let surface = v.surface("rs_ellipse_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 1.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(9.579674907653065e-07),
            vertex: Some(9.57967490765307e-07),
        },
        "A5",
    );
}

/// tests/mkface/after_revsurf_and_offset/A6.
pub fn a6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set(
        "rs_ellipse",
        revsurf(&basis, [0.0, 40.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_ellipse");
    v.set("rs_ellipse_of", offset(&basis, 5.0));
    let basis = v.surface("rs_ellipse_of");
    v.set("rs_ellipse_of_of", offset(&basis, 5.0));
    let surface = v.surface("rs_ellipse_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 1.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(9.80428995460796e-07),
            vertex: Some(9.804289954607965e-07),
        },
        "A6",
    );
}

/// tests/mkface/after_revsurf_and_offset/A7.
pub fn a7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set(
        "rs_hyperbola",
        revsurf(&basis, [0.0, 400.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_hyperbola");
    v.set("rs_hyperbola_of", offset(&basis, 5.0));
    let surface = v.surface("rs_hyperbola_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.805445134738555e-07),
            vertex: Some(4.805445134738557e-07),
        },
        "A7",
    );
}

/// tests/mkface/after_revsurf_and_offset/A8.
pub fn a8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set(
        "rs_hyperbola",
        revsurf(&basis, [0.0, 400.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_hyperbola");
    v.set("rs_hyperbola_of", offset(&basis, 5.0));
    let basis = v.surface("rs_hyperbola_of");
    v.set("rs_hyperbola_of_of", offset(&basis, 5.0));
    let surface = v.surface("rs_hyperbola_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.591929416817199e-07),
            vertex: Some(1.5919294168171995e-07),
        },
        "A8",
    );
}

/// tests/mkface/after_revsurf_and_offset/A9.
pub fn a9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set(
        "rs_parabola",
        revsurf(&basis, [0.0, 400.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_parabola");
    v.set("rs_parabola_of", offset(&basis, 10.0));
    let surface = v.surface("rs_parabola_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.5769964559560117e-07),
            vertex: Some(1.5769964559560122e-07),
        },
        "A9",
    );
}

/// tests/mkface/after_revsurf_and_offset/B1.
pub fn b1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set(
        "rs_parabola",
        revsurf(&basis, [0.0, 400.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_parabola");
    v.set("rs_parabola_of", offset(&basis, 10.0));
    let basis = v.surface("rs_parabola_of");
    v.set("rs_parabola_of_of", offset(&basis, 5.0));
    let surface = v.surface("rs_parabola_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.5770976546370927e-07),
            vertex: Some(1.5770976546370932e-07),
        },
        "B1",
    );
}

/// tests/mkface/after_revsurf_and_offset/B2.
pub fn b2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set(
        "rs_beziercurve_nr",
        revsurf(&basis, [-5.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_beziercurve_nr");
    v.set("rs_beziercurve_nr_of", offset(&basis, 1.0));
    let surface = v.surface("rs_beziercurve_nr_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(2.3677180258750644e-07),
            vertex: Some(2.411363402481465e-07),
        },
        "B2",
    );
}

/// tests/mkface/after_revsurf_and_offset/B3.
pub fn b3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set(
        "rs_beziercurve_nr",
        revsurf(&basis, [-5.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_beziercurve_nr");
    v.set("rs_beziercurve_nr_of", offset(&basis, 1.0));
    let basis = v.surface("rs_beziercurve_nr_of");
    v.set("rs_beziercurve_nr_of_of", offset(&basis, 1.0));
    let surface = v.surface("rs_beziercurve_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.618047209178782e-07),
            vertex: Some(4.618047209178783e-07),
        },
        "B3",
    );
}

/// tests/mkface/after_revsurf_and_offset/B4.
pub fn b4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_r",
        beziercurve(&[
            3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0, 1.0,
        ]),
    );
    let basis = v.curve("beziercurve_r");
    v.set(
        "rs_beziercurve_r",
        revsurf(&basis, [-5.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_beziercurve_r");
    v.set("rs_beziercurve_r_of", offset(&basis, 1.0));
    let surface = v.surface("rs_beziercurve_r_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.003890359842028e-07),
            vertex: Some(6.00389035984203e-07),
        },
        "B4",
    );
}

/// tests/mkface/after_revsurf_and_offset/B5.
pub fn b5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_r",
        beziercurve(&[
            3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0, 1.0,
        ]),
    );
    let basis = v.curve("beziercurve_r");
    v.set(
        "rs_beziercurve_r",
        revsurf(&basis, [-5.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_beziercurve_r");
    v.set("rs_beziercurve_r_of", offset(&basis, 1.0));
    let basis = v.surface("rs_beziercurve_r_of");
    v.set("rs_beziercurve_r_of_of", offset(&basis, 1.0));
    let surface = v.surface("rs_beziercurve_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(3.342899267611633e-07),
            vertex: Some(3.342899267611634e-07),
        },
        "B5",
    );
}
