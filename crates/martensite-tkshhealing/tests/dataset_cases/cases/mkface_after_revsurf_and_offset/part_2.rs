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

/// tests/mkface/after_revsurf_and_offset/B6.
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
    v.set(
        "rs_bspline_3d_p_nr",
        revsurf(&basis, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_p_nr");
    v.set("rs_bspline_3d_p_nr_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_p_nr_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(4.7047338874526666e-07),
            vertex: Some(4.7047338874526677e-07),
        },
        "B6",
    );
}

/// tests/mkface/after_revsurf_and_offset/B7.
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
    v.set(
        "rs_bspline_3d_p_nr",
        revsurf(&basis, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_p_nr");
    v.set("rs_bspline_3dp_p_nr_of", offset(&basis, -0.1));
    let basis = v.surface("rs_bspline_3dp_p_nr_of");
    v.set("rs_bspline_3d_p_nr_of_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_p_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.706032179330517e-07),
            vertex: Some(6.706032179330519e-07),
        },
        "B7",
    );
}

/// tests/mkface/after_revsurf_and_offset/B8.
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
    v.set(
        "rs_bspline_3d_p_r",
        revsurf(&basis, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_p_r");
    v.set("rs_bspline_3d_p_r_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_p_r_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.896457395680723e-07),
            vertex: Some(5.896457395680725e-07),
        },
        "B8",
    );
}

/// tests/mkface/after_revsurf_and_offset/B9.
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
    v.set(
        "rs_bspline_3d_p_r",
        revsurf(&basis, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_p_r");
    v.set("rs_bspline_3d_p_r_of", offset(&basis, -0.1));
    let basis = v.surface("rs_bspline_3d_p_r_of");
    v.set("rs_bspline_3d_p_r_of_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_p_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(1.3737211935176142e-06),
            vertex: Some(1.3737211935176146e-06),
        },
        "B9",
    );
}

/// tests/mkface/after_revsurf_and_offset/C1.
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
    v.set(
        "rs_bspline_3d_np_nc_nr",
        revsurf(&basis, [0.0, -1.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_nc_nr");
    v.set("rs_bspline_3d_np_nc_nr_of", offset(&basis, 0.1));
    let surface = v.surface("rs_bspline_3d_np_nc_nr_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.774664910922965e-07),
            vertex: Some(5.774664910922967e-07),
        },
        "C1",
    );
}

/// tests/mkface/after_revsurf_and_offset/C2.
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
    v.set(
        "rs_bspline_3d_np_nc_nr",
        revsurf(&basis, [0.0, -1.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_nc_nr");
    v.set("rs_bspline_3d_np_nc_nr_of", offset(&basis, 0.1));
    let basis = v.surface("rs_bspline_3d_np_nc_nr_of");
    v.set("rs_bspline_3d_np_nc_nr_of_of", offset(&basis, 0.1));
    let surface = v.surface("rs_bspline_3d_np_nc_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(7.012143102330512e-07),
            vertex: Some(7.012143102330515e-07),
        },
        "C2",
    );
}

/// tests/mkface/after_revsurf_and_offset/C3.
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
    v.set(
        "rs_bspline_3d_np_nc_r",
        revsurf(&basis, [0.0, -1.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_nc_r");
    v.set("rs_bspline_3d_np_nc_r_of", offset(&basis, 0.1));
    let surface = v.surface("rs_bspline_3d_np_nc_r_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.061552636000902e-07),
            vertex: Some(6.061552636000904e-07),
        },
        "C3",
    );
}

/// tests/mkface/after_revsurf_and_offset/C4.
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
    v.set(
        "rs_bspline_3d_np_nc_r",
        revsurf(&basis, [0.0, -1.0, 0.0, 1.0, 0.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_nc_r");
    v.set("rs_bspline_3d_np_nc_r_of", offset(&basis, 0.1));
    let basis = v.surface("rs_bspline_3d_np_nc_r_of");
    v.set("rs_bspline_3d_np_nc_r_of_of", offset(&basis, 0.1));
    let surface = v.surface("rs_bspline_3d_np_nc_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.499359797083884e-07),
            vertex: Some(5.499359797083886e-07),
        },
        "C4",
    );
}

/// tests/mkface/after_revsurf_and_offset/C5.
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
    v.set(
        "rs_bspline_3d_np_c_nr",
        revsurf(&basis, [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_c_nr");
    v.set("rs_bspline_3d_np_c_nr_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_np_c_nr_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.749811735211961e-07),
            vertex: Some(5.749811735211963e-07),
        },
        "C5",
    );
}

/// tests/mkface/after_revsurf_and_offset/C6.
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
    v.set(
        "rs_bspline_3d_np_c_nr",
        revsurf(&basis, [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_c_nr");
    v.set("rs_bspline_3d_np_c_nr_of", offset(&basis, -0.1));
    let basis = v.surface("rs_bspline_3d_np_c_nr_of");
    v.set("rs_bspline_3d_np_c_nr_of_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_np_c_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.060553800372629e-07),
            vertex: Some(6.060553800372631e-07),
        },
        "C6",
    );
}

/// tests/mkface/after_revsurf_and_offset/C7.
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
    v.set(
        "rs_bspline_3d_np_c_r",
        revsurf(&basis, [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_c_r");
    v.set("rs_bspline_3d_np_c_r_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_np_c_r_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.152030684877089e-07),
            vertex: Some(5.152030684877091e-07),
        },
        "C7",
    );
}

/// tests/mkface/after_revsurf_and_offset/C8.
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
    v.set(
        "rs_bspline_3d_np_c_r",
        revsurf(&basis, [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
    );
    let basis = v.surface("rs_bspline_3d_np_c_r");
    v.set("rs_bspline_3d_np_c_r_of", offset(&basis, -0.1));
    let basis = v.surface("rs_bspline_3d_np_c_r_of");
    v.set("rs_bspline_3d_np_c_r_of_of", offset(&basis, -0.1));
    let surface = v.surface("rs_bspline_3d_np_c_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.3, 0.5, 0.3, 0.5]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(5.93368834549386e-07),
            vertex: Some(5.933688345493862e-07),
        },
        "C8",
    );
}
