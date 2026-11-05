//! `bugs_modalg_5`: the `bug24157` scripts, their loops and time limits.
#![allow(unused_imports)]
use super::bugs_modalg_5_hand_9::{names_of, translated_copy, view_result};
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

/// The last lines of each `bug24157` script: the time limit, then the view of the result.
fn low_performance_check(s: &mut dyn Session, seconds: f64, limit: f64) {
    if seconds > limit {
        println!("Error: low performance");
    } else {
        println!("OK: high performance");
    }
    view_result(s);
}

pub fn bug24157_8(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    make_box(s, "b", [0., 0., 0.], [100., 100., 100.]);
    explode(s, "b", Some(ShapeEnum::Face));
    copy(s, "b_1", "fx");
    copy(s, "b_3", "fy");
    copy(s, "b_5", "fz");
    let mut faces = Vec::new();
    for j in 1..19 {
        translated_copy(
            s,
            "fx",
            format!("fx_{j}"),
            [(j * 2) as f64, 0., 0.],
            &mut faces,
        );
    }
    for j in 1..19 {
        translated_copy(
            s,
            "fy",
            format!("fy_{j}"),
            [0., (j * 2) as f64, 0.],
            &mut faces,
        );
    }
    for j in 1..19 {
        translated_copy(
            s,
            "fz",
            format!("fz_{j}"),
            [0., 0., (j * 2) as f64],
            &mut faces,
        );
    }
    compound(s, &names_of(&faces), "b1");
    let mut bop = ScriptBop::default();
    bop.add_compound(s, "b1");
    bop.bfillds_timed();
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 15.);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::part_count(s, "b", ShapeEnum::Face, 6);
    same_shape(s, "fx", "b_1");
    is_kind(s, "fx_1", ShapeEnum::Face);
    crate::support::bugs_commands::beyond::argument_count(&bop, 54);
}

/// `sp` moved into its place in the nested groups of `bug24157_6` and `_7`.
fn sphere_groups(s: &mut dyn Session, groups: usize) -> ScriptBop {
    make_box(s, "b1", [0., 0., 0.], [100., 100., 100.]);
    psphere(s, "sp", 1.);
    explode(s, "sp", Some(ShapeEnum::Face));
    copy(s, "sp_1", "sp");
    let (ds, big) = (2.5, 10.);
    let mut faces = Vec::new();
    for n_group in 0..groups {
        for k_group in 0..groups {
            for m_group in 0..groups {
                for n in 0..3 {
                    for k in 0..3 {
                        for m in 0..3 {
                            let name = format!("sp_{n_group}_{k_group}_{m_group}_{n}_{k}_{m}");
                            tcopy(s, "sp", &name);
                            ttranslate(s, &name, [2.5, 2.5, 2.5]);
                            ttranslate(s, &name, [k as f64 * ds, m as f64 * ds, n as f64 * ds]);
                            ttranslate(
                                s,
                                &name,
                                [
                                    m_group as f64 * big,
                                    k_group as f64 * big,
                                    n_group as f64 * big,
                                ],
                            );
                            faces.push(name);
                        }
                    }
                }
            }
        }
    }
    compound(s, &names_of(&faces), "b2");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_compound(s, "b2");
    bop.bfillds();
    bop
}

pub fn bug24157_6(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    let mut bop = sphere_groups(s, 3);
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 24.);
}

pub fn bug24157_7(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    let mut bop = sphere_groups(s, 4);
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 135.);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::argument_count(&bop, 1729);
}

/// `count` copies of the moved face `f`, under `name` as a compound.
fn stacked_faces(
    s: &mut dyn Session,
    from: &str,
    shift: [f64; 3],
    prefix: &str,
    along: [f64; 3],
    compound_name: &str,
) {
    copy(s, from, "f");
    ttranslate(s, "f", shift);
    let mut names = Vec::new();
    for i in 1..5 {
        let at = (i * 2) as f64;
        translated_copy(
            s,
            "f",
            format!("{prefix}_{i}"),
            along.map(|axis| axis * at),
            &mut names,
        );
    }
    compound(s, &names_of(&names), compound_name);
}

pub fn bug24157_10(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    make_box(s, "b0", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b", [0., 0., 0.], [12., 12., 12.]);
    explode(s, "b", Some(ShapeEnum::Face));
    stacked_faces(s, "b_1", [0., -1., -1.], "fx", [1., 0., 0.], "b1");
    crate::support::bugs_commands::view::donly(s, &["b0", "b1"]);
    stacked_faces(s, "b_3", [-1., 0., -1.], "fy", [0., 1., 0.], "b2");
    crate::support::bugs_commands::view::donly(s, &["b0", "b1", "b2"]);
    stacked_faces(s, "b_5", [-1., -1., 0.], "fz", [0., 0., 1.], "b3");
    make_box(s, "bx", [0.2, 0.2, 0.2], [1.6, 1.6, 1.6]);
    explode(s, "bx", Some(ShapeEnum::Face));
    let mut small = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                for m in 1..6 {
                    let delta = [(i * 2) as f64, (j * 2) as f64, (k * 2) as f64];
                    translated_copy(
                        s,
                        &format!("bx_{m}"),
                        format!("sx1_{i}_{j}_{k}_{m}"),
                        delta,
                        &mut small,
                    );
                }
            }
        }
    }
    compound(s, &names_of(&small), "b4");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b0"]);
    for name in ["b1", "b2", "b3", "b4"] {
        bop.add_compound(s, name);
    }
    bop.bfillds();
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 5.);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::part_count(s, "b", ShapeEnum::Face, 6);
    is_kind(s, "sx1_0_0_0_1", ShapeEnum::Face);
    crate::support::bugs_commands::beyond::argument_count(&bop, 638);
}

