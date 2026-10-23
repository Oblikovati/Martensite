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

/// tests/mkface/after_extsurf_and_offset/C9.
pub fn c9(s: &mut dyn Session) {
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
    v.set("es_beziercurve_r_of", offset(&basis, 1.0));
    let basis = v.surface("es_beziercurve_r_of");
    v.set("es_beziercurve_r_of_of", offset(&basis, 5.0));
    log::todo("Error : Incorrect input parameters are not processed correctly");
    log::required("Faulty shapes in variables faulty_1 to faulty");
    let surface = v.surface("es_beziercurve_r_of_of");
    if let Ok(face) = mkface_bounds_checked(&surface, [0.0, 3.0, 0.0, 30.0]) {
        v.set("result", face);
        log::puts("Error : Incorrect input parameters are not processed correctly.");
    }
    mkface_end(
        &mut v,
        MaxTolerances {
            face: None,
            edge: None,
            vertex: None,
        },
        "C9",
    );
}

/// tests/mkface/after_extsurf_and_offset/D1.
pub fn d1(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_nc_nr",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_nc_nr");
    v.set("es_bspline_3d_np_nc_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_nc_nr");
    v.set("es_bspline_3d_np_nc_nr_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_nc_nr_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-06),
            edge: Some(1e-06),
            vertex: Some(1e-06),
        },
        "D1",
    );
}

/// tests/mkface/after_extsurf_and_offset/D2.
pub fn d2(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_c_nr",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_c_nr");
    v.set("es_bspline_3d_np_c_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_c_nr");
    v.set("es_bspline_3d_np_c_nr_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_c_nr_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-06),
            edge: Some(1e-06),
            vertex: Some(1e-06),
        },
        "D2",
    );
}
