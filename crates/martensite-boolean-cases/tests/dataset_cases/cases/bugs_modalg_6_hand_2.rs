//! `bugs_modalg_6`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::beyond::child_count,
    bugs_commands::geometry::{line, mkedge_range, plane_framed},
    bugs_commands::inspect::checknbshapes_message,
    bugs_commands::options_bop::{ScriptBop, SectionFlags, bsection_with},
    bugs_commands::repair::settolerance,
    bugs_commands::view::{don, donly, fit, smallview},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    commands::{explode, pcone, pcylinder, trotate, ttranslate},
    construction::copy,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcut, bsection},
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::{mkface, mkface_bounded, plane},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug14531(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC14531");
    println!("============");
    println!();
    pcylinder(s, "c", 10., 20.);
    pcone(s, "x", 20., 0., 20.);
    trotate(s, "x", [0., 0., 20.], [0., 1., 0.], 45.);
    ttranslate(s, "x", [5., 0., 0.]);
    ttranslate(s, "x", [0., 0., 1.]);
    ttranslate(s, "c", [-5., 0., 0.]);
    explode(s, "c", Some(ShapeEnum::Face));
    explode(s, "x", Some(ShapeEnum::Face));
    copy(s, "c_1", "a1");
    copy(s, "x_1", "a2");
    fit(s);
    two_step.bop(s, "a1", "a2");
    two_step.result(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(3033.79));
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[7, 12, 5, 4, 0, 0, 0, 1, 29],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug22793(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC22793");
    println!("============");
    println!();
    restore(s, "bug22793_Positive.brep", "p");
    restore(s, "bug22793_Void.brep", "v");
    checkshape_named(s, "p");
    tolerance_report(s, "p");
    checkshape_named(s, "v");
    tolerance_report(s, "v");
    two_step.bop(s, "p", "v");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(1.89516e+07));
    checkshape_named(s, "result");
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[16, 24, 10, 10, 1, 1, 0, 1, 63],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug22794(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC22793");
    println!("============");
    println!();
    restore(s, "bug22793_Positive.brep", "p");
    restore(s, "bug22794b.brep", "v");
    checkshape_named(s, "p");
    tolerance_report(s, "p");
    checkshape_named(s, "v");
    tolerance_report(s, "v");
    two_step.bop(s, "p", "v");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(1.89654e+007));
    checkshape_named(s, "result");
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[20, 30, 14, 14, 2, 2, 0, 1, 83],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug23585(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC23585");
    println!("============");
    println!();
    restore(s, "bug23585a.brep", "p");
    restore(s, "bug23585b.brep", "v");
    checkshape_named(s, "p");
    tolerance_report(s, "p");
    checkshape_named(s, "v");
    tolerance_report(s, "v");
    two_step.bop(s, "p", "v");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(14796.8));
    checkshape_named(s, "result");
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[18, 30, 14, 14, 1, 1, 0, 1, 79],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24094(s: &mut dyn Session) {
    println!("========");
    println!("OCC24094");
    println!("========");
    println!();
    restore(s, "bug24094_face.brep", "a");
    let var_p = plane_framed([183.6, 0., 0.], [1., 0., 0.], [0., 1., 0.]);
    mkface(s, "f", &var_p);
    bsection(s, "r1", "a", "f");
    bsection_with(
        s,
        "r2",
        ("a", "f"),
        SectionFlags {
            pcurve_on1: true,
            pcurve_on2: true,
            approximation: false,
        },
    );
    bsection_with(
        s,
        "r3",
        ("a", "f"),
        SectionFlags {
            pcurve_on1: true,
            pcurve_on2: false,
            approximation: true,
        },
    );
    bsection_with(
        s,
        "r4",
        ("a", "f"),
        SectionFlags {
            pcurve_on1: false,
            pcurve_on2: false,
            approximation: true,
        },
    );
    bsection_with(
        s,
        "r5",
        ("a", "f"),
        SectionFlags {
            pcurve_on1: false,
            pcurve_on2: false,
            approximation: false,
        },
    );
    checknbshapes(
        s,
        "r1",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checknbshapes(
        s,
        "r2",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checknbshapes(
        s,
        "r3",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checknbshapes(
        s,
        "r4",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checknbshapes(
        s,
        "r5",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn bug24417(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC24417");
    println!("============");
    println!();
    restore(s, "bug24417_circle.brep", "a");
    restore(s, "bug24417_face.brep", "b");
    checkshape_named(s, "a");
    checkshape_named(s, "b");
    two_step.bop(s, "a", "b");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(19.8372));
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[2, 1, 0, 0, 0, 0, 0, 1, 4],
        "Result of BRepAlgoAPISection",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25152(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25152");
    println!("============");
    println!();
    println!("New current number of isos : 0");
    restore(s, "bug25152.brep", "a");
    explode(s, "a", None);
    checkshape_named(s, "a_1");
    tolerance_report(s, "a_1");
    checkshape_named(s, "a_2");
    tolerance_report(s, "a_2");
    two_step.bop(s, "a_1", "a_2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(64.4145));
    checkshape_named(s, "result");
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[18, 26, 12, 11, 1, 1, 0, 1, 70],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "a", 2);
}

pub fn bug28486_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28486");
    println!("========");
    println!();
    let var_p = plane([0., 0., 0.], [0., 0., 1.]);
    mkface_bounded(s, "f1", &var_p, (-10., 10., -10., 10.));
    copy(s, "f1", "f2");
    trotate(s, "f2", [0., 0., 0.], [1., 0., 0.], 20.);
    let var_l = line([0., 0., 0.], [0., 1., 0.]);
    mkedge_range(s, "e", &var_l, -8., 8.);
    trotate(s, "e", [0., 0., 0.], [1., 0., 0.], 10.);
    settolerance(s, "e", 1.5);
    donly(s, &["f1", "f2", "e"]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["f1", "f2"]);
    bop.add_tools(s, &["e"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(4), NbCount::Wires(8)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(800.));
    checkview(s, &CheckView::of("result"));
}

pub fn bug28626_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC28626");
    println!("========");
    println!();
    restore(s, "bug28626_case1_shapes.brep", "b");
    explode(s, "b", None);
    bcut(s, "result", "b_1", "b_2");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Solids(1),
            NbCount::Shells(1),
            NbCount::Faces(10),
            NbCount::Wires(10),
            NbCount::Edges(20),
            NbCount::Vertices(12),
        ],
    );
    checkmaxtol_ref(s, "result", "8.00001e-007");
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.35999e+006));
    smallview(s);
    don(s, &["b", "result"]);
    fit(s);
    screenshot(s, "photo");
}

pub fn bug28690_3(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28690");
    println!("========");
    println!();
    restore(s, "bug28690_gdml_G1_tore_sphere.brep", "c");
    explode(s, "c", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c_1"]);
    bop.add_tools(s, &["c_2"]);
    bop.bfillds();
    bop.bbuild(s, "result1");
    checkshape_named(s, "result1");
    checknbshapes(
        s,
        "result1",
        Counting::Located,
        &[
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(5),
            NbCount::Solids(3),
        ],
    );
    checkprops(s, "result1", Prop::Area, Expected::Mass(4420.05));
    checkprops(s, "result1", Prop::Volume, Expected::Mass(15877.8));
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(1),
            NbCount::Faces(1),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(112.393));
    checkprops(s, "result", Prop::Volume, Expected::Mass(89.7477));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "c", 2);
}

pub fn bug28745_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case1_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.2767e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(7.83594e+008));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28745_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case2_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.59907e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.75861e+008));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28745_3(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case3_common.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Common);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(7.99947e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2.05023e+009));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(2), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}