/// The star of `bug24157_4`: alternate vertices on two circles, closed into a planar face.
fn star_face(s: &mut dyn Session) {
    let count = 5;
    let (outer, inner, z) = (10., 4., 0.);
    let pi = std::f64::consts::PI;
    for i in 0..count {
        let angle = i as f64 * 2. * pi / count as f64;
        vertex(
            s,
            &format!("v1_{i}"),
            [
                outer * martensite_cmath::cos(angle),
                outer * martensite_cmath::sin(angle),
                z,
            ],
        );
        let turned = angle + pi / count as f64;
        vertex(
            s,
            &format!("v2_{i}"),
            [
                inner * martensite_cmath::cos(turned),
                inner * martensite_cmath::sin(turned),
                z,
            ],
        );
    }
    copy(s, "v1_0", &format!("v1_{count}"));
    let mut edges = Vec::new();
    for i in 0..count {
        let (first, second) = (format!("e1_{i}"), format!("e2_{i}"));
        edge(s, &first, &format!("v1_{i}"), &format!("v2_{i}"));
        edge(s, &second, &format!("v2_{i}"), &format!("v1_{}", i + 1));
        edges.push(first);
        edges.push(second);
    }
    crate::support::construction::wire(s, "w", &names_of(&edges));
    crate::support::construction::mkplane(s, "bs", "w");
    // Beyond the script: checks an earlier test of this case made.
    same_shape(s, "v1_5", "v1_0");
}

pub fn bug24157_4(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    star_face(s);
    make_box(s, "b", [0., 0., 0.], [700., 820., 1.]);
    explode(s, "b", Some(ShapeEnum::Face));
    copy(s, "b_5", "b1");
    let mut names = Vec::new();
    for i in 1..41 {
        for j in 1..41 {
            translated_copy(
                s,
                "bs",
                format!("bs_{i}_{{{j}}}"),
                [i as f64 * 17., j as f64 * 20., 0.],
                &mut names,
            );
        }
    }
    compound(s, &names_of(&names), "b2");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_compound(s, "b2");
    bop.bfillds();
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 43.);
    // Beyond the script: checks an earlier test of this case made.
    crate::support::bugs_commands::beyond::part_count(s, "b", ShapeEnum::Face, 6);
    is_kind(s, "bs_1_{1}", ShapeEnum::Face);
    crate::support::bugs_commands::beyond::argument_count(&bop, 1601);
}

/// The grid of long edges of `bug24157_5`, in both directions.
fn long_edges(s: &mut dyn Session, cx: &mut Vec<String>) {
    let (length, margin, step) = (300., 10., 10.);
    vertex(s, "v1", [step, -margin, 0.]);
    vertex(s, "v2", [step, length + margin, 0.]);
    edge(s, "ex", "v1", "v2");
    vertex(s, "v1", [-margin, step, 0.]);
    vertex(s, "v2", [length + margin, step, 0.]);
    edge(s, "ey", "v1", "v2");
    for i in 0..29 {
        translated_copy(s, "ex", format!("ex{i}"), [i as f64 * step, 0., 0.], cx);
    }
    for i in 0..29 {
        translated_copy(s, "ey", format!("ey{i}"), [0., i as f64 * step, 0.], cx);
    }
}

pub fn bug24157_5(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    make_box(s, "b", [0., 0., 0.], [300., 300., 20.]);
    explode(s, "b", Some(ShapeEnum::Face));
    copy(s, "b_5", "b1");
    let mut cx = Vec::new();
    long_edges(s, &mut cx);
    crate::support::commands::pcylinder(s, "x", 1., 2.);
    explode(s, "x", Some(ShapeEnum::Edge));
    tcopy(s, "x_3", "ex");
    for i in 0..30 {
        for j in 0..30 {
            for k in 0..3 {
                for m in 0..3 {
                    let name = format!("ex_{i}_{j}_{k}_{m}");
                    tcopy(s, "ex", &name);
                    ttranslate(s, &name, [i as f64 * 10., j as f64 * 10., 0.]);
                    ttranslate(s, &name, [(2 + k * 3) as f64, (2 + m * 3) as f64, 0.]);
                    cx.push(name);
                }
            }
        }
    }
    compound(s, &names_of(&cx), "b2");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_compound(s, "b2");
    bop.bfillds();
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 20.);
}

pub fn bug24157_9(s: &mut dyn Session) {
    println!("============");
    println!("OCC24157");
    println!("============");
    println!();
    restore(s, "bug24157_fstar", "b1");
    let mut names = Vec::new();
    for i in 0..24 {
        let name = format!("b1_{i}");
        tcopy(s, "b1", &name);
        trotate(s, &name, [0., 0., 0.], [0., 0., 1.], i as f64 * 15.);
        names.push(name);
    }
    compound(s, &names_of(&names), "q");
    let mut bop = ScriptBop::default();
    bop.add_compound(s, "q");
    bop.bfillds();
    let seconds = bop.bbuild_timed(s, "result");
    low_performance_check(s, seconds, 100.);
}
