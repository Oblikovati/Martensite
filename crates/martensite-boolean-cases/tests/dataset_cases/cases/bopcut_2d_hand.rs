//! `bopcut_2d` bodies written by hand: they keep checks an earlier test made during the script.
use crate::support::{
    beyond_script::{input_state, kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    construction::setflags_locked,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
    volume_commands::settolerance,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn D5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    input_state(s, "a", false, 1e-7);
    input_state(s, "b", false, 1e-7);
    settolerance(s, "a", 0.1);
    setflags_locked(s, "b");
    input_state(s, "a", false, 0.1);
    input_state(s, "b", true, 1e-7);
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Wire);
}
