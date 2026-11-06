//! `bugs_modalg_7` bodies written by hand: sections, repairs, options and progress.
#![allow(unused_imports)]
use crate::support::bugs_commands::progress::tracked;
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::{
        cells::ScriptCells,
        geometry::{
            circle_framed, ellipse_framed, frame_along, frame_at, line, mkedge_range, pcylinder_on,
            plane_at, psphere_on, trim_curve,
        },
        inspect::{
            bopapicheck, bopargcheck, bopcheck_level, bopcheck_text, checkmaxtol_value,
            checksection_count, compare, whatis,
        },
        options_bop::{ScriptBop, SectionFlags, bsection_with, generated, modified},
        pcurves::{checklength, pcurve_of_edge, pcurves_of_face, translate_pcurve},
        repair::{fixshape, settolerance},
        view::{donly, fit, smallview},
    },
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview, screenshot},
    commands::{bound_shape, compound, explode, make_box, psphere, ttranslate},
    construction::{mkplane, revol, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::{bcut, bsection},
    properties::{Prop, props},
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{circle_along, mkedge, mkface, mkface_bounded, plane, polyline},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::{
    bop_algo::{
        BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, BOPAlgoSplitter,
        BuilderHooks, Operation, perform_with_filler_progress,
    },
    brep_algo_api::{
        BRepAlgoAPIBuilderAlgo, BRepAlgoAPICheck, BRepAlgoAPICommon, BRepAlgoAPICut,
        BRepAlgoAPIFuse, BRepAlgoAPISection, BRepAlgoAPISplitter,
    },
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;

fn view_of(s: &mut dyn Session, name: &str) {
    checkview(s, &CheckView::of(name));
}

pub fn bug29511(s: &mut dyn Session) {
    println!("============");
    println!("OCC29511");
    println!("============");
    println!();
    restore(s, "bug29511_face.brep", "face");
    restore(s, "bug29511_prism.brep", "prism");
    let flags = SectionFlags {
        pcurve_on1: false,
        pcurve_on2: false,
        approximation: true,
    };
    bsection_with(s, "result", ("face", "prism"), flags);
    smallview(s);
    donly(s, &["result"]);
    fit(s);
    checkprops(s, "result", Prop::Length, Expected::Mass(764.018));
    screenshot(s, "photo");
}

pub fn bug25104(s: &mut dyn Session) {
    println!("========");
    println!("OCC25104");
    println!("========");
    println!();
    restore(s, "bug25104_whale2-draw-path.brep", "path");
    crate::support::commands::prism(s, "Extrude001", "path", [0., 0., 5.]);
    let answer = bopcheck_text(s, "Extrude001", &BopOptions::default());
    if answer.trim() != "This shape seems to be OK." {
        println!("ERROR: OCC25104 is reproduced. Prism has been created with errors.");
    }
}

pub fn bug27224(s: &mut dyn Session) {
    println!("========");
    println!("OCC27224");
    println!("========");
    println!();
    restore(s, "buc60290a.rle", "sol1");
    restore(s, "buc60290b.rle", "sol2");
    explode(s, "sol1", Some(ShapeEnum::Face));
    explode(s, "sol2", Some(ShapeEnum::Face));
    bsection(s, "result", "sol2_1", "sol1_1");
    if checksection_count(s, "result") != 0 {
        println!("Error : Incorrect result of boolean operation");
    } else {
        println!("OK : Correct result of boolean operation");
    }
    smallview(s);
    donly(s, &["result"]);
    fit(s);
    screenshot(s, "photo");
}

pub fn bug28157(s: &mut dyn Session) {
    println!("========");
    println!("OCC28157");
    println!("========");
    println!();
    let mut bop = ScriptBop::default();
    bop.options.run_parallel = true;
    for (file, name) in [("bug28157_init.brep", "n"), ("bug28157_solid.brep", "s")] {
        restore(s, file, name);
        whatis(s, name);
        tolerance_report(s, name);
        checkshape_named(s, name);
        bopcheck_text(s, name, &bop.options);
    }
    bop.bop(s, "s", "n");
    bop.bbop(s, "r", Operation::Common);
    whatis(s, "r");
    explode(s, "r", None);
    whatis(s, "r_1");
    tolerance_report(s, "r_1");
    settolerance(s, "r_1", 1.0e-7);
    fixshape(s, "rr", "r_1");
    tolerance_report(s, "rr");
    checkshape_named(s, "rr");
    if !bopcheck_text(s, "rr", &bop.options).contains("This shape seems to be OK") {
        println!("Error: there are vertex-vertex intersections");
    }
    view_of(s, "rr");
}

pub fn bug30354(s: &mut dyn Session) {
    println!("========");
    println!("0030354: BOP Cut doesn't modify the attached face");
    println!("========");
    println!();
    restore(s, "bug30354_Face_14.brep", "b1");
    restore(s, "bug30354_vol_neg.brep", "b2");
    explode(s, "b2", Some(ShapeEnum::Face));
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "r_0", Operation::Common);
    bop.bbop(s, "r_1", Operation::Cut);
    bop.bbop(s, "r_2", Operation::Section);
    for name in ["r_0", "r_1", "r_2"] {
        checkshape_named(s, name);
        if bopargcheck(s, (name, None), None, "").contains("Faulties") {
            println!("Error: bopargcheck has found some faulties in {name}");
        }
    }
    let one = [NbCount::Wires(1), NbCount::Faces(1)];
    checknbshapes(s, "r_0", Counting::Located, &one);
    checkprops(s, "r_0", Prop::Area, Expected::Mass(0.000284873));
    checknbshapes(s, "r_1", Counting::Located, &one);
    checkprops(s, "r_1", Prop::Area, Expected::Mass(0.000284873));
    checknbshapes(
        s,
        "r_2",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checkprops(s, "r_2", Prop::Length, Expected::Mass(0.00670288));
    checksection_expecting(s, "r_2", 2);
    crate::support::construction::copy(s, "r_1", "result");
    crate::support::bugs_commands::inspect::checkmaxtol_min(s, "result", 1.0e-4, "1.0e-4");
    view_of(s, "result");
}

pub fn bug30396_1(s: &mut dyn Session) {
    println!("========");
    println!("0030396: Infinite recursion during ShapeFix after BRepAlgoAPICut");
    println!("========");
    println!();
    let lists: [&[&str]; 2] = [
        &["a_2", "a_3", "a_4", "a_5", "a_6", "a_7"],
        &["a_6", "a_2", "a_7", "a_3", "a_4", "a_5"],
    ];
    let operations = [
        (
            "common",
            Operation::Common,
            [33, 33, 6, 6],
            3.24433,
            25.9962,
        ),
        ("fuse", Operation::Fuse, [23, 17, 1, 1], 11.1953, 58.8635),
        ("cut", Operation::Cut, [29, 23, 1, 1], 7.95093, 53.1541),
        ("cut21", Operation::Cut21, [0, 0, 0, 0], 0., 0.),
    ];
    let mut bop = ScriptBop::default();
    bop.options.fuzzy = 1.0e-2;
    bop.options.non_destructive = true;
    for (index, tools) in lists.iter().enumerate() {
        restore(s, "bug30396.brep", "a");
        explode(s, "a", None);
        bop.clear_objects();
        bop.clear_tools();
        bop.add_objects(s, &["a_1"]);
        bop.add_tools(s, tools);
        bop.bfillds();
        for (word, kind, [wires, faces, shells, solids], volume, area) in operations {
            let name = format!("r_{}_{word}", index + 1);
            bop.bbop(s, &name, kind);
            let counts = [
                NbCount::Wires(wires),
                NbCount::Faces(faces),
                NbCount::Shells(shells),
                NbCount::Solids(solids),
            ];
            checknbshapes(s, &name, Counting::Located, &counts);
            let (volume, area) = if kind == Operation::Cut21 {
                (Expected::Empty, Expected::Empty)
            } else {
                (Expected::Mass(volume), Expected::Mass(area))
            };
            checkprops(s, &name, Prop::Area, area);
            checkprops(s, &name, Prop::Volume, volume);
            checkshape_named(s, &name);
            if !bopcheck_text(s, &name, &bop.options).contains("OK") {
                println!(
                    "Error: result {} of {word} is self-interfered shape",
                    index + 1
                );
            }
        }
    }
    bop.reset_options();
    view_of(s, "r_1_cut");
}

pub fn bug27936(s: &mut dyn Session) {
    println!("========");
    println!("OCC27936");
    println!("========");
    println!();
    restore(s, "bug27936_rsi.brep", "s");
    explode(s, "s", None);
    let mut bop = ScriptBop::default();
    bop.options.run_parallel = true;
    bop.bop(s, "s_1", "s_2");
    bop.bbop(s, "result", Operation::Common);
    if explode(s, "result", None).is_empty() {
        println!("ERROR: OCC27936 is reproduced. Result of bopcommon is empty.");
    }
    checkshape_named(s, "result");
    let counts = [
        NbCount::Vertices(4),
        NbCount::Edges(4),
        NbCount::Wires(1),
        NbCount::Faces(1),
        NbCount::Shells(0),
        NbCount::Solids(0),
        NbCount::CompSolids(0),
        NbCount::Compounds(1),
        NbCount::Shapes(11),
    ];
    checknbshapes(s, "result", Counting::Located, &counts);
}

pub fn bug24365(s: &mut dyn Session) {
    println!("TODO OCC24365 ALL: ERROR: OCC24365 is reproduced.");
    println!("========");
    println!("OCC24365");
    println!("========");
    println!();
    restore(s, "bug24365_faceA.brep", "a");
    restore(s, "bug24365_faceB.brep", "b");
    bsection(s, "result", "a", "b");
    if props(Prop::Length, &bound_shape(s, "result"), None) == 0. {
        println!("ERROR: OCC24365 is reproduced. Section of two shapes is not built.");
    }
    smallview(s);
    fit(s);
    screenshot(s, "photo");
}

/// One prepared result over the shared intersections; `kind` chooses the receiver.
fn prepared_result(filler: &mut BOPAlgoPaveFiller, first: &Shape, second: &Shape, kind: Operation) {
    tracked(|range| {
        let mut operation = BOPAlgoBOP::new();
        operation.add_argument(first);
        operation.add_tool(second);
        operation.set_operation(kind);
        perform_with_filler_progress(&mut operation, filler, range).unwrap()
    });
}

fn prepared_section(filler: &mut BOPAlgoPaveFiller, first: &Shape, second: &Shape) {
    tracked(|range| {
        let mut section = BOPAlgoSection::new();
        section.add_argument(first);
        section.add_argument(second);
        perform_with_filler_progress(&mut section, filler, range).unwrap()
    });
}

fn one_step_result(first: &Shape, second: &Shape, kind: Operation) {
    tracked(|range| {
        let mut operation = BOPAlgoBOP::new();
        operation.add_argument(first);
        operation.add_tool(second);
        operation.set_operation(kind);
        BuilderHooks::perform_with_progress(&mut operation, range).unwrap()
    });
}

fn api_result(first: &Shape, second: &Shape, kind: Operation) {
    tracked(|range| match kind {
        Operation::Section => {
            let mut section = BRepAlgoAPISection::from_shapes(first, second, None, false).unwrap();
            section.build_with_progress(range)
        }
        Operation::Common => BRepAlgoAPICommon::from_shapes(first, second)
            .unwrap()
            .build_with_progress(range),
        Operation::Fuse => BRepAlgoAPIFuse::from_shapes(first, second)
            .unwrap()
            .build_with_progress(range),
        Operation::Cut => BRepAlgoAPICut::from_shapes(first, second)
            .unwrap()
            .build_with_progress(range),
        _ => BRepAlgoAPICut::from_shapes(second, first)
            .unwrap()
            .build_with_progress(range),
    });
}

pub fn bug21264(s: &mut dyn Session) {
    println!("============================================");
    println!("0021264: Modeling Algorithms - Progress indicator for Boolean operations");
    println!("============================================");
    println!();
    println!(
        "Progress Indicator defaults: tcl mode is ON, console mode is OFF, graphical mode is OFF"
    );
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [5., 5., 5.], [10., 10., 10.]);
    let (first, second) = (bound_shape(s, "b1"), bound_shape(s, "b2"));
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    tracked(|range| filler.perform_with_progress(range));
    let booleans = [
        Operation::Fuse,
        Operation::Common,
        Operation::Cut,
        Operation::Cut21,
    ];
    for kind in booleans {
        prepared_result(&mut filler, &first, &second, kind);
    }
    prepared_section(&mut filler, &first, &second);
    for kind in booleans {
        one_step_result(&first, &second, kind);
    }
    api_result(&first, &second, Operation::Section);
    tracked(|range| filler.perform_with_progress(range));
    for kind in [
        Operation::Common,
        Operation::Fuse,
        Operation::Cut,
        Operation::Cut21,
    ] {
        prepared_result(&mut filler, &first, &second, kind);
    }
    prepared_section(&mut filler, &first, &second);
    tracked(|range| {
        let mut builder = BOPAlgoBuilder::new();
        builder.set_arguments(&arguments);
        perform_with_filler_progress(&mut builder, &mut filler, range).unwrap()
    });
    let tools: List<Shape> = [second.clone()].into_iter().collect();
    let objects: List<Shape> = [first.clone()].into_iter().collect();
    tracked(|range| {
        let mut splitter = BOPAlgoSplitter::new();
        splitter.set_arguments(&objects);
        splitter.set_tools(&tools);
        perform_with_filler_progress(&mut splitter, &mut filler, range).unwrap()
    });
    for kind in [
        Operation::Common,
        Operation::Fuse,
        Operation::Cut,
        Operation::Cut21,
        Operation::Section,
    ] {
        api_result(&first, &second, kind);
    }
    tracked(|range| {
        let mut builder = BRepAlgoAPIBuilderAlgo::new();
        builder.set_arguments(&arguments);
        builder.build_with_progress(range)
    });
    let mut splitter = BRepAlgoAPISplitter::new();
    splitter.set_arguments(&objects);
    splitter.set_tools(&tools);
    tracked(|range| splitter.build_with_progress(range));
    let mut checker = crate::case_bopcheck::configured_checker(splitter.shape());
    tracked(|range| checker.perform_with_progress(range));
    println!("{}", crate::case_bopcheck::complete_response(&checker));
}
