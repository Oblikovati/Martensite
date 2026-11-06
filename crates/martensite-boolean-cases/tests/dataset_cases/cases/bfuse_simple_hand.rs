//! `bfuse_simple` bodies written by hand: the script reads a command's answer.
use crate::support::{
    beyond_script::{kind_is, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::make_box,
    construction::updatetolerance,
    maxtolerance::maxtolerance,
    one_step::bfuse,
    properties::Prop,
    session::Session,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `puts [maxtolerance name]`: the command writes its answer, then the script prints it.
fn written_maxtolerance(s: &dyn Session, name: &str) -> String {
    let answer = maxtolerance(s, name);
    print!("{answer}");
    println!("{answer}");
    answer
}

/// `lindex $answer at`, a number.
fn word_of(answer: &str, at: usize) -> f64 {
    let word = answer
        .split_whitespace()
        .nth(at)
        .expect("a word of the answer");
    word.parse()
        .unwrap_or_else(|cause| panic!("{word}: {cause}"))
}

/// Beyond the script: the three words the script reads, as a native run has them.
fn read_as(answer: &str, expected: f64) {
    assert_eq!(
        [14, 18, 24].map(|at| word_of(answer, at)),
        [expected; 3],
        "{answer}"
    );
}

pub fn L1(s: &mut dyn Session) {
    println!("BUC60476");
    println!(
        " Precision management doesn't follow the rule of tolerance (Tol(V) > Tol(E) > Tol(F) "
    );
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [5., 5., 5.], [10., 10., 10.]);
    updatetolerance(s, "b1", true);
    updatetolerance(s, "b2", true);
    read_as(&written_maxtolerance(s, "b1"), 1e-6);
    read_as(&written_maxtolerance(s, "b2"), 1e-6);
    bfuse(s, "result", "b1", "b2");
    let tolerance = written_maxtolerance(s, "result");
    let min_edge_tolerance = word_of(&tolerance, 18);
    let min_vertex_tolerance = word_of(&tolerance, 24);
    println!();
    if min_edge_tolerance > min_vertex_tolerance {
        println!(
            " Error: The rule of tolerance Tol(V) > Tol(E) > Tol(F) is not observed . Tolerance min on vertex > Tolerance min on edge "
        );
    }
    println!(" The rule of tolerance Tol(V) > Tol(E) > Tol(F) is observed .");
    checkprops(s, "result", Prop::Area, Expected::Mass(1050.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "600");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "600");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
