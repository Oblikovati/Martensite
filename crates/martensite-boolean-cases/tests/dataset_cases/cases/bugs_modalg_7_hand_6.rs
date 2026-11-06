//! `bugs_modalg_7`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::geometry::{frame_along, ptorus_on},
    bugs_commands::inspect::bopcheck_ok,
    bugs_commands::options_bop::ScriptBop,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder},
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug31858_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=======================================================");
    println!(
        "0031858: Modeling Algorithms - boolean operation crash between the given cylinder and torus."
    );
    println!("=======================================================");
    println!();
    pcylinder(s, "c", 500., 200.);
    ptorus_on(
        s,
        "t",
        frame_along(
            [-2.31895142368858e-15, -4.49296838339662e-15, 200.],
            [0., 0., 1.],
        ),
        449.367136080235,
        50.6328639197654,
    );
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c"]);
    bop.add_tools(s, &["t"]);
    bop.bfillds();
    bop.bbop(s, "r0", Operation::Common);
    bop.bbop(s, "r1", Operation::Fuse);
    bop.bbop(s, "r2", Operation::Cut);
    bop.bbop(s, "r3", Operation::Cut21);
    bop.bbop(s, "r4", Operation::Section);
    bop.bbuild(s, "rgf");
    checkshape_named(s, "r0");
    bopcheck_ok(
        s,
        "r0",
        &bop.options,
        "Error: the r0 is a self-interfering shape",
    );
    checkshape_named(s, "r1");
    bopcheck_ok(
        s,
        "r1",
        &bop.options,
        "Error: the r1 is a self-interfering shape",
    );
    checkshape_named(s, "r2");
    bopcheck_ok(
        s,
        "r2",
        &bop.options,
        "Error: the r2 is a self-interfering shape",
    );
    checkshape_named(s, "r3");
    bopcheck_ok(
        s,
        "r3",
        &bop.options,
        "Error: the r3 is a self-interfering shape",
    );
    checkshape_named(s, "r4");
    bopcheck_ok(
        s,
        "r4",
        &bop.options,
        "Error: the r4 is a self-interfering shape",
    );
    checkshape_named(s, "rgf");
    bopcheck_ok(
        s,
        "rgf",
        &bop.options,
        "Error: the rgf is a self-interfering shape",
    );
    checknbshapes(
        s,
        "r0",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r0", Prop::Area, Expected::Mass(735041.));
    checkprops(s, "r0", Prop::Volume, Expected::Mass(1.13701e+07));
    checknbshapes(
        s,
        "r1",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r1", Prop::Area, Expected::Mass(2.36232e+06));
    checkprops(s, "r1", Prop::Volume, Expected::Mass(1.6845e+08));
    checknbshapes(
        s,
        "r2",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r2", Prop::Area, Expected::Mass(2.36232e+06));
    checkprops(s, "r2", Prop::Volume, Expected::Mass(1.45709e+08));
    checknbshapes(
        s,
        "r3",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r3", Prop::Area, Expected::Mass(735041.));
    checkprops(s, "r3", Prop::Volume, Expected::Mass(1.13701e+07));
    checkprops(s, "r4", Prop::Length, Expected::Mass(5646.91));
    checksection_expecting(s, "r4", 0);
    checknbshapes(
        s,
        "rgf",
        Counting::Typed,
        &[
            NbCount::Wires(7),
            NbCount::Faces(6),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    checkprops(s, "rgf", Prop::Area, Expected::Mass(3.8324e+06));
    checkprops(s, "rgf", Prop::Volume, Expected::Mass(1.6845e+08));
    checkview(s, &CheckView::of("r0"));
}

pub fn bug31858_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=======================================================");
    println!(
        "0031858: Modeling Algorithms - boolean operation crash between the given cylinder and torus."
    );
    println!("=======================================================");
    println!();
    pcylinder(s, "c", 500., 200.);
    ptorus_on(
        s,
        "t",
        frame_along(
            [-2.31895142368858e-15, -4.49296838339662e-15, 200.],
            [0., 0., 1.],
        ),
        449.367136080235,
        50.6328639197654,
    );
    bop.options.fuzzy = 1.0e-6;
    bop.options.non_destructive = true;
    bop.options.run_parallel = true;
    bop.options.fill_history = false;
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c"]);
    bop.add_tools(s, &["t"]);
    bop.bfillds();
    bop.bbop(s, "r0", Operation::Common);
    bop.bbop(s, "r1", Operation::Fuse);
    bop.bbop(s, "r2", Operation::Cut);
    bop.bbop(s, "r3", Operation::Cut21);
    bop.bbop(s, "r4", Operation::Section);
    bop.bbuild(s, "rgf");
    bop.reset_options();
    checkshape_named(s, "r0");
    bopcheck_ok(
        s,
        "r0",
        &bop.options,
        "Error: the r0 is a self-interfering shape",
    );
    checkshape_named(s, "r1");
    bopcheck_ok(
        s,
        "r1",
        &bop.options,
        "Error: the r1 is a self-interfering shape",
    );
    checkshape_named(s, "r2");
    bopcheck_ok(
        s,
        "r2",
        &bop.options,
        "Error: the r2 is a self-interfering shape",
    );
    checkshape_named(s, "r3");
    bopcheck_ok(
        s,
        "r3",
        &bop.options,
        "Error: the r3 is a self-interfering shape",
    );
    checkshape_named(s, "r4");
    bopcheck_ok(
        s,
        "r4",
        &bop.options,
        "Error: the r4 is a self-interfering shape",
    );
    checkshape_named(s, "rgf");
    bopcheck_ok(
        s,
        "rgf",
        &bop.options,
        "Error: the rgf is a self-interfering shape",
    );
    checknbshapes(
        s,
        "r0",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r0", Prop::Area, Expected::Mass(735041.));
    checkprops(s, "r0", Prop::Volume, Expected::Mass(1.13701e+07));
    checknbshapes(
        s,
        "r1",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r1", Prop::Area, Expected::Mass(2.36232e+06));
    checkprops(s, "r1", Prop::Volume, Expected::Mass(1.6845e+08));
    checknbshapes(
        s,
        "r2",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r2", Prop::Area, Expected::Mass(2.36232e+06));
    checkprops(s, "r2", Prop::Volume, Expected::Mass(1.45709e+08));
    checknbshapes(
        s,
        "r3",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r3", Prop::Area, Expected::Mass(735041.));
    checkprops(s, "r3", Prop::Volume, Expected::Mass(1.13701e+07));
    checkprops(s, "r4", Prop::Length, Expected::Mass(5646.91));
    checksection_expecting(s, "r4", 0);
    checknbshapes(
        s,
        "rgf",
        Counting::Typed,
        &[
            NbCount::Wires(7),
            NbCount::Faces(6),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    checkprops(s, "rgf", Prop::Area, Expected::Mass(3.8324e+06));
    checkprops(s, "rgf", Prop::Volume, Expected::Mass(1.6845e+08));
    checkview(s, &CheckView::of("r0"));
}

pub fn bug31974(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=======================================================");
    println!("0031974: Modeling Algorithms - Strange behavior of boolean operations on this BREP");
    println!("=======================================================");
    println!();
    restore(s, "bug31974.brep", "s");
    explode(s, "s", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_1"]);
    bop.add_tools(s, &["s_2"]);
    bop.bfillds();
    bop.bbop(s, "r0", Operation::Common);
    bop.bbop(s, "r1", Operation::Fuse);
    bop.bbop(s, "r2", Operation::Cut);
    bop.bbop(s, "r3", Operation::Cut21);
    bop.bbop(s, "r4", Operation::Section);
    bop.bbuild(s, "rgf");
    checkshape_named(s, "r0");
    checkshape_named(s, "r1");
    checkshape_named(s, "r2");
    checkshape_named(s, "r3");
    checkshape_named(s, "r4");
    checkshape_named(s, "rgf");
    checknbshapes(
        s,
        "r0",
        Counting::Typed,
        &[
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r0", Prop::Area, Expected::Mass(0.0739843));
    checkprops(s, "r0", Prop::Volume, Expected::Mass(0.00122155));
    checknbshapes(
        s,
        "r1",
        Counting::Typed,
        &[
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r1", Prop::Area, Expected::Mass(7.27512));
    checkprops(s, "r1", Prop::Volume, Expected::Mass(1.12443));
    checknbshapes(
        s,
        "r2",
        Counting::Typed,
        &[
            NbCount::Wires(8),
            NbCount::Faces(8),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r2", Prop::Area, Expected::Mass(5.98712));
    checkprops(s, "r2", Prop::Volume, Expected::Mass(0.998778));
    checknbshapes(
        s,
        "r3",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r3", Prop::Area, Expected::Mass(1.36198));
    checkprops(s, "r3", Prop::Volume, Expected::Mass(0.124436));
    checkprops(s, "r4", Prop::Length, Expected::Mass(0.656552));
    checksection_expecting(s, "r4", 0);
    checknbshapes(
        s,
        "rgf",
        Counting::Typed,
        &[
            NbCount::Wires(12),
            NbCount::Faces(12),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    checkprops(s, "rgf", Prop::Area, Expected::Mass(7.42309));
    checkprops(s, "rgf", Prop::Volume, Expected::Mass(1.12443));
    checkview(s, &CheckView::of("rgf"));
}
