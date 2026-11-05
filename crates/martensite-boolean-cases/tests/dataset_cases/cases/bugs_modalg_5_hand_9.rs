//! `bugs_modalg_5` bodies written by hand: variables, loops and read-back values.
#![allow(unused_imports)]
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::{
        beyond::{child_count, is_kind, same_shape},
        inspect::{bopcheck_text, vertex_dump},
        options_bop::ScriptBop,
        repair::{mkvolume, sewing},
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{compound, explode, make_box, psphere, trotate, ttranslate},
    construction::{copy, edge, nurbsconvert, tcopy, vertex},
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes, nbshapes},
    one_step::bsection,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{mkface, mkface_bounded, polyline},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(crate) fn view_result(s: &mut dyn Session) {
    checkview(s, &CheckView::of("result"));
}

pub(crate) fn names_of(owned: &[String]) -> Vec<&str> {
    owned.iter().map(String::as_str).collect()
}

pub fn bug24620(s: &mut dyn Session) {
    println!("============");
    println!("OCC24620");
    println!("============");
    println!();
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [10., 0., 0.]);
    vertex(s, "v3", [4., -5., 0.]);
    vertex(s, "v4", [4., 5., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v3", "v4");
    compound(s, &["e1", "e2"], "b1");
    crate::support::bugs_commands::inspect::bopcheck_binding(s, "b1");
    // `x0` holds nothing, and `compound` skips a null shape.
    compound(s, &["x1", "b1"], "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &all_nine([4, 2, 0, 0, 0, 0, 0, 2, 8]),
    );
    view_result(s);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::bound_not_other(s, "x1", "x0");
}

pub fn bug25801(s: &mut dyn Session) {
    println!("=========");
    println!("CR25801");
    println!("=========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    let log = bopcheck_text(s, "b1", &BopOptions::default());
    if log.contains("Elapsed time") {
        println!("Error: bopcheck show the execution time without -t key");
    } else {
        println!("OK: bopcheck does not show the execution time without -t key");
    }
    // Beyond the script: checks an earlier test of this case made.
    assert!(
        !log.contains("Tps:"),
        "bopcheck without -t: expected no time, got {log:?}"
    );
}

pub fn bug23881(s: &mut dyn Session) {
    println!("============");
    println!("OCC23881");
    println!("============");
    println!();
    polyline(
        s,
        "w1",
        &[
            [0., 0., 0.],
            [1., 0., 0.],
            [1., 1., 0.],
            [0., 1., 0.],
            [0., 0., 0.],
        ],
    );
    polyline(
        s,
        "w2",
        &[
            [0., 1., 0.],
            [1., 1., 0.],
            [1., 2., 0.],
            [0., 2., 0.],
            [0., 1., 0.],
        ],
    );
    crate::support::construction::mkplane(s, "f1", "w1");
    crate::support::construction::mkplane(s, "f2", "w2");
    sewing(s, "s1", 0.0001, &["f1", "f2"]);
    let plane = crate::support::surface_commands::plane([0., 1., 0.], [0., 1., 0.]);
    mkface(s, "f3", &plane);
    bsection(s, "r", "s1", "f3");
    compound(s, &["r", "s1"], "c");
    let sewn_edges = nbshapes(s, "s1", Counting::Located)[1];
    let whole_edges = nbshapes(s, "c", Counting::Located)[1];
    if sewn_edges != whole_edges {
        println!("Error: section does not share common edge with original shape");
    }
    renamevar(s, "r", "result");
    view_result(s);
}

pub fn bug25354_50(s: &mut dyn Session) {
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b", [-5., -5., -5.]);
    let mut faces = Vec::new();
    for i in 0..41 {
        let name = format!("b{{{i}}}");
        psphere(s, &name, i as f64 * 0.1 + 5.);
        explode(s, &name, Some(ShapeEnum::Face));
        faces.push(format!("{name}_1"));
    }
    compound(s, &names_of(&faces), "q");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b"]);
    bop.add_tools(s, &["q"]);
    bop.bfillds_timed();
    bop.bbop_timed(s, "result", Operation::Section);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &all_nine([530, 524, 0, 0, 0, 0, 0, 1, 1055]),
    );
    view_result(s);
}

fn all_nine(counts: [usize; 9]) -> [NbCount; 9] {
    [
        NbCount::Vertices(counts[0]),
        NbCount::Edges(counts[1]),
        NbCount::Wires(counts[2]),
        NbCount::Faces(counts[3]),
        NbCount::Shells(counts[4]),
        NbCount::Solids(counts[5]),
        NbCount::CompSolids(counts[6]),
        NbCount::Compounds(counts[7]),
        NbCount::Shapes(counts[8]),
    ]
}

/// `tcopy` of a prototype, moved by `delta`, kept in `names`.
pub(crate) fn translated_copy(
    s: &mut dyn Session,
    from: &str,
    name: String,
    delta: [f64; 3],
    names: &mut Vec<String>,
) {
    tcopy(s, from, &name);
    ttranslate(s, &name, delta);
    names.push(name);
}

pub fn bug25354_49(s: &mut dyn Session) {
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b", [0., 0., 0.], [100., 100., 100.]);
    explode(s, "b", Some(ShapeEnum::Face));
    let mut faces = Vec::new();
    for i in 1..10 {
        let at = (i * 10) as f64;
        translated_copy(s, "b_1", format!("f1_{i}"), [at, 0., 0.], &mut faces);
    }
    for i in 1..10 {
        let at = (i * 10) as f64;
        translated_copy(s, "b_3", format!("f3_{i}"), [0., at, 0.], &mut faces);
    }
    for i in 1..10 {
        let at = (i * 10) as f64;
        translated_copy(s, "b_5", format!("f5_{i}"), [0., 0., at], &mut faces);
    }
    for i in 1..11 {
        let name = format!("s_{i}");
        psphere(s, &name, (i * 5) as f64);
        ttranslate(s, &name, [50., 50., 50.]);
        faces.push(name);
    }
    compound(s, &names_of(&faces), "q");
    let mut bop = ScriptBop::default();
    bop.add_compound(s, "q");
    bop.bfillds_timed();
    bop.bbop_timed(s, "result", Operation::Section);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &all_nine([2781, 7332, 0, 0, 0, 0, 0, 1, 10114]),
    );
    view_result(s);
}

