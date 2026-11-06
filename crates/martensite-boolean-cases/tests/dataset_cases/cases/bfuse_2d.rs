//! `bfuse_2d`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bfuse,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
};

pub fn A1(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(635.826));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(12)],
    );
}

pub fn A2(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1085.29));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(14)],
    );
}

pub fn A3(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1085.29));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(20), NbCount::Edges(20)],
    );
}

pub fn A4(s: &mut dyn Session) {
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(928.558));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(13)],
    );
}

pub fn A5(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1045.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(5)],
    );
}

pub fn A6(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1157.7));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(6)],
    );
}

pub fn A7(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1252.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn A8(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1127.57));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn A9(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1316.05));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn B1(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1953.57));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(9)],
    );
}

pub fn B2(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(2126.59));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(11)],
    );
}

pub fn B3(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(2126.59));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(11)],
    );
}

pub fn B4(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn B5(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(725.427));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn B6(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(963.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn B7(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(695.295));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn B8(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(963.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn B9(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1521.3));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn C1(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1694.32));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(8)],
    );
}

pub fn C2(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1694.32));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn C3(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn C4(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(725.427));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn C5(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(963.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn C6(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(695.295));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn C7(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(963.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn C8(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1377.51));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}

pub fn C9(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1694.32));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(8)],
    );
}

pub fn D1(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1694.32));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}
