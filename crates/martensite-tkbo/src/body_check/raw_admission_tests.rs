//! Invalid stored tolerances must be refused before native mutation (#490).
use super::*;
use crate::profile_extrusion::DisplayTolerance;
use martensite_tkbrep::{
    bintools::{BinShapeSet, BinShapeSetBase},
    brep::{BRepTool, access},
    topods::{edge, face},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalTrimQuery, MetricBezier, certify_cylindrical_trim,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::{num::NonZeroUsize, sync::Arc};

fn cylinder() -> Shape {
    BRepPrimAPIMakeCylinder::new(10., 4.)
        .shape()
        .unwrap()
        .clone()
}
fn raw(shape: &Shape) -> f64 {
    match shape.shape_type().unwrap() {
        ShapeEnum::Vertex => access::vertex_tshape(shape, "fixture").unwrap().tolerance(),
        ShapeEnum::Edge => access::edge_tshape(shape, "fixture").unwrap().tolerance(),
        ShapeEnum::Face => access::face_tshape(shape, "fixture").unwrap().tolerance(),
        _ => panic!("expected tolerance-bearing shape"),
    }
}
fn set_raw(shape: &Shape, value: f64) {
    match shape.shape_type().unwrap() {
        ShapeEnum::Vertex => access::vertex_tshape(shape, "fixture")
            .unwrap()
            .set_tolerance(value),
        ShapeEnum::Edge => access::edge_tshape(shape, "fixture")
            .unwrap()
            .set_tolerance(value),
        ShapeEnum::Face => access::face_tshape(shape, "fixture")
            .unwrap()
            .set_tolerance(value),
        _ => panic!("expected tolerance-bearing shape"),
    }
}
fn resolution(cap: ToleranceCap) -> Resolution {
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        cap,
    )
}
fn snapshot(body: &Shape, names: &TopologyNames) -> (Vec<u64>, Vec<u8>) {
    let values = [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .into_iter()
        .flat_map(|kind| names.all(kind))
        .map(|shape| raw(&shape).to_bits())
        .collect();
    let mut set = BinShapeSet::new();
    set.add(body).unwrap();
    let mut stream = IoStream::from_bytes(Vec::new());
    set.write(&mut stream, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; usize::try_from(stream.tellp()).unwrap()];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    (values, bytes)
}
fn assert_refused(kind: ShapeEnum, value: f64, cap: ToleranceCap) {
    let body = cylinder();
    let names = TopologyNames::of(&body).unwrap();
    let subject = names.all(kind).remove(0);
    set_raw(&subject, value);
    let before = snapshot(&body, &names);
    let report = validate_body(&body, &resolution(cap)).unwrap();
    assert!(
        !report.is_valid(),
        "kind={kind:?}, value={value}, cap={cap:?}"
    );
    assert_eq!(snapshot(&body, &names), before, "refusal mutated input");
    assert_admission_report(&report, names.name(&subject), value);
}
fn assert_admission_report(report: &ValidationReport, subject: TopologyRef, value: f64) {
    assert_skipped_native_stages(report);
    let tolerance = report.stage(ValidationStage::Tolerance);
    assert!(
        report
            .first_non_pass()
            .is_some_and(|first| std::ptr::eq(first, tolerance))
    );
    assert!(!report.is_exact());
    assert_eq!(tolerance.verdict, StageVerdict::Failed);
    assert_eq!(tolerance.mode, CheckMode::Exact);
    assert_eq!(tolerance.findings.len(), 1);
    assert_eq!(tolerance.findings[0].subject, subject);
    assert!(
        matches!(tolerance.findings[0].kind, FindingKind::InvalidTolerance { tolerance }
        if tolerance.to_bits() == value.to_bits())
    );
}
fn assert_skipped_native_stages(report: &ValidationReport) {
    for stage in [
        ValidationStage::Topology,
        ValidationStage::Geometry,
        ValidationStage::SelfIntersection,
    ] {
        assert_eq!(report.stage(stage).verdict, StageVerdict::NotRun);
    }
}
fn assert_all_policies(kind: ShapeEnum, value: f64) {
    for cap in [
        ToleranceCap::NoLimit,
        ToleranceCap::Report(1e-5),
        ToleranceCap::Refuse(1e-5),
    ] {
        assert_refused(kind, value, cap);
    }
}
#[test]
fn invalid_raw_values_refuse_before_native_checks_without_mutation() {
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for value in [f64::NAN, -1., f64::INFINITY, f64::NEG_INFINITY] {
            assert_all_policies(kind, value);
        }
    }
}

