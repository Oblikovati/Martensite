//! `bugs_modalg_6` bodies written by hand, third file.
use crate::support::{
    bugs_commands::{
        beyond::child_kinds,
        inspect::{checknbshapes_selected, tolerance_read_edge_face},
        options_bop::ScriptBop,
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug22634(s: &mut dyn Session) {
    println!("============");
    println!("OCC22634");
    println!("============");
    println!();
    restore(s, "bug22634_shape.brep", "shape");
    restore(s, "bug22634_tool.brep", "tool");
    checkshape_named(s, "shape");
    checkshape_named(s, "tool");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "shape", "tool");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Volume, Expected::Mass(5.61524e+007));
    checkshape_named(s, "result");
    // The script's expected text has the letter `o` for its vertices, so only the others compare.
    checknbshapes_selected(
        s,
        "result",
        Counting::Typed,
        &[
            (1, 12),
            (2, 6),
            (3, 6),
            (4, 1),
            (5, 1),
            (6, 0),
            (7, 1),
            (8, 35),
        ],
        "Result obtained by Boolean cut operation",
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug27524(s: &mut dyn Session) {
    println!("TODO 0027524 ALL: Error: pcurve deviation is greater than shape tolerance");
    println!("=========");
    println!("CR27524");
    println!("=========");
    println!();
    let shape_tolerance = 0.0069791772900578497;
    let tolerance = 1.0e-7;
    restore(s, "bug27524.brep", "aS");
    checkshape_named(s, "aS");
    explode(s, "aS", Some(ShapeEnum::Edge));
    let edge = crate::support::commands::bound_shape(s, "aS_3");
    explode(s, "aS", Some(ShapeEnum::Face));
    let distance =
        tolerance_read_edge_face(&edge, &crate::support::commands::bound_shape(s, "aS_1"));
    if distance > shape_tolerance + tolerance {
        println!("Error: pcurve deviation is greater than shape tolerance");
    }
}

pub fn bug27759(s: &mut dyn Session) {
    println!("========");
    println!("OCC27759");
    println!("========");
    println!();
    restore(s, "bug27759_shapes.brep", "b");
    explode(s, "b", None);
    let mut bop = ScriptBop::default();
    bop.options.fuzzy = 1.0e-3;
    bop.direct(s, "result", ("b_1", "b_2"), Operation::Fuse);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(2), NbCount::Solids(2)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(3.45465));
    checkprops(s, "result", Prop::Volume, Expected::Mass(0.176959));
    bop.options.fuzzy = 0.0;
    // Beyond the script: checks an earlier test of this case made.
    child_kinds(s, "b", &[ShapeEnum::Compound, ShapeEnum::Compound]);
}

pub fn bug27032(s: &mut dyn Session) {
    println!("========");
    println!("OCC27032");
    println!("========");
    println!();
    restore(s, "OCC27032_sh.brep", "a");
    explode(s, "a", None);
    crate::support::one_step::bcut(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.80949e+006));
    // Beyond the script: checks an earlier test of this case made.
    child_kinds(s, "a", &[ShapeEnum::Compound, ShapeEnum::Solid]);
}