/// One family of `bug25232_9`'s crossing faces, in `compound name`.
fn crossing_faces(s: &mut dyn Session, name: &str, normal: [f64; 3]) {
    compound(s, &[], name);
    for i in 0..21 {
        let step = (i - 10) as f64;
        let plane = crate::support::surface_commands::plane(normal.map(|axis| axis * step), normal);
        let face = format!("f_{i}");
        mkface_bounded(s, &face, &plane, (-10., 10., -10., 10.));
        crate::support::commands::add(s, &face, name);
    }
}

pub fn bug25232_9(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    crossing_faces(s, "b1", [0., 0., 1.]);
    crossing_faces(s, "b2", [1., 0., 0.]);
    crossing_faces(s, "b3", [0., 1., 0.]);
    let surface = crate::support::bugs_commands::geometry::sphere([0., 0., 0.], 10.);
    mkface(s, "f", &surface);
    mkvolume(s, "result", &["b1", "b2", "b3", "f"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(50513.3));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &all_nine([11037, 31880, 30668, 30668, 9832, 9832, 0, 1, 123918]),
    );
    view_result(s);
}

/// One axis of `bug24639`'s grid of edges, as a NURBS compound.
fn edge_grid(
    s: &mut dyn Session,
    axis: char,
    end: [f64; 3],
    offset: impl Fn(f64, f64) -> [f64; 3],
) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", end);
    let prototype = format!("e{axis}");
    edge(s, &prototype, "v1", "v2");
    let mut names = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            let at = offset(i as f64 + 1., j as f64 + 1.);
            translated_copy(
                s,
                &prototype,
                format!("e{axis}_{i}_{{{j}}}"),
                at,
                &mut names,
            );
        }
    }
    compound(s, &names_of(&names), &format!("b{axis}"));
}

