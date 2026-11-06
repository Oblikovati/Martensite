//! The `bopfuse_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn H5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a158", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(31198.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a148", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(35342.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(36598.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(10603.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(9581.86));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a157", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(9581.86));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a158", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(47123.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a160", "a");
    restore(s, "b160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(58622.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a206", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(33784.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a206", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(35044.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a350", "a");
    restore(s, "b350", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(120.576));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_fra60275a.rle", "a");
    restore(s, "fra60275b.rle", "b");
    tcopy(s, "b", "w");
    mkplane(s, "f", "w");
    prism(s, "pp", "f", [0., 0., 100.]);
    two_step.bop(s, "a", "pp");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(358551.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "pp"], Prop::Area),
    );
}

pub fn J9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts20578a.rle", "a");
    restore(s, "CTO904_cts20578b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(759758.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_buc60093a.rle", "a");
    restore(s, "buc60093b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(4356.19));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro11615a.rle", "a");
    restore(s, "CTO904_pro11615b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(418.68));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "cts17786a.rle", "a");
    restore(s, "CTO904_cts17786b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(0.00372003));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_fra50047a.rle", "a");
    restore(s, "fra50047b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(508496.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro9481a.rle", "a");
    restore(s, "pro9481b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(448769.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts21143a.rle", "a");
    restore(s, "cts21143b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(37364.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO907_topo002-o1.brep", "coque1");
    restore(s, "CTO907_topo002-o2.brep", "coque2");
    explode(s, "coque1", Some(ShapeEnum::Shell));
    explode(s, "coque2", Some(ShapeEnum::Shell));
    two_step.bop(s, "coque1_1", "coque2_1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(3950.89));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["coque1_1", "coque2_1"], Prop::Area),
    );
}

pub fn K9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_fus-object1.brep", "a");
    restore(s, "CTO909_fus-object2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(67333.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "Jyls3", "a");
    explode(s, "a", Some(ShapeEnum::Shell));
    copy(s, "a_1", "a");
    restore(s, "Jyls4", "b");
    explode(s, "b", Some(ShapeEnum::Shell));
    copy(s, "b_1", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(45600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts20769a.rle", "a");
    restore(s, "cts20769a.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(92385.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro15203a.rle", "c1");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    restore(s, "pro15203c.rle", "c2");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    two_step.bop(s, "c1", "c2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
}

pub fn L6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "pro15203a.rle", "c1");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    restore(s, "pro15203d.rle", "c2");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    two_step.bop(s, "c1", "c2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
}

pub fn L7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_outil.brep", "a");
    restore(s, "CTO902_cts20528-part.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(98808.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("CTS22187");
    println!();
    println!();
    restore(s, "CTO909_fus-object1.brep", "o1");
    restore(s, "CTO909_fus-object2.brep", "o2");
    two_step.bop(s, "o1", "o2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(67333.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["o1", "o2"], Prop::Area),
    );
}

pub fn M1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("GER60861");
    println!("Cut");
    println!();
    restore(s, "CTO909_object.brep", "a");
    restore(s, "CTO909_tool_4.brep", "b");
    explode(s, "a", Some(ShapeEnum::Shell));
    explode(s, "b", Some(ShapeEnum::Shell));
    two_step.bop(s, "a_1", "b_1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(22963.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "b_1"], Prop::Area),
    );
}

pub fn M3(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
}
