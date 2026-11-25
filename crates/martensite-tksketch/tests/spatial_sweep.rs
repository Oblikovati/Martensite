// SPDX-License-Identifier: GPL-2.0-only
//! The E10 rounded open sheet through production recipes, native history and export.
use martensite_tkbo::{
    body_check::{CheckMode, StageVerdict, ValidationStage},
    profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap},
};
use martensite_tkbrep::{
    bintools,
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::{geom::GeomPlane, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tksketch::spatial_sweep::{
    DetachedSweep, SweepEdgeHistory, SweepFaceHistory, SweepFailure, SweepHistory, SweepImage,
    SweepInput, SweepOptions, SweepPathPart, SweepResult, SweepStage, SweepSupportEvidence,
    sweep_rounded_shell,
};
use martensite_tksketch3d::{
    Edit, LineId, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
    wire::{DirectedLineUse, LineDirection, WireClosure, WireRecipe},
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::num::NonZeroU64;

static_assertions::assert_impl_all!(SweepResult: Send, Sync);
static_assertions::assert_impl_all!(SweepFailure: Send, Sync);
static_assertions::assert_impl_all!(SweepOptions: Send, Sync);
static_assertions::assert_impl_all!(SweepHistory: Send, Sync);
static_assertions::assert_impl_all!(SweepInput: Send, Sync);
static_assertions::assert_impl_all!(SweepStage: Send, Sync);
static_assertions::assert_impl_all!(SweepPathPart: Send, Sync);
static_assertions::assert_impl_all!(SweepFaceHistory: Send, Sync);
static_assertions::assert_impl_all!(SweepEdgeHistory: Send, Sync);
static_assertions::assert_impl_all!(SweepImage: Send, Sync);
static_assertions::assert_impl_all!(SweepSupportEvidence: Send, Sync);
static_assertions::assert_impl_all!(DetachedSweep: Send, Sync);

const SPINE: [[f64; 3]; 4] = [[0., 0., 0.], [4., 0., 0.], [4., 4., 0.], [4., 4., 4.]];
const SECTION: [[f64; 3]; 4] = [
    [0., -0.5, -0.5],
    [0., 0.5, -0.5],
    [0., 0.5, 0.5],
    [0., -0.5, 0.5],
];
const SPINE_KEYS: [u64; 4] = [51, 7, 109, 43];
const PROFILE_KEYS: [u64; 4] = [1, 29, 5, 17];
const SPINE_USES: [(u64, usize, usize, bool); 3] =
    [(11, 0, 1, false), (3, 2, 1, true), (19, 2, 3, false)];
const PROFILE_USES: [(u64, usize, usize, bool); 4] = [
    (2, 0, 1, false),
    (13, 1, 2, false),
    (23, 2, 3, false),
    (59, 3, 0, false),
];

fn id(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}
fn point_id(value: u64) -> PointId {
    PointId::new(id(value))
}
fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn uses(lines: &[(u64, usize, usize, bool)]) -> Vec<DirectedLineUse> {
    lines
        .iter()
        .map(|&(key, _, _, rev)| DirectedLineUse {
            line: LineId::new(id(key)),
            direction: if rev {
                LineDirection::Reversed
            } else {
                LineDirection::Forward
            },
        })
        .collect()
}

fn snapshot(points: [[f64; 3]; 4], ids: [u64; 4], lines: &[(u64, usize, usize, bool)]) -> Snapshot {
    let points = ids.into_iter().zip(points).map(|(key, value)| {
        Edit::InsertPoint(SketchPoint {
            id: point_id(key),
            position: point(value),
            role: PointRole::Defining,
        })
    });
    let lines = lines.iter().map(|&(key, a, b, _)| {
        Edit::InsertLine(SketchLine {
            id: LineId::new(id(key)),
            endpoints: [point_id(ids[a]), point_id(ids[b])],
        })
    });
    Snapshot::empty(MinimumLength::new(1e-6).unwrap())
        .apply(&points.chain(lines).collect::<Vec<_>>(), || false)
        .unwrap()
}

fn spine(points: [[f64; 3]; 4]) -> WireRecipe {
    snapshot(points, SPINE_KEYS, &SPINE_USES)
        .wire_recipe(&uses(&SPINE_USES), WireClosure::Open, 3, || false)
        .unwrap()
}

fn profile(points: [[f64; 3]; 4]) -> WireRecipe {
    snapshot(points, PROFILE_KEYS, &PROFILE_USES)
        .wire_recipe(&uses(&PROFILE_USES), WireClosure::Closed, 6, || false)
        .unwrap()
}

fn options() -> SweepOptions {
    SweepOptions {
        resolution: Resolution::new(
            1e-6,
            1e-6,
            1e-6,
            DisplayTolerance {
                chordal: 1e-3,
                angular: 0.1,
            },
            ToleranceCap::Refuse(1e-5),
        ),
        boundary_tolerance: 1e-6,
        source_trace_limit: 1e-8,
        approximation_limit: 1e-6,
    }
}

#[test]
fn e10_sweep_binds_all_native_sheet_members_and_both_directed_sources() {
    let result =
        sweep_rounded_shell(&spine(SPINE), &profile(SECTION), options(), || false).unwrap();
    assert_eq!(result.options(), options());
    assert!(result.surface_error_estimate() <= options().approximation_limit);
    assert!(result.achieved_tolerance().max() <= 1e-5);
    for (stage, mode) in [
        (ValidationStage::Topology, CheckMode::Exact),
        (ValidationStage::Geometry, CheckMode::Sampled),
        (ValidationStage::SelfIntersection, CheckMode::NativeSearch),
        (ValidationStage::Tolerance, CheckMode::Sampled),
    ] {
        assert_eq!(
            result.validation().stage(stage).verdict,
            StageVerdict::Passed
        );
        assert_eq!(result.validation().stage(stage).mode, mode);
    }
    assert_eq!(
        result.support_evidence(),
        SweepSupportEvidence {
            bands: 12,
            planar_corners: 4,
            revolved_corners: 2,
            terminal_vertices: 8,
            trace_limit: 1e-8,
        }
    );
    let detached = result.export_brep(|| false).unwrap();
    assert_eq!(detached.shell.shape_type().unwrap(), ShapeEnum::Shell);
    assert_eq!(
        result.history().collect::<Vec<_>>(),
        detached
            .images
            .iter()
            .map(|image| &image.history)
            .collect::<Vec<_>>()
    );
    assert_members(&detached);
    assert_faces(&detached);
    assert_corner_area(&detached);
    assert_terminal_points(&detached, [[0., 0., 0.], [4., 4., 4.]]);
}

fn assert_members(detached: &DetachedSweep) {
    for (kind, count) in [
        (ShapeEnum::Face, 18),
        (ShapeEnum::Edge, 42),
        (ShapeEnum::Vertex, 24),
    ] {
        let native = Explorer::new(&detached.shell, kind, ShapeEnum::Shape)
            .unwrap()
            .collect::<Vec<_>>();
        let images = detached
            .images
            .iter()
            .filter(|image| image.shape.shape_type().unwrap() == kind)
            .collect::<Vec<_>>();
        assert_eq!(images.len(), count);
        assert!(native.iter().all(|shape| {
            images
                .iter()
                .filter(|image| image.shape.is_same(shape))
                .count()
                == 1
        }));
        assert!(
            images
                .iter()
                .all(|image| native.iter().any(|shape| shape.is_same(&image.shape)))
        );
    }
}

fn assert_faces(detached: &DetachedSweep) {
    let segments = uses(&SPINE_USES);
    let profile = uses(&PROFILE_USES);
    let faces = detached
        .images
        .iter()
        .filter_map(|image| match image.history {
            SweepHistory::Face(history) => Some(history),
            _ => None,
        })
        .collect::<Vec<_>>();
    for path in &segments {
        for side in &profile {
            assert!(faces.contains(&SweepFaceHistory {
                profile: *side,
                path: SweepPathPart::Band(*path)
            }));
        }
    }
    for i in 0..2 {
        let path = SweepPathPart::Corner {
            station: point_id(SPINE_KEYS[i + 1]),
            incoming: segments[i],
            outgoing: segments[i + 1],
        };
        assert_eq!(
            faces
                .iter()
                .filter(|face| face.path == path && profile.contains(&face.profile))
                .count(),
            3
        );
    }
}

fn assert_corner_area(detached: &DetachedSweep) {
    let mut witnesses = 0;
    for image in &detached.images {
        if !matches!(&image.history, SweepHistory::Face(h) if matches!(h.path, SweepPathPart::Corner { .. }))
        {
            continue;
        }
        let face = topods::face(&image.shape).unwrap();
        let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
        if surface.downcast_ref::<GeomPlane>().is_none() {
            continue;
        }
        let mut area = GProps::default();
        BRepGProp::surface_properties(&face, &mut area, false, false).unwrap();
        assert!((area.mass() - std::f64::consts::PI / 16.).abs() <= 1e-10);
        witnesses += 1;
    }
    assert_eq!(witnesses, 4);
}

fn assert_terminal_points(detached: &DetachedSweep, stations: [[f64; 3]; 2]) {
    for (i, station) in stations.into_iter().enumerate() {
        let indexes = [0, 3];
        let key = point_id(SPINE_KEYS[indexes[i]]);
        let boundary = detached
            .images
            .iter()
            .filter_map(|image| match &image.history {
                SweepHistory::Edge(history)
                    if history.spine_station == Some(key) && history.faces.len() == 1 =>
                {
                    Some(&image.shape)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(boundary.len(), 4);
        for [_, y, z] in SECTION {
            let expected = if i == 0 {
                point([station[0], station[1] + y, station[2] + z])
            } else {
                point([station[0] - y, station[1] - z, station[2]])
            };
            assert!(boundary.iter().any(|edge| has_vertex(edge, expected)));
        }
    }
}

fn has_vertex(edge: &Shape, expected: Point3) -> bool {
    Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .any(|vertex| {
            BRepTool::pnt(&topods::vertex(&vertex).unwrap())
                .unwrap()
                .distance(&expected)
                <= 1e-10
        })
}

#[test]
fn authored_scope_and_cancellation_refuse_before_publication() {
    let mut diagonal = SPINE;
    diagonal[3][0] += 0.25;
    let mut short = SPINE;
    short[3][2] = 0.25;
    let mut off_center = SECTION;
    off_center.iter_mut().for_each(|pt| pt[1] += 0.25);
    let opening = spine(SPINE);
    let section = profile(SECTION);
    assert!(matches!(
        sweep_rounded_shell(&spine(diagonal), &section, options(), || false),
        Err(SweepFailure::PathScope)
    ));
    assert!(matches!(
        sweep_rounded_shell(&spine(short), &section, options(), || false),
        Err(SweepFailure::Clearance { leg: 2, .. })
    ));
    assert!(matches!(
        sweep_rounded_shell(&opening, &profile(off_center), options(), || false),
        Err(SweepFailure::ProfileScope)
    ));
    assert!(matches!(
        sweep_rounded_shell(&opening, &section, options(), || true),
        Err(SweepFailure::Cancelled(SweepStage::Admission))
    ));
}

#[test]
fn source_edit_and_export_mutation_leave_completed_sweep_isolated() {
    let source = snapshot(SPINE, SPINE_KEYS, &SPINE_USES);
    let uses = uses(&SPINE_USES);
    let first = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    let section = profile(SECTION);
    let sweep = sweep_rounded_shell(&first, &section, options(), || false).unwrap();
    let changed = source
        .apply(
            &[Edit::MovePoint {
                point: point_id(SPINE_KEYS[3]),
                position: point([4., 4., 6.]),
            }],
            || false,
        )
        .unwrap();
    let changed = changed
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    let next = sweep_rounded_shell(&changed, &section, options(), || false).unwrap();
    assert_eq!(
        sweep.history().collect::<Vec<_>>(),
        next.history().collect::<Vec<_>>()
    );
    let export = sweep.export_brep(|| false).unwrap();
    let original = archive(&export.shell);
    let vertex = export
        .images
        .iter()
        .find(|image| matches!(image.history, SweepHistory::Vertex(_)))
        .unwrap();
    let vertex = topods::vertex(&vertex.shape).unwrap();
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&vertex, point([99., 99., 99.]), 1e-6)
        .unwrap();
    assert_ne!(archive(&export.shell), original);
    assert_eq!(
        archive(&sweep.export_brep(|| false).unwrap().shell),
        original
    );
    assert_terminal_points(
        &sweep.export_brep(|| false).unwrap(),
        [[0., 0., 0.], [4., 4., 4.]],
    );
    assert_terminal_points(
        &next.export_brep(|| false).unwrap(),
        [[0., 0., 0.], [4., 4., 6.]],
    );
    let mut polls = 0;
    assert!(matches!(
        sweep.export_brep(|| {
            polls += 1;
            polls >= 40
        }),
        Err(SweepFailure::Cancelled(SweepStage::Export))
    ));
}

fn archive(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    let options = bintools::WriteOptions {
        with_triangles: false,
        with_normals: false,
        ..Default::default()
    };
    bintools::write(shape, &mut stream, options, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; stream.tellp() as usize];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

#[test]
fn fresh_processes_reproduce_the_sheet_archive_and_authored_history() {
    assert_eq!(process_receipt(), process_receipt());
}

fn process_receipt() -> Vec<String> {
    let receipt = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "emit_sweep_receipt", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        receipt.status.success(),
        "{}",
        String::from_utf8_lossy(&receipt.stdout)
    );
    let text = String::from_utf8(receipt.stdout).unwrap();
    let lines = text
        .lines()
        .filter(|line| line.starts_with("SWEEP_"))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 2);
    lines
}

#[test]
fn emit_sweep_receipt() {
    use std::fmt::Write;
    let result =
        sweep_rounded_shell(&spine(SPINE), &profile(SECTION), options(), || false).unwrap();
    let detached = result.export_brep(|| false).unwrap();
    let mut bytes = String::new();
    for byte in archive(&detached.shell) {
        write!(bytes, "{byte:02x}").unwrap();
    }
    println!("SWEEP_ARCHIVE={bytes}");
    println!("SWEEP_HISTORY={:?}", result.history().collect::<Vec<_>>());
}

#[path = "spatial_sweep/solid.rs"]
mod solid;