pub fn bug24639(s: &mut dyn Session) {
    println!("=========");
    println!("OCC24639");
    println!("=========");
    println!();
    let side = 5. + 1.;
    edge_grid(s, 'z', [0., 0., side], |i, j| [i, j, 0.]);
    edge_grid(s, 'x', [side, 0., 0.], |i, j| [0., i, j]);
    edge_grid(s, 'y', [0., side, 0.], |i, j| [i, 0., j]);
    for name in ["bx", "by", "bz"] {
        nurbsconvert(s, name, name);
    }
    let mut bop = ScriptBop::default();
    for name in ["bx", "by", "bz"] {
        bop.add_compound(s, name);
    }
    bop.options.run_parallel = false;
    let sequential = bop.bfillds_timed();
    bop.options.run_parallel = true;
    let parallel = bop.bfillds_timed();
    if parallel > 0.8 * sequential {
        println!("Error: Parallel execution does not give expected performance speed up");
    }
}

pub fn bug23985(s: &mut dyn Session) {
    println!("========");
    println!("OCC23985");
    println!("========");
    println!();
    restore(s, "bug23985_b1.brep", "b1");
    restore(s, "bug23985_b2.brep", "b2");
    bsection(s, "result", "b1", "b2");
    let count = explode(s, "result", None).len();
    println!("Result is {count} section edge");
    if count > 0 {
        println!("23985 OK");
    } else {
        println!("Faulty 23985");
    }
    view_result(s);
}

pub fn bug24496(s: &mut dyn Session) {
    println!("============");
    println!("OCC24496");
    println!("============");
    println!();
    restore(s, "bug24496_face1.brep", "f1");
    restore(s, "bug24496_face2.brep", "f2");
    bsection(s, "result", "f1", "f2");
    if explode(s, "result", None).len() != 2 {
        println!("Error: wrong number of intersections. Should be result_1 and result_2");
    } else {
        println!("OK: bsection works properly");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(11.7031));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::alone_count(s, "result", 4);
}

/// The three coordinate lines of `bug24811`: a reference value within the vertex's tolerance.
fn point_within_tolerance(
    point: usize,
    coordinates: [f64; 3],
    reference: [f64; 3],
    tolerance: f64,
) {
    for index in 0..3 {
        let (at, wanted) = (coordinates[index], reference[index]);
        let label = format!("point{point}_{}", index + 1);
        if wanted >= at - tolerance && wanted <= at + tolerance {
            println!("OK: {label} is correct");
        } else {
            println!("Error: {label} is incorrect");
        }
    }
}

pub fn bug24811(s: &mut dyn Session) {
    println!("============");
    println!("OCC24811");
    println!("============");
    println!();
    restore(s, "bug24811_e1.brep", "curve");
    restore(s, "bug24811_e2.brep", "circle");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "curve", "circle");
    two_step.result(s, "result", Operation::Cut);
    if explode(s, "result", None).len() != 4 {
        println!(
            "Error: wrong number of intersections. Should be result_1 result_2 result_3 result_4"
        );
    } else {
        println!("OK: right number of intersections");
    }
    explode(s, "result_2", None);
    let (first, tolerance) = vertex_dump(s, "result_2_1");
    point_within_tolerance(1, first, [-2.22458486160362e-016, 1., 0.], tolerance);
    let (second, tolerance) = vertex_dump(s, "result_2_2");
    point_within_tolerance(
        2,
        second,
        [0.0202691578002498, 0.999794559518151, 0.],
        tolerance,
    );
    view_result(s);
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "result_2", 2);
}

pub fn bug25111(s: &mut dyn Session) {
    println!("============");
    println!("OCC25111");
    println!("============");
    println!();
    let first = crate::support::bugs_commands::geometry::restore_surface("bug25111_s1.draw");
    let second = crate::support::bugs_commands::geometry::restore_surface("bug25111_s2.draw");
    mkface(s, "f1", &first);
    mkface(s, "f2", &second);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "f1", "f2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(53.1714));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &all_nine([4, 2, 0, 0, 0, 0, 0, 1, 7]),
    );
    view_result(s);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::alone_count(s, "result", 3);
}

pub fn bug25346_1(s: &mut dyn Session) {
    println!("==========");
    println!("OCC25346");
    println!("==========");
    println!();
    restore(s, "CTO902_pro14235a.rle", "b1");
    restore(s, "CTO902_pro14235b.rle", "b2");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    // `bfillds -s` names a key the command does not know, then runs as without it.
    println!("Warning: invalid key");
    bop.bfillds();
    bop.bbuild(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(470659.));
    checkshape_named(s, "result");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Solids(3)]);
    view_result(s);
}
