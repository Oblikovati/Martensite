// SPDX-License-Identifier: GPL-2.0-only
//! Native Phase 6 ordinary extrusion, original XYZ and stable history.
use martensite_tkbo::{
    body_check::{StageVerdict, ValidationStage},
    measured::Evidence,
    profile_extrusion::{
        DeclineCause, DisplayTolerance, HistoryRole, PlanarProofWorkLimit, Resolution,
        SelfIntersectionEvidence, ToleranceCap,
    },
};
use martensite_tkbrep::{
    bintools,
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::{self, Shape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::{
    geom::{GeomLine, GeomPlane},
    top_abs::ShapeEnum,
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tksketch::spatial_prism::{
    DetachedSpatialPrism, SpatialPrismFailure, SpatialPrismHistory, SpatialPrismImage,
    SpatialPrismOptions, SpatialPrismResult, SpatialPrismSource, SpatialPrismStage, extrude_wire,
};
use martensite_tksketch3d::{
    Edit, LineId, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
    wire::{DirectedLineUse, LineDirection, WireClosure, WireRecipe},
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::{cmp::Ordering, num::NonZeroU64};

static_assertions::assert_impl_all!(SpatialPrismOptions: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismResult: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismHistory: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismSource: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismImage: Send, Sync);
static_assertions::assert_impl_all!(DetachedSpatialPrism: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismFailure: Send, Sync);
static_assertions::assert_impl_all!(SpatialPrismStage: Send, Sync);

const POINTS: [[f64; 3]; 4] = [[2., -3., 5.], [5., -3., 5.], [5., 1., 8.], [2., 1., 8.]];
const DIRECTION: [f64; 3] = [0., -3., 4.];
const POINT_KEYS: [u64; 4] = [91, 5, 77, 22];
const LINE_KEYS: [u64; 4] = [40, 11, 36, 17];

fn pid(id: u64) -> PointId {
    PointId::new(NonZeroU64::new(id).unwrap())
}

fn lid(id: u64) -> LineId {
    LineId::new(NonZeroU64::new(id).unwrap())
}

fn pt([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

fn uses() -> [DirectedLineUse; 4] {
    let directions = [
        LineDirection::Forward,
        LineDirection::Reversed,
        LineDirection::Forward,
        LineDirection::Forward,
    ];
    std::array::from_fn(|i| DirectedLineUse {
        direction: directions[i],
        line: lid(LINE_KEYS[i]),
    })
}

fn snapshot(points: [[f64; 3]; 4]) -> Snapshot {
    let points = POINT_KEYS.into_iter().zip(points).map(|(id, xyz)| {
        Edit::InsertPoint(SketchPoint {
            id: pid(id),
            role: PointRole::Defining,
            position: pt(xyz),
        })
    });
    let lines = [(0, 1), (2, 1), (2, 3), (3, 0)]
        .into_iter()
        .zip(LINE_KEYS)
        .map(|((a, b), id)| {
            Edit::InsertLine(SketchLine {
                id: lid(id),
                endpoints: [pid(POINT_KEYS[a]), pid(POINT_KEYS[b])],
            })
        });
    let edits: Vec<_> = points.chain(lines).collect();
    Snapshot::empty(MinimumLength::new(1e-6).unwrap())
        .apply(&edits, || false)
        .unwrap()
}

fn recipe(snapshot: &Snapshot) -> WireRecipe {
    snapshot
        .wire_recipe(&uses(), WireClosure::Closed, 6, || false)
        .unwrap()
}

fn options() -> SpatialPrismOptions {
    SpatialPrismOptions {
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
        planar_work: PlanarProofWorkLimit::PairBisections(16),
        source_trace_limit: 1e-8,
    }
}

fn result(recipe: &WireRecipe, direction: [f64; 3]) -> SpatialPrismResult {
    extrude_wire(recipe, direction, options(), || false).unwrap()
}

#[test]
fn exact_oblique_wire_extrudes_both_signed_directions_with_complete_history() {
    let wire = recipe(&snapshot(POINTS));
    for (direction, sign, expected_volume) in [
        (DIRECTION, Ordering::Greater, 75.),
        (DIRECTION.map(|v| -v), Ordering::Less, 75.),
        ([2., -4., 7.], Ordering::Greater, 120.),
    ] {
        let prism = result(&wire, direction);
        assert_eq!(prism.anchors(), [pid(91), pid(5), pid(77)]);
        assert_eq!(prism.sweep_sign(), sign);
        assert!(prism.source_trace_bound() <= 1e-12);
        assert!(prism.achieved_tolerance().max() <= 1e-5);
        assert_eq!(prism.volume().evidence(), Evidence::Proven);
        assert!(
            prism.volume().encloses(expected_volume),
            "{:?}",
            prism.volume()
        );
        assert_eq!(prism.source_trace_limit(), options().source_trace_limit);
        assert_eq!(prism.resolution(), options().resolution);
        assert!(prism.findings().is_empty());
        assert_ordered_validation(&prism);
        let native = prism.export_brep(|| false).unwrap();
        assert_complete_history(&native, direction, expected_volume, &prism);
    }
}

fn assert_ordered_validation(prism: &SpatialPrismResult) {
    for stage in [
        ValidationStage::Topology,
        ValidationStage::Geometry,
        ValidationStage::SelfIntersection,
        ValidationStage::Tolerance,
    ] {
        assert_eq!(
            prism.validation().stage(stage).verdict,
            StageVerdict::Passed
        );
    }
    assert_eq!(
        prism.self_intersection_certificate(),
        SelfIntersectionEvidence::AffineSweepOfCertifiedRegion
    );
}

fn assert_complete_history(
    native: &DetachedSpatialPrism,
    direction: [f64; 3],
    expected_volume: f64,
    prism: &SpatialPrismResult,
) {
    assert_eq!(native.solid.shape_type().unwrap(), ShapeEnum::Solid);
    assert_eq!(native.images.len(), 26);
    for (kind, count) in [
        (ShapeEnum::Face, 6),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Vertex, 8),
    ] {
        let mut all = martensite_tkbrep::toptools::IndexedMapOfShape::default();
        topexp::map_shapes_of_type(&native.solid, kind, &mut all).unwrap();
        assert_eq!(all.extent(), count);
    }
    let mut measure = GProps::default();
    BRepGProp::volume_properties(&native.solid, &mut measure, true, false, false).unwrap();
    assert!((measure.mass() - expected_volume).abs() <= 1e-11);
    assert!(
        prism.volume().encloses(measure.mass()),
        "actual native mass {} outside {:?}",
        measure.mass(),
        prism.volume()
    );
    check_caps(native);
    for (index, key) in POINT_KEYS.into_iter().enumerate() {
        check_point_images(native, pid(key), POINTS[index], direction);
    }
    for (index, source) in uses().into_iter().enumerate() {
        check_line_images(native, source, index);
    }
}

fn image(native: &DetachedSpatialPrism, source: SpatialPrismSource, role: HistoryRole) -> &Shape {
    let images: Vec<_> = native
        .images
        .iter()
        .filter(|img| img.history.source == source && img.history.role == role)
        .collect();
    let [only] = images.as_slice() else {
        panic!("{source:?} {role:?}: expected exactly one image");
    };
    &only.shape
}

fn check_caps(native: &DetachedSpatialPrism) {
    for (cap_role, edge_role) in [
        (HistoryRole::StartCap, HistoryRole::StartEdge),
        (HistoryRole::EndCap, HistoryRole::EndEdge),
    ] {
        let cap = image(native, SpatialPrismSource::Boundary, cap_role);
        let mut measure = GProps::default();
        BRepGProp::surface_properties(cap, &mut measure, false, false).unwrap();
        assert!((measure.mass() - 15.).abs() <= 1e-12);

        for key in LINE_KEYS {
            let edge = image(native, SpatialPrismSource::Line(lid(key)), edge_role);
            assert!(contains(cap, ShapeEnum::Edge, edge));
        }
    }
}

fn check_point_images(
    native: &DetachedSpatialPrism,
    point: PointId,
    expected: [f64; 3],
    direction: [f64; 3],
) {
    let source = SpatialPrismSource::Point(point);
    let start = image(native, source, HistoryRole::StartVertex);
    let end = image(native, source, HistoryRole::EndVertex);
    let lateral = image(native, source, HistoryRole::LateralEdge);
    for (actual, expected) in [
        (start, expected),
        (end, std::array::from_fn(|i| expected[i] + direction[i])),
    ] {
        let actual_point = BRepTool::pnt(&topods::vertex(actual).unwrap()).unwrap();
        assert!(actual_point.distance(&pt(expected)) <= 1e-12);
        assert!(contains(lateral, ShapeEnum::Vertex, actual));
    }
}

fn check_line_images(native: &DetachedSpatialPrism, line: DirectedLineUse, order: usize) {
    let source = SpatialPrismSource::Line(line.line);
    for traced in native
        .images
        .iter()
        .filter(|img| img.history.source == source)
    {
        assert_eq!(traced.history.line_direction, Some(line.direction));
    }
    let face = image(native, source, HistoryRole::SideFace);
    for (edge_role, vertex_role) in [
        (HistoryRole::StartEdge, HistoryRole::StartVertex),
        (HistoryRole::EndEdge, HistoryRole::EndVertex),
    ] {
        let edge = image(native, source, edge_role);
        assert!(contains(face, ShapeEnum::Edge, edge));
        for index in [order, (order + 1) % POINT_KEYS.len()] {
            let point = image(
                native,
                SpatialPrismSource::Point(pid(POINT_KEYS[index])),
                vertex_role,
            );
            assert!(contains(edge, ShapeEnum::Vertex, point));
        }
    }
}

fn contains(parent: &Shape, kind: ShapeEnum, image: &Shape) -> bool {
    topexp::Explorer::new(parent, kind, ShapeEnum::Shape)
        .unwrap()
        .any(|found| found.is_same(image))
}

#[test]
fn tiny_signed_skew_precedes_projection_and_model_tolerances() {
    for (height, sign) in [
        (8.0_f64.next_up(), Ordering::Greater),
        (8.0_f64.next_down(), Ordering::Less),
    ] {
        let mut skew = POINTS;
        skew[3][2] = height;
        assert!(matches!(
            extrude_wire(&recipe(&snapshot(skew)), DIRECTION, options(), || false),
            Err(SpatialPrismFailure::Skew { anchors, point, sign: got })
                if anchors == [pid(91), pid(5), pid(77)] && point == pid(22) && sign == got
        ));
    }
}

#[test]
fn topology_trace_direction_and_tolerance_refusals_keep_their_causes() {
    let source = snapshot(POINTS);
    let open = source
        .wire_recipe(&uses()[..3], WireClosure::Open, 3, || false)
        .unwrap();
    assert!(matches!(
        extrude_wire(&open, DIRECTION, options(), || false),
        Err(SpatialPrismFailure::OpenWire)
    ));
    let wire = recipe(&source);
    assert!(matches!(
        extrude_wire(&wire, [0., 4., 3.], options(), || false),
        Err(SpatialPrismFailure::TangentDirection(_))
    ));
    let guarded = SpatialPrismOptions {
        source_trace_limit: 1e-18,
        ..options()
    };
    assert!(matches!(
        extrude_wire(&wire, DIRECTION, guarded, || false),
        Err(SpatialPrismFailure::SourceTrace { bound, limit, .. }) if bound > limit && limit == 1e-18
    ));
    let mut refused = options();
    refused.resolution.cap = ToleranceCap::Refuse(2e-8);
    assert!(matches!(
        extrude_wire(&wire, DIRECTION, refused, || false),
        Err(SpatialPrismFailure::Region(failure)) if matches!(failure.decline().cause, DeclineCause::ToleranceOverBudget { .. })
    ));
}

#[test]
fn cancellation_after_sweep_prevents_publication_and_preserves_source() {
    let wire = recipe(&snapshot(POINTS));
    let mut checkpoints = 0;
    let ready = extrude_wire(&wire, DIRECTION, options(), || {
        checkpoints += 1;
        false
    })
    .unwrap();
    let after_sweep = checkpoints - ready.history().len() - 1;
    assert!(matches!(
        extrude_wire(&wire, DIRECTION, options(), cancel_at(after_sweep)),
        Err(SpatialPrismFailure::Cancelled(SpatialPrismStage::Sweep))
    ));
    assert!(matches!(
        extrude_wire(&wire, DIRECTION, options(), cancel_at(checkpoints)),
        Err(SpatialPrismFailure::Cancelled(SpatialPrismStage::History))
    ));
    let rebuilt = result(&wire, DIRECTION);
    assert_eq!(
        rebuilt.history().collect::<Vec<_>>(),
        ready.history().collect::<Vec<_>>()
    );
    assert_eq!(
        archive(&rebuilt.export_brep(|| false).unwrap().solid),
        archive(&ready.export_brep(|| false).unwrap().solid)
    );
}

fn cancel_at(target: usize) -> impl FnMut() -> bool {
    let mut count = 0;
    move || {
        count += 1;
        count == target
    }
}

#[test]
fn edited_source_and_detached_native_mutation_preserve_original_prism() {
    let source = snapshot(POINTS);
    let prism = result(&recipe(&source), DIRECTION);
    let original = prism.export_brep(|| false).unwrap();
    let sibling = prism.export_brep(|| false).unwrap();
    let expected = archive(&original.solid);
    assert!(!original.solid.is_same(&sibling.solid));
    mutate_cap(&original);
    assert_ne!(archive(&original.solid), expected);
    assert_eq!(archive(&sibling.solid), expected);
    mutate(&original);
    assert_ne!(archive(&original.solid), expected);
    assert_eq!(archive(&sibling.solid), expected);
    assert_eq!(
        archive(&prism.export_brep(|| false).unwrap().solid),
        expected
    );
    let edits = [1, 2].map(|index| {
        let [_, y, z] = POINTS[index];
        Edit::MovePoint {
            point: pid(POINT_KEYS[index]),
            position: pt([6., y, z]),
        }
    });
    let edited = source.apply(&edits, || false).unwrap();
    let rebuilt = result(&recipe(&edited), DIRECTION);
    assert!(rebuilt.volume().encloses(100.), "{:?}", rebuilt.volume());
    assert_eq!(
        rebuilt.history().collect::<Vec<_>>(),
        prism.history().collect::<Vec<_>>()
    );
}

fn mutate_cap(native: &DetachedSpatialPrism) {
    let cap = topods::face(image(
        native,
        SpatialPrismSource::Boundary,
        HistoryRole::StartCap,
    ))
    .unwrap();
    let (surface, _) = BRepTool::surface_with_location(&cap).unwrap();
    surface
        .downcast_ref::<GeomPlane>()
        .unwrap()
        .set_pln(Plane3::new(pt([70.; 3]), Direction3::Z).unwrap());
}

fn mutate(native: &DetachedSpatialPrism) {
    let vertex = topods::vertex(image(
        native,
        SpatialPrismSource::Point(pid(91)),
        HistoryRole::StartVertex,
    ))
    .unwrap();
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&vertex, pt([80.; 3]), 1e-7)
        .unwrap();
    let edge = topods::edge(image(
        native,
        SpatialPrismSource::Line(lid(40)),
        HistoryRole::StartEdge,
    ))
    .unwrap();
    let curve = BRepTool::curve_with_location(&edge).unwrap();
    curve
        .curve
        .downcast_ref::<GeomLine>()
        .unwrap()
        .set_location(pt([82.; 3]));
}

fn archive(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    let options = bintools::WriteOptions {
        with_triangles: false,
        with_normals: false,
        ..Default::default()
    };
    bintools::write(shape, &mut stream, options, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; usize::try_from(stream.tellp()).unwrap()];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

#[path = "spatial_prism/retained.rs"]
mod retained;

#[path = "spatial_prism/replay.rs"]
mod replay;
