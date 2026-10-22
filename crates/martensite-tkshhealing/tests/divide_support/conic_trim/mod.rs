//! In-process full conversion/replay across conic policy and trim families.
mod fixture;
use fixture::ConicCase;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Edge},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::{
    shape_extend::ShapeExtendStatus, shape_upgrade::ShapeUpgradeShapeConvertToBezier,
};

#[test]
fn full_operation_conic_modes_trims_orientations_and_context_replay_match_native() {
    let mut rows =
        include_str!("../../fixtures/behavior/bezier_conic_trim_family.expected").lines();
    for id in 0..64 {
        let case = ConicCase::from_id(id);
        let converter = configured_converter(&case);
        for phase in 0..2 {
            let changed = converter.perform(phase == 0).unwrap();
            assert_policy_outcome(&case, changed, &converter);
            assert_case((id, phase), changed, &converter, &mut rows);
        }
    }
    assert!(rows.next().is_none());
}
fn configured_converter(case: &ConicCase) -> ShapeUpgradeShapeConvertToBezier {
    let converter = ShapeUpgradeShapeConvertToBezier::new(case.edge().into_shape());
    converter.set_3d_conversion(true);
    converter.set_3d_line_conversion(false);
    converter.set_3d_circle_conversion(case.modes & 1 != 0);
    converter.set_3d_conic_conversion(case.modes & 2 != 0);
    converter
}
const FLAGS: [ShapeExtendStatus; 5] = [
    ShapeExtendStatus::Done1,
    ShapeExtendStatus::Done2,
    ShapeExtendStatus::Fail1,
    ShapeExtendStatus::Fail2,
    ShapeExtendStatus::Fail3,
];
fn case_edges(converter: &ShapeUpgradeShapeConvertToBezier) -> Vec<Edge> {
    Explorer::new(&converter.result(), ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::edge(&shape).unwrap())
        .collect()
}
fn case_header(
    key: (usize, usize),
    changed: bool,
    converter: &ShapeUpgradeShapeConvertToBezier,
    count: usize,
) -> String {
    let kind = converter.result().shape_type().unwrap() as i32;
    let prefix = format!(
        "case {} {} {} {kind} {count}",
        key.0,
        key.1,
        i32::from(changed)
    );
    let flags = FLAGS.map(|flag| i32::from(converter.status_flag(flag)).to_string());
    format!("{prefix} {}", flags.join(" "))
}
fn assert_case(
    key: (usize, usize),
    changed: bool,
    converter: &ShapeUpgradeShapeConvertToBezier,
    rows: &mut std::str::Lines<'_>,
) {
    let edges = case_edges(converter);
    assert_eq!(
        case_header(key, changed, converter, edges.len()),
        rows.next().unwrap(),
        "case {key:?}"
    );
    for edge in edges {
        assert_curve(key.0, key.1, &edge, rows.next().unwrap());
    }
}
fn assert_policy_outcome(
    case: &ConicCase,
    changed: bool,
    converter: &ShapeUpgradeShapeConvertToBezier,
) {
    let expected = case.converts_to_bezier();
    assert_eq!(changed, expected);
    for edge in case_edges(converter) {
        let curve = BRepTool::curve(&edge).unwrap().0;
        let bezier = !curve
            .cast::<martensite_tkg3d::geom::GeomBezierCurve>()
            .is_null();
        assert_eq!(bezier, expected);
    }
}
fn assert_curve(id: usize, phase: usize, edge: &Edge, line: &str) {
    let expected: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(expected[0], "curve");
    let (curve, first, last) = BRepTool::curve(edge).unwrap();
    let kind = curve.get().unwrap().dynamic_type().name();
    assert_eq!(kind, expected[1], "case {id} phase {phase}");
    let orientation = expected[2].parse::<i32>().unwrap();
    assert_eq!(edge.orientation() as i32, orientation);
    let mut values = vec![first, last];
    for parameter in [first, (first + last) * 0.5, last] {
        let point = curve.value(parameter).unwrap();
        values.extend([point.x(), point.y(), point.z()]);
    }
    assert_numbers(id, phase, &values, &expected[3..]);
}
fn assert_numbers(id: usize, phase: usize, values: &[f64], expected: &[&str]) {
    assert_eq!(values.len(), expected.len());
    for (actual, word) in values.iter().zip(expected) {
        let value = word.parse::<f64>().unwrap();
        assert!(
            (actual - value).abs() <= 1e-14,
            "case{id} phase{phase}: {actual} != {value}"
        );
    }
}
