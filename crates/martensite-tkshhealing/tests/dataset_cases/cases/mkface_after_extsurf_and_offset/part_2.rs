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

/// after extsurf and offset — B6.
pub fn b6(s: &mut dyn Session) {
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
            face: Some(1e-07),
            edge: Some(6.34648823020948e-07),
            vertex: Some(6.346488230209482e-07),
        },
        "B6",
    );
}

/// after extsurf and offset — B7.
pub fn b7(s: &mut dyn Session) {
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
    let basis = v.surface("es_bspline_3d_np_nc_nr_of");
    v.set("es_bspline_3d_np_nc_nr_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_nc_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(2.5990931826522705e-07),
            vertex: Some(2.5990931826522716e-07),
        },
        "B7",
    );
}

/// after extsurf and offset — B8.
pub fn b8(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_np_nc_r_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_nc_r_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(7.615414472359766e-07),
            vertex: Some(7.615414472359769e-07),
        },
        "B8",
    );
}

/// after extsurf and offset — B9.
pub fn b9(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_np_nc_r_of", offset(&basis, 1.0));
    let basis = v.surface("es_bspline_3d_np_nc_r_of");
    v.set("es_bspline_3d_np_nc_r_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_nc_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.434885516576482e-07),
            vertex: Some(6.434885516576484e-07),
        },
        "B9",
    );
}

/// after extsurf and offset — C1.
pub fn c1(s: &mut dyn Session) {
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
    let basis = v.surface("es_bspline_3d_np_c_nr_of");
    v.set("es_bspline_3d_np_c_nr_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_c_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(2.5990931826522705e-07),
            vertex: Some(2.5990931826522716e-07),
        },
        "C1",
    );
}

/// after extsurf and offset — C2.
pub fn c2(s: &mut dyn Session) {
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
    let basis = v.surface("es_bspline_3d_np_c_nr_of");
    v.set("es_bspline_3d_np_c_nr_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_c_nr_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(2.5990931826522705e-07),
            vertex: Some(2.5990931826522716e-07),
        },
        "C2",
    );
}

/// after extsurf and offset — C3.
pub fn c3(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_np_c_r_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_c_r_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(7.615414472359766e-07),
            vertex: Some(7.615414472359769e-07),
        },
        "C3",
    );
}

/// after extsurf and offset — C4.
pub fn c4(s: &mut dyn Session) {
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
    v.set("es_bspline_3d_np_c_r_of", offset(&basis, 1.0));
    let basis = v.surface("es_bspline_3d_np_c_r_of");
    v.set("es_bspline_3d_np_c_r_of_of", offset(&basis, 1.0));
    let surface = v.surface("es_bspline_3d_np_c_r_of_of");
    v.set("result", mkface_bounds(&surface, [0.0, 1.0, 0.0, 3.0]));
    mkface_end(
        &mut v,
        MaxTolerances {
            face: Some(1e-07),
            edge: Some(6.434885516576482e-07),
            vertex: Some(6.434885516576484e-07),
        },
        "C4",
    );
}

/// after extsurf and offset — C5.
pub fn c5(s: &mut dyn Session) {
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
            edge: Some(8.493383353216945e-07),
            vertex: Some(8.493383353216948e-07),
        },
        "C5",
    );
}

/// after extsurf and offset — C6.
pub fn c6(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set("es_beziercurve_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_nr");
    v.set("es_beziercurve_nr_of", offset(&basis, 1.0));
    log::todo("Error : Incorrect input parameters are not processed correctly");
    log::required("Faulty shapes in variables faulty_1 to faulty");
    let surface = v.surface("es_beziercurve_nr_of");
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
        "C6",
    );
}

/// after extsurf and offset — C7.
pub fn c7(s: &mut dyn Session) {
    let mut v = Vars::new(s);
    v.set(
        "beziercurve_nr",
        beziercurve(&[3.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 4.0, 5.0, 2.0]),
    );
    let basis = v.curve("beziercurve_nr");
    v.set("es_beziercurve_nr", extsurf(&basis, [0.0, 0.0, 1.0]));
    let basis = v.surface("es_beziercurve_nr");
    v.set("es_beziercurve_nr_of", offset(&basis, 1.0));
    let basis = v.surface("es_beziercurve_nr_of");
    v.set("es_beziercurve_nr_of_of", offset(&basis, 5.0));
    log::todo("Error : Incorrect input parameters are not processed correctly");
    log::required("Faulty shapes in variables faulty_1 to faulty");
    let surface = v.surface("es_beziercurve_nr_of_of");
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
        "C7",
    );
}

/// after extsurf and offset — C8.
pub fn c8(s: &mut dyn Session) {
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
    log::todo("Error : Incorrect input parameters are not processed correctly");
    log::required("Faulty shapes in variables faulty_1 to faulty");
    let surface = v.surface("es_beziercurve_r_of");
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
        "C8",
    );
}
