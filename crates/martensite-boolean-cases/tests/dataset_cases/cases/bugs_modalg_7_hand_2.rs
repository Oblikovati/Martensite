//! `tests/bugs/modalg_7`: Booleans sharing one prepared session, with their results checked.
use crate::support::{
    bop_session::{BopOptions, BopSession},
    checkprops::{Expected, checkprops},
    checkshape::checkshape,
    checkview::{CheckView, checkview},
    named_checks::{
        Counting, bopargcheck_faulties, bopcheck, checkmaxtol, checknbshapes, checknbshapes_ref,
        checkprops_equal, checksection, maximal_tolerance,
    },
    named_shapes::{bound, copy, explode_children, restore, tcopy},
    properties::Prop,
    session::Session,
};
use martensite_tkbo::bop_algo::Operation;

macro_rules! brep {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../../../../external/martensite-test-data/dataset-7.9.0/brep/",
            $name
        ))
    };
}

fn solid_counts(wires: usize, faces: usize) -> [(&'static str, usize); 4] {
    [("WIRE", wires), ("FACE", faces), ("SHELL", 1), ("SOLID", 1)]
}

fn checks_area_volume(s: &dyn Session, name: &str, area: f64, volume: f64) {
    checkprops(s, name, Prop::Area, Expected::Mass(area));
    checkprops(s, name, Prop::Volume, Expected::Mass(volume));
}

/// `bbop name op` for the five operations, in the scripts' order.
fn five_operations(s: &mut dyn Session, bop: &mut BopSession, prefix: &str) {
    let operations = [
        Operation::Common,
        Operation::Fuse,
        Operation::Cut,
        Operation::Cut21,
        Operation::Section,
    ];
    for (index, kind) in operations.into_iter().enumerate() {
        bop.bbop(s, &format!("{prefix}{index}"), kind);
    }
}

pub fn bug25994(s: &mut dyn Session) {
    restore(s, "body", brep!("bug25994_body.brep"));
    restore(s, "wing", brep!("bug25994_wing.brep"));
    let mut bop = BopSession::default();
    bop.arguments(s, &["body"], &["wing"]);
    bop.bfillds();
    bop.bbop(s, "rs", Operation::Section);
    // The script asks for `result`, not yet bound, which reports no alone vertices.
    if checksection(s, "result", None) != 0 {
        println!("Error : Section is not closed");
    }
    if maximal_tolerance(&bound(s, "rs")) > 5.0e-5 {
        println!("Error: Tolerance after section is too large");
    }
    bop.bbop(s, "result", Operation::Fuse);
    checkshape(&bound(s, "result"));
    checknbshapes(s, "result", Counting::Default, &solid_counts(15, 13));
    checks_area_volume(s, "result", 1.76161e+006, 1.07392e+008);
    checkview(s, &CheckView::of("result"));
}

fn argcheck(s: &dyn Session, name: &str) {
    if bopargcheck_faulties(&bound(s, name), &BopOptions::default()) {
        println!("Error: bopargcheck has found some faulties in {name}");
    }
}

fn solid_result(s: &dyn Session, name: &str, counts: [usize; 2], masses: [f64; 2]) {
    checkshape(&bound(s, name));
    checknbshapes(
        s,
        name,
        Counting::Default,
        &solid_counts(counts[0], counts[1]),
    );
    checks_area_volume(s, name, masses[0], masses[1]);
    argcheck(s, name);
}

pub fn bug30202_1(s: &mut dyn Session) {
    restore(s, "b1", brep!("bug30202_src.brep"));
    restore(s, "b2", brep!("bug30202_tool.brep"));
    let mut bop = BopSession::default();
    bop.arguments(s, &["b1"], &["b2"]);
    bop.bfillds();
    five_operations(s, &mut bop, "r_");
    solid_result(s, "r_0", [10, 10], [3.07923, 0.15225]);
    solid_result(s, "r_1", [33, 31], [128.978, 46.7138]);
    solid_result(s, "r_2", [31, 29], [117.75, 45.3505]);
    solid_result(s, "r_3", [12, 12], [14.3071, 1.2111]);
    checkshape(&bound(s, "r_4"));
    checknbshapes(s, "r_4", Counting::Default, &[("VERTEX", 8), ("EDGE", 8)]);
    checkprops(s, "r_4", Prop::Length, Expected::Mass(8.28981));
    argcheck(s, "r_4");
    checksection(s, "r_4", Some(0));
    copy(s, "r_1", "result");
    checkmaxtol(s, "result", 1.0e-4);
    checkview(s, &CheckView::of("result"));
}

fn self_interfering(s: &dyn Session, name: &str, options: &BopOptions, words: &str) {
    if !bopcheck(&bound(s, name), options).contains("OK") {
        println!("Error: {name} {words}");
    }
}

/// The second, swapped session of `bug30760`: `cut` and `cut21` trade places.
fn bug30760_swapped(s: &mut dyn Session) {
    explode_children(s, "a21");
    let mut bop = BopSession::default();
    bop.arguments(s, &["a21_2"], &["a21_1"]);
    bop.bfillds();
    bop.bbop(s, "r21_0", Operation::Common);
    bop.bbop(s, "r21_1", Operation::Fuse);
    bop.bbop(s, "r21_3", Operation::Cut);
    bop.bbop(s, "r21_2", Operation::Cut21);
    bop.bbop(s, "r21_4", Operation::Section);
    bop.bbuild(s, "r21_5");
}

pub fn bug30760(s: &mut dyn Session) {
    restore(s, "a12", brep!("bug30760_shapes.brep"));
    tcopy(s, "a12", "a21");
    explode_children(s, "a12");
    let mut bop = BopSession::default();
    bop.arguments(s, &["a12_1"], &["a12_2"]);
    bop.bfillds();
    five_operations(s, &mut bop, "r12_");
    bop.bbuild(s, "r12_5");
    bug30760_swapped(s);
    let options = BopOptions::default();
    for index in 0..6 {
        let (first, second) = (format!("r12_{index}"), format!("r21_{index}"));
        checkshape(&bound(s, &first));
        self_interfering(s, &first, &options, "is self-intersecting shape");
        checkprops_equal(s, &first, &second);
        checknbshapes_ref(s, &first, &second);
    }
    for index in 0..6 {
        let name = format!("r21_{index}");
        checkshape(&bound(s, &name));
        self_interfering(s, &name, &options, "is self-intersecting shape");
    }
    bug30760_masses(s);
    checkview(s, &CheckView::of("r12_5"));
}

fn bug30760_masses(s: &mut dyn Session) {
    let total = Counting::Total;
    checks_area_volume(s, "r12_0", 0.0550925, 0.000106046);
    checknbshapes(s, "r12_0", total, &solid_counts(14, 14));
    checks_area_volume(s, "r12_1", 0.84475, 0.00502274);
    checknbshapes(s, "r12_1", total, &solid_counts(34, 34));
    checks_area_volume(s, "r12_2", 0.765963, 0.00366102);
    checknbshapes(s, "r12_2", total, &solid_counts(24, 24));
    checks_area_volume(s, "r12_3", 0.117011, 0.00125568);
    checknbshapes(s, "r12_3", total, &solid_counts(14, 14));
    checkprops(s, "r12_4", Prop::Length, Expected::Mass(1.99738));
    checksection(s, "r12_4", Some(8));
    checks_area_volume(s, "r12_5", 0.99316, 0.00512878);
    let solids = [("WIRE", 43), ("FACE", 43), ("SHELL", 3), ("SOLID", 3)];
    checknbshapes(s, "r12_5", total, &solids);
}

/// The body of both `bug31850` cases, which differ only in the options of the session.
fn bug31850(s: &mut dyn Session, options: BopOptions) {
    restore(s, "s", brep!("bug31850.brep"));
    explode_children(s, "s");
    let mut bop = BopSession::with_options(options);
    bop.arguments(s, &["s_1"], &["s_2"]);
    bop.bfillds();
    five_operations(s, &mut bop, "r");
    bop.bbuild(s, "rgf");
    // The options are back to their defaults for the checks: `boptions -default`, or never changed.
    let checking = BopOptions::default();
    for name in ["r0", "r1", "r2", "r3", "r4", "rgf"] {
        checkshape(&bound(s, name));
        self_interfering(s, name, &checking, "is a self-interfering shape");
    }
    let total = Counting::Total;
    checknbshapes(s, "r0", total, &solid_counts(28, 28));
    checks_area_volume(s, "r0", 1107.94, 472.151);
    checknbshapes(s, "r1", total, &solid_counts(50, 45));
    checks_area_volume(s, "r1", 14959.7, 37939.8);
    checknbshapes(s, "r2", total, &solid_counts(33, 28));
    checks_area_volume(s, "r2", 5244.61, 2516.56);
    checknbshapes(s, "r3", total, &solid_counts(45, 45));
    checks_area_volume(s, "r3", 10823., 34951.1);
    checkprops(s, "r4", Prop::Length, Expected::Mass(203.491));
    checksection(s, "r4", Some(10));
    let solids = [("WIRE", 78), ("FACE", 73), ("SHELL", 3), ("SOLID", 3)];
    checknbshapes(s, "rgf", total, &solids);
    checks_area_volume(s, "rgf", 17175.6, 37939.8);
    checkview(s, &CheckView::of("r0"));
}

pub fn bug31850_1(s: &mut dyn Session) {
    bug31850(s, BopOptions::default());
}

pub fn bug31850_2(s: &mut dyn Session) {
    let options = BopOptions {
        fuzzy: 1.0e-6,
        non_destructive: true,
        run_parallel: true,
        fill_history: false,
    };
    bug31850(s, options);
}

pub fn bug32578(s: &mut dyn Session) {
    restore(s, "f", brep!("bug32578_face.brep"));
    restore(s, "ce", brep!("bug32578_edges.brep"));
    let options = BopOptions {
        fuzzy: 0.01,
        ..BopOptions::default()
    };
    let edges = explode_children(s, "ce");
    let tools: Vec<&str> = edges.iter().map(String::as_str).collect();
    let mut bop = BopSession::with_options(options);
    bop.arguments(s, &["f"], &tools);
    bop.bfillds();
    bop.bsplit(s, "result");
    checkshape(&bound(s, "result"));
    if !bopcheck(&bound(s, "result"), &options).contains("This shape seems to be OK") {
        println!("Error: result is a self-interferring shape");
    }
    let counts = [
        ("VERTEX", 646),
        ("EDGE", 1234),
        ("WIRE", 589),
        ("FACE", 589),
    ];
    checknbshapes(s, "result", Counting::Total, &counts);
    checkprops(s, "result", Prop::Area, Expected::Mass(351.033));
    checkview(s, &CheckView::of("result"));
}
