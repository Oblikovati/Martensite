//! `bugs_modalg_6` bodies written by hand: history queries, read-back values and loops.
use crate::support::{
    bugs_commands::{
        geometry::{prism_semi_infinite, restore_surface, trim_surface},
        inspect::{bopargcheck, checksection_count, whatis_has, xbounds},
        options_bop::{ScriptBop, isdeleted, modified},
        view::{donly, fit, smallview, xwd},
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    commands::{bound_shape, explode, make_box, ttranslate},
    construction::{NO_VALID_RANGE, valid_range_answer},
    nbshapes::{Counting, NbCount, checknbshapes, nbshapes},
    one_step::bsection,
    properties::{Prop, props},
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{mkface, plane},
    tolerance_check::{checkreal, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug26565_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC26565");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [5., 0., 0.], [10., 10., 10.]);
    explode(s, "b2", Some(ShapeEnum::Shell));
    crate::support::construction::copy(s, "b2_1", "b2");
    make_box(s, "b3", [5., 0., 5.], [10., 10., 10.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1", "b2"]);
    bop.add_tools(s, &["b3"]);
    bop.bfillds();
    bop.bbop(s, "r", Operation::Common);
    nbshapes(s, "r", Counting::Located);
    bop.bbop(s, "r", Operation::Cut);
    nbshapes(s, "r", Counting::Located);
    explode(s, "r", None);
    for (name, kind) in [("r_1", "SHELL"), ("r_2", "SOLID"), ("r_3", "SOLID")] {
        if whatis_has(s, name, kind) {
            println!("OK : There is {kind}; Compsolid is good");
        } else {
            println!("Error : There is not {kind}; Compsolid is bad");
        }
    }
}

fn bounds_match(s: &dyn Session, face: &str) {
    let bounds = xbounds(s, face);
    let expected = [
        ("UMin", "0.0"),
        ("UMax", "1.0"),
        ("VMin", "-1e+100"),
        ("VMax", "0.0"),
    ];
    for (index, (name, wanted)) in expected.into_iter().enumerate() {
        checkreal(name, &bounds[index], wanted, 0.0, 1.0e-7);
    }
}

pub fn bug28795(s: &mut dyn Session) {
    println!("=======");
    println!("OCC28795");
    println!("=======");
    println!();
    make_box(s, "mb", [-0.5, -0.5, -0.5], [1., 1., 1.]);
    explode(s, "mb", Some(ShapeEnum::Face));
    prism_semi_infinite(s, "pryz", "mb_1", [1., 0., 0.]);
    make_box(s, "ab", [0., -1., -1.], [2., 2., 2.]);
    explode(s, "ab", Some(ShapeEnum::Face));
    explode(s, "pryz", Some(ShapeEnum::Face));
    bounds_match(s, "pryz_1");
    bsection(s, "rs", "ab_2", "pryz_1");
    bounds_match(s, "pryz_1");
}

/// The line pair of `bug26393`: `modified` and `isdeleted`, with what the script requires of each.
fn modified_as_expected(
    s: &mut dyn Session,
    history: &martensite_tkernel::standard::Handle<martensite_tkbrep::brep_tools::History>,
    (name, shape): (&str, &str),
    unmodified: bool,
) {
    let answer = modified(s, name, history, shape);
    let wanted = if unmodified {
        "The shape has not been modified."
    } else {
        ""
    };
    if answer != wanted {
        let tail = if unmodified {
            "does not work."
        } else {
            "does not work correctly."
        };
        println!("ERROR: OCC26393 is reproduced. Command modified {tail}");
    }
    if isdeleted(s, history, shape) != "Not deleted." {
        println!("ERROR: OCC26393 is reproduced. Command isdeleted does not work.");
    }
    // Beyond the script: an earlier test checked that only a modified shape publishes a name.
    assert_eq!(
        s.shape(name).is_some(),
        !unmodified,
        "modified {name}: named publication"
    );
}

pub fn bug26393(s: &mut dyn Session) {
    println!("========");
    println!("OCC26393");
    println!("========");
    println!();
    restore(s, "OCC26393-w.brep", "w");
    restore(s, "OCC26393-v1.brep", "v0");
    restore(s, "OCC26393-v2.brep", "v1");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["w"]);
    bop.add_tools(s, &["v0", "v1"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    let history = bop
        .saved_history()
        .expect("savehistory: expected a history");
    for (name, vertex) in [("v0m", "v0"), ("v1m", "v1")] {
        if modified(s, name, &history, vertex) != "The shape has not been modified." {
            println!("ERROR: OCC26393 is reproduced. Command modified does not work.");
        }
    }
    for vertex in ["v0", "v1"] {
        if isdeleted(s, &history, vertex) != "Not deleted." {
            println!("ERROR: OCC26393 is reproduced. Command isdeleted does not work.");
        }
    }
    explode(s, "w", Some(ShapeEnum::Edge));
    modified_as_expected(s, &history, ("w1m", "w_1"), false);
    modified_as_expected(s, &history, ("w2m", "w_2"), true);
    modified_as_expected(s, &history, ("w3m", "w_3"), true);
    modified_as_expected(s, &history, ("w4m", "w_4"), false);
    modified_as_expected(s, &history, ("w5m", "w_5"), true);
}

/// `mkface` of two restored surfaces, the first trimmed, shown, then their section.
fn trimmed_section(s: &mut dyn Session, files: [&str; 2], bounds: (f64, f64, f64, f64)) {
    let first = trim_surface(&restore_surface(files[0]), bounds);
    let second = restore_surface(files[1]);
    mkface(s, "f1", &first);
    mkface(s, "f2", &second);
    smallview(s);
    donly(s, &["f1", "f2"]);
    fit(s);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "f1", "f2");
    two_step.section(s, "result");
}

pub fn bug27079_3(s: &mut dyn Session) {
    println!("============");
    println!("OCC27079");
    println!("============");
    println!();
    trimmed_section(
        s,
        ["bug27079_s1.draw", "bug27079_s2.draw"],
        (-5., 5., 0., 20.),
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(13.7747));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
    screenshot(s, "photo");
}

pub fn bug27079_4(s: &mut dyn Session) {
    println!("============");
    println!("OCC27079");
    println!("============");
    println!();
    let pi = std::f64::consts::PI;
    trimmed_section(
        s,
        ["bug27079_s3.draw", "bug27079_s4.draw"],
        (pi - pi / 30., pi, -5., -2.),
    );
    checksection(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.5688));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(2)],
    );
    screenshot(s, "photo");
}

pub fn bug26910(s: &mut dyn Session) {
    println!("============");
    println!("OCC26910");
    println!("============");
    println!();
    restore(s, "bug26910.brep", "a");
    explode(s, "a", None);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["a_1"]);
    bop.add_tools(s, &["a_2"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    let count = explode(s, "r", Some(ShapeEnum::Vertex)).len();
    if count != 5 {
        println!("Error: {count} intersections were not found between line and circle");
    } else {
        println!("Found {count} intersection between line and circle");
    }
    smallview(s);
    fit(s);
    screenshot(s, "photo");
}

pub fn bug26310_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC26310");
    println!("========");
    println!();
    restore(s, "OCC26310-b1.brep", "b1");
    restore(s, "OCC26310-b2.brep", "b2");
    bsection(s, "result", "b1", "b2");
    smallview(s);
    donly(s, &["result"]);
    if checksection_count(s, "result") != 0 {
        println!("ERROR: the section is not closed");
    } else {
        println!("The section is OK");
    }
    fit(s);
    screenshot(s, "photo");
}

/// The common start of `bug26310_3` and `_4`, with the operands in the script's order.
fn cuts_of_either_order(s: &mut dyn Session, object: &str, tool: &str) {
    restore(s, "OCC26310-b1.brep", "b1");
    restore(s, "OCC26310-b2.brep", "b2");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &[object]);
    bop.add_tools(s, &[tool]);
    bop.bfillds();
    bop.bbop(s, "rr", Operation::Section);
    bop.bbop(s, "res1", Operation::Cut);
    bop.bbop(s, "res2", Operation::Cut21);
    checkshape_named(s, "res1");
    checkshape_named(s, "res2");
    let valid = "Shape(s) seem(s) to be valid for BOP.\n";
    let first = bopargcheck(s, ("res1", None), None, "");
    let second = bopargcheck(s, ("res2", None), None, "");
    if !first.eq_ignore_ascii_case(valid) {
        println!("ERROR. res1 is not valid for BOP");
    }
    if !second.eq_ignore_ascii_case(valid) {
        println!("ERROR. res2 is not valid for BOP");
    }
    let counts = [
        NbCount::Wires(6),
        NbCount::Faces(6),
        NbCount::Shells(1),
        NbCount::Solids(1),
    ];
    checknbshapes(s, "res1", Counting::Typed, &counts);
    checknbshapes(s, "res2", Counting::Typed, &counts);
    smallview(s);
}

