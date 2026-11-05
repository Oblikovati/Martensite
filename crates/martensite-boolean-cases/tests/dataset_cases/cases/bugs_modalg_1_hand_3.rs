//! `bugs_modalg_1`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::repair::revol_copied,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    construction::{edge, mkplane, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};

pub fn bug13211_6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13211");
    println!("============");
    println!();
    vertex(s, "v1", [136.06, 187.51, 0.0]);
    vertex(s, "v2", [136.06, 143.01, 0.0]);
    edge(s, "e1", "v1", "v2");
    vertex(s, "v3", [136.06, 143.01, 0.0]);
    vertex(s, "v4", [128.08, 143.01, 0.0]);
    edge(s, "e2", "v3", "v4");
    vertex(s, "v5", [128.08, 143.01, 0.0]);
    vertex(s, "v6", [127.88, 164.75, 0.0]);
    edge(s, "e3", "v5", "v6");
    vertex(s, "v7", [127.88, 164.75, 0.0]);
    vertex(s, "v8", [116.11, 176.00, 0.0]);
    edge(s, "e4", "v7", "v8");
    vertex(s, "v9", [116.11, 176.00, 0.0]);
    vertex(s, "v10", [136.06, 187.51, 0.0]);
    edge(s, "e5", "v9", "v10");
    wire(s, "twire", &["e1", "e2", "e3", "e4", "e5"]);
    mkplane(s, "tface", "twire");
    revol_copied(s, "r1", "tface", [153.08, 143.01, 0.0], [1., 0., 0.], 360.);
    revol_copied(s, "r2", "tface", [136.06, 68.01, 0.0], [0., 1., 0.], 360.);
    two_step.bop(s, "r1", "r2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(228.699));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(15),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(29),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
