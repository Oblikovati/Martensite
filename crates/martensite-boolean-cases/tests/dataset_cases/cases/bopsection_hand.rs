//! `bopsection` bodies written by hand: the case below has a budget of its own.
use crate::support::{
    beyond_script::{kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn E3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(40.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "600");
    kind_is(s, "b1", ShapeEnum::Shell);
    printed_mass(s, "b2", Prop::Area, "400");
    kind_is(s, "b2", ShapeEnum::Shell);
}
