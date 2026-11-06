//! The `bcut_2d` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn Q6(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(469.89));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn Q7(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1080.64));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(6)],
    );
}

pub fn Q8(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn Q9(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1080.64));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(5)],
    );
}

pub fn R1(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn R2(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
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

pub fn R3(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(469.89));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn R4(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(111.749));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn R5(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn R6(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(350.));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn R7(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn R8(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(81.6175));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn R9(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn S1(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(350.));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn S2(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn S3(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(763.834));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn S4(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(469.89));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn S5(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1080.64));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(6)],
    );
}

pub fn S6(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn S7(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1080.64));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(5)],
    );
}

pub fn S8(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}
