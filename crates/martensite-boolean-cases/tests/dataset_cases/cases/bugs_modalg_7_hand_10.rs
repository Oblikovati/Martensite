//! `bugs_modalg_7` bodies written by hand, second file: read-back values and options.
use crate::support::bugs_commands::geometry::{line, mkedge_range};
use crate::support::{
    beyond_script::{all_counts, printed_mass},
    bop_session::BopOptions,
    bugs_commands::{
        geometry::{cylinder, frame_along, pcylinder_on, torus, trim_u},
        inspect::{bopargcheck, bopcheck_text, checkmaxtol_value},
        options_bop::ScriptBop,
        repair::{invert, occ25446, settolerance},
        view::{don, fit, smallview},
    },
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview, screenshot},
    commands::{bound_shape, compound, make_box, psphere, trotate, ttranslate},
    construction::{edge, nurbsconvert, vertex},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::bcut,
    properties::Prop,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{cone, mkface, mkface_bounded, plane},
    tolerance_check::checkmaxtol_ref,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::{CheckStatus, Operation};
use std::f64::consts::PI;

pub fn bug26077_1(s: &mut dyn Session) {
    println!("TODO OCC26077 ALL: ERROR: OCC26077 is reproduced.");
    println!("========");
    println!("OCC26077");
    println!("========");
    println!();
    psphere(s, "s", 10.);
    nurbsconvert(s, "result", "s");
    let answer = bopargcheck(s, ("result", None), None, "");
    if answer.trim() != "Shape(s) seem(s) to be valid for BOP." {
        println!(
            "ERROR: OCC26077 is reproduced. Command bopargcheck returns FAIL status for nurbsconvert of sphere."
        );
    }
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::argcheck_statuses(
        s,
        "result",
        &[
            CheckStatus::SelfIntersect,
            CheckStatus::GeomAbsC0,
            CheckStatus::GeomAbsC0,
        ],
    );
}

fn periodic_face_checks(s: &mut dyn Session, name: &str, pins: FacePins) {
    let report = checkshape(&bound_shape(s, name));
    let response = bopcheck_text(s, name, &BopOptions::default());
    if !response.contains("This shape seems to be OK") {
        println!("Error: BRepLib_MakeFace produces invalid faces on periodic surfaces; {name}");
    }
    let view = CheckView {
        image: name,
        ..CheckView::of(name)
    };
    checkview(s, &view);
    // Beyond the script: what is pinned is the improvement, not only the absence of the failure.
    assert!(
        report.is_valid(),
        "checkshape {name}: expected a valid face"
    );
    assert_eq!(response, " This shape seems to be OK.\n", "bopcheck {name}");
    all_counts(s, name, pins.counts);
    printed_mass(s, name, Prop::Area, pins.area);
}

/// The library's result on one face; the decision file holds native's beside it.
#[derive(Clone, Copy)]
struct FacePins {
    counts: [usize; 9],
    area: &'static str,
}

/// A recorded improvement, `support::process::RustImprovement`: Martensite keeps the seam
/// on these faces, and native's interferences on them are the bug the TODO names.
pub fn bug27378(s: &mut dyn Session) {
    println!(
        "TODO OCC27378 ALL: Error: BRepLib_MakeFace produces invalid faces on periodic surfaces"
    );
    println!("========");
    println!("OCC27378");
    println!("========");
    println!();
    let cyl = cylinder(10.);
    mkface_bounded(s, "fcyl", &cyl, (PI, 3. * PI, -10., 10.));
    periodic_face_checks(
        s,
        "fcyl",
        FacePins {
            counts: [2, 3, 1, 1, 0, 0, 0, 0, 7],
            area: "1256.64",
        },
    );
    let sph = crate::support::bugs_commands::geometry::sphere([0., 0., 0.], 10.);
    let spht = trim_u(&sph, PI, 3. * PI);
    mkface(s, "fsph", &spht);
    periodic_face_checks(
        s,
        "fsph",
        FacePins {
            counts: [2, 3, 1, 1, 0, 0, 0, 0, 7],
            area: "1256.64",
        },
    );
    let con = cone([0., 0., 0.], [0., 0., 1.], 30., 0.);
    mkface_bounded(s, "fcon", &con, (PI, 3. * PI, 0., 20.));
    periodic_face_checks(
        s,
        "fcon",
        FacePins {
            counts: [2, 3, 1, 1, 0, 0, 0, 0, 7],
            area: "628.319",
        },
    );
    let tor = torus(20., 5.);
    mkface_bounded(s, "ftor1", &tor, (PI, 3. * PI, 0., 2. * PI));
    periodic_face_checks(
        s,
        "ftor1",
        FacePins {
            counts: [1, 2, 1, 1, 0, 0, 0, 0, 5],
            area: "3947.84",
        },
    );
    mkface_bounded(s, "ftor2", &tor, (0., 2. * PI, PI, 3. * PI));
    periodic_face_checks(
        s,
        "ftor2",
        FacePins {
            counts: [1, 2, 1, 1, 0, 0, 0, 0, 5],
            area: "3947.84",
        },
    );
}