pub fn bug26310_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC26310");
    println!("========");
    println!();
    cuts_of_either_order(s, "b1", "b2");
    donly(s, &["rr"]);
    fit(s);
    donly(s, &["res1"]);
    xwd(s, "1");
    donly(s, &["res2"]);
    fit(s);
    xwd(s, "2");
}

pub fn bug26310_4(s: &mut dyn Session) {
    println!("========");
    println!("OCC26310");
    println!("========");
    println!();
    cuts_of_either_order(s, "b2", "b1");
    donly(s, &["res1"]);
    fit(s);
    xwd(s, "1");
    donly(s, &["rr"]);
    fit(s);
    donly(s, &["res2"]);
    xwd(s, "2");
}

/// The script's loop over the section's edges: an edge with no valid range is an error.
fn every_edge_has_a_valid_range(s: &mut dyn Session, name: &str) {
    for part in explode(s, name, None) {
        let answer = valid_range_answer(&bound_shape(s, &part))
            .unwrap_or_else(|failure| panic!("validrange {part}: {failure:?}"));
        println!("{answer}");
        if answer == NO_VALID_RANGE {
            let length = props(Prop::Length, &bound_shape(s, &part), None);
            let tolerance = tolerance_report(s, &part);
            println!("Error: edge {part} (length={length}, tolerance={tolerance}) is too small");
        }
    }
}

