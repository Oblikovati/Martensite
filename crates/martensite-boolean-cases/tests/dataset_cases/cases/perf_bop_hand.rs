//! `tests/perf/bop`: the same pair at four scales, each operation timed.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::compound,
    construction::tscale,
    one_step::{bfuse, bsection},
    perf_commands::Chrono,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
};

fn restored_pairs(s: &mut dyn Session) {
    println!(" BUC60068 ");
    println!("If scales by 1000, time to compute is 10 times greater ");
    restore(s, "buc60068a.rle", "a");
    restore(s, "buc60068b.rle", "b");
    restore(s, "buc60068a.rle", "c");
    restore(s, "buc60068b.rle", "d");
    restore(s, "buc60068a.rle", "e");
    restore(s, "buc60068b.rle", "f");
    restore(s, "buc60068a.rle", "g");
    restore(s, "buc60068b.rle", "h");
}

/// `dchrono x start`, the operation, `dchrono x stop counter label`.
fn timed(s: &mut dyn Session, label: &str, operation: impl FnOnce(&mut dyn Session)) {
    let mut chrono = Chrono::started();
    operation(s);
    chrono.stop();
    chrono.counter(label);
}

fn scaled(s: &mut dyn Session, names: [&str; 2], factor: f64) {
    tscale(s, names[0], [0.; 3], factor);
    tscale(s, names[1], [0.; 3], factor);
}

const OPERANDS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub fn bfuse_complex_Q3(s: &mut dyn Session) {
    restored_pairs(s);
    timed(s, "BFuseAB", |s| bfuse(s, "resab", "a", "b"));
    scaled(s, ["c", "d"], 100.);
    timed(s, "BFuseCD", |s| bfuse(s, "rescd", "c", "d"));
    scaled(s, ["e", "f"], 1000.);
    timed(s, "BFuseEF", |s| bfuse(s, "resef", "e", "f"));
    scaled(s, ["g", "h"], 10000.);
    timed(s, "BFuseGH", |s| bfuse(s, "resgh", "g", "h"));
    compound(s, &["resab", "rescd", "resef", "resgh"], "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5e+13));
    checkview(s, &CheckView::of("result").otherwise(&OPERANDS, Prop::Area));
}

pub fn bsection_R6(s: &mut dyn Session) {
    restored_pairs(s);
    timed(s, "BSectionAB", |s| bsection(s, "resab", "a", "b"));
    scaled(s, ["c", "d"], 100.);
    timed(s, "BSectionCD", |s| bsection(s, "rescd", "c", "d"));
    scaled(s, ["e", "f"], 1000.);
    timed(s, "BSectionEF", |s| bsection(s, "resef", "e", "f"));
    scaled(s, ["g", "h"], 10000.);
    timed(s, "BSectionGH", |s| bsection(s, "resgh", "g", "h"));
    compound(s, &["resab", "rescd", "resef", "resgh"], "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.41538e+07));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&OPERANDS, Prop::Length),
    );
}
