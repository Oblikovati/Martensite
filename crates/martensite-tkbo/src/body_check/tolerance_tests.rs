// SPDX-License-Identifier: GPL-2.0-only
//! Raw-value admission precedes the effective tolerance floor (#490).
use super::*;
use crate::body_check::{StageVerdict, ValidationStage, validate_body};
use crate::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tkbrep::brep::access::{edge_tshape, face_tshape, vertex_tshape};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(1., 2., 3.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
fn set_raw(shape: &Shape, value: f64) {
    match shape.shape_type().unwrap() {
        ShapeEnum::Vertex => vertex_tshape(shape, "test").unwrap().set_tolerance(value),
        ShapeEnum::Edge => edge_tshape(shape, "test").unwrap().set_tolerance(value),
        ShapeEnum::Face => face_tshape(shape, "test").unwrap().set_tolerance(value),
        _ => panic!("expected a vertex, edge or face"),
    }
}
fn subjects(names: &TopologyNames) -> [Shape; 3] {
    [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face].map(|kind| names.all(kind).remove(0))
}
fn findings_for(shape: &Shape, name: TopologyRef, budget: Option<f64>) -> Vec<Finding> {
    let mut findings = Vec::new();
    value_findings(shape, name, budget, &mut findings).unwrap();
    findings
}
fn assert_invalid_value(shape: &Shape, name: TopologyRef, value: f64) {
    set_raw(shape, value);
    for budget in [None, Some(1e-9)] {
        let findings = findings_for(shape, name, budget);
        assert_eq!(
            findings.len(),
            1,
            "kind={:?}, value={value}",
            shape.shape_type()
        );
        let FindingKind::InvalidTolerance { tolerance } = findings[0].kind else {
            panic!("expected invalid tolerance, got {:?}", findings[0]);
        };
        assert_eq!(tolerance.to_bits(), value.to_bits());
        assert_eq!(findings[0].subject, name);
        assert_eq!(findings[0].severity, Severity::Defect);
    }
}
#[test]
fn invalid_raw_values_are_reported_without_a_cap_for_every_kind() {
    let names = TopologyNames::of(&box_shape()).unwrap();
    for shape in subjects(&names) {
        for value in [f64::NAN, -1., f64::INFINITY, f64::NEG_INFINITY] {
            assert_invalid_value(&shape, names.name(&shape), value);
        }
    }
}
#[test]
fn finite_nonnegative_raw_values_keep_the_effective_floor() {
    let names = TopologyNames::of(&box_shape()).unwrap();
    let floor = martensite_tkernel::precision::confusion();
    for shape in subjects(&names) {
        for value in [0., -0., f64::from_bits(1), floor, 1e-3] {
            set_raw(&shape, value);
            assert!(findings_for(&shape, names.name(&shape), None).is_empty());
            let effective = stored_tolerance(&shape).unwrap();
            assert_eq!(effective, value.max(floor));
            assert!(findings_for(&shape, names.name(&shape), Some(effective)).is_empty());
            let capped = findings_for(&shape, names.name(&shape), Some(effective.next_down()));
            assert!(
                matches!(capped[0].kind, FindingKind::ToleranceAboveBudget { tolerance, budget }
                if tolerance.to_bits() == effective.to_bits() && budget == effective.next_down())
            );
        }
    }
}
#[test]
fn cap_comparisons_keep_the_effective_floor_for_every_kind() {
    let names = TopologyNames::of(&box_shape()).unwrap();
    let floor = martensite_tkernel::precision::confusion();
    for shape in subjects(&names) {
        set_raw(&shape, 0.);
        let findings = findings_for(&shape, names.name(&shape), Some(floor / 2.));
        assert_eq!(findings.len(), 1);
        assert!(
            matches!(findings[0].kind, FindingKind::ToleranceAboveBudget { tolerance, budget }
            if tolerance == floor && budget == floor / 2.)
        );
    }
}
#[test]
fn nesting_retains_effective_values_and_order_findings() {
    let names = TopologyNames::of(&box_shape()).unwrap();
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for shape in names.all(kind) {
            set_raw(&shape, 0.);
        }
    }
    let mut findings = Vec::new();
    nesting_findings(&names, &mut findings).unwrap();
    assert!(findings.is_empty());
    let edge = names.all(ShapeEnum::Edge).remove(0);
    set_raw(&edge, 1e-3);
    nesting_findings(&names, &mut findings).unwrap();
    assert_eq!(findings.len(), 2);
    assert!(findings.iter().all(|finding| matches!(finding.kind,
        FindingKind::ToleranceOrder { inner, outer }
        if inner == 1e-3 && outer == martensite_tkernel::precision::confusion())));
}
fn resolution() -> Resolution {
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::NoLimit,
    )
}
fn assert_invalid_report(kind: ShapeEnum, value: f64) {
    let shape = box_shape();
    let names = TopologyNames::of(&shape).unwrap();
    set_raw(&names.all(kind)[0], value);
    let report = validate_body(&shape, &resolution()).unwrap();
    assert!(!report.is_valid(), "kind={kind:?}, value={value}");
    let tolerance = report.stage(ValidationStage::Tolerance);
    assert_eq!(tolerance.verdict, StageVerdict::Failed);
    assert!(tolerance.findings.iter().any(
        |finding| matches!(finding.kind, FindingKind::InvalidTolerance { tolerance }
            if tolerance.to_bits() == value.to_bits())
    ));
}
#[test]
fn malformed_box_tolerances_never_publish_a_valid_report() {
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for value in [f64::NAN, -1.] {
            assert_invalid_report(kind, value);
        }
    }
}
