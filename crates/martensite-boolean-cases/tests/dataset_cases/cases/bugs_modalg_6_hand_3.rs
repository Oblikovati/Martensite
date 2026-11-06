//! `bugs_modalg_6`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::options_bop::ScriptBop,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    construction::vertex,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::polyline,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug28745_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case4_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.10175e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(4.81128e+007));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(3), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28745_5(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case5_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.71804e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.39439e+009));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(3), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28745_6(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case6_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.71804e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.39439e+009));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(3), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28745_7(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28745");
    println!("========");
    println!();
    restore(s, "bug28745_case7_cut.brep", "b");
    explode(s, "b", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b_1"]);
    bop.add_tools(s, &["b_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.10175e+006));
    checkprops(s, "result", Prop::Volume, Expected::Mass(4.81128e+007));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Faces(3), NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug28775(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC28775");
    println!("========");
    println!();
    vertex(s, "v1", [1., 0., 0.]);
    polyline(s, "p1", &[[0., 0., 0.], [2., 0., 0.]]);
    polyline(s, "p2", &[[1., 0., -1.], [1., 0., 1.]]);
    bop.options.non_destructive = true;
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["v1", "p1", "p2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(4)]);
}
