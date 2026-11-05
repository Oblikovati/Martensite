//! Formatter fixtures construct explicit fault results; real backend fixtures live in the target.
use super::*;
const GROUP_STATUSES: [CheckStatus; 7] = [
    CheckStatus::SelfIntersect,
    CheckStatus::TooSmallEdge,
    CheckStatus::NonRecoverableFace,
    CheckStatus::IncompatibilityOfVertex,
    CheckStatus::IncompatibilityOfEdge,
    CheckStatus::GeomAbsC0,
    CheckStatus::InvalidCurveOnSurface,
];

const GROUP_NAMES: [&str; 7] = [
    "s1si_1", "s1se_1", "s1bf_1", "s1bv_1", "s1be_1", "s1C0_1", "s1COnS_1",
];
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

fn point_shape(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

fn blank_report() -> RetainedFullArgumentReport {
    RetainedFullArgumentReport {
        output: String::new(),
        bindings: Vec::new(),
        copied_second: None,
    }
}

fn fault(status: CheckStatus, shape: &Shape) -> CheckResult {
    let mut result = CheckResult::new();
    result.set_check_status(status);
    result.set_shape1(shape);
    result.add_faulty_shape1(shape);
    result
}

#[test]
fn ordered_groups_bind_both_actual_fault_lists_without_deduplication() {
    let first = point_shape(1.);
    let second = point_shape(2.);
    let mut result = fault(CheckStatus::SelfIntersect, &first);
    result.add_faulty_shape1(&first);
    result.set_shape2(&second);
    result.add_faulty_shape2(&second);
    let mut sides = [SideGroups::default(), SideGroups::default()];
    let mut report = blank_report();
    collect_result(&result, &mut sides, &mut report, &mut false).unwrap();
    assert_eq!(
        report.output,
        "Made faulty shape: s1si_1\nMade faulty shape: s2si_1\n"
    );
    assert_eq!(sides[0].totals[1], 2);
    assert_eq!(sides[1].totals[1], 1);
    assert_compound_children(&report.bindings[0].1, &[first.clone(), first]);
    assert_compound_children(&report.bindings[1].1, &[second]);
}

fn assert_compound_children(compound: &Shape, expected: &[Shape]) {
    let mut iterator = ShapeIterator::new(compound, true, true);
    for shape in expected {
        assert!(iterator.more());
        assert!(iterator.value().unwrap().is_same(shape));
        iterator.next();
    }
    assert!(!iterator.more());
}

#[test]
fn all_fault_categories_preserve_native_names_and_real_counts() {
    let shape = point_shape(1.);
    let mut sides = [SideGroups::default(), SideGroups::default()];
    let mut report = blank_report();
    for status in GROUP_STATUSES {
        collect_result(&fault(status, &shape), &mut sides, &mut report, &mut false).unwrap();
    }
    let names: Vec<&str> = report
        .bindings
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(names, GROUP_NAMES);
    assert!(
        report
            .output
            .ends_with("Made faulty shape: s1COnS_1 (MaxDist = 0, MaxPar = 0)\n")
    );
}

#[test]
fn curve_distance_uses_actual_side_values_and_interpreter_precision() {
    let shape = point_shape(1.);
    let mut result = fault(CheckStatus::InvalidCurveOnSurface, &shape);
    result.set_shape2(&shape);
    result.add_faulty_shape2(&shape);
    result.set_max_distance1(0.1);
    result.set_max_parameter1(0.5);
    result.set_max_distance2(0.25);
    result.set_max_parameter2(0.75);
    let mut report = blank_report();
    let mut sides = [SideGroups::default(), SideGroups::default()];
    collect_result(&result, &mut sides, &mut report, &mut false).unwrap();
    assert_eq!(
        report.output,
        "Made faulty shape: s1COnS_1 (MaxDist = 0.10000000000000001, MaxPar = 0.5)\nMade faulty shape: s2COnS_1 (MaxDist = 0.25, MaxPar = 0.75)\n"
    );
}

#[test]
fn unsupported_face_category_is_ignored_but_unknown_status_is_retained() {
    let mut report = blank_report();
    let mut sides = [SideGroups::default(), SideGroups::default()];
    let shape = point_shape(1.);
    let mut unknown = false;
    for (status, expected) in [
        (CheckStatus::IncompatibilityOfFace, false),
        (CheckStatus::CheckUnknown, true),
    ] {
        collect_result(
            &fault(status, &shape),
            &mut sides,
            &mut report,
            &mut unknown,
        )
        .unwrap();
        assert_eq!(unknown, expected);
    }
    assert!(report.bindings.is_empty());
}

#[test]
fn summary_collapses_bad_types_and_keeps_disabled_pair_modes() {
    let mut sides = [SideGroups::default(), SideGroups::default()];
    sides[0].cases[0] = 2;
    sides[0].cases[2] = 1;
    let mut output = String::new();
    output::summaries(&sides, true, &mut output);
    assert!(output.starts_with("Faulties for FIRST  shape found : 2\n"));
    assert!(output.contains("Shapes are not suppotrted by BOP: YES\n"));
    assert!(output.contains("Too close vertices              : DISABLED\n"));
    assert!(output.contains("Too close edges                 : DISABLED\n"));
    assert!(output.ends_with("\nFaulties for SECOND  shape found : 0\n"));
}