/// `baddobjects a; baddtools b; if {![regexp "Warning" [bfillds]]} ...; bbuild result; checkshape`.
fn warning_expected(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["a"]);
    bop.add_tools(s, &["b"]);
    if !bop.bfillds_answer().contains("Warning") {
        println!("Error: WarningStatus has not been set");
    }
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
}

pub fn bug28786_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC28786");
    println!("========");
    println!();
    let first = plane([0., 0., 0.], [1., 0., 0.]);
    mkface_bounded(s, "f1", &first, (-10., 10., -10., 10.));
    let second = plane([0., 0., 0.], [1., 0., 1.]);
    mkface_bounded(s, "f2", &second, (-10., 10., -10., 10.));
    compound(s, &["f1", "f2"], "a");
    let l = line([0., 0., 0.], [0., 1., 0.]);
    mkedge_range(s, "b", &l, -11., 11.);
    warning_expected(s);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Edges(15), NbCount::Faces(4)],
    );
}

pub fn bug28786_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC28786");
    println!("========");
    println!();
    vertex(s, "v", [0., 0., 0.]);
    vertex(s, "v1", [10., 0., 0.]);
    vertex(s, "v2", [0., 0., -3.]);
    vertex(s, "v3", [5., 0., 0.]);
    edge(s, "e1", "v", "v1");
    edge(s, "e2", "v2", "v3");
    compound(s, &["e1", "e2"], "a");
    vertex(s, "v4", [5., 0., 5.]);
    vertex(s, "v5", [5., 0., -5.]);
    edge(s, "b", "v4", "v5");
    warning_expected(s);
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(5)]);
}

pub fn bug28786_4(s: &mut dyn Session) {
    println!("========");
    println!("OCC28786");
    println!("========");
    println!();
    vertex(s, "v", [0., 0., 0.]);
    vertex(s, "v1", [1.0e-5, 0., 0.]);
    settolerance(s, "v1", 1.0e-5);
    vertex(s, "v2", [0., 1.0e-5, 0.]);
    edge(s, "a", "v", "v1");
    edge(s, "b", "v", "v2");
    warning_expected(s);
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(2)]);
}

pub fn bug28786_5(s: &mut dyn Session) {
    println!("========");
    println!("OCC28786");
    println!("========");
    println!();
    vertex(s, "v", [0., 0., 0.]);
    vertex(s, "v1", [10., 0., 0.]);
    edge(s, "e1", "v", "v1");
    edge(s, "e2", "v", "v1");
    compound(s, &["e1", "e2"], "a");
    vertex(s, "v3", [-5., 0., 0.]);
    vertex(s, "v4", [15., 0., 0.]);
    edge(s, "b", "v3", "v4");
    warning_expected(s);
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(3)]);
}

pub fn bug29910_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC29910: Implementation to Debian80-64 : Regressions in Modeling Algorithms");
    println!("========");
    println!();
    println!(
        "TODO ?OCC29910 Linux: Error :  is WRONG because number of EDGE entities in shape \"rs\" is 16"
    );
    println!(
        "TODO ?OCC29910 Linux: Error :  is WRONG because number of SHELL entities in shape \"result\" is 5"
    );
    println!(
        "TODO ?OCC29910 Linux: Error :  is WRONG because number of SOLID entities in shape \"result\" is 5"
    );
    println!("TODO ?OCC29910 Linux: Error : The area of result shape is");
    println!("TODO ?OCC29910 Linux: Error : The volume of result shape is");
    crate::support::commands::ptorus(s, "t1", 100., 10.);
    crate::support::commands::ptorus(s, "t2", 100., 10.);
    ttranslate(s, "t2", [100., 0., 0.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["t1"]);
    bop.add_tools(s, &["t2"]);
    bop.bfillds();
    bop.bbop(s, "rs", Operation::Section);
    checksection_expecting(s, "rs", 0);
    checkshape_named(s, "rs");
    if bopargcheck(s, ("rs", None), None, "").contains("Faulties") {
        println!("Error: bopargcheck has found some faulties in result");
    }
    checkmaxtol_ref(s, "rs", "7.5e-6");
    checknbshapes(s, "rs", Counting::Located, &[NbCount::Edges(14)]);
    checkprops(s, "rs", Prop::Length, Expected::Mass(330.096));
    bop.bbuild(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(6), NbCount::Solids(6)],
    );
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(82663.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(382445.));
    smallview(s);
    don(s, &["result"]);
    fit(s);
    screenshot(s, "photo");
}

