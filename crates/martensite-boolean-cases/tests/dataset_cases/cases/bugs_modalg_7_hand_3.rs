//! `bugs_modalg_7`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::inspect::{bopargcheck, bopcheck_ok, checknbshapes_message},
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::fuseedge,
    bugs_commands::view::{donly, fit, smallview, xwd},
    checkprops::{CheckTolerances, Expected, checkprops, checkprops_within},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    commands::{explode, make_box, pcylinder, psphere, trotate, ttranslate},
    construction::nurbsconvert,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcut, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Glue;
use martensite_tkbo::bop_algo::Operation;

pub fn bug23386(s: &mut dyn Session) {
    println!("========");
    println!("OCC23386");
    println!("========");
    println!();
    restore(s, "bug23386_a.brep", "a");
    restore(s, "bug23386_b.brep", "b");
    bfuse(s, "result", "a", "b");
    checknbshapes_message(
        s,
        "a",
        Counting::Typed,
        &[383, 686, 297, 297, 2, 1, 0, 0, 1666],
        "First initial shape",
    );
    checknbshapes_message(
        s,
        "b",
        Counting::Typed,
        &[6, 9, 5, 5, 1, 1, 0, 0, 27],
        "Second initial shape",
    );
    checknbshapes_message(
        s,
        "result",
        Counting::Typed,
        &[388, 695, 302, 302, 3, 2, 0, 1, 1693],
        "result of fuse operation",
    );
}

pub fn bug24692(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC24692 ALL: Faulty shapes in variables faulty_1 to faulty_");
    println!("========");
    println!("OCC24692");
    println!("========");
    println!();
    pcylinder(s, "cyl", 10., 40.);
    make_box(s, "abox", [0., 0., 0.], [10., 40., 25.]);
    ttranslate(s, "abox", [0., -20., 5.]);
    two_step.bop(s, "cyl", "abox");
    two_step.result(s, "fuse1", Operation::Fuse);
    psphere(s, "asphere", 7.);
    ttranslate(s, "asphere", [0., 0., 3.]);
    make_box(s, "abox2", [0., 0., 0.], [5., 30., 25.]);
    ttranslate(s, "abox2", [-2., -15., -11.]);
    two_step.bop(s, "abox2", "asphere");
    two_step.result(s, "fuse2", Operation::Fuse);
    two_step.bop(s, "fuse1", "fuse2");
    two_step.result(s, "result", Operation::Cut);
    fuseedge(s, "result");
    checkshape_named(s, "result_1");
}

