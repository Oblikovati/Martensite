//! The `bcommon_2d` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn I2(s: &mut dyn Session) {
    restore(s, "case_5_shell.brep", "a");
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

pub fn I3(s: &mut dyn Session) {
    restore(s, "case_5_shell.brep", "a");
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

pub fn I4(s: &mut dyn Session) {
    restore(s, "case_5_shell.brep", "a");
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

pub fn I5(s: &mut dyn Session) {
    restore(s, "case_5_shell.brep", "a");
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

pub fn I6(s: &mut dyn Session) {
    restore(s, "case_5_shell.brep", "a");
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

pub fn I7(s: &mut dyn Session) {
    restore(s, "case_5_face.brep", "a");
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

pub fn I8(s: &mut dyn Session) {
    restore(s, "case_5_face.brep", "a");
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

pub fn I9(s: &mut dyn Session) {
    restore(s, "case_5_face.brep", "a");
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

pub fn J1(s: &mut dyn Session) {
    restore(s, "case_5_face.brep", "a");
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

pub fn K8(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn K9(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L1(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L2(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L3(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L4(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L5(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L6(s: &mut dyn Session) {
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L7(s: &mut dyn Session) {
    restore(s, "case_7_shell.brep", "a");
    restore(s, "case_7_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L8(s: &mut dyn Session) {
    restore(s, "case_7_shell.brep", "a");
    restore(s, "case_7_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn L9(s: &mut dyn Session) {
    restore(s, "case_7_shell.brep", "a");
    restore(s, "case_7_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M1(s: &mut dyn Session) {
    restore(s, "case_7_shell.brep", "a");
    restore(s, "case_7_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M2(s: &mut dyn Session) {
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M3(s: &mut dyn Session) {
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M4(s: &mut dyn Session) {
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M5(s: &mut dyn Session) {
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M6(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(181.079));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn M7(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.966));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn M8(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(3.83899));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn M9(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(36.0874));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(4)],
    );
}

pub fn N1(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(126.81));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(4)],
    );
}

pub fn N2(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire6.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}