pub fn bug29973_1(s: &mut dyn Session) {
    println!("========");
    println!(
        "OCC29973: Modeling Algorithms - access violation within BRepAlgoAPICut on empty input shape"
    );
    println!("========");
    println!();
    // What `pload QAcommands` answers.
    println!("1");
    compound(s, &[], "c");
    make_box(s, "b", [0., 0., 0.], [1., 2., 3.]);
    if !occ25446(s, "r", "b", "c").contains("Test complete") {
        println!("Error: test incomplete");
    }
    compound(s, &["b"], "bc");
    checknbshapes_like(s, "r", "bc");
    checkprops_equal(s, "r", "bc");
    // Beyond the script: checks an earlier test of this case made.
    crate::support::beyond_script::same_masses(s, "r", "bc");
}

pub fn bug30154_1(s: &mut dyn Session) {
    println!("========");
    println!(
        "0030154: Modeling Algorithms - Boolean Operation on planar geometry hangs inside BRepLib::FindValidRange()"
    );
    println!("========");
    println!();
    let surface = plane([165424660., 597500., 42834196.], [-1., 0., 0.]);
    mkface(s, "f", &surface);
    make_box(
        s,
        "b",
        [-94190864., -46229000., -17178478.4],
        [519231048., 93653000., 120025348.8],
    );
    invert(s, "b");
    bcut(s, "result", "f", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.12407e+16));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Wires(1), NbCount::Faces(1)],
    );
}

fn tolerance_below(s: &dyn Session, limit: f64) {
    if checkmaxtol_value(s, "result") > limit {
        println!("Error: bad tolerance of result");
    }
}

fn six_counts(c: [usize; 6]) -> [NbCount; 6] {
    [
        NbCount::Vertices(c[0]),
        NbCount::Edges(c[1]),
        NbCount::Wires(c[2]),
        NbCount::Faces(c[3]),
        NbCount::Shells(c[4]),
        NbCount::Solids(c[5]),
    ]
}

pub fn bug32470(s: &mut dyn Session) {
    println!("============================================");
    println!("OCC32470: BOP wrong result on sphere and box");
    println!("============================================");
    println!();
    psphere(s, "sph", 10.);
    make_box(s, "b", [0., 0., 0.], [20., 100., 100.]);
    ttranslate(s, "b", [0., -50., -50.]);
    trotate(s, "b", [0., 0., 10.], [0., -1., 0.], 45.);
    bcut(s, "result", "sph", "b");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &six_counts([3, 5, 2, 2, 1, 1]),
    );
    tolerance_below(s, 1.0e-5);
    checkprops(s, "result", Prop::Area, Expected::Mass(1229.69));
    checkprops(s, "result", Prop::Volume, Expected::Mass(3945.6));
}

pub fn bug32502(s: &mut dyn Session) {
    println!("=================================================");
    println!("OCC32502: BOP wrong result on sphere and cylinder");
    println!("=================================================");
    println!();
    psphere(s, "sph", 10.);
    let frame = crate::support::bugs_commands::geometry::frame_framed(
        [0., 0., 7.],
        [0., 1., 0.],
        [0., 0., 1.],
    );
    pcylinder_on(s, "cyl", frame, 3., 40.);
    ttranslate(s, "cyl", [0., -20., 0.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "sph", "cyl");
    two_step.result(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &six_counts([4, 11, 5, 5, 1, 1]),
    );
    tolerance_below(s, 3.0e-5);
    checkprops(s, "result", Prop::Area, Expected::Mass(1750.1));
    checkprops(s, "result", Prop::Volume, Expected::Mass(4947.2));
}

pub fn bug24632_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC24632");
    println!("========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10., 0., 0.], [10., 10., 10.]);
    pcylinder_on(
        s,
        "cut",
        frame_along([-10., 5., 5.], [1., 0., 0.]),
        2.5,
        40.,
    );
    checkshape_named(s, "cut");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1", "b2"]);
    bop.bfillds();
    bop.bbuild(s, "c");
    checkshape_named(s, "c");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "c", "cut");
    two_step.result(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    let counts = [
        NbCount::Wires(16),
        NbCount::Faces(13),
        NbCount::Shells(2),
        NbCount::Solids(2),
    ];
    checknbshapes(s, "result", Counting::Located, &counts);
    checkprops(s, "result", Prop::Volume, Expected::Mass(1607.3));
    checkview(s, &CheckView::of("result"));
}
