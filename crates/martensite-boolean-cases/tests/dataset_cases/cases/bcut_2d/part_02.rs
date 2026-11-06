//! The `bcut_2d` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn E2(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn E3(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn E4(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(234.903));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn E5(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(234.903));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn E6(s: &mut dyn Session) {
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn E7(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn E8(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn E9(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn F1(s: &mut dyn Session) {
    restore(s, "case_3_face.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(226.366));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn F2(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn F3(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn F4(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn F5(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn F6(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn F7(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn F8(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(226.366));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn F9(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(569.46));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(9)],
    );
}

pub fn G1(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn G2(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn G3(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(589.551));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn G4(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn G5(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn G6(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1446.99));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(8)],
    );
}

pub fn G7(s: &mut dyn Session) {
    restore(s, "case_4_solid.brep", "a");
    restore(s, "case_4_wire4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1446.99));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(8)],
    );
}

pub fn H3(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(616.124));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(4)],
    );
}

pub fn H4(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(490.752));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn H5(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1308.66));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(13), NbCount::Edges(8)],
    );
}

pub fn H6(s: &mut dyn Session) {
    restore(s, "case_4_face.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(929.278));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(16), NbCount::Edges(11)],
    );
}

pub fn H7(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(344.173));
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
    restore(s, "case_5_edge2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(344.173));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H9(s: &mut dyn Session) {
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(135.758));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}
