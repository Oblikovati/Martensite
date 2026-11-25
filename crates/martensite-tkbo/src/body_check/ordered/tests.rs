// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::profile_extrusion::DisplayTolerance;
use martensite_tkbrep::{
    brep::{BRepBuilder, access::vertex_tshape},
    topexp::Explorer,
    topods::{edge, face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::gp::{Direction2, Point2};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn cylinder() -> Shape {
    BRepPrimAPIMakeCylinder::new(10., 4.)
        .shape()
        .unwrap()
        .clone()
}

fn resolution(cap: f64) -> Resolution {
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(cap),
    )
}

fn first_vertex(body: &Shape) -> Shape {
    TopologyNames::of(body).unwrap().all(ShapeEnum::Vertex)[0].clone()
}

fn stopped(body: &Shape) -> ValidationReport {
    let original = resolution(1e-5);
    let BodyGeometryValidation::Stopped(report) = begin_body_validation(body, &original).unwrap()
    else {
        panic!("the earlier stage must block the checkpoint");
    };
    assert_eq!(report, validate_body(body, &original).unwrap());
    report
}

#[test]
fn invalid_resolution_precedes_body_indexing() {
    let rejected = begin_body_validation(&Shape::default(), &resolution(-1.))
        .err()
        .unwrap();
    assert_eq!(
        rejected,
        ResolutionRejected {
            field: "cap",
            value: -1.,
        }
    );
}

#[test]
fn raw_tolerance_refusal_has_no_native_stage_results() {
    let body = cylinder();
    vertex_tshape(&first_vertex(&body), "raw tolerance fixture")
        .unwrap()
        .set_tolerance(-1.);
    let report = stopped(&body);
    use StageVerdict::{Failed, NotRun};
    assert_eq!(
        report.stages.map(|stage| stage.verdict),
        [NotRun, NotRun, NotRun, Failed]
    );
}

#[test]
fn topology_refusal_short_circuits_the_geometry_checkpoint() {
    let report = stopped(&first_vertex(&cylinder()));
    use StageVerdict::{Failed, NotRun};
    assert_eq!(
        report.stages.map(|stage| stage.verdict),
        [Failed, NotRun, NotRun, NotRun]
    );
}

#[test]
fn incompatible_surface_trace_blocks_intersection_and_tolerance() {
    let report = stopped(&inconsistent_box());
    use StageVerdict::{Failed, NotRun, Passed};
    assert_eq!(
        report.stages.map(|stage| stage.verdict),
        [Passed, Failed, NotRun, NotRun]
    );
}

fn inconsistent_box() -> Shape {
    let solid = BRepPrimAPIMakeBox::from_sizes(1., 1., 1.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let faces = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let target = face(faces.current()).unwrap();
    let edges = Explorer::new(&target, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let target_edge = edge(edges.current()).unwrap();
    let line = Geom2dLine::from_point(Point2::new(100., 100.), Direction2::new(1., 0.).unwrap());
    let displaced: Handle<dyn Geom2dCurve> = Handle::new(line).into_upcast();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&target_edge, &displaced, &target, 1e-7)
        .unwrap();
    solid
}

#[test]
fn ready_prefix_and_isolated_tolerance_use_the_ordinary_native_stages() {
    for (budget, expected) in [(1e-5, StageVerdict::Passed), (1e-12, StageVerdict::Failed)] {
        let body = cylinder();
        let original = resolution(budget);
        let BodyGeometryValidation::Ready(ready) = begin_body_validation(&body, &original).unwrap()
        else {
            panic!("topology and geometry should qualify at either cap");
        };
        let prefix = ready.report();
        let native = validate_body(&cylinder(), &original).unwrap();
        let ordinary_tolerance = ready.check_late_tolerance();
        check_common_stages(&prefix, &native);
        assert_eq!(
            &ordinary_tolerance,
            native.stage(ValidationStage::Tolerance)
        );
        assert_eq!(ordinary_tolerance.verdict, expected);
        assert_eq!(prefix.tolerance_budget, Some(budget));
    }
}

fn check_common_stages(prefix: &ValidationReport, native: &ValidationReport) {
    use StageVerdict::{NotRun, Passed};
    assert_eq!(prefix.census, native.census);
    assert_eq!(prefix.census.len(), 1);
    for stage in [ValidationStage::Topology, ValidationStage::Geometry] {
        assert_eq!(prefix.stage(stage), native.stage(stage));
        assert_eq!(prefix.stage(stage).verdict, Passed);
    }
    assert_eq!(
        prefix.stage(ValidationStage::Topology).mode,
        CheckMode::Exact
    );
    assert_eq!(
        prefix.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert_eq!(
        native.stage(ValidationStage::SelfIntersection).mode,
        CheckMode::NativeSearch
    );
    for stage in [
        ValidationStage::SelfIntersection,
        ValidationStage::Tolerance,
    ] {
        assert_eq!(prefix.stage(stage).verdict, NotRun);
    }
    assert!(!prefix.is_valid());
}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<BodyGeometryValidation<'static>>();
    shared::<OrderedBodyGeometry<'static>>();
};
