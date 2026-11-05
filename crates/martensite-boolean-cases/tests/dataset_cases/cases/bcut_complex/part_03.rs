//! The `bcut_complex` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn J8(s: &mut dyn Session) {
    restore(s, "CTO902_pro14235a.rle", "a");
    restore(s, "CTO902_pro14235b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(426392.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J9(s: &mut dyn Session) {
    restore(s, "cts20456c.rle", "c");
    restore(s, "CTO902_cts20456d.rle", "d");
    bcut(s, "result", "c", "d");
    checkprops(s, "result", Prop::Area, Expected::Mass(344545.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "d"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20496c.rle", "part");
    restore(s, "cts20496d.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(229719.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K2(s: &mut dyn Session) {
    restore(s, "CTO902_pro14235c.rle", "c");
    restore(s, "CTO902_pro14235d.rle", "d");
    bcut(s, "result", "c", "d");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.55789e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "d"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K3(s: &mut dyn Session) {
    restore(s, "CTO904_cts20339-part.rle", "part");
    restore(s, "cts20339-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(407561.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K4(s: &mut dyn Session) {
    restore(s, "CTO904_cts20366-part.rle", "part");
    restore(s, "cts20366-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(82853.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K5(s: &mut dyn Session) {
    restore(s, "CTO904_cts20370-part.rle", "part");
    restore(s, "cts20370-tool.rle", "tool");
    bsection(s, "sec", "part", "tool");
    checkshape_top(s, "sec");
    bcut(s, "result", "part", "tool");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(273365.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "258752");
    kind_is(s, "part", ShapeEnum::Solid);
    printed_mass(s, "tool", Prop::Area, "62831.9");
    kind_is(s, "tool", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K6(s: &mut dyn Session) {
    restore(s, "CTO902_cts20397-part.rle", "part");
    restore(s, "CTO902_cts20397-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(109758.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K8(s: &mut dyn Session) {
    restore(s, "CTO902_cts20551b.rle", "a");
    restore(s, "cts20551-tool.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(430968.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K9(s: &mut dyn Session) {
    restore(s, "cts20552-part.rle", "a");
    restore(s, "cts20552-tool.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(221130.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L1(s: &mut dyn Session) {
    restore(s, "cts20767-part.rle", "a");
    restore(s, "CTO904_cts20767-tool.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(65803.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L4(s: &mut dyn Session) {
    restore(s, "CTO904_cts20366-part.rle", "a");
    restore(s, "cts20366-tool.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(82853.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L5(s: &mut dyn Session) {
    restore(s, "CTO904_fra60977-part.rle", "a");
    restore(s, "fra60977b.rle", "b");
    prism(s, "b", "b", [0., 0., -50.]);
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(302130.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L7(s: &mut dyn Session) {
    restore(s, "buc60092a.rle", "a");
    restore(s, "CTO904_buc60220b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(90179.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L9(s: &mut dyn Session) {
    restore(s, "CTO904_cts20088a.rle", "a");
    restore(s, "CTO904_cts20088b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.37605e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn M3(s: &mut dyn Session) {
    restore(s, "cts21182a.rle", "a");
    restore(s, "cts21182b.rle", "skface");
    revol(s, "b", "skface", [0., 0., 0.], [0., 0., 1.], 360.);
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(172676.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn M7(s: &mut dyn Session) {
    restore(s, "cts21289a.rle", "a");
    restore(s, "cts21289b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(165835.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn M8(s: &mut dyn Session) {
    restore(s, "cts21296a.rle", "a");
    restore(s, "cts21296b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(330624.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N1(s: &mut dyn Session) {
    restore(s, "CTO904_hkg60156a.rle", "a");
    restore(s, "CTO904_hkg60156b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1927.45));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N2(s: &mut dyn Session) {
    restore(s, "pro14630a.rle", "a");
    restore(s, "pro14630b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(198000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N3(s: &mut dyn Session) {
    restore(s, "usa60180a.rle", "a");
    restore(s, "usa60180b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(88325.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N6(s: &mut dyn Session) {
    restore(s, "CIN900_pro14hma.rle", "a");
    restore(s, "pro14928b.rle", "aface");
    prism(s, "b", "aface", [0., 500., 0.]);
    explode(s, "b", None);
    bcut(s, "result", "a", "b_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(562034.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N8(s: &mut dyn Session) {
    restore(s, "pro14957a.rle", "a");
    restore(s, "pro14957b.rle", "b");
    explode(s, "a", Some(ShapeEnum::Solid));
    bcut(s, "result", "a_1", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(90833.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a_1", Prop::Area, "91140.5");
    kind_is(s, "a_1", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "6853.91");
    kind_is(s, "b", ShapeEnum::Solid);
    part_count(s, "a", Some(ShapeEnum::Solid), 2);
    view_mass_is_nonzero(s, "result");
}

pub fn O1(s: &mut dyn Session) {
    restore(s, "CTO904_cts21347a.rle", "a");
    restore(s, "CTO904_cts21347b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1495.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn O2(s: &mut dyn Session) {
    restore(s, "pro15042.rle", "a");
    explode(s, "a", None);
    bcut(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Area, Expected::Mass(8387.96));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a_1", Prop::Area, "8387.96");
    kind_is(s, "a_1", ShapeEnum::Shell);
    printed_mass(s, "a_2", Prop::Area, "11381.9");
    kind_is(s, "a_2", ShapeEnum::Shell);
    part_count(s, "a", None, 2);
    view_mass_is_nonzero(s, "result");
}

pub fn O9(s: &mut dyn Session) {
    restore(s, "cts20549a.rle", "a");
    restore(s, "CTO904_cts20549b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(22561.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P4(s: &mut dyn Session) {
    restore(s, "CTO909_part_1.brep", "a");
    restore(s, "CTO909_tool_2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.64215e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P5(s: &mut dyn Session) {
    restore(s, "CTO909_shape_1.brep", "a");
    restore(s, "cts20528-tool.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(4041.96));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P7(s: &mut dyn Session) {
    restore(s, "CTO904_cts21347a.rle", "a");
    restore(s, "CTO909_tool.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1507.32));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P8(s: &mut dyn Session) {
    restore(s, "pro16389a.rle", "base");
    restore(s, "pro16389b.rle", "prism");
    bcut(s, "result", "base", "prism");
    checkprops(s, "result", Prop::Area, Expected::Mass(312467.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "prism"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P9(s: &mut dyn Session) {
    println!("BUC60092");
    println!("cut");
    println!();
    restore(s, "buc60092a.rle", "a");
    restore(s, "buc60092b.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(88508.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
