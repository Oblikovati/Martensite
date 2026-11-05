//! The `bsection` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn O4(s: &mut dyn Session) {
    restore(s, "pro14151a.rle", "a");
    restore(s, "CFI_pro14fjq.rle", "b");
    bsection(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Length),
    );
}

pub fn O5(s: &mut dyn Session) {
    restore(s, "pro14151a.rle", "a");
    restore(s, "CFI_pro14fjq.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O6(s: &mut dyn Session) {
    restore(s, "pro14151a.rle", "a");
    restore(s, "CFI_pro14fjq.rle", "b");
    bsection(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Length),
    );
}

pub fn O7(s: &mut dyn Session) {
    restore(s, "CIN900_cts21hlo.rle", "a");
    restore(s, "CIN900_cts21hlp.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(751.43));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O8(s: &mut dyn Session) {
    restore(s, "CIN900_cts21hlq.rle", "a");
    restore(s, "CIN900_cts21hlr.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(79.1327));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O9(s: &mut dyn Session) {
    restore(s, "buc60302.rle", "a");
    let geom_planz = plane_along([0., 0., 0.], [0., 0., 1.]);
    mkface_bounded(s, "fass", &geom_planz.surface, (-100., 100., -100., 100.));
    bsection(s, "result", "a", "fass");
    checkprops(s, "result", Prop::Length, Expected::Mass(25.708));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "fass"], Prop::Length),
    );
}

pub fn P1(s: &mut dyn Session) {
    restore(s, "CIN901_intcqhmi.rle", "m");
    explode(s, "m", None);
    compound(s, &["m_1", "m_3"], "c1");
    compound(s, &["m_2", "m_4"], "c2");
    bsection(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Length, Expected::Mass(40.5172));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Length),
    );
}

pub fn P2(s: &mut dyn Session) {
    restore(s, "CIN901_intcqhmj.rle", "m");
    explode(s, "m", None);
    bsection(s, "result", "m_1", "m_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(126.535));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["m_1", "m_2"], Prop::Length),
    );
}

pub fn P4(s: &mut dyn Session) {
    restore(s, "intcq004.brep", "m");
    explode(s, "m", None);
    compound(s, &["m_1", "m_3", "m_5"], "c1");
    bsection(s, "s", "c1", "m_2");
    bsection(s, "s2", "m_4", "m_2");
    compound(s, &["s", "s2"], "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(682.142));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "m_2", "m_4", "m_2"], Prop::Length),
    );
}

pub fn P7(s: &mut dyn Session) {
    restore(s, "CIN902_intcqhmr.rle", "m");
    explode(s, "m", None);
    bsection(s, "result", "m_1", "m_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(202.944));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["m_1", "m_2"], Prop::Length),
    );
}

pub fn Q1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20339a.rle", "a");
    restore(s, "CTO902_cts20339b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1676.89));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn Q2(s: &mut dyn Session) {
    restore(s, "CTO902_cts20527-tool.rle", "a");
    restore(s, "CTO904_cts20527b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(935.487));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn Q3(s: &mut dyn Session) {
    pcylinder(s, "c1", 10., 20.);
    pcylinder(s, "c2", 5., 20.);
    ttranslate(s, "c2", [5., 0., 10.]);
    bsection(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Length, Expected::Mass(41.4159));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Length),
    );
}

pub fn Q4(s: &mut dyn Session) {
    restore(s, "CTO904_cts20370-part.rle", "part");
    restore(s, "cts20370-tool.rle", "tool");
    bsection(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Length, Expected::Mass(1113.89));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Length),
    );
}

pub fn Q5(s: &mut dyn Session) {
    restore(s, "CTO902_cts20482a.rle", "part");
    restore(s, "CTO902_cts20482b.rle", "tool1");
    bsection(s, "result", "part", "tool1");
    checkprops(s, "result", Prop::Length, Expected::Mass(408.37));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool1"], Prop::Length),
    );
}

pub fn Q6(s: &mut dyn Session) {
    restore(s, "CTO902_cts20482a.rle", "part");
    restore(s, "CTO902_cts20482c.rle", "tool2");
    bsection(s, "result", "part", "tool2");
    checkprops(s, "result", Prop::Length, Expected::Mass(492.637));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool2"], Prop::Length),
    );
}

pub fn Q7(s: &mut dyn Session) {
    restore(s, "CTO902_cts20484a.rle", "part");
    restore(s, "cts20484-tool1.rle", "tool1");
    bsection(s, "result", "part", "tool1");
    checkprops(s, "result", Prop::Length, Expected::Mass(386.561));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool1"], Prop::Length),
    );
}

pub fn Q8(s: &mut dyn Session) {
    restore(s, "cts20549a.rle", "a");
    restore(s, "CTO904_cts20549b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(381.332));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn Q9(s: &mut dyn Session) {
    restore(s, "spo0139a.rle", "a");
    restore(s, "spo0139b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(202.576));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn R3(s: &mut dyn Session) {
    restore(s, "heal-Face1.rle", "a");
    restore(s, "Plan.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1686.21));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn R4(s: &mut dyn Session) {
    restore(s, "CTO909_objects.brep", "a");
    explode(s, "a", Some(ShapeEnum::Shell));
    copy(s, "a_1", "a");
    restore(s, "CTO909_tool_4.brep", "b");
    explode(s, "b", Some(ShapeEnum::Shell));
    copy(s, "b_1", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(28.14));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn R7(s: &mut dyn Session) {
    println!("BUC60092");
    println!("section");
    println!();
    restore(s, "buc60092a.rle", "a");
    restore(s, "buc60092b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(162.012));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn R9(s: &mut dyn Session) {
    println!("BUC60462");
    println!("Section");
    println!();
    restore(s, "buc60462a.brep", "a");
    restore(s, "buc60462b.brep", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(282.421));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn S1(s: &mut dyn Session) {
    println!("CTS21801");
    println!("section");
    println!();
    pcylinder(s, "c1", 20., 100.);
    pcylinder(s, "c2", 20., 100.);
    ttranslate(s, "c2", [0., 0., 50.]);
    bsection(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Length, Expected::Mass(301.327));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Length),
    );
}
