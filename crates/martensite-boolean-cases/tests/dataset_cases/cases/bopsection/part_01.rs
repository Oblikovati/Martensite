//! The `bopsection` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    pcylinder(s, "cyl", 1., 4.);
    vertex(s, "va", [0., -1., 0.]);
    vertex(s, "vb", [0., 1., 0.]);
    vertex(s, "vc", [0., 1., 2.]);
    vertex(s, "vd", [0., -1., 2.]);
    edge(s, "e1", "va", "vb");
    edge(s, "e2", "vb", "vc");
    edge(s, "e3", "vc", "vd");
    edge(s, "e4", "vd", "va");
    wire(s, "w", &["e1", "e2", "e3", "e4"]);
    crate::support::topology_commands::mkplane(s, "f", "w", true);
    prism(s, "priz", "f", [1., 0., 0.]);
    two_step.bop(s, "priz", "cyl");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(14.2832));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["priz", "cyl"], Prop::Length),
    );
}

pub fn A2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_fra60275a.rle", "a");
    restore(s, "fra60275b.rle", "b");
    tcopy(s, "b", "w");
    mkplane(s, "f", "w");
    prism(s, "pp", "f", [0., 0., 100.]);
    two_step.bop(s, "a", "pp");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(539.158));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "pp"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "315233");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "pp", Prop::Area, "73184.6");
    kind_is(s, "pp", ShapeEnum::Solid);
}

pub fn A4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_pro13075a.rle", "a");
    restore(s, "CTO904_pro13075b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(99.3262));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts20578a.rle", "a");
    restore(s, "CTO904_cts20578b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(728.763));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_buc60093a.rle", "a");
    restore(s, "buc60093b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(151.416));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60172a.rle", "a");
    restore(s, "buc60172b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(878.56));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro11615a.rle", "a");
    restore(s, "CTO904_pro11615b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(11.6));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "cts17786a.rle", "a");
    restore(s, "CTO904_cts17786b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.0976741));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_fra50047a.rle", "a");
    restore(s, "fra50047b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(628.319));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro9481a.rle", "a");
    restore(s, "pro9481b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(900.779));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "cts20975a.rle", "a");
    restore(s, "CTO904_cts20975b.rle", "b");
    bopaddpcs(s, "a");
    btolx(s, "a");
    bopaddpcs(s, "b");
    btolx(s, "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(128.364));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "5552");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "3915.96");
    kind_is(s, "b", ShapeEnum::Solid);
}

pub fn B7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts21143a.rle", "a");
    restore(s, "cts21143b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(301.584));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_fus-object1.brep", "a");
    restore(s, "CTO909_fus-object2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(120.859));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_bad_cut_outil.brep", "a");
    restore(s, "CTO909_bad_cut_part.brep", "b");
    two_step.bop(s, "b", "a");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(20.7531));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Length),
    );
}

pub fn C5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts20769a.rle", "a");
    restore(s, "cts20769a.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(312.73));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro15203a.rle", "c1");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    restore(s, "pro15203c.rle", "c2");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    two_step.bop(s, "c1", "c2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(24.3614));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Length),
    );
}

pub fn C8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_outil.brep", "a");
    restore(s, "CTO902_cts20528-part.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(382.621));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "m2_o", "m2_o");
    restore(s, "m2_t", "m2_t");
    two_step.bop(s, "m2_o", "m2_t");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(67.466));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["m2_o", "m2_t"], Prop::Length),
    );
}

pub fn D7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("GER60861");
    println!("Section");
    println!();
    restore(s, "CTO909_object.brep", "a");
    restore(s, "CTO909_tool_4.brep", "b");
    explode(s, "a", Some(ShapeEnum::Shell));
    explode(s, "b", Some(ShapeEnum::Shell));
    two_step.bop(s, "a_1", "b_1");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(74.2717));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "b_1"], Prop::Length),
    );
}

pub fn E1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("PRO15203");
    println!("Fuse");
    println!();
    restore(s, "pro15203a.rle", "c1");
    restore(s, "pro15203c.rle", "c2");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    two_step.bop(s, "c1", "c2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(24.3614));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Length),
    );
}

pub fn E2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh2a", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(134.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh4", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(30.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh7", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(40.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh8", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(46.5685));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh9", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(160.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh10", "b1");
    restore(s, "shx", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(212.548));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn E9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(25.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn F1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "g1", "b1");
    restore(s, "g2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(189.813));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}
