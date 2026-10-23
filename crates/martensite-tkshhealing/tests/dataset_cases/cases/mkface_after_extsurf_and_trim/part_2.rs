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

/// tests/mkface/after_extsurf_and_trim/B6.
pub fn b6(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_p_nr_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("es_bspline_3d_p_nr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
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

/// tests/mkface/after_extsurf_and_trim/B7.
pub fn b7(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_p_nr_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_nr_tr");
    v.set(
        "es_bspline_3d_p_nr_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_p_nr_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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

/// tests/mkface/after_extsurf_and_trim/B8.
pub fn b8(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_p_r_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let surface = v.surface("es_bspline_3d_p_r_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
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

/// tests/mkface/after_extsurf_and_trim/B9.
pub fn b9(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_p_r_tr", trim(&basis, [0.0, 1.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_p_r_tr");
    v.set(
        "es_bspline_3d_p_r_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_p_r_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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

/// tests/mkface/after_extsurf_and_trim/C1.
pub fn c1(s: &mut dyn Session) {
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
    v.set(
        "es_bspline_3d_np_nc_nr_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let surface = v.surface("es_bspline_3d_np_nc_nr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
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

/// tests/mkface/after_extsurf_and_trim/C2.
pub fn c2(s: &mut dyn Session) {
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
    v.set(
        "es_bspline_3d_np_nc_nr_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let basis = v.surface("es_bspline_3d_np_nc_nr_tr");
    v.set(
        "es_bspline_3d_np_nc_nr_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_np_nc_nr_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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

/// tests/mkface/after_extsurf_and_trim/C3.
pub fn c3(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_nc_r",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 2.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_nc_r");
    v.set("es_bspline_3d_np_nc_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_nc_r");
    v.set(
        "es_bspline_3d_np_nc_r_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let surface = v.surface("es_bspline_3d_np_nc_r_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
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

/// tests/mkface/after_extsurf_and_trim/C4.
pub fn c4(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_nc_r",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 2.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_nc_r");
    v.set("es_bspline_3d_np_nc_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_nc_r");
    v.set(
        "es_bspline_3d_np_nc_r_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let basis = v.surface("es_bspline_3d_np_nc_r_tr");
    v.set(
        "es_bspline_3d_np_nc_r_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_np_nc_r_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
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

/// tests/mkface/after_extsurf_and_trim/C5.
pub fn c5(s: &mut dyn Session) {
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
    v.set(
        "es_bspline_3d_np_c_nr_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let surface = v.surface("es_bspline_3d_np_c_nr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "C5",
    );
}

/// tests/mkface/after_extsurf_and_trim/C6.
pub fn c6(s: &mut dyn Session) {
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
    v.set(
        "es_bspline_3d_np_c_nr_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let basis = v.surface("es_bspline_3d_np_c_nr_tr");
    v.set(
        "es_bspline_3d_np_c_nr_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_np_c_nr_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "C6",
    );
}

/// tests/mkface/after_extsurf_and_trim/C7.
pub fn c7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_c_r",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_c_r");
    v.set("es_bspline_3d_np_c_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_c_r");
    v.set(
        "es_bspline_3d_np_c_r_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let surface = v.surface("es_bspline_3d_np_c_r_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.5, 0.0, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "C7",
    );
}

/// tests/mkface/after_extsurf_and_trim/C8.
pub fn c8(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "bspline_3d_np_c_r",
        bsplinecurve(
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0,
            ],
            false,
        ),
    );
    let basis = v.curve("bspline_3d_np_c_r");
    v.set("es_bspline_3d_np_c_r", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_bspline_3d_np_c_r");
    v.set(
        "es_bspline_3d_np_c_r_tr",
        trim(&basis, [0.0, 1.0, 0.0, 1.0]),
    );
    let basis = v.surface("es_bspline_3d_np_c_r_tr");
    v.set(
        "es_bspline_3d_np_c_r_tr_tr",
        trim(&basis, [0.0, 0.5, 0.0, 0.5]),
    );
    let surface = v.surface("es_bspline_3d_np_c_r_tr_tr");
    v.set("result", mkface_bounds(&surface, [0.0, 0.25, 0.0, 0.25]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1e-07),
            vertex: Some(1e-07),
        },
        "C8",
    );
}
