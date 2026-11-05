//! `bugs_modalg_7` bodies written by hand: variables, conditions and the argument lists.
use crate::support::restore::restore;
use crate::support::{
    beyond_script::{all_counts, masses_are_zero, same_masses},
    bop_session::{BopOptions, BopSession},
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::checkshape_named,
    checkview::screenshot,
    commands::{compound, make_box},
    named_checks::bopargcheck_faulties,
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::{bcut, bsection},
    properties::Prop,
    section_check::checksection_expecting,
    session::{Session, ViewStep},
    surface_commands::{cone, mkface_bounded},
    tolerance_check::checkmaxtol_ref,
};
use martensite_tkbo::bop_algo::Operation;

/// The conical faces of the two section cases: `v` runs over the whole million.
const FULL_CONE: (f64, f64, f64, f64) = (0., 6.2831853071795862, 0., 1000000.);
const CONE_NORMAL: [f64; 3] = [0., -1., 1.1102230246251565e-016];

struct LongCones<'a> {
    first: (&'a str, f64, f64),
    second: (&'a str, f64, f64),
    image: &'a str,
    reference_tolerance: &'a str,
    length: f64,
}

fn cone_face(s: &mut dyn Session, (name, y, degrees): (&str, f64, f64)) {
    let surface = cone([0., y, 0.], CONE_NORMAL, degrees, 0.);
    mkface_bounded(s, name, &surface, FULL_CONE);
}

fn long_cone_section(s: &mut dyn Session, case: &LongCones<'_>) {
    println!("========");
    println!("0029807: Impossible to cut cone from prism");
    println!("========");
    cone_face(s, case.first);
    cone_face(s, case.second);
    bsection(s, "result", case.first.0, case.second.0);
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&[case.first.0, case.second.0]));
    checksection_expecting(s, "result", 0);
    screenshot(s, case.image);
    checkmaxtol_ref(s, "result", case.reference_tolerance);
    checkprops(s, "result", Prop::Length, Expected::Mass(case.length));
    let counts = [NbCount::Vertices(1), NbCount::Edges(1)];
    checknbshapes(s, "result", Counting::Located, &counts);
    checkshape_named(s, "result");
    let result = s.shape("result").expect("bsection result").clone();
    if bopargcheck_faulties(&result, &BopOptions::default()) {
        println!("Error: bopargcheck has found some faulties in result");
    }
    // Beyond the script: every count, not only the two it names.
    all_counts(s, "result", [1, 1, 0, 0, 0, 0, 0, 1, 3]);
}

pub fn bug29807_svm01(s: &mut dyn Session) {
    let case = LongCones {
        first: ("f2", 518.47000000000003, 14.999999999912038),
        second: ("f6", -440.74363604000001, 45.110284878807235),
        image: "photo_3d",
        reference_tolerance: "7.3189259943803184e-007",
        length: 2202.91,
    };
    long_cone_section(s, &case);
}

pub fn bug29807_svm02(s: &mut dyn Session) {
    let case = LongCones {
        first: ("f1", -60.919306349999999, 28.800000000062262),
        second: ("f5", -309.47272469000001, 43.999999999485127),
        image: "photo_3d",
        reference_tolerance: "6.6226289034767669e-007",
        length: 1993.34,
    };
    long_cone_section(s, &case);
}

/// `bbop` of every kind, in the script's order, under `<prefix>_0` to `<prefix>_4`.
fn all_results(s: &mut dyn Session, bop: &mut BopSession, names: [&str; 5]) {
    bop.bbop(s, names[0], Operation::Common);
    bop.bbop(s, names[1], Operation::Fuse);
    bop.bbop(s, names[2], Operation::Cut);
    bop.bbop(s, names[3], Operation::Cut21);
    bop.bbop(s, names[4], Operation::Section);
}

fn empty_results_checked(s: &mut dyn Session, name: &str) {
    checkshape_named(s, name);
    checknbshapes(s, name, Counting::Located, &[NbCount::Shapes(1)]);
    checkprops(s, name, Prop::Area, Expected::Empty);
    checkprops(s, name, Prop::Volume, Expected::Empty);
}

fn solid_result_checked(
    s: &mut dyn Session,
    name: &str,
    faces: usize,
    solids: usize,
    masses: [f64; 2],
) {
    checkshape_named(s, name);
    let counts = [
        NbCount::Wires(faces),
        NbCount::Faces(faces),
        NbCount::Solids(solids),
        NbCount::Shells(solids),
    ];
    checknbshapes(s, name, Counting::Located, &counts);
    checkprops(s, name, Prop::Area, Expected::Mass(masses[0]));
    checkprops(s, name, Prop::Volume, Expected::Mass(masses[1]));
}

fn same_as(s: &mut dyn Session, name: &str, reference: &str) {
    checkshape_named(s, name);
    checknbshapes_like(s, name, reference);
    checkprops_equal(s, name, reference);
}

pub fn bug29973_2(s: &mut dyn Session) {
    println!("========");
    println!(
        "OCC29973: Modeling Algorithms - access violation within BRepAlgoAPICut on empty input shape"
    );
    println!("========");
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [5., 5., 5.], [10., 10., 10.]);
    compound(s, &[], "c");
    let mut bop = BopSession::default();
    bop.arguments(s, &["b1", "b2"], &["c"]);
    bop.bfillds();
    all_results(s, &mut bop, ["r_0", "r_1", "r_2", "r_3", "r_4"]);
    empty_results_checked(s, "r_0");
    solid_result_checked(s, "r_1", 12, 1, [1050., 1875.]);
    solid_result_checked(s, "r_2", 18, 3, [1350., 1875.]);
    empty_results_checked(s, "r_3");
    checkshape_named(s, "r_4");
    let counts = [NbCount::Vertices(6), NbCount::Edges(6)];
    checknbshapes(s, "r_4", Counting::Located, &counts);
    checkprops(s, "r_4", Prop::Length, Expected::Mass(30.));
    bop.arguments(s, &["c"], &["b1", "b2"]);
    bop.bfillds();
    all_results(s, &mut bop, ["r1_0", "r1_1", "r1_2", "r1_3", "r1_4"]);
    same_as(s, "r1_0", "r_0");
    same_as(s, "r1_1", "r_1");
    same_as(s, "r1_2", "r_3");
    same_as(s, "r1_3", "r_2");
    same_as(s, "r1_4", "r_4");
    beyond_the_script_empty_input(s);
}

/// The script compares lengths only, and checks no length on its empty results.
fn beyond_the_script_empty_input(s: &dyn Session) {
    masses_are_zero(s, "r_0");
    masses_are_zero(s, "r_3");
    for (name, reference) in [
        ("r1_0", "r_0"),
        ("r1_1", "r_1"),
        ("r1_2", "r_3"),
        ("r1_3", "r_2"),
        ("r1_4", "r_4"),
    ] {
        same_masses(s, name, reference);
    }
}

/// `llength [whatis result]`: a bound shape describes itself in more than three words.
pub fn bug24424(s: &mut dyn Session) {
    println!("========");
    println!("OCC24424");
    println!("========");
    restore(s, "bug24424_a.brep", "a");
    restore(s, "bug24424_b.brep", "b");
    bcut(s, "result", "a", "b");
    if s.shape("result").is_none() {
        println!("ERROR: OCC24424 is reproduced. Result of bcut operation is empty.");
    }
}
