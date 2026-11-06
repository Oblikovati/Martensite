//! Counterpart of the recorded capture.
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSCurve, TopOpeBRepDSCurveExplorer, TopOpeBRepDSDataStructure,
};

fn explorer_curve(output: &BufferedTokens, curve: &TopOpeBRepDSCurve) {
    output.int(curve.ds_index());
    output.flag(curve.keep());
    output.real(curve.tolerance());
    output.flag(curve.curve().is_null());
}

fn explorer_walk(output: &BufferedTokens, explorer: &mut TopOpeBRepDSCurveExplorer<'_>) {
    while explorer.more() {
        output.int(explorer.index());
        explorer_curve(output, explorer.curve());
        explorer.next();
    }
    output.int(explorer.index());
    explorer_curve(output, explorer.curve());
    output.int(explorer.nb_curve());
    output.flag(explorer.more());
    output.int(explorer.index());
}

fn explorer_source(count: i32, gap: i32, mask: i32) -> TopOpeBRepDSDataStructure {
    let mut curves = TopOpeBRepDSDataStructure::new();
    for slot in 0..count {
        if slot == 1 {
            curves.change_nb_curves(curves.nb_curves() + gap);
        }
        let mut curve = TopOpeBRepDSCurve::default();
        curve.set_tolerance(0.25 * f64::from(slot + 1));
        curve.change_keep(mask & (1 << slot) != 0);
        curves.add_curve(&mut curve);
    }
    curves.change_curve(-1).set_tolerance(12.0);
    curves
}

/// Compares filtered traversal and lookup; `curve_explorer_case(output, &[3., 2., 5., 1., 4.])`.
pub fn curve_explorer_case(output: &BufferedTokens, values: &[f64]) {
    let mut reader = Reader::new(values);
    let curves = explorer_source(reader.integer(), reader.integer(), reader.integer());
    let only_kept = reader.integer() != 0;
    let query = reader.integer();
    let mut explorer = TopOpeBRepDSCurveExplorer::new(&curves, only_kept);
    output.flag(explorer.is_curve(query));
    output.flag(explorer.is_curve_keep(query));
    explorer_curve(output, explorer.curve_at(query));
    explorer_walk(output, &mut explorer);
    explorer.init(&curves, !only_kept);
    explorer_walk(output, &mut explorer);
    output.end();
}

/// Edits the borrowed source during traversal; `curve_explorer_mutation_case(output, &[2.])`.
pub fn curve_explorer_mutation_case(output: &BufferedTokens, values: &[f64]) {
    let gap = Reader::new(values).integer();
    let mut curves = explorer_source(2, gap, 1);
    let mut explorer = TopOpeBRepDSCurveExplorer::new_mut(&mut curves, true);
    let source = explorer.source_mut().expect("exclusive explorer source");
    source.change_keep_curve(2 + gap, true);
    let index = source.add_curve(&mut TopOpeBRepDSCurve::default());
    output.flag(explorer.is_curve(index));
    explorer_curve(output, explorer.curve_at(index));
    explorer_walk(output, &mut explorer);
    explorer_clear_source(output, &mut curves);
    output.end();
}

fn explorer_clear_source(output: &BufferedTokens, curves: &mut TopOpeBRepDSDataStructure) {
    let mut explorer = TopOpeBRepDSCurveExplorer::new_mut(curves, true);
    let source = explorer.source_mut().expect("exclusive explorer source");
    source.init();
    source.change_curve(-1).set_tolerance(12.0);
    output.flag(explorer.more());
    explorer_curve(output, explorer.curve());
    explorer_curve(output, explorer.curve_at(1));
    explorer.next();
    output.flag(explorer.more());
    output.int(explorer.index());
}