pub fn bug25023(s: &mut dyn Session) {
    println!("========");
    println!("OCC25023");
    println!("========");
    println!();
    restore(s, "bug25023_a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "bug25023_b.brep", "b");
    checkshape_named(s, "b");
    bopargcheck(s, ("b", Some("a")), Some(Operation::Cut), "");
    bcut(s, "result", "b", "a");
    checkshape_named(s, "result");
}

pub fn bug25542(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25542: Boolean operation fai-lure for Cylinder and Cone in critical location.");
    println!("============");
    println!();
    restore(s, "bug25542_cylinder.brep", "b1");
    restore(s, "bug25542_cone.brep", "b2");
    smallview(s);
    two_step.bop(s, "b1", "b2");
    println!();
    println!("FUSE");
    two_step.result(s, "rfu", Operation::Fuse);
    donly(s, &["rfu"]);
    fit(s);
    xwd(s, "rfu");
    println!();
    println!("COMMON");
    two_step.result(s, "rco", Operation::Common);
    donly(s, &["rco"]);
    fit(s);
    xwd(s, "rco");
    println!();
    println!("CUT");
    two_step.result(s, "rcu", Operation::Cut);
    donly(s, &["rcu"]);
    fit(s);
    xwd(s, "rcu");
    println!();
    println!("TUC");
    two_step.result(s, "rtu", Operation::Cut21);
    donly(s, &["rtu"]);
    fit(s);
    xwd(s, "rtu");
    checknbshapes_message(
        s,
        "rfu",
        Counting::Typed,
        &[6, 14, 8, 8, 1, 1, 0, 1, 39],
        "FUSE",
    );
    checknbshapes_message(
        s,
        "rco",
        Counting::Typed,
        &[3, 5, 3, 3, 1, 1, 0, 1, 17],
        "COMMON",
    );
    checknbshapes_message(
        s,
        "rcu",
        Counting::Typed,
        &[5, 9, 5, 5, 1, 1, 0, 1, 27],
        "CUT",
    );
    checknbshapes_message(
        s,
        "rtu",
        Counting::Typed,
        &[4, 8, 4, 4, 1, 1, 0, 1, 23],
        "TUC",
    );
    checkshape_named(s, "rfu");
    checkshape_named(s, "rco");
    checkshape_named(s, "rcu");
    checkshape_named(s, "rtu");
    checkprops(s, "rfu", Prop::Area, Expected::Mass(59099.9));
    checkprops(s, "rco", Prop::Area, Expected::Mass(6951.3));
    checkprops(s, "rcu", Prop::Area, Expected::Mass(57145.3));
    checkprops(s, "rtu", Prop::Area, Expected::Mass(7759.96));
}

pub fn bug25966(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("============");
    println!("OCC25966");
    println!("============");
    println!();
    restore(s, "bug25966_Revolution.brep", "Revolution");
    trotate(s, "Revolution", [0., 0., 0.], [1., 0., 0.], 90.);
    restore(s, "bug25966_Pad009.brep", "Pad009");
    ttranslate(s, "Pad009", [0., 0., -2.]);
    restore(s, "bug25966_Pad.brep", "Pad");
    bfuse(s, "Fusion007", "Revolution", "Pad009");
    bfuse(s, "Fusion008", "Pad", "Fusion007");
    bfuse(s, "Fusion009", "Pad", "Pad009");
    bfuse(s, "Fusion010", "Revolution", "Fusion009");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["Pad"]);
    bop.add_tools(s, &["Pad009", "Revolution"]);
    bop.bfillds();
    bop.bbop(s, "Fusion011", Operation::Fuse);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["Revolution"]);
    bop.add_tools(s, &["Pad009", "Pad"]);
    bop.bfillds();
    bop.bbop(s, "Fusion006", Operation::Fuse);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["Pad009"]);
    bop.add_tools(s, &["Revolution", "Pad"]);
    bop.bfillds();
    bop.bbop(s, "Fusion012", Operation::Fuse);
    checkshape_named(s, "Fusion006");
    checkshape_named(s, "Fusion008");
    checkshape_named(s, "Fusion010");
    checkshape_named(s, "Fusion011");
    checkshape_named(s, "Fusion012");
    checkprops(s, "Fusion012", Prop::Area, Expected::Mass(18712.8));
    checknbshapes(
        s,
        "Fusion012",
        Counting::Located,
        &[
            NbCount::Wires(26),
            NbCount::Faces(23),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
}

pub fn bug26563(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC26563");
    println!("========");
    println!();
    restore(s, "bug26563_BodyShape.brep", "bodyshape");
    restore(s, "bug26563_CutShape.brep", "cutshape");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["bodyshape"]);
    bop.add_tools(s, &["cutshape"]);
    bop.options.fuzzy = 1e-5;
    bop.bapibop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.52043e+7));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(234),
            NbCount::Edges(354),
            NbCount::Wires(122),
            NbCount::Faces(121),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(834),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug26933(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC26933");
    println!("========");
    println!();
    restore(s, "bug26933.brep", "a");
    explode(s, "a", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["a_1"]);
    bop.add_tools(s, &["a_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Section);
    smallview(s);
    donly(s, &["a_1", "result"]);
    fit(s);
    screenshot(s, "photo");
    checkprops_within(
        s,
        "result",
        Prop::Length,
        Expected::Mass(945.22),
        CheckTolerances {
            deviation: 0.1,
            ..CheckTolerances::default()
        },
    );
}

pub fn bug27928(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("==============================================================");
    println!("OCC27928: BOP common produces empty compound");
    println!("==============================================================");
    restore(s, "bug27928_b1.brep", "b1");
    restore(s, "bug27928_b2.brep", "b2");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "r0", Operation::Common);
    bop.bbop(s, "r1", Operation::Fuse);
    bop.bbop(s, "r2", Operation::Cut);
    bop.bbop(s, "r3", Operation::Cut21);
    bop.bbop(s, "r4", Operation::Section);
    checkshape_named(s, "r0");
    bopcheck_ok(
        s,
        "r0",
        &bop.options,
        "Error: the result of BOP is self-interfering shape",
    );
    checkshape_named(s, "r1");
    bopcheck_ok(
        s,
        "r1",
        &bop.options,
        "Error: the result of BOP is self-interfering shape",
    );
    checkshape_named(s, "r2");
    bopcheck_ok(
        s,
        "r2",
        &bop.options,
        "Error: the result of BOP is self-interfering shape",
    );
    checkshape_named(s, "r3");
    bopcheck_ok(
        s,
        "r3",
        &bop.options,
        "Error: the result of BOP is self-interfering shape",
    );
    checkshape_named(s, "r4");
    bopcheck_ok(
        s,
        "r4",
        &bop.options,
        "Error: the result of BOP is self-interfering shape",
    );
    checknbshapes(
        s,
        "r0",
        Counting::Located,
        &[
            NbCount::Wires(8),
            NbCount::Faces(8),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r0", Prop::Area, Expected::Mass(21820.6));
    checkprops(s, "r0", Prop::Volume, Expected::Mass(221499.));
    checknbshapes(
        s,
        "r1",
        Counting::Located,
        &[
            NbCount::Wires(14),
            NbCount::Faces(14),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r1", Prop::Area, Expected::Mass(22455.1));
    checkprops(s, "r1", Prop::Volume, Expected::Mass(224813.));
    checknbshapes(
        s,
        "r2",
        Counting::Located,
        &[
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r2", Prop::Area, Expected::Mass(2075.44));
    checkprops(s, "r2", Prop::Volume, Expected::Mass(1489.33));
    checknbshapes(
        s,
        "r3",
        Counting::Located,
        &[
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r3", Prop::Area, Expected::Mass(2521.83));
    checkprops(s, "r3", Prop::Volume, Expected::Mass(1824.69));
    checknbshapes(
        s,
        "r4",
        Counting::Typed,
        &[NbCount::Vertices(13), NbCount::Edges(19)],
    );
    checkprops(s, "r4", Prop::Length, Expected::Mass(825.645));
    checkview(s, &CheckView::of("r0"));
}

pub fn bug28485_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28485");
    println!("========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10., 2., 2.], [6., 6., 6.]);
    nurbsconvert(s, "b1", "b1");
    nurbsconvert(s, "b2", "b2");
    bop.options.non_destructive = true;
    bop.options.run_parallel = true;
    bop.options.fuzzy = 0.;
    bop.glue = Glue::Shift;
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(24),
            NbCount::Wires(12),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(744.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1216.));
}