/// `bug28690` and `_1`: the moved faces cut by the plane X=0, then the script's checks.
fn translated_section(
    s: &mut dyn Session,
    file: &str,
    delta: [f64; 3],
    counts: [usize; 2],
    length: f64,
) {
    restore(s, file, "a");
    ttranslate(s, "a", delta);
    let surface = plane([0., 0., 0.], [1., 0., 0.]);
    mkface(s, "f", &surface);
    bsection(s, "r", "a", "f");
    let expected = [NbCount::Vertices(counts[0]), NbCount::Edges(counts[1])];
    checknbshapes(s, "r", Counting::Located, &expected);
    checkshape_named(s, "r");
    checkprops(s, "r", Prop::Length, Expected::Mass(length));
    every_edge_has_a_valid_range(s, "r");
}

pub fn bug28690(s: &mut dyn Session) {
    println!("========");
    println!("OCC28690");
    println!("========");
    println!();
    translated_section(
        s,
        "bug28690_m8_faces.brep",
        [-20000., -584.41225425562368, 0.],
        [4, 3],
        594.045,
    );
}

pub fn bug28690_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC28690");
    println!("========");
    println!();
    translated_section(
        s,
        "bug28690_m8_faces_1.brep",
        [-20000., -2597.4029071873542, -8.9312813979176155],
        [3, 2],
        3896.32,
    );
}

pub fn bug28492_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC28492");
    println!("========");
    println!();
    restore(s, "a350", "a");
    restore(s, "b350", "b");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["a"]);
    bop.add_tools(s, &["b"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Shells(3), NbCount::Solids(3)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(253.741));
    checkprops(s, "result", Prop::Volume, Expected::Mass(82.9156));
    checkshape_named(s, "result");
    if !bopargcheck(s, ("result", None), None, "").contains("to be valid for BOP") {
        println!("Error: bopargcheck failed");
    }
    checkview(s, &CheckView::of("result"));
}

pub fn bug25102(s: &mut dyn Session) {
    println!("========");
    println!("OCC25102");
    println!("========");
    println!();
    restore(s, "bug25102.brep", "c");
    explode(s, "c", None);
    bsection(s, "result", "c_1", "c_2");
    if checksection_count(s, "result") != 2 {
        println!("Error : Section is incorrect");
    } else {
        println!("Section is correct");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(49.734));
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(44)]);
    smallview(s);
    fit(s);
    screenshot(s, "photo");
}

pub fn bug27221(s: &mut dyn Session) {
    println!("============");
    println!("OCC27221");
    println!("============");
    println!();
    restore(s, "bug27221.brep", "a");
    explode(s, "a", None);
    crate::support::construction::mkplane(s, "f", "a_2");
    crate::support::commands::prism(s, "p", "f", [-25., 0., 0.]);
    crate::support::one_step::bcut(s, "result", "a_1", "p");
    checkprops(s, "result", Prop::Volume, Expected::Mass(56860.2));
    checkprops(s, "result", Prop::Area, Expected::Mass(72076.));
    checkshape_named(s, "result");
    bopargcheck(s, ("result", None), None, "");
    crate::support::bugs_commands::inspect::checknbshapes_selected(
        s,
        "result",
        Counting::Typed,
        &[(4, 1), (5, 1), (3, 18)],
        "NbShapes in result",
    );
    checkview(s, &CheckView::of("result").deciding(Prop::Volume));
}
