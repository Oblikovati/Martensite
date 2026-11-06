//! The `bfuse_complex` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn M2(s: &mut dyn Session) {
    restore(s, "cts21240a.rle", "a");
    restore(s, "cts21240b.rle", "skface");
    prism(s, "b", "skface", [0., 0., 1.]);
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(19213.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "19077.9");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "1835.18");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn M3(s: &mut dyn Session) {
    restore(s, "cts21244a.rle", "a");
    restore(s, "cts21244b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(184913.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn M4(s: &mut dyn Session) {
    restore(s, "cts21251a.rle", "a");
    restore(s, "cts21251b.rle", "skface");
    revol(s, "b", "skface", [0., 0., 0.], [0., -1., 0.], 360.);
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(7463.16));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "6685.31");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "5703.86");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn M5(s: &mut dyn Session) {
    restore(s, "cts21306a.rle", "a");
    restore(s, "cts21306b.rle", "b");
    restore(s, "cts21306c.rle", "c");
    bfuse(s, "rab", "a", "b");
    bfuse(s, "result", "rab", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(12335.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rab", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "8586.71");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "6482.52");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "rab", Prop::Area, "9406.97");
    kind_is(s, "rab", ShapeEnum::Compound);
    printed_mass(s, "c", Prop::Area, "8605.49");
    kind_is(s, "c", ShapeEnum::Compound);
    view_mass_is_nonzero(s, "result");
}

pub fn M6(s: &mut dyn Session) {
    restore(s, "cts21306b.rle", "b");
    restore(s, "cts21306c.rle", "c");
    bfuse(s, "result", "b", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(9411.14));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N1(s: &mut dyn Session) {
    restore(s, "CTO904_pro14893a.rle", "a");
    restore(s, "pro14893b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(22717.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N3(s: &mut dyn Session) {
    restore(s, "CTO904_pro14785a.rle", "a");
    restore(s, "CTO904_pro14785b.rle", "b");
    restore(s, "CTO904_pro14785c.rle", "c");
    bfuse(s, "res", "a", "b");
    bfuse(s, "result", "res", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(533100.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["res", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "358452");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "98734.6");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "res", Prop::Area, "417819");
    kind_is(s, "res", ShapeEnum::Compound);
    printed_mass(s, "c", Prop::Area, "252021");
    kind_is(s, "c", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn N4(s: &mut dyn Session) {
    restore(s, "pro14892a.rle", "a");
    restore(s, "pro14892b.rle", "aface");
    prism(s, "b", "aface", [0., 0., -50.]);
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(601864.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "542055");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "229295");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn N5(s: &mut dyn Session) {
    restore(s, "CTO904_fra61146a.rle", "a");
    restore(s, "CTO904_fra61146b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(254248.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N6(s: &mut dyn Session) {
    restore(s, "CTO904_fra61146a.rle", "a");
    restore(s, "CTO904_fra61146b.rle", "b");
    bfuse(s, "result", "b", "a");
    checkprops(s, "result", Prop::Area, Expected::Mass(254248.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N7(s: &mut dyn Session) {
    restore(s, "pro15049a.rle", "a");
    restore(s, "pro15049b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(741091.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn N9(s: &mut dyn Session) {
    restore(s, "pro15150a.rle", "a");
    restore(s, "pro15150b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(259563.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn O1(s: &mut dyn Session) {
    restore(s, "pro15203a.rle", "a");
    restore(s, "pro15203c.rle", "c");
    explode(s, "a", Some(ShapeEnum::Face));
    explode(s, "c", Some(ShapeEnum::Face));
    bfuse(s, "result", "a_1", "c_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "c_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a_1", Prop::Area, "285.023");
    kind_is(s, "a_1", ShapeEnum::Face);
    printed_mass(s, "c_1", Prop::Area, "285.023");
    kind_is(s, "c_1", ShapeEnum::Face);
    part_count(s, "a", Some(ShapeEnum::Face), 1);
    part_count(s, "c", Some(ShapeEnum::Face), 1);
    view_mass_is_nonzero(s, "result");
}

pub fn O2(s: &mut dyn Session) {
    restore(s, "pro15203a.rle", "a");
    restore(s, "pro15203d.rle", "d");
    explode(s, "a", Some(ShapeEnum::Face));
    explode(s, "d", Some(ShapeEnum::Face));
    bfuse(s, "result", "a_1", "d_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "d_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a_1", Prop::Area, "285.023");
    kind_is(s, "a_1", ShapeEnum::Face);
    printed_mass(s, "d_1", Prop::Area, "285.023");
    kind_is(s, "d_1", ShapeEnum::Face);
    part_count(s, "a", Some(ShapeEnum::Face), 1);
    part_count(s, "d", Some(ShapeEnum::Face), 1);
    view_mass_is_nonzero(s, "result");
}

pub fn O3(s: &mut dyn Session) {
    restore(s, "pro15203b.rle", "b");
    restore(s, "pro15203c.rle", "c");
    explode(s, "b", Some(ShapeEnum::Face));
    explode(s, "c", Some(ShapeEnum::Face));
    bfuse(s, "result", "b_1", "c_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b_1", "c_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b_1", Prop::Area, "285.023");
    kind_is(s, "b_1", ShapeEnum::Face);
    printed_mass(s, "c_1", Prop::Area, "285.023");
    kind_is(s, "c_1", ShapeEnum::Face);
    part_count(s, "b", Some(ShapeEnum::Face), 1);
    part_count(s, "c", Some(ShapeEnum::Face), 1);
    view_mass_is_nonzero(s, "result");
}

pub fn O4(s: &mut dyn Session) {
    restore(s, "pro15203b.rle", "b");
    restore(s, "pro15203d.rle", "d");
    explode(s, "b", Some(ShapeEnum::Face));
    explode(s, "d", Some(ShapeEnum::Face));
    bfuse(s, "result", "b_1", "d_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b_1", "d_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b_1", Prop::Area, "285.023");
    kind_is(s, "b_1", ShapeEnum::Face);
    printed_mass(s, "d_1", Prop::Area, "285.023");
    kind_is(s, "d_1", ShapeEnum::Face);
    part_count(s, "b", Some(ShapeEnum::Face), 1);
    part_count(s, "d", Some(ShapeEnum::Face), 1);
    view_mass_is_nonzero(s, "result");
}

pub fn O7(s: &mut dyn Session) {
    restore(s, "CTO909_IR2_TuyoPipe20.brep", "pip");
    restore(s, "CTO909_IR2_sphere1_20.brep", "sp1");
    bfuse(s, "result", "pip", "sp1");
    checkprops(s, "result", Prop::Area, Expected::Mass(35184.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["pip", "sp1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn P2(s: &mut dyn Session) {
    restore(s, "pro15203b.rle", "c1");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    restore(s, "pro15203c.rle", "c2");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c1", Prop::Area, "285.023");
    kind_is(s, "c1", ShapeEnum::Face);
    printed_mass(s, "c2", Prop::Area, "285.023");
    kind_is(s, "c2", ShapeEnum::Face);
    part_count(s, "c1", Some(ShapeEnum::Face), 0);
    part_count(s, "c2", Some(ShapeEnum::Face), 0);
    view_mass_is_nonzero(s, "result");
}

pub fn P3(s: &mut dyn Session) {
    restore(s, "pro15203b.rle", "c1");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    restore(s, "pro15203d.rle", "c2");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c1", Prop::Area, "285.023");
    kind_is(s, "c1", ShapeEnum::Face);
    printed_mass(s, "c2", Prop::Area, "285.023");
    kind_is(s, "c2", ShapeEnum::Face);
    part_count(s, "c1", Some(ShapeEnum::Face), 0);
    part_count(s, "c2", Some(ShapeEnum::Face), 0);
    view_mass_is_nonzero(s, "result");
}

pub fn Q2(s: &mut dyn Session) {
    restore(s, "CTO900_pro10658a.rle", "a");
    restore(s, "pro10658b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(8231.06));
    checkprops(s, "result", Prop::Volume, Expected::Mass(30472.5));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(9),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "4609.64");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "4027.15");
    kind_is(s, "b", ShapeEnum::Solid);
}

pub fn Q5(s: &mut dyn Session) {
    println!("BUC60382");
    println!("Problem with fuse operation ");
    println!();
    restore(s, "IR2_TuyoPipeBSpline_20_draw2.brep", "a");
    restore(s, "IR2_sphere2_20.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(35185.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q6(s: &mut dyn Session) {
    println!("CTS20769");
    println!("Creating fuse is impossible");
    println!();
    restore(s, "cts20769a.brep", "a");
    restore(s, "CTO904_cts20769a.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(92385.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q7(s: &mut dyn Session) {
    println!("GER60806");
    println!("Fuse doesn't give the waiting result.");
    println!();
    restore(s, "CFE900_ger60gdc.rle", "base");
    restore(s, "ger60806b.rle", "tool");
    bcut(s, "res1", "tool", "base");
    explode(s, "res1", Some(ShapeEnum::Solid));
    bfuse(s, "result", "base", "res1_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(181990.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "res1_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "tool", Prop::Area, "17882");
    kind_is(s, "tool", ShapeEnum::Solid);
    printed_mass(s, "base", Prop::Area, "180933");
    kind_is(s, "base", ShapeEnum::Compound);
    printed_mass(s, "res1_1", Prop::Area, "3598.89");
    kind_is(s, "res1_1", ShapeEnum::Solid);
    part_count(s, "res1", Some(ShapeEnum::Solid), 2);
    view_mass_is_nonzero(s, "result");
}
