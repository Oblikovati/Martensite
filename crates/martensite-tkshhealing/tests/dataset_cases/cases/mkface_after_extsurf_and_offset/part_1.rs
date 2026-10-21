//! Part of the tests/mkface/after_extsurf_and_offset case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871: each
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

/// tests/mkface/after_extsurf_and_offset/A1.
pub fn a1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("es_line", extsurf(&basis, [1.0, 0.0, 0.0]));
    let basis = v.surface("es_line");
    v.set("es_line_of", offset(&basis, 5.0));
    let surface = v.surface("es_line_of");
    v.set("result", mkface_bounds(&surface, [0.0, 20.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_offset/A2.
pub fn a2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("line", line([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    let basis = v.curve("line");
    v.set("es_line", extsurf(&basis, [1.0, 0.0, 0.0]));
    let basis = v.surface("es_line");
    v.set("es_line_of", offset(&basis, 5.0));
    let basis = v.surface("es_line_of");
    v.set("es_line_of_of", offset(&basis, 5.0));
    let surface = v.surface("es_line_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 20.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_offset/A3.
pub fn a3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0]));
    let basis = v.curve("circle");
    v.set("es_circle", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_circle");
    v.set("es_circle_of", offset(&basis, 5.0));
    let surface = v.surface("es_circle_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.1864731039853525e-06),
            vertex: Some(1.186473103985353e-06),
        },
        "A3",
    );
}

/// tests/mkface/after_extsurf_and_offset/A4.
pub fn a4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("circle", circle(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0]));
    let basis = v.curve("circle");
    v.set("es_circle", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_circle");
    v.set("es_circle_of", offset(&basis, 5.0));
    let basis = v.surface("es_circle_of");
    v.set("es_circle_of_of", offset(&basis, 5.0));
    let surface = v.surface("es_circle_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.4095203923333596e-07),
            vertex: Some(1.4095203923333602e-07),
        },
        "A4",
    );
}

/// tests/mkface/after_extsurf_and_offset/A5.
pub fn a5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set("es_ellipse", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_ellipse");
    v.set("es_ellipse_of", offset(&basis, 5.0));
    let surface = v.surface("es_ellipse_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(7.005803463115772e-07),
            vertex: Some(7.005803463115774e-07),
        },
        "A5",
    );
}

/// tests/mkface/after_extsurf_and_offset/A6.
pub fn a6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("ellipse", ellipse(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("ellipse");
    v.set("es_ellipse", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_ellipse");
    v.set("es_ellipse_of", offset(&basis, 5.0));
    let basis = v.surface("es_ellipse_of");
    v.set("es_ellipse_of_of", offset(&basis, 5.0));
    let surface = v.surface("es_ellipse_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(8.493383353216945e-07),
            vertex: Some(8.493383353216948e-07),
        },
        "A6",
    );
}

/// tests/mkface/after_extsurf_and_offset/A7.
pub fn a7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set("es_hyperbola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_hyperbola");
    v.set("es_hyperbola_of", offset(&basis, 5.0));
    let surface = v.surface("es_hyperbola_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.975826575992074e-07),
            vertex: Some(4.975826575992076e-07),
        },
        "A7",
    );
}

/// tests/mkface/after_extsurf_and_offset/A8.
pub fn a8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("hyperbola", hyperbola(&[0.0, 0.0, 0.0, 30.0, 10.0]));
    let basis = v.curve("hyperbola");
    v.set("es_hyperbola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_hyperbola");
    v.set("es_hyperbola_of", offset(&basis, 5.0));
    let basis = v.surface("es_hyperbola_of");
    v.set("es_hyperbola_of_of", offset(&basis, 5.0));
    let surface = v.surface("es_hyperbola_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(9.953938903335782e-07),
            vertex: Some(9.953938903335786e-07),
        },
        "A8",
    );
}

/// tests/mkface/after_extsurf_and_offset/A9.
pub fn a9(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set("es_parabola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_parabola");
    v.set("es_parabola_of", offset(&basis, 5.0));
    let surface = v.surface("es_parabola_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.0920201413103251e-06),
            vertex: Some(1.0920201413103255e-06),
        },
        "A9",
    );
}

/// tests/mkface/after_extsurf_and_offset/B1.
pub fn b1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set("parabola", parabola(&[0.0, 0.0, 0.0, 30.0]));
    let basis = v.curve("parabola");
    v.set("es_parabola", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_parabola");
    v.set("es_parabola_of", offset(&basis, 5.0));
    let basis = v.surface("es_parabola_of");
    v.set("es_parabola_of_of", offset(&basis, 5.0));
    let surface = v.surface("es_parabola_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 3.0, 0.0, 40.0]));
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

/// tests/mkface/after_extsurf_and_offset/B2.
pub fn b2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_p_nr",
        bsplinecurve(
            &[
                2.0, 4.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0,
                0.0, 1.0, 2.0, 0.0, 0.0, 1.0,
            ],
            true,
        ),
    );
    let basis = v.curve("bspline_3d_p_nr");
    v.set("es_bspline_3d_p_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_nr");
    v.set("es_bspline_3d_p_nr_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_p_nr_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.603940224995333e-07),
            vertex: Some(5.603940224995335e-07),
        },
        "B2",
    );
}

/// tests/mkface/after_extsurf_and_offset/B3.
pub fn b3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_p_nr",
        bsplinecurve(
            &[
                2.0, 4.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0,
                0.0, 1.0, 2.0, 0.0, 0.0, 1.0,
            ],
            true,
        ),
    );
    let basis = v.curve("bspline_3d_p_nr");
    v.set("es_bspline_3d_p_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_nr");
    v.set("es_bspline_3d_p_nr_of", offset(&basis, 1.0));
    let basis = v.surface("es_bspline_3d_p_nr_of");
    v.set("es_bspline_3d_p_nr_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_p_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(3.2685874571489216e-07),
            vertex: Some(3.2685874571489216e-07),
        },
        "B3",
    );
}

/// tests/mkface/after_extsurf_and_offset/B4.
pub fn b4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_p_r",
        bsplinecurve(
            &[
                2.0, 4.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0,
                0.0, 1.0, 2.0, 0.0, 0.0, 1.0,
            ],
            true,
        ),
    );
    let basis = v.curve("bspline_3d_p_r");
    v.set("es_bspline_3d_p_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_r");
    v.set("es_bspline_3d_p_r_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_p_r_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(2.1334895038963668e-07),
            vertex: Some(2.1334895038963673e-07),
        },
        "B4",
    );
}

/// tests/mkface/after_extsurf_and_offset/B5.
pub fn b5(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_p_r",
        bsplinecurve(
            &[
                2.0, 4.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0,
                0.0, 1.0, 2.0, 0.0, 0.0, 1.0,
            ],
            true,
        ),
    );
    let basis = v.curve("bspline_3d_p_r");
    v.set("es_bspline_3d_p_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_r");
    v.set("es_bspline_3d_p_r_of", offset(&basis, 1.0));
    let basis = v.surface("es_bspline_3d_p_r_of");
    v.set("es_bspline_3d_p_r_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_p_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.593947419289836e-07),
            vertex: Some(4.593947419289837e-07),
        },
        "B5",
    );
}