#[test]
fn curve_bounds_preserve_ordered_modes_and_raw_refusal() {
    let body = cylinder();
    let resolution = resolution(ToleranceCap::Refuse(1e-4));
    let bounds = CylindricalCurveBounds::default();
    let read = |checkpoint| match checkpoint {
        BodyGeometryValidation::Ready(value) => value.report(),
        BodyGeometryValidation::Stopped(report) => report,
    };
    let normal = || read(begin_body_validation(&body, &resolution).unwrap());
    let provisioned =
        || read(begin_body_validation_with_curve_bounds(&body, &resolution, &bounds).unwrap());
    let report = provisioned();
    assert_eq!(report, normal());
    assert_eq!(
        report.stage(ValidationStage::Topology).mode,
        CheckMode::Exact
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).verdict,
        StageVerdict::Passed
    );
    let names = TopologyNames::of(&body).unwrap();
    let edge = names.all(ShapeEnum::Edge).remove(0);
    set_raw(&edge, -1.);
    let report = provisioned();
    assert_eq!(report, normal());
    assert_admission_report(&report, names.name(&edge), -1.);
}

fn scoped_fixture_bounds(body: &Shape) -> (CylindricalCurveBounds, Shape) {
    let lateral = TopologyNames::of(body)
        .unwrap()
        .all(ShapeEnum::Face)
        .into_iter()
        .filter_map(|shape| face(&shape).ok())
        .find(|face| {
            BRepTool::surface(face)
                .unwrap()
                .downcast_ref::<GeomCylindricalSurface>()
                .is_some()
        })
        .unwrap();
    let member = TopologyNames::of(&lateral)
        .unwrap()
        .all(ShapeEnum::Edge)
        .into_iter()
        .filter_map(|shape| edge(&shape).ok())
        .find(|edge| BRepTool::is_closed_on_face(edge, &lateral).unwrap())
        .unwrap();
    let mut bounds = CylindricalCurveBounds::default();
    // The analytic native seam is ineligible for this Bézier receipt. It still
    // exercises the same native cap-table route as a foreseen cache miss.
    let proof = certify_cylindrical_trim(
        CylindricalTrimQuery {
            curve: MetricBezier {
                poles: &[[10., 0., 0.], [10., 0., 4.]],
                weights: Some(&[1., 1.]),
            },
            pcurve: &[[0., 0.], [0., 4.]],
            radius: 10.,
            spatial_limit: 1e-7,
            options: Default::default(),
        },
        || false,
    )
    .unwrap();
    bounds.insert(member.clone(), lateral, Arc::new(proof));
    (bounds, member.into_shape())
}

#[test]
fn scoped_native_faces_preserve_passes_and_serial_finding_order() {
    let body = cylinder();
    let (bounds, member) = scoped_fixture_bounds(&body);
    for stage in [CheckerStage::Topology, CheckerStage::Geometry] {
        assert_scoped_passes_and_findings(&body, &bounds, &member, stage);
    }
}

fn assert_scoped_passes_and_findings(
    body: &Shape,
    bounds: &CylindricalCurveBounds,
    member: &Shape,
    stage: CheckerStage,
) {
    let names = TopologyNames::of(body).unwrap();
    let limit = NonZeroUsize::MIN.saturating_add(1);
    assert_worker_scope(body, bounds, stage == CheckerStage::Geometry, limit);
    let read = |worker_limit| {
        native::checker_findings(
            body,
            stage,
            &names,
            Some(CurveChecks {
                bounds,
                worker_limit,
            }),
        )
    };
    assert!(read(limit).is_empty());
    assert_eq!(read(limit), read(NonZeroUsize::MIN));
    let bad_edge = access::edge_tshape(member, "fixture").unwrap();
    bad_edge.set_same_parameter(false);
    let ordinary = read(NonZeroUsize::MIN);
    assert!(!ordinary.is_empty());
    assert_eq!(read(limit), ordinary);
    bad_edge.set_same_parameter(true);
}

fn assert_worker_scope(
    body: &Shape,
    bounds: &CylindricalCurveBounds,
    geometry: bool,
    limit: NonZeroUsize,
) {
    assert!(
        BRepCheckAnalyzer::try_scoped_cylindrical_checks(body, bounds, geometry, NonZeroUsize::MIN)
            .is_none()
    );
    if std::thread::available_parallelism().is_ok_and(|count| count >= limit) {
        assert!(
            BRepCheckAnalyzer::try_scoped_cylindrical_checks(body, bounds, geometry, limit)
                .is_some()
        );
    }
}
#[test]
fn admitted_cylinder_retains_all_four_required_checks() {
    let report = validate_body(&cylinder(), &resolution(ToleranceCap::Refuse(1e-5))).unwrap();
    assert!(report.is_valid());
    assert_eq!(
        report.stage(ValidationStage::Topology).mode,
        CheckMode::Exact
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert_eq!(
        report.stage(ValidationStage::SelfIntersection).mode,
        CheckMode::NativeSearch
    );
    assert_eq!(report.tolerance_budget, Some(1e-5));
}
