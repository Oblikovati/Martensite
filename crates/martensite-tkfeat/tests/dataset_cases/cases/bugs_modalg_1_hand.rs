//! The `bugs/modalg_1` scripts that branch on what a command answers.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    feat_script::FeatScript,
    inspect::{max_tolerance_of, maxtolerance_line, renamevar},
    properties::Prop,
    restore::restore,
    splitshape::splitshape,
};

pub fn buc60942(s: &mut FeatScript<'_>) {
    println!("============");
    println!("BUC60942");
    println!("============");
    println!();
    restore(s, "BUC60942_splitshape.brep", "c");
    explode(s, "c", None);
    checkshape_named(s, "c_1");
    checkshape_named(s, "c_2");
    renamevar(s, "c_1", "f");
    renamevar(s, "c_2", "w");
    let before = maxtolerance_line(s, "w");
    let (max_edge_1, max_vertex_1) = (
        max_tolerance_of(&before, "Edge"),
        max_tolerance_of(&before, "Vertex"),
    );
    splitshape(s, "result", "f", &["f", "w"], &[]);
    let after = maxtolerance_line(s, "w");
    let max_edge_2 = max_tolerance_of(&after, "Edge");
    let _max_vertex_2 = max_tolerance_of(&after, "Vertex");
    // The script compares the first vertex value with itself; its second operand is kept as written.
    #[allow(clippy::eq_op)]
    if max_edge_1 < max_edge_2 || max_vertex_1 < max_vertex_1 {
        println!(
            "Faulty BUC60942: max tolerance of the input wire has been increased by about 10 times."
        );
    } else {
        println!("BUC60942 OK: Tolerance of shape is correct");
    }
    checkprops(s, "result", Prop::Area, Expected::Mass(37.7032));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
