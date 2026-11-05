//! Snapshot only after all pcurve work has joined, retaining source edge/face identities.
use super::fixture::PCurveCase;
use super::*;
/// Emit only owned-source geometry; `geometry_rows(&case)` contains no inferred identities.
pub(super) fn geometry_rows(case: &PCurveCase) -> String {
    let mut rows = Vec::new();
    for edge in &case.edges {
        for face in &case.faces {
            rows.push(edge_row(case, edge, face));
        }
    }
    format!("[{}]", rows.join(","))
}
fn edge_row(case: &PCurveCase, edge: &Edge, face: &Face) -> String {
    let structure = case.filler.data_structure().unwrap();
    let curve = BRepTool::curve_on_face(edge, face).unwrap();
    let (first, last) = topexp::vertices(edge, false);
    format!(
        "[{},{},{},{},{},{},{}]",
        structure.index(edge),
        structure.index(face),
        !curve.pcurve.is_null(),
        BRepTool::edge_tolerance(edge).unwrap().to_bits(),
        BRepTool::vertex_tolerance(&first).unwrap().to_bits(),
        BRepTool::vertex_tolerance(&last).unwrap().to_bits(),
        pcurve_row(edge, face)
    )
}
fn pcurve_row(edge: &Edge, face: &Face) -> String {
    let curve = BRepTool::curve_on_face(edge, face).unwrap();
    if curve.pcurve.is_null() {
        return "[]".to_owned();
    }
    let first = curve.pcurve.value(curve.first).unwrap();
    let last = curve.pcurve.value(curve.last).unwrap();
    format!(
        "[{},{},{},{},{},{}]",
        curve.first.to_bits(),
        curve.last.to_bits(),
        first.x().to_bits(),
        first.y().to_bits(),
        last.x().to_bits(),
        last.y().to_bits()
    )
}
#[test]
fn existing_pcurve_geometry_matches_owned_native_baselines() {
    for line in include_str!("baseline.expected").lines() {
        let (kind, expected) = line.split_once('|').unwrap();
        let mut case = super::fixture::pcurve_case(kind.parse().unwrap(), false);
        case.filler.make_pcurves().unwrap();
        assert_eq!(
            geometry_rows(&case),
            expected,
            "geometry baseline kind {kind}"
        );
    }
}
