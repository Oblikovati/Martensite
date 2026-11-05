//! `bugs_modalg_1` bodies written by hand: variables, conditions and read-back values.
use crate::support::{
    bugs_commands::{
        beyond::same_shape,
        geometry::{mkcurve, tcopy_checked},
        inspect::{
            checksection_count, checksection_put, checkshape_report_line, distmini, dump_real,
            maxtolerance, maxtolerance_put, vertex_max_tolerance,
        },
        options_bop::ScriptBop,
    },
    checkprops::{Expected, checkprops},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview},
    commands::{bound_shape, explode},
    construction::copy,
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes, nbshapes},
    one_step::{bcut, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{mkface, plane},
    topology_commands::halfspace,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

fn view_result(s: &mut dyn Session) {
    checkview(s, &CheckView::of("result"));
}

pub fn buc60533(s: &mut dyn Session) {
    println!("==========");
    println!("BUC60533");
    println!("==========");
    restore(s, "buc60533a.brep", "part");
    checkshape_named(s, "part");
    restore(s, "buc60533b.brep", "trTool");
    checkshape_named(s, "trTool");
    let mut bop = ScriptBop::default();
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["part", "trTool"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    println!(" BUC60533 OK: function CUT works properly");
    explode(s, "r", None);
    copy(s, "r_1", "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.5003e+007));
    checkshape_named(s, "result");
    view_result(s);
    // Beyond the script: checks an earlier test of this case made.
    same_shape(s, "result", "r_1");
}

pub fn buc60712(s: &mut dyn Session) {
    println!("==========");
    println!("BUC60712");
    println!("==========");
    println!();
    println!("==================================");
    println!("It takes visual check for this BUG");
    println!("==================================");
    restore(s, "buc60712a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60712b.brep", "b");
    checkshape_named(s, "b");
    explode(s, "a", Some(ShapeEnum::Face));
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "a_1", "a_2");
    two_step.result(s, "r12", Operation::Fuse);
    two_step.bop(s, "a_3", "a_4");
    two_step.result(s, "r34", Operation::Fuse);
    two_step.bop(s, "r12", "r34");
    two_step.result(s, "r", Operation::Fuse);
    two_step.bop(s, "r", "b");
    two_step.section(s, "result");
    if checksection_count(s, "result") != 0 {
        println!(" Faulty BUC60712: Result shape is INcorrect !!! ");
    } else {
        println!("BUC60712 OK: Result shape is correct ");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(1.88145));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

pub fn buc60880(s: &mut dyn Session) {
    println!("============");
    println!("BUC60880");
    println!("============");
    println!();
    restore(s, "BUC60880_sec_error2.brep", "sh");
    checkshape_named(s, "sh");
    let pl = plane([65000., 19000., 2600.], [1., 0., 0.]);
    mkface(s, "f", &pl);
    bsection(s, "result", "sh", "f");
    let edges = nbshapes(s, "result", Counting::Located)[1];
    if edges == 1 {
        println!("BUC60880 OK: SECTION operation was made properly");
    } else {
        // The script's other branch calls `OCC333`, a command that does not exist: a script error.
        crate::support::script_error::script_error("invalid command name \"OCC333\"");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(1424.94));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

/// The script's six lines on a cut's name: `halfspace`, `bcut`, `checkshape`.
fn halfspace_cut(s: &mut dyn Session, tool: (&str, &str), at: [f64; 3], cut: (&str, &str, &str)) {
    let (restored, name) = tool;
    restore(s, restored, name);
    let (space, previous, result) = cut;
    halfspace(s, space, name, at);
    bcut(s, result, previous, space);
}

fn checkshape_r_says_ok(s: &dyn Session, name: &str) {
    if checkshape_report_line(s, name).contains("OK") {
        println!("Checking by checkshape - OK");
    } else {
        println!("Faulty : mistakes are found in checked shape by checkshape command");
    }
}

/// The four `mkcurve` lines over the first four edges of `cut`.
fn curves_of_four_edges(s: &mut dyn Session, cut: &str) {
    explode(s, cut, Some(ShapeEnum::Edge));
    for index in 1..=4 {
        mkcurve(s, &format!("{cut}_{index}"));
    }
}

fn tolerance_limit(s: &dyn Session, name: &str, limit: f64, faulty: &str, fine: &str) {
    let maxima = maxtolerance(s, name);
    let over = [maxima.face, maxima.edge, maxima.vertex]
        .into_iter()
        .any(|value| value.unwrap() > limit);
    println!("{}", if over { faulty } else { fine });
}

fn buc60555_start(s: &mut dyn Session) {
    println!("CPU and elapsed time limit set to 1000 seconds");
    restore(s, "buc60555a.brep", "part");
    checkshape_named(s, "part");
    restore(s, "buc60555b.brep", "trTool1");
    checkshape_named(s, "trTool1");
    halfspace(s, "hs1", "trTool1", [20000., 10000., 0.]);
    bcut(s, "cut1", "part", "hs1");
    checkshape_named(s, "cut1");
}

pub fn buc60555_1(s: &mut dyn Session) {
    println!("===========");
    println!("BUC60555");
    println!("CUT Case 1");
    println!("===========");
    buc60555_start(s);
    tcopy_checked(s, "cut1", "result");
    curves_of_four_edges(s, "cut1");
    tolerance_limit(
        s,
        "cut1",
        1.,
        "Faulty :Tolerance of shape is more then 1.0",
        "Tolerance of shape is less then 1.0",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(3.77048e+007));
    checkshape_named(s, "result");
    view_result(s);
}

pub fn buc60555_2(s: &mut dyn Session) {
    println!("=============");
    println!("BUC60555");
    println!("CUT Case2");
    println!("=============");
    buc60555_start(s);
    checkshape_r_says_ok(s, "cut1");
    maxtolerance_put(s, "cut1");
    halfspace_cut(
        s,
        ("buc60555c.brep", "trTool2"),
        [10000., 10000., 0.],
        ("hs2", "cut1", "cut2"),
    );
    tcopy_checked(s, "cut2", "result");
    curves_of_four_edges(s, "cut2");
    checkshape_r_says_ok(s, "cut2");
    checkshape_named(s, "cut2");
    tolerance_limit(
        s,
        "cut2",
        1.,
        "Faulty : Tolerance of shape is more then 1.0",
        "Tolerance of shape is less then 1.0",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(3.66928e+007));
    checkshape_named(s, "result");
    view_result(s);
}

pub fn buc60555_3(s: &mut dyn Session) {
    println!("==============");
    println!("BUC60555");
    println!("CUT Case 3");
    println!("==============");
    buc60555_start(s);
    checkshape_r_says_ok(s, "cut1");
    maxtolerance(s, "cut1");
    halfspace_cut(
        s,
        ("buc60555c.brep", "trTool2"),
        [10000., 10000., 0.],
        ("hs2", "cut1", "cut2"),
    );
    checkshape_named(s, "cut2");
    checkshape_r_says_ok(s, "cut2");
    maxtolerance(s, "cut2");
    halfspace_cut(
        s,
        ("buc60555d.brep", "trTool3"),
        [20000., 1000., 0.],
        ("hs3", "cut2", "cut3"),
    );
    tcopy_checked(s, "cut3", "result");
    curves_of_four_edges(s, "cut3");
    checkshape_r_says_ok(s, "cut3");
    checkshape_named(s, "cut3");
    tolerance_limit(
        s,
        "cut3",
        2.,
        "Faulty : Tolerance of shape is more then 2.0",
        "Tolerance of shape is less then 2.0",
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(1.24886e+007));
    checkshape_named(s, "result");
    view_result(s);
}

/// The common tail of the three known-problem section cases.
fn zero_length_tail(s: &mut dyn Session) {
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

fn inputs_checked(s: &mut dyn Session, first: &str, second: &str) {
    restore(s, first, "a");
    checkshape_named(s, "a");
    restore(s, second, "b");
    checkshape_named(s, "b");
}

/// `distmini d last_vertex b_5`, then the tolerance of the vertex as the regexp reads it.
fn distance_against_tolerance(s: &mut dyn Session, vertex: &str) {
    explode(s, "b", Some(ShapeEnum::Edge));
    explode(s, "b_4", Some(ShapeEnum::Vertex));
    let distance = distmini(s, "d", vertex, "b_5");
    let dist_text = dump_real("d_val", distance);
    let tolerance = vertex_max_tolerance(s, vertex);
    if distance > tolerance.0 {
        println!(
            "Faulty : the distance is {dist_text}. It is more {}.",
            tolerance.1
        );
    }
}

pub fn buc60663_2(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Faulty : the distance is");
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("=====================================");
    println!("BUC60663");
    println!("=====================================");
    println!();
    println!("It takes visual check for this BUG.");
    println!("=====================================");
    println!();
    inputs_checked(s, "buc60663a.brep", "buc60663b.brep");
    bsection(s, "result", "a", "b");
    let parts = explode(s, "result", Some(ShapeEnum::Vertex));
    let last = parts
        .last()
        .unwrap_or_else(|| panic!("result: expected vertices"))
        .clone();
    distance_against_tolerance(s, &last);
    zero_length_tail(s);
}

pub fn bug144(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Faulty : the distance is");
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("================");
    println!("BUC60663");
    println!("OCC144");
    println!("================");
    println!();
    inputs_checked(s, "buc60663a.brep", "buc60663b.brep");
    bsection(s, "result", "a", "b");
    checksection_put(s, "result");
    let parts = explode(s, "result", Some(ShapeEnum::Vertex));
    let last = parts
        .last()
        .unwrap_or_else(|| panic!("result: expected vertices"))
        .clone();
    distance_against_tolerance(s, &last);
    zero_length_tail(s);
}

pub fn buc60690(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Faulty : The distance between");
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("========================");
    println!("BUC60690");
    println!("========================");
    println!();
    restore(s, "buc60690a.brep", "a");
    restore(s, "buc60690b.brep", "b");
    if checkshape(&bound_shape(s, "a")).has_faulty() {
        println!(
            "Faulty BUC60690 (shape 1): Source shape is invalid. It was detected by Checkshape command"
        );
        println!("Body of the script was NOT executed");
        renamevar(s, "a", "result");
    } else {
        println!("BUC60690 OK (shape 1): Source shape is valid");
        buc60690_second_shape(s);
    }
    zero_length_tail(s);
}

fn buc60690_second_shape(s: &mut dyn Session) {
    if checkshape(&bound_shape(s, "b")).has_faulty() {
        println!(
            "Faulty BUC60690 (shape 2): Source shape is invalid. It was detected by Checkshape command"
        );
        println!("Body of the script was NOT executed");
        renamevar(s, "b", "result");
        return;
    }
    println!("BUC60690 OK (shape 2): Source shape is valid");
    bsection(s, "result", "a", "b");
    let parts = explode(s, "result", Some(ShapeEnum::Vertex));
    let first = distmini(s, "d1", "result_1", "b");
    let last = distmini(s, "d2", parts.last().unwrap(), "b");
    for (distance, word, value) in [(first, "first", "d1_val"), (last, "last", "d2_val")] {
        let text = dump_real(value, distance);
        if distance > 1e-7 {
            println!(
                "Faulty : The distance between {word} vertex and shellB is {text}. It is more then 1e-7."
            );
        }
    }
}

pub fn bug15850_4(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC15850");
    println!("============");
    println!();
    restore(s, "OCC15850_bb.brep", "bb");
    restore(s, "OCC15850_bc.brep", "bc");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "bb", "bc");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(37.6991));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(9),
        ],
    );
    if checksection_count(s, "result") != 0 {
        println!("Faulty OCC15850");
    } else {
        println!("OK OCC15850");
    }
    view_result(s);
}
