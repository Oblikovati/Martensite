// SPDX-License-Identifier: GPL-2.0-only
use super::{archive, lid, options, pid, pt};
use martensite_tkbo::{
    body_check::{CheckMode, StageVerdict},
    measured::Evidence,
    profile_extrusion::{HistoryRole, PlanarProofWorkLimit, ToleranceCap},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::geom::{GeomCircle, GeomCylindricalSurface};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Direction3;
use martensite_tksketch::spatial_prism::{
    DetachedMixedSpatialPrism, MixedPrismArcTrim, MixedSpatialPrismHistory, MixedSpatialPrismImage,
    MixedSpatialPrismResult, MixedSpatialPrismSource, SpatialPrismFailure, SpatialPrismStage,
    extrude_mixed_wire,
};
use martensite_tksketch3d::{
    ArcFitTolerance, ArcId, CurveId, Edit, MinimumLength, PointRole, SketchLine, SketchPoint,
    Snapshot,
    bend::{BendIds, BendOptions, BendRequest},
    wire::{
        LineDirection::{Forward, Reversed},
        WireClosure,
        mixed::{DirectedCurveUse, MixedWireRecipe},
    },
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
    num::NonZeroU64,
};

static_assertions::assert_impl_all!(MixedSpatialPrismResult: Send, Sync);
static_assertions::assert_impl_all!(MixedPrismArcTrim: Send, Sync);
static_assertions::assert_impl_all!(MixedSpatialPrismSource: Send, Sync);
static_assertions::assert_impl_all!(MixedSpatialPrismHistory: Send, Sync);
static_assertions::assert_impl_all!(MixedSpatialPrismImage: Send, Sync);
static_assertions::assert_impl_all!(DetachedMixedSpatialPrism: Send, Sync);

const AREA: f64 = 875. + 25. * FRAC_PI_4;
const VOLUME: f64 = 2625. + 75. * FRAC_PI_4;

fn arc_id() -> ArcId {
    ArcId::new(NonZeroU64::new(10).unwrap())
}

fn label(id: u64, xyz: [f64; 3]) -> Edit {
    Edit::InsertPoint(SketchPoint {
        id: pid(id),
        role: PointRole::Defining,
        position: pt(xyz),
    })
}

fn line(id: u64, first: u64, last: u64) -> Edit {
    Edit::InsertLine(SketchLine {
        id: lid(id),
        endpoints: [pid(first), pid(last)],
    })
}

fn seed(place: impl Fn([f64; 3]) -> [f64; 3], skew: bool) -> Snapshot {
    let corners = [
        label(1, place([30., 0., 0.])),
        label(2, place([60., 0., 0.])),
        label(3, place([60., 30., 0.])),
        label(14, place([30., 30., 0.])),
        line(4, 1, 2),
        line(5, 2, 3),
        line(20, 3, 14),
    ];
    let draft = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(&corners, || false)
        .unwrap();
    let closure = if skew {
        vec![
            label(15, [30., 30., 1.]),
            label(16, [30., 0., 1.]),
            line(21, 14, 15),
            line(22, 15, 16),
            line(23, 16, 1),
        ]
    } else {
        vec![line(21, 14, 1)]
    };
    draft.apply(&closure, || false).unwrap()
}

fn bend(source: &Snapshot) -> Snapshot {
    let request = BendRequest {
        lines: [lid(4), lid(5)],
        corner: pid(2),
        radius: 5.,
        ids: BendIds {
            lines: [lid(8), lid(9)],
            arc: arc_id(),
            tangencies: [pid(6), pid(7)],
            center: pid(11),
        },
        options: BendOptions {
            max_position_error: 0.,
            max_radius_error: 0.,
            max_tangent_angle: f64::EPSILON,
            arc_fit: ArcFitTolerance::new(1e-12).unwrap(),
        },
    };
    source.bend_lines(request, || false).unwrap().sketch
}

fn rounded_wire(
    place: impl Fn([f64; 3]) -> [f64; 3],
    reversed: bool,
    skew: bool,
) -> MixedWireRecipe {
    let snapshot = bend(&seed(place, skew));
    let mut curves = vec![
        CurveId::Line(lid(8)),
        CurveId::Arc(arc_id()),
        CurveId::Line(lid(9)),
        CurveId::Line(lid(20)),
        CurveId::Line(lid(21)),
    ];
    if skew {
        curves.extend([CurveId::Line(lid(22)), CurveId::Line(lid(23))]);
    }
    let direction = if reversed { Reversed } else { Forward };
    if reversed {
        curves.reverse();
    }
    let uses: Vec<_> = curves
        .into_iter()
        .map(|curve| DirectedCurveUse { curve, direction })
        .collect();
    snapshot
        .mixed_wire_recipe(&uses, WireClosure::Closed, 64, || false)
        .unwrap()
}

fn image(
    native: &DetachedMixedSpatialPrism,
    source: MixedSpatialPrismSource,
    role: HistoryRole,
) -> &Shape {
    &native
        .images
        .iter()
        .find(|entry| entry.history.source == source && entry.history.role == role)
        .unwrap()
        .shape
}

fn as_curve(use_: DirectedCurveUse) -> MixedSpatialPrismSource {
    MixedSpatialPrismSource::Curve(use_)
}

fn start_end_caps(native: &DetachedMixedSpatialPrism) -> [Face; 2] {
    [HistoryRole::StartCap, HistoryRole::EndCap]
        .map(|role| topods::face(image(native, MixedSpatialPrismSource::Boundary, role)).unwrap())
}

fn arc_edges(native: &DetachedMixedSpatialPrism, arc: MixedSpatialPrismSource) -> [Edge; 2] {
    [HistoryRole::StartEdge, HistoryRole::EndEdge]
        .map(|role| topods::edge(image(native, arc, role)).unwrap())
}

fn assert_curved_traces(native: &DetachedMixedSpatialPrism, arc: MixedSpatialPrismSource) {
    let side = topods::face(image(native, arc, HistoryRole::SideFace)).unwrap();
    let (surface, _) = BRepTool::surface_with_location(&side).unwrap();
    let cylinder = surface.downcast_ref::<GeomCylindricalSurface>().unwrap();
    assert_eq!(cylinder.radius(), 5.);
    for (index, (cap, edge)) in start_end_caps(native)
        .into_iter()
        .zip(arc_edges(native, arc))
        .enumerate()
    {
        let curve = BRepTool::curve_with_location(&edge).unwrap();
        assert_eq!(
            curve.curve.downcast_ref::<GeomCircle>().unwrap().radius(),
            5.
        );
        assert_eq!([curve.first, curve.last], [0., FRAC_PI_2]);
        let cap_trim = BRepTool::curve_on_face(&edge, &cap).unwrap();
        assert_eq!(
            cap_trim
                .pcurve
                .downcast_ref::<Geom2dCircle>()
                .unwrap()
                .radius(),
            5.
        );
        assert!(cap_trim.is_stored);
        let side_trim = BRepTool::curve_on_face(&edge, &side).unwrap();
        assert!(side_trim.is_stored);
        assert_eq!([side_trim.first, side_trim.last], [0., FRAC_PI_2]);
        let line = side_trim.pcurve.downcast_ref::<Geom2dLine>().unwrap();
        assert_eq!(line.location().y(), [0., -3.][index]);
        assert_eq!([line.direction().x(), line.direction().y()], [1., 0.]);
        let [a, b] = [pid(6), pid(7)].map(|point| {
            let role = [HistoryRole::StartVertex, HistoryRole::EndVertex][index];
            topods::vertex(image(native, MixedSpatialPrismSource::Point(point), role)).unwrap()
        });
        let (first, last) = topexp::vertices(&edge, false);
        assert!(first.is_same(&a) && last.is_same(&b));
    }
}

#[test]
fn rounded_bend_extrusion_proves_stored_caps_cylinder_and_closed_form_volume() {
    let source = rounded_wire(|p| p, false, false);
    let result = extrude_mixed_wire(&source, [0., 0., 3.], options(), || false).unwrap();
    let report = result.validation();
    assert!(
        report
            .stages
            .iter()
            .all(|stage| stage.verdict == StageVerdict::Passed)
    );
    assert_eq!(report.stages[2].mode, CheckMode::NativeSearch);
    let [census] = report.census.as_slice() else {
        panic!("one solid");
    };
    assert_eq!(
        (census.vertices, census.edges, census.faces, census.genus),
        (10, 15, 7, Some(0))
    );
    assert!(result.area().encloses(AREA));
    assert!(result.volume().encloses(VOLUME));
    assert_eq!(result.area().evidence(), Evidence::Proven);
    assert_eq!(result.volume().evidence(), Evidence::Proven);
    assert!(result.volume().error_bound < 1e-10);
    assert_eq!(result.history().len(), 32);
    assert_eq!(result.arc_trim().arc, arc_id());
    assert_eq!(result.arc_trim().center_control, pid(11));
    assert_eq!(result.arc_trim().endpoints, [pid(6), pid(7)]);
    assert_eq!(result.arc_trim().parameters, [0., FRAC_PI_2]);
    let native = result.export_brep(|| false).unwrap();
    let arc = as_curve(DirectedCurveUse {
        curve: CurveId::Arc(arc_id()),
        direction: Forward,
    });
    assert_curved_traces(&native, arc);
    let mut props = GProps::new();
    BRepGProp::volume_properties_eps(&native.solid, &mut props, 1e-10, true, false).unwrap();
    assert!((props.mass() - VOLUME).abs() < 1e-8);
}

#[test]
fn reversed_original_cycle_and_negative_translated_cardinal_sweep_retain_semantics() {
    let source = rounded_wire(|[x, y, z]| [y + 100., z + 20., x + 10.], true, false);
    let result = extrude_mixed_wire(&source, [0., -3., 0.], options(), || false).unwrap();
    assert!(result.validation().is_valid());
    assert!(result.area().encloses(AREA) && result.volume().encloses(VOLUME));
    assert_eq!(result.signed_height(), -3.);
    assert_eq!(result.arc_trim().parameters, [FRAC_PI_2, 0.]);
    assert_eq!(result.arc_trim().endpoints, [pid(7), pid(6)]);
    let native = result.export_brep(|| false).unwrap();
    let arc = as_curve(DirectedCurveUse {
        curve: CurveId::Arc(arc_id()),
        direction: Reversed,
    });
    assert_curved_traces(&native, arc);
    let [first, last] = start_end_caps(&native);
    for (cap, y) in [(first, 20.), (last, 17.)] {
        let (surface, _) = BRepTool::surface_with_location(&cap).unwrap();
        let plane = surface
            .downcast_ref::<martensite_tkg3d::geom::GeomPlane>()
            .unwrap()
            .pln();
        assert_eq!(plane.location().y(), y);
        assert_eq!(plane.position().direction(), Direction3::Y);
    }
}

#[test]
fn mixed_prism_refuses_skew_oblique_unrepresentable_tiny_trace_and_cap() {
    let source = rounded_wire(|p| p, false, false);
    assert!(matches!(
        extrude_mixed_wire(&source, [1., 0., 3.], options(), || false),
        Err(SpatialPrismFailure::MixedDirection(_))
    ));
    let skew = rounded_wire(|p| p, false, true);
    assert!(matches!(
        extrude_mixed_wire(&skew, [0., 0., 3.], options(), || false),
        Err(SpatialPrismFailure::MixedPoint { .. })
    ));
    let large = rounded_wire(|[x, y, _]| [x, y, 2.0_f64.powi(53)], false, false);
    assert!(
        matches!(extrude_mixed_wire(&large, [0., 0., 1.], options(), || false), Err(SpatialPrismFailure::MixedPoint { point, .. }) if point == pid(11))
    );
    let mut strict = options();
    strict.source_trace_limit = 1e-18;
    assert!(matches!(
        extrude_mixed_wire(&source, [0., 0., 3.], strict, || false),
        Err(SpatialPrismFailure::MixedTrace { .. })
    ));
    strict = options();
    strict.resolution.cap = ToleranceCap::Refuse(1e-9);
    assert!(matches!(
        extrude_mixed_wire(&source, [0., 0., 3.], strict, || false),
        Err(SpatialPrismFailure::Region(_))
    ));
    strict = options();
    strict.planar_work = PlanarProofWorkLimit::PairBisections(0);
    let zero = extrude_mixed_wire(&source, [0., 0., 3.], strict, || false).unwrap();
    assert_eq!(zero.region_proof_work().consumed_pair_bisections, 0);
    assert!(matches!(
        extrude_mixed_wire(&source, [0., 0., 3.], options(), || true),
        Err(SpatialPrismFailure::Cancelled(SpatialPrismStage::Plane))
    ));
}

#[test]
fn mixed_prism_exports_detach_curves_and_parallel_receipts() {
    let source = rounded_wire(|p| p, false, false);
    let result = extrude_mixed_wire(&source, [0., 0., 3.], options(), || false).unwrap();
    let detached = result.export_brep(|| false).unwrap();
    let arc = as_curve(DirectedCurveUse {
        curve: CurveId::Arc(arc_id()),
        direction: Forward,
    });
    let bytes = archive(&detached.solid);
    let [cap, _] = start_end_caps(&detached);
    let [edge, _] = arc_edges(&detached, arc);
    let cap_trim = BRepTool::curve_on_face(&edge, &cap).unwrap();
    cap_trim
        .pcurve
        .downcast_ref::<Geom2dCircle>()
        .unwrap()
        .set_radius(50.)
        .unwrap();
    let side = topods::face(image(&detached, arc, HistoryRole::SideFace)).unwrap();
    let (surface, _) = BRepTool::surface_with_location(&side).unwrap();
    surface
        .downcast_ref::<GeomCylindricalSurface>()
        .unwrap()
        .set_radius(75.)
        .unwrap();
    assert_ne!(archive(&detached.solid), bytes);
    std::thread::scope(|scope| {
        let rounds: Vec<_> = (0..4)
            .map(|_| {
                let (source, result) = (&source, &result);
                scope.spawn(move || {
                    let fresh =
                        extrude_mixed_wire(source, [0., 0., 3.], options(), || false).unwrap();
                    assert_eq!(fresh.validation(), result.validation());
                    assert!(fresh.history().eq(result.history()));
                    [
                        result.export_brep(|| false).unwrap(),
                        fresh.export_brep(|| false).unwrap(),
                    ]
                })
            })
            .collect();
        for round in rounds {
            for native in round.join().unwrap() {
                assert_eq!(archive(&native.solid), bytes);
                assert_curved_traces(&native, arc);
            }
        }
    });
}

#[test]
fn cancelled_mixed_sweep_never_publishes_a_partial_result() {
    let source = rounded_wire(|p| p, false, false);
    let mut poll_count = 0;
    extrude_mixed_wire(&source, [0., 0., 3.], options(), || {
        poll_count += 1;
        false
    })
    .unwrap();
    let mut retired = 0;
    let result = extrude_mixed_wire(&source, [0., 0., 3.], options(), || {
        retired += 1;
        retired == poll_count
    });
    assert!(matches!(
        result,
        Err(SpatialPrismFailure::Cancelled(SpatialPrismStage::History))
    ));
    assert!(
        extrude_mixed_wire(&source, [0., 0., 3.], options(), || false)
            .unwrap()
            .volume()
            .encloses(VOLUME)
    );
}

#[test]
fn mixed_archive_and_history_are_process_and_legacy_precision_independent() {
    let plain = recorded_mixed_prism("0");
    assert_eq!(plain, recorded_mixed_prism("0"));
    assert_eq!(plain, recorded_mixed_prism("1"));
}

fn recorded_mixed_prism(altered_precision: &str) -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .env(
            "MARTENSITE_TEST_MIXED_PRISM_LEGACY_PRECISION",
            altered_precision,
        )
        .args([
            "--exact",
            "mixed::emit_mixed_prism_archive",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .split_once("MIXED_PRISM_ARCHIVE=")
        .unwrap()
        .1
        .lines()
        .take(2)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn emit_mixed_prism_archive() {
    use std::fmt::Write;
    let Ok(mode) = std::env::var("MARTENSITE_TEST_MIXED_PRISM_LEGACY_PRECISION") else {
        return;
    };
    if mode == "1" {
        martensite_tktopalgo::brep_lib::set_precision(0.5);
    }
    let source = rounded_wire(|p| p, false, false);
    let result = extrude_mixed_wire(&source, [0., 0., 3.], options(), || false).unwrap();
    let bytes = archive(&result.export_brep(|| false).unwrap().solid);
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex, "{byte:02x}").unwrap();
    }
    println!("MIXED_PRISM_ARCHIVE={hex}");
    println!(
        "MIXED_PRISM_HISTORY={:?}",
        result.history().collect::<Vec<_>>()
    );
}
