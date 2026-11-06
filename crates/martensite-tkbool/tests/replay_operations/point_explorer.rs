//! Counterpart of the recorded capture.
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSDataStructure, TopOpeBRepDSPoint, TopOpeBRepDSPointExplorer,
};
use martensite_tkmath::gp::Point3;

fn explorer_point(output: &BufferedTokens, point: &TopOpeBRepDSPoint) {
    output.pnt(point.point());
    output.flag(point.keep());
}

fn point_source(count: i32, mask: i32) -> TopOpeBRepDSDataStructure {
    let mut source = TopOpeBRepDSDataStructure::new();
    for slot in 0..count {
        let mut point = TopOpeBRepDSPoint::new(
            Point3::new(f64::from(slot + 1), 2., 3.),
            0.25 * f64::from(slot + 1),
        );
        point.change_keep(mask & (1 << slot) != 0);
        source.add_point(&point);
    }
    source
}

fn point_walk(output: &BufferedTokens, explorer: &mut TopOpeBRepDSPointExplorer<'_>) {
    while explorer.more() {
        output.int(explorer.index());
        explorer_point(output, explorer.point().unwrap());
        output.real(explorer.point().unwrap().tolerance());
        explorer.next();
    }
    output.int(explorer.index());
    explorer_point(output, explorer.point().unwrap());
    output.int(explorer.nb_point());
    output.flag(explorer.more());
    output.int(explorer.index());
}

fn point_default(output: &BufferedTokens, explorer: &mut TopOpeBRepDSPointExplorer<'_>) {
    output.flag(explorer.more());
    output.int(explorer.index());
    explorer_point(output, explorer.point().unwrap());
    explorer.next();
    output.int(explorer.index());
}

/// Exercises binding, filtering and recount; `point_explorer_case(output, &[4., 5., 1., 2.])`.
pub fn point_explorer_case(output: &BufferedTokens, values: &[f64]) {
    let mut reader = Reader::new(values);
    let source = point_source(reader.integer(), reader.integer());
    let kept = reader.integer() != 0;
    let query = reader.integer();
    let mut explorer = TopOpeBRepDSPointExplorer::default();
    point_default(output, &mut explorer);
    explorer.init(&source, kept);
    output.flag(explorer.is_point(query));
    output.flag(explorer.is_point_keep(query));
    explorer_point(output, explorer.point_at(query));
    point_walk(output, &mut explorer);
    explorer.init(&source, !kept);
    point_walk(output, &mut explorer);
    output.end();
}

/// Compares live edits and stale-current failure after clearing; `point_explorer_mutation_case(output, &[])`.
pub fn point_explorer_mutation_case(output: &BufferedTokens, _: &[f64]) {
    let mut source = point_source(2, 1);
    let mut explorer = TopOpeBRepDSPointExplorer::new_mut(&mut source, true);
    let editable = explorer.source_mut().unwrap();
    editable.change_keep_point(2, true).unwrap();
    editable.add_point(&TopOpeBRepDSPoint::new(Point3::new(3., 2., 3.), 0.75));
    output.flag(explorer.is_point(3));
    explorer_point(output, explorer.point_at(3));
    point_walk(output, &mut explorer);
    point_clear_source(output, &mut source);
    output.end();
}

fn point_clear_source(output: &BufferedTokens, source: &mut TopOpeBRepDSDataStructure) {
    let mut explorer = TopOpeBRepDSPointExplorer::new_mut(source, true);
    explorer.source_mut().unwrap().init();
    output.flag(explorer.more());
    output.attempt(|| explorer.point().map(|point| explorer_point(output, point)));
    explorer_point(output, explorer.point_at(1));
    explorer.next();
    output.flag(explorer.more());
    output.int(explorer.index());
}
