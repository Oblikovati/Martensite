//! `bsection` bodies written by hand: they read a command's answer or make a face on a wire.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass},
    bopcurves::{BopCurvesOptions, bopcurves_bound},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::explode,
    construction::wire,
    one_step::bsection,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::{Session, ViewStep},
    surface_commands::{circle_oriented, mkedge, mkface_bounded},
    tolerance_check::checkreal,
    topology_commands::mkface_of_wire,
    topology_commands::plane_along,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn M3(s: &mut dyn Session) {
    restore(s, "GEN758_nofog.rle", "a");
    explode(s, "a", Some(ShapeEnum::Face));
    let good_nb_curv = 1;
    let start_points = [[
        0.206_392_063_395_452_24,
        0.692_608_328_433_853,
        0.0,
        0.017_002_507_022_347_624,
    ]];
    let options = BopCurvesOptions {
        pcurves: true,
        start_points: &start_points,
    };
    let curves = bopcurves_bound(s, "a_1", "a_2", options).answer;
    let tolerance_reached = curves.tolerance_reached.expect("a curve and its tolerance");
    checkreal(
        "Tolerance Reached",
        &tolerance_reached,
        "2.8312498500210269e-07",
        0.0,
        1.0e-2,
    );
    if curves.curves != good_nb_curv {
        println!(
            "Error: {good_nb_curv} curves are expected but {} are found!",
            curves.curves
        );
    }
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Clear);
    // The script's other branch never runs: it is reached with more than one curve.
    s.view(ViewStep::Disp(&["c_1"]));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Xwd("bsection_M3_curv"));
    // Beyond the script: an earlier test of this case read two faces.
    part_count(s, "a", Some(ShapeEnum::Face), 2);
    s.view(ViewStep::Donly(&["a_1", "a_2"]));
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(22.2575));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn P9(s: &mut dyn Session) {
    let geom_pl = plane_along([0., 0., 0.], [0., 0., 1.]);
    mkface_bounded(s, "fa", &geom_pl.surface, (-1., 1., -1., 1.));
    let azval = 20.;
    let dipval = 80.;
    let radval = 0.5;
    let az = azval * std::f64::consts::PI / 180.;
    let dip = dipval * std::f64::consts::PI / 180.;
    let dx = martensite_cmath::sin(dip) * martensite_cmath::sin(az);
    let dy = martensite_cmath::sin(dip) * martensite_cmath::cos(az);
    let dz = martensite_cmath::cos(dip);
    let ux = martensite_cmath::cos(az);
    let uy = martensite_cmath::sin(az);
    let uz = 0.;
    let circ_curve = circle_oriented([0.5, 0.5, 0.], [dx, dy, dz], [ux, uy, uz], radval);
    let geom_pl2 = plane_along([0.5, 0.5, 0.], [dx, dy, dz]);
    mkedge(s, "edg", &circ_curve);
    wire(s, "wir", &["edg"]);
    mkface_of_wire(s, "fb", &geom_pl2, "wir");
    bsection(s, "result", "fa", "fb");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["fa", "fb"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "fa", Prop::Area, "4");
    kind_is(s, "fa", ShapeEnum::Face);
    printed_mass(s, "fb", Prop::Area, "0.785398");
    kind_is(s, "fb", ShapeEnum::Face);
}
