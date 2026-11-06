//! The `bcommon_2d` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn E3(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(13)],
    );
}

pub fn E4(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn E5(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn E6(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(292.732));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(4)],
    );
}

pub fn E7(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn E8(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn E9(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(13)],
    );
}

pub fn F1(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(66.3661));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn F2(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn F3(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(7)],
    );
}

pub fn F4(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(345.975));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(13)],
    );
}

pub fn F5(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(66.3661));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn F6(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1231.36));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn F7(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1231.36));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn F8(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(375.748));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn F9(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(2553.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(3)],
    );
}

pub fn G1(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(2553.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(3)],
    );
}

pub fn G2(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(633.071));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(9), NbCount::Edges(7)],
    );
}

pub fn G3(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(633.071));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(9), NbCount::Edges(7)],
    );
}

pub fn G8(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(615.235));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn G9(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(474.547));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H1(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1245.29));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(5)],
    );
}

pub fn H2(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1150.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(15), NbCount::Edges(10)],
    );
}

pub fn H3(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(887.185));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn H4(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(887.185));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn H5(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(829.541));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn H6(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H7(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H8(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn H9(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_wire4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn I1(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_wire5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(3000.52));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(2)],
    );
}
