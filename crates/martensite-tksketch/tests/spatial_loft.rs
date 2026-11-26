// SPDX-License-Identifier: GPL-2.0-only
//! Ordinary corresponding Loft through production wire, builder, validation and export.
use martensite_tkbo::{
    body_check::{CheckMode, StageVerdict, ValidationStage},
    measured::Evidence,
    profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap},
};
use martensite_tkbrep::{
    bintools,
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tksketch::spatial_loft::{
    DetachedLoft, LoftFailure, LoftHistory, LoftImage, LoftLine, LoftOptions, LoftPoint,
    LoftResult, LoftSection, LoftSectionId, LoftStage, loft_ruled_pair,
};
use martensite_tksketch3d::{
    Edit, LineId, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
    wire::{DirectedLineUse, LineDirection, WireClosure, WireRecipe},
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::{cmp::Ordering, num::NonZeroU64};

static_assertions::assert_impl_all!(LoftResult: Send, Sync);
static_assertions::assert_impl_all!(LoftFailure: Send, Sync);
static_assertions::assert_impl_all!(LoftOptions: Send, Sync);
static_assertions::assert_impl_all!(LoftSection<'static>: Send, Sync);
static_assertions::assert_impl_all!(LoftSectionId: Send, Sync);
static_assertions::assert_impl_all!(LoftStage: Send, Sync);
static_assertions::assert_impl_all!(LoftHistory: Send, Sync);
static_assertions::assert_impl_all!(LoftLine: Send, Sync);
static_assertions::assert_impl_all!(LoftPoint: Send, Sync);
static_assertions::assert_impl_all!(LoftImage: Send, Sync);
static_assertions::assert_impl_all!(DetachedLoft: Send, Sync);

const BASE: [[f64; 3]; 4] = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
const KEYS: [u64; 4] = [91, 5, 77, 22];
const LINES: [u64; 4] = [40, 11, 36, 17];

fn id(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}
fn pt([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn options() -> LoftOptions {
    LoftOptions {
        resolution: Resolution::new(
            1e-6,
            1e-7,
            1e-9,
            DisplayTolerance {
                chordal: 1e-3,
                angular: 0.1,
            },
            ToleranceCap::Refuse(1e-5),
        ),
        source_trace_limit: 1e-8,
    }
}

fn snapshot(points: [[f64; 3]; 4]) -> Snapshot {
    let points = KEYS.into_iter().zip(points).map(|(key, xyz)| {
        Edit::InsertPoint(SketchPoint {
            id: PointId::new(id(key)),
            position: pt(xyz),
            role: PointRole::Defining,
        })
    });
    let lines = [(0, 1), (2, 1), (2, 3), (3, 0)]
        .into_iter()
        .zip(LINES)
        .map(|((a, b), key)| {
            Edit::InsertLine(SketchLine {
                id: LineId::new(id(key)),
                endpoints: [PointId::new(id(KEYS[a])), PointId::new(id(KEYS[b]))],
            })
        });
    Snapshot::empty(MinimumLength::new(1e-6).unwrap())
        .apply(&points.chain(lines).collect::<Vec<_>>(), || false)
        .unwrap()
}

fn uses() -> [DirectedLineUse; 4] {
    std::array::from_fn(|i| DirectedLineUse {
        line: LineId::new(id(LINES[i])),
        direction: if i == 1 {
            LineDirection::Reversed
        } else {
            LineDirection::Forward
        },
    })
}

fn wire(source: &Snapshot) -> WireRecipe {
    source
        .wire_recipe(&uses(), WireClosure::Closed, 6, || false)
        .unwrap()
}

fn ruled<'a>(first: &'a WireRecipe, last: &'a WireRecipe) -> [LoftSection<'a>; 2] {
    [
        LoftSection {
            id: LoftSectionId(id(17)),
            wire: first,
        },
        LoftSection {
            id: LoftSectionId(id(3)),
            wire: last,
        },
    ]
}

fn upper(scale: f64, shift: [f64; 3]) -> [[f64; 3]; 4] {
    BASE.map(|p| std::array::from_fn(|axis| p[axis] * scale + shift[axis]))
}

#[test]
fn e10_loft_and_scaled_slanted_frustum_match_independent_measures_and_actual_images() {
    let lower = wire(&snapshot(BASE));
    for (scale, shift, volume) in [(1., [0., 0., 3.], 12.), (2., [0.5, -1., 3.], 28.)] {
        let higher = wire(&snapshot(upper(scale, shift)));
        let loft = loft_ruled_pair(ruled(&lower, &higher), options(), || false).unwrap();
        assert_eq!(loft.station_sign(), Ordering::Greater);
        assert_eq!(loft.volume().evidence(), Evidence::Proven);
        assert!(loft.source_trace_bound() <= 1e-12);
        let expected = [
            ValidationStage::Topology,
            ValidationStage::Geometry,
            ValidationStage::SelfIntersection,
            ValidationStage::Tolerance,
        ];
        for stage in expected {
            assert_eq!(loft.validation().stage(stage).verdict, StageVerdict::Passed);
        }
        assert_eq!(
            loft.validation().stage(ValidationStage::Topology).mode,
            CheckMode::Exact
        );
        assert_eq!(
            loft.validation()
                .stage(ValidationStage::SelfIntersection)
                .mode,
            CheckMode::NativeSearch
        );
        let detached = loft.export_brep(|| false).unwrap();
        assert_volume(&loft, &detached, volume);
        assert_images(&detached, scale, shift);
    }
}

fn assert_volume(loft: &LoftResult, detached: &DetachedLoft, expected: f64) {
    let mut props = GProps::default();
    BRepGProp::volume_properties(&detached.solid, &mut props, true, false, false).unwrap();
    assert!((props.mass() - expected).abs() <= 1e-10);
    assert!(
        loft.volume().encloses(expected) && loft.volume().encloses(props.mass()),
        "expected {expected}, actual {}, enclosure {:?}",
        props.mass(),
        loft.volume()
    );
}

fn assert_images(native: &DetachedLoft, scale: f64, shift: [f64; 3]) {
    assert_eq!(native.images.len(), 26);
    let sections = [LoftSectionId(id(17)), LoftSectionId(id(3))];
    for (i, section) in sections.into_iter().enumerate() {
        let cap = image(native, LoftHistory::Cap(section));
        let mut props = GProps::default();
        BRepGProp::surface_properties(cap, &mut props, false, false).unwrap();
        let expected = if i == 0 { 4. } else { 4. * scale * scale };
        assert!((props.mass() - expected).abs() <= 1e-10);
        let center = if i == 0 { [0.; 3] } else { shift };
        assert!(props.centre_of_mass().distance(&pt(center)) <= 1e-10);
    }
    for directed in uses() {
        let keys = sections.map(|section| LoftLine { section, directed });
        let side = image(native, LoftHistory::Side(keys));
        for key in keys {
            assert!(contains(
                side,
                ShapeEnum::Edge,
                image(native, LoftHistory::SectionEdge(key))
            ));
        }
    }
    for point in KEYS.map(|key| PointId::new(id(key))) {
        let keys = sections.map(|section| LoftPoint { section, point });
        let edge = image(native, LoftHistory::Rail(keys));
        for key in keys {
            assert!(contains(
                edge,
                ShapeEnum::Vertex,
                image(native, LoftHistory::SectionVertex(key))
            ));
        }
    }
}

fn image(native: &DetachedLoft, history: LoftHistory) -> &Shape {
    let mut found = native
        .images
        .iter()
        .filter(|entry| entry.history == history);
    let exact = found.next().unwrap();
    assert!(found.next().is_none());
    &exact.shape
}

fn contains(parent: &Shape, kind: ShapeEnum, exact: &Shape) -> bool {
    Explorer::new(parent, kind, ShapeEnum::Shape)
        .unwrap()
        .any(|found| found.is_same(exact))
}

#[test]
fn refusal_preserves_explicit_order_distinct_ids_and_exact_coplanarity() {
    let lower = wire(&snapshot(BASE));
    let higher = snapshot(upper(1., [0., 0., 3.]));
    let mut shifted = uses();
    shifted.rotate_left(1);
    let rephased = higher
        .wire_recipe(&shifted, WireClosure::Closed, 6, || false)
        .unwrap();
    assert!(matches!(
        loft_ruled_pair(ruled(&lower, &rephased), options(), || false),
        Err(LoftFailure::Incompatible { .. })
    ));
    assert!(matches!(
        loft_ruled_pair(ruled(&lower, &lower), options(), || false),
        Err(LoftFailure::CoincidentPlanes)
    ));
    let upper = wire(&higher);
    let mut pair = ruled(&lower, &upper);
    pair[1].id = pair[0].id;
    assert!(matches!(
        loft_ruled_pair(pair, options(), || false),
        Err(LoftFailure::RepeatedSection(_))
    ));
    let open = higher
        .wire_recipe(&uses()[..3], WireClosure::Open, 3, || false)
        .unwrap();
    assert!(matches!(
        loft_ruled_pair(ruled(&lower, &open), options(), || false),
        Err(LoftFailure::OpenSection(_))
    ));
    let mut skew = crate::upper(1., [0., 0., 3.]);
    skew[3][2] = 3.0_f64.next_up();
    let skew = wire(&snapshot(skew));
    assert!(matches!(
        loft_ruled_pair(ruled(&lower, &skew), options(), || false),
        Err(LoftFailure::SectionPlane { section, .. }) if section == LoftSectionId(id(3))
    ));
}

#[test]
fn rebuilt_source_and_mutated_export_leave_old_native_body_independent() {
    let a = wire(&snapshot(BASE));
    let source = snapshot(upper(1., [0., 0., 3.]));
    let b = wire(&source);
    let old = loft_ruled_pair(ruled(&a, &b), options(), || false).unwrap();
    let published = old.export_brep(|| false).unwrap();
    let before = archive(&published.solid);
    let target = image(
        &published,
        LoftHistory::SectionVertex(LoftPoint {
            section: LoftSectionId(id(3)),
            point: PointId::new(id(KEYS[0])),
        }),
    );
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&topods::vertex(target).unwrap(), pt([90., 90., 90.]), 1e-7)
        .unwrap();
    assert_ne!(archive(&published.solid), before);
    assert_eq!(archive(&old.export_brep(|| false).unwrap().solid), before);
    let updates = KEYS
        .into_iter()
        .zip(upper(1., [0., 0., 5.]))
        .map(|(key, xyz)| Edit::MovePoint {
            point: PointId::new(id(key)),
            position: pt(xyz),
        })
        .collect::<Vec<_>>();
    let edited = source.apply(&updates, || false).unwrap();
    let fresh = loft_ruled_pair(ruled(&a, &wire(&edited)), options(), || false).unwrap();
    let native = fresh.export_brep(|| false).unwrap();
    assert_volume(&fresh, &native, 20.);
    assert_ne!(archive(&native.solid), before);
    assert_eq!(
        fresh.history().collect::<Vec<_>>(),
        old.history().collect::<Vec<_>>()
    );
}

#[test]
fn cancellation_drops_published_candidate_or_detached_copy() {
    let lower = wire(&snapshot(BASE));
    let upper = wire(&snapshot(upper(1., [0., 0., 3.])));
    let pair = ruled(&lower, &upper);
    let mut checkpoints = 0;
    let complete = loft_ruled_pair(pair, options(), || {
        checkpoints += 1;
        false
    })
    .unwrap();
    let mut count = 0;
    assert!(matches!(
        loft_ruled_pair(pair, options(), || {
            count += 1;
            count == checkpoints
        }),
        Err(LoftFailure::Cancelled(LoftStage::Publication))
    ));
    let mut count = 0;
    assert!(matches!(
        complete.export_brep(|| {
            count += 1;
            count == 5
        }),
        Err(LoftFailure::Cancelled(LoftStage::Export))
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
fn reversed_section_order_changes_station_sign_with_the_same_native_volume() {
    let first = wire(&snapshot(BASE));
    let last = wire(&snapshot(upper(2., [0.5, -1., 3.])));
    let reversed = loft_ruled_pair(ruled(&last, &first), options(), || false).unwrap();
    assert_eq!(reversed.station_sign(), Ordering::Less);
    assert_volume(&reversed, &reversed.export_brep(|| false).unwrap(), 28.);
}

#[test]
fn fresh_processes_reproduce_complete_native_archive_and_authored_history() {
    fn receipt() -> (String, String) {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "emit_ruled_loft_receipt", "--nocapture"])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let text = String::from_utf8(out.stdout).unwrap();
        let lines = text
            .lines()
            .filter(|line| line.starts_with("LOFT_"))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        (lines[0].into(), lines[1].into())
    }
    assert_eq!(receipt(), receipt());
}

#[test]
fn emit_ruled_loft_receipt() {
    use std::fmt::Write;
    let first = wire(&snapshot(BASE));
    let last = wire(&snapshot(upper(2., [0.5, -1., 3.])));
    let loft = loft_ruled_pair(ruled(&first, &last), options(), || false).unwrap();
    let export = loft.export_brep(|| false).unwrap();
    let mut bytes = String::new();
    for byte in archive(&export.solid) {
        write!(bytes, "{byte:02x}").unwrap();
    }
    println!("LOFT_ARCHIVE={bytes}");
    println!("LOFT_HISTORY={:?}", loft.history().collect::<Vec<_>>());
}
