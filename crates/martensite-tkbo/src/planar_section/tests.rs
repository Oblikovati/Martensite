// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbrep::{
    brep::{BRepTVertex, BRepTool},
    topexp::Explorer,
    topods::{self, Edge, Face, Wire},
};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use martensite_tkg3d::{
    geom::GeomPlane,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Axis3, Direction3, Plane3, Point3, Quaternion, Transform3, TransformForm, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon};
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicUsize, Ordering};

const SETTINGS: PlanarSectionOptions = PlanarSectionOptions {
    behavior: PlanarSectionBehavior::ConvexLinearV1,
    max_position_error: 1e-7,
};
const EXPECTED: [[f64; 3]; 2] = [[0., -2., 5.], [0., 2., 5.]];
const FACE_ID: u64 = 5;
const TOOL_ID: u64 = 9;

fn stamp(occurrence: u64) -> PlanarSectionStamp {
    PlanarSectionStamp {
        document: *b"section-fixtures",
        occurrence: NonZeroU64::new(occurrence).unwrap(),
        revision: 41,
    }
}

fn setup() -> Face {
    BRepPrimAPIMakeBox::from_points(Point3::new(-3., -2., 1.), Point3::new(3., 2., 5.))
        .unwrap()
        .top_face()
        .unwrap()
}

fn work_plane(origin: [f64; 3], normal: Direction3) -> NamedSectionPlane {
    let [x, y, z] = origin;
    NamedSectionPlane {
        stamp: stamp(TOOL_ID),
        plane: Plane3::new(Point3::new(x, y, z), normal).unwrap(),
    }
}

fn x_zero() -> NamedSectionPlane {
    work_plane([0., 0., 0.], Direction3::X)
}

fn bound_edges(face: &Face) -> Vec<(SectionBoundaryId, Edge)> {
    Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .enumerate()
        .map(|(i, shape)| {
            (
                SectionBoundaryId(NonZeroU64::new(100 + i as u64).unwrap()),
                topods::edge(&shape).unwrap(),
            )
        })
        .collect()
}

fn bindings(edges: &[(SectionBoundaryId, Edge)]) -> Vec<SectionBoundaryBinding<'_>> {
    edges
        .iter()
        .map(|(id, edge)| SectionBoundaryBinding { id: *id, edge })
        .collect()
}

fn run(
    face: &Face,
    edges: &[(SectionBoundaryId, Edge)],
    tool: NamedSectionPlane,
    progress: &ProgressRange,
) -> Result<PlanarSectionResult, PlanarSectionFailure> {
    let boundary = bindings(edges);
    section_face_with_work_plane(
        PlanarSectionFace {
            stamp: stamp(FACE_ID),
            face,
            boundary: &boundary,
        },
        tool,
        SETTINGS,
        progress,
    )
}

fn cut(face: &Face, tool: NamedSectionPlane) -> Result<PlanarSectionResult, PlanarSectionFailure> {
    run(face, &bound_edges(face), tool, &ProgressRange::default())
}

fn segment(result: &PlanarSectionResult) -> &PlanarSectionSegment {
    let PlanarSectionOutcome::Segment(segment) = &result.outcome else {
        panic!("segment expected, got {:?}", result.outcome);
    };
    segment
}

#[test]
fn native_block_section_verifies_both_ancestors_and_stored_pcurves() {
    let face = setup();
    let raw_edges = bound_edges(&face);
    let result = run(&face, &raw_edges, x_zero(), &ProgressRange::default()).unwrap();
    let segment = segment(&result);
    assert_eq!(result.parents.face, stamp(FACE_ID));
    assert_eq!(result.parents.work_plane, stamp(TOOL_ID));
    assert_eq!(result.work.trim_uses, 4);
    assert_eq!(result.work.native_builds, 1);
    assert_eq!(segment.evidence.endpoints, EXPECTED);
    assert_eq!(
        segment.native_fuzzy_length,
        martensite_tkernel::precision::confusion()
    );
    assert_eq!(segment.evidence.native_parameters, [-1., 3.]);
    assert_eq!(segment.evidence.native_error_bound, 0.);
    assert_eq!(segment.evidence.position_error_bound, 0.);
    assert_eq!(segment.evidence.vertex_error_bound, 0.);
    assert_eq!(segment.evidence.stored_pcurve_error_bounds, [Some(0.); 2]);
    assert_eq!(
        segment.evidence.parent_uv,
        [
            [[[3., 3.], [0., 0.]], [[3., 3.], [4., 4.]]],
            [[[5., 5.], [2., 2.]], [[5., 5.], [-2., -2.]]]
        ]
    );
    for (index, origin) in segment.origin.iter().enumerate() {
        let SectionEndpointOrigin::Edge {
            boundary,
            parameter,
        } = origin
        else {
            panic!("trim-edge interior expected, got {origin:?}");
        };
        let (_, edge) = raw_edges.iter().find(|(id, _)| *id == boundary.id).unwrap();
        let (line, _, _) = BRepTool::curve(edge).unwrap();
        assert_eq!(*parameter, [3., 3.]);
        assert_eq!(
            *line
                .get()
                .unwrap()
                .eval_d0(parameter[0])
                .unwrap()
                .coordinates(),
            EXPECTED[index]
        );
    }
}

#[test]
fn completed_empty_and_degeneracies_keep_separate_typed_states() {
    let face = setup();
    let parallel = cut(&face, work_plane([0., 0., 0.], Direction3::Z)).unwrap();
    assert_eq!(
        parallel.outcome,
        PlanarSectionOutcome::Empty(PlanarSectionEmpty::ParallelDistinct)
    );
    let miss = cut(&face, work_plane([4., 0., 0.], Direction3::X)).unwrap();
    assert_eq!(
        miss.outcome,
        PlanarSectionOutcome::Empty(PlanarSectionEmpty::TrimMiss)
    );
    assert_eq!(
        [parallel.work.native_builds, miss.work.native_builds],
        [0, 0]
    );
    let overlap = cut(&face, work_plane([0., 0., 5.], Direction3::Z)).unwrap_err();
    assert!(matches!(
        overlap.cause,
        PlanarSectionCause::CoincidentSupports
    ));
    let border = cut(&face, work_plane([3., 0., 0.], Direction3::X)).unwrap_err();
    assert!(matches!(
        border.cause,
        PlanarSectionCause::BoundaryOverlap(_)
    ));
    let tangent = work_plane([3., 2., 5.], Direction3::new(1., 1., 0.).unwrap());
    let point = cut(&face, tangent).unwrap_err();
    assert!(
        matches!(point.cause, PlanarSectionCause::PointOnly(SectionEndpointOrigin::Vertex { incoming, outgoing }) if incoming.id != outgoing.id)
    );
}

fn malformed_axis_plane() -> Plane3 {
    let mut frame = Axis3::default();
    let mut position = 1;
    let invalid_axes = r#""Location": [0, 0, 6], "Direction": [1, 0, 0], "XDirection": [1, 0, 0], "YDirection": [0, 1, 0]"#;
    assert!(frame.init_from_json(invalid_axes, &mut position).unwrap());
    Plane3::from_position(frame)
}

#[test]
fn inconsistent_stored_support_frame_refuses_before_classifying_empty() {
    let face = setup();
    let malformed = malformed_axis_plane();
    assert_eq!(malformed.axis().direction(), Direction3::X);
    assert_eq!(malformed.x_axis().direction(), Direction3::X);
    let tool = NamedSectionPlane {
        plane: malformed,
        ..x_zero()
    };
    let wrong_tool = cut(&face, tool).unwrap_err();
    assert!(matches!(
        wrong_tool.cause,
        PlanarSectionCause::Certificate(PlanarSectionError::Frame { chart: 1, .. })
    ));

    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    surface
        .downcast_ref::<GeomPlane>()
        .unwrap()
        .set_pln(malformed);
    let wrong_face = cut(&face, x_zero()).unwrap_err();
    assert!(matches!(
        wrong_face.cause,
        PlanarSectionCause::Certificate(PlanarSectionError::Frame { chart: 0, .. })
    ));
}

#[test]
fn indirect_stored_tool_retains_its_handed_chart_and_directs_actual_section() {
    let face = setup();
    let mut tool = x_zero();
    tool.plane.u_reverse();
    assert!(!tool.plane.direct().unwrap());
    let result = cut(&face, tool).unwrap();
    let evidence = segment(&result).evidence;
    assert_eq!(evidence.endpoints, [EXPECTED[1], EXPECTED[0]]);
    assert_eq!(
        evidence.parent_uv[1],
        [[[-5., -5.], [-2., -2.]], [[-5., -5.], [2., 2.]]]
    );
    assert_eq!(result.work.native_builds, 1);
}

#[test]
fn located_and_reversed_faces_preserve_direction_and_local_chart() {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(10., 20., 30.));
    let shift = Location::new(transform);
    let face = setup().moved(&shift, false).unwrap();
    let face = topods::face(&face).unwrap();
    let tool = work_plane([10., 0., 0.], Direction3::X);
    let direct = cut(&face, tool).unwrap();
    let reversed_face = Face::unchecked(face.oriented(Orientation::Reversed));
    let reversed = cut(&reversed_face, tool).unwrap();
    assert_eq!(
        segment(&direct).evidence.endpoints,
        [[10., 18., 35.], [10., 22., 35.]]
    );
    assert_eq!(
        segment(&reversed).evidence.endpoints,
        [[10., 22., 35.], [10., 18., 35.]]
    );
    let mut direct_uv = segment(&direct).evidence.parent_uv;
    for parent in &mut direct_uv {
        parent.reverse();
    }
    assert_eq!(segment(&reversed).evidence.parent_uv, direct_uv);
}

fn rotate_face(transform: Transform3) -> Face {
    let located = setup().moved(&Location::new(transform), false).unwrap();
    topods::face(&located).unwrap()
}

fn quarter_y() -> Transform3 {
    // This quaternion's squared norm is 2, giving an exact signed matrix.
    Transform3::from_quaternion(Quaternion::new(0., 1., 0., 1.))
}

#[test]
fn lying_unit_translation_form_cannot_classify_a_transverse_section_as_empty() {
    let baseline = cut(&setup(), x_zero()).unwrap();
    assert_eq!(segment(&baseline).evidence.endpoints, EXPECTED);
    let mut malformed = quarter_y();
    assert_eq!(malformed.value(1, 3).unwrap(), 1.);
    malformed.set_form(TransformForm::Translation);
    let face = rotate_face(malformed);
    let effective = BRepTool::surface(&face).unwrap();
    let actual = effective.downcast_ref::<GeomPlane>().unwrap().pln();
    assert_eq!(actual.axis().direction(), Direction3::Z);
    assert_eq!(actual.location().z(), 5.);
    let failure = cut(&face, x_zero()).unwrap_err();
    assert_eq!(failure.parents.face, stamp(FACE_ID));
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::Placement(martensite_tkmath::gp::UnitPlacementError::Shortcut(
            TransformForm::Translation
        ))
    ));
}

#[test]
fn composed_valid_map_cannot_hide_a_contradictory_source_datum() {
    let rotation = quarter_y();
    let mut bad = rotation;
    bad.set_form(TransformForm::Translation);
    let placement = Location::new(rotation)
        .multiplied(&Location::new(bad))
        .unwrap();
    assert_eq!(
        placement
            .transformation()
            .checked_unit_placement_rows()
            .unwrap(),
        rotation.checked_unit_placement_rows().unwrap()
    );
    let located = setup().moved(&placement, false).unwrap();
    let face = topods::face(&located).unwrap();
    let tool = work_plane([0., 0., 0.], Direction3::Z);
    let failure = cut(&face, tool).unwrap_err();
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::Placement(martensite_tkmath::gp::UnitPlacementError::Shortcut(
            TransformForm::Translation
        ))
    ));
}

#[test]
fn ordinary_quarter_rotation_retains_the_source_chart() {
    let face = rotate_face(quarter_y());
    let tool = work_plane([0., 0., 0.], Direction3::Z);
    let result = cut(&face, tool).unwrap();
    let observed = &segment(&result).evidence;
    assert_eq!(observed.endpoints, [[5., 2., 0.], [5., -2., 0.]]);
    assert_eq!(
        observed.parent_uv[0],
        [[[3., 3.], [4., 4.]], [[3., 3.], [0., 0.]]]
    );
}

#[test]
fn boundary_binding_cannot_be_recovered_from_input_order_or_value_proximity() {
    let face = setup();
    let mut expected = bound_edges(&face);
    let progress = ProgressRange::default();
    let first = run(&face, &expected, x_zero(), &progress).unwrap();
    expected.reverse();
    assert_eq!(run(&face, &expected, x_zero(), &progress).unwrap(), first);
    expected[1].0 = expected[0].0;
    let failure = run(&face, &expected, x_zero(), &progress).unwrap_err();
    assert!(
        matches!(failure.cause, PlanarSectionCause::BoundaryBinding(id) if id == expected[0].0)
    );
    let unrelated = bound_edges(&setup());
    assert!(run(&face, &unrelated, x_zero(), &progress).is_err());
}

#[test]
fn successful_output_is_isolated_retryable_and_concurrent() {
    fn shared<T: Send + Sync>() {}
    shared::<PlanarSectionResult>();
    shared::<PlanarSectionFailure>();
    let face = setup();
    let result = cut(&face, x_zero()).unwrap();
    std::thread::scope(|scope| {
        let threads = [(); 4].map(|()| scope.spawn(|| cut(&face, x_zero()).unwrap()));
        for thread in threads {
            assert_eq!(thread.join().unwrap(), result);
        }
    });
    let edge = &bound_edges(&face)[0].1;
    let [vertex, _] = topology::directed_vertices(edge).unwrap();
    let storage = vertex.tshape().downcast::<BRepTVertex>();
    storage.get().unwrap().set_pnt(Point3::new(90., 60., 55.));
    assert_eq!(segment(&result).evidence.endpoints, EXPECTED);
    assert!(matches!(
        cut(&face, x_zero()).unwrap_err().cause,
        PlanarSectionCause::BoundarySource { cause, .. } if matches!(cause.as_ref(), PlanarSectionCause::Certificate(PlanarSectionError::InconsistentBorder(_)))
    ));
}

fn xy_wire(points: &[[f64; 2]]) -> Wire {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for &[x, y] in points {
        polygon.add_point(Point3::new(x, y, 5.)).unwrap();
    }
    polygon.close().unwrap();
    polygon.wire().unwrap()
}

fn xy_face(points: &[[f64; 2]]) -> BRepBuilderAPIMakeFace {
    let plane = Plane3::new(Point3::new(0., 0., 5.), Direction3::Z).unwrap();
    BRepBuilderAPIMakeFace::from_plane_wire(plane, &xy_wire(points), true).unwrap()
}

#[test]
fn straight_native_wire_admits_exact_planar_projection_when_pcurves_are_absent() {
    let face = xy_face(&[[-3., -2.], [3., -2.], [3., 2.], [-3., 2.]])
        .face()
        .unwrap();
    let edges = bound_edges(&face);
    assert!(
        edges
            .iter()
            .all(|(_, edge)| !BRepTool::curve_on_face(edge, &face).unwrap().is_stored)
    );
    let result = run(&face, &edges, x_zero(), &ProgressRange::default()).unwrap();
    assert_eq!(segment(&result).evidence.endpoints, EXPECTED);
}

#[test]
fn unsupported_trim_shapes_refuse_before_publishing_empty_or_partial_output() {
    let concave = xy_face(&[
        [-3., -2.],
        [3., -2.],
        [3., 0.],
        [0., 0.],
        [0., 2.],
        [-3., 2.],
    ])
    .face()
    .unwrap();
    let missed = work_plane([4., 0., 0.], Direction3::X);
    let failure = cut(&concave, missed).unwrap_err();
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::Certificate(PlanarSectionError::NonConvexTrim)
    ));

    let mut holed = xy_face(&[[-3., -2.], [3., -2.], [3., 2.], [-3., 2.]]);
    holed
        .add(&xy_wire(&[[-1., -1.], [-1., 1.], [1., 1.], [1., -1.]]))
        .unwrap();
    let failure = cut(&holed.face().unwrap(), x_zero()).unwrap_err();
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::TopologyCount {
            child: ShapeEnum::Wire,
            actual: 2,
            ..
        }
    ));
}

#[test]
fn actual_native_oblique_cuts_preserve_edge_and_vertex_lineage() {
    let diagonal = work_plane([0., 0., 5.], Direction3::new(3., 1., 0.).unwrap());
    let cut_rectangle = cut(&setup(), diagonal).unwrap();
    let rectangle = segment(&cut_rectangle);
    let expected = [[2. / 3., -2., 5.], [-2. / 3., 2., 5.]];
    for (actual, expected) in rectangle.evidence.endpoints.into_iter().zip(expected) {
        for (coordinate, reference) in actual.into_iter().zip(expected) {
            assert!((coordinate - reference).abs() <= 1e-13);
        }
    }
    assert!(
        rectangle
            .origin
            .iter()
            .all(|origin| matches!(origin, SectionEndpointOrigin::Edge { .. }))
    );
    assert!(rectangle.evidence.position_error_bound <= SETTINGS.max_position_error);

    let face = xy_face(&[[-2., -2.], [2., -2.], [2., 2.], [-2., 2.]])
        .face()
        .unwrap();
    let diagonal = work_plane([0., 0., 5.], Direction3::new(1., 1., 0.).unwrap());
    let cut_square = cut(&face, diagonal).unwrap();
    let square = segment(&cut_square);
    let exact_endpoints = [[2., -2., 5.], [-2., 2., 5.]];
    let upper = square.evidence.position_error_bound;
    assert!(upper <= 1e-14);
    for (stored, exact) in square.evidence.endpoints.into_iter().zip(exact_endpoints) {
        for (component, reference) in stored.into_iter().zip(exact) {
            assert!((component - reference).abs() <= upper);
        }
    }
    assert_eq!(
        square.evidence.parent_uv[0],
        [[[2., 2.], [-2., -2.]], [[-2., -2.], [2., 2.]]]
    );
    assert!(
        square
            .origin
            .iter()
            .all(|origin| matches!(origin, SectionEndpointOrigin::Vertex { .. }))
    );
}

#[test]
fn raw_source_tolerance_and_requested_spatial_allowance_remain_separate() {
    let face = setup();
    let edges = bound_edges(&face);
    let [vertex, _] = topology::directed_vertices(&edges[0].1).unwrap();
    let storage = vertex.tshape().downcast::<BRepTVertex>();
    storage.get().unwrap().set_tolerance(-2.);
    let failure = run(&face, &edges, x_zero(), &ProgressRange::default()).unwrap_err();
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::SourceTolerance { actual: -2., .. }
    ));
}

struct CancelAfter {
    polls: AtomicUsize,
    limit: usize,
}

impl ProgressIndicatorHooks for CancelAfter {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
    }
}

#[test]
fn cancellation_and_typed_source_refusal_publish_no_partial_geometry() {
    let face = setup();
    let edges = bound_edges(&face);
    for limit in [0, 10, 50] {
        let indicator = ProgressIndicator::new(Box::new(CancelAfter {
            polls: AtomicUsize::new(0),
            limit,
        }));
        let cancelled = run(&face, &edges, x_zero(), &indicator.start()).unwrap_err();
        assert!(matches!(cancelled.cause, PlanarSectionCause::Cancelled));
    }
    assert_eq!(
        segment(&cut(&face, x_zero()).unwrap()).evidence.endpoints,
        EXPECTED
    );

    let counterfeit = Face::unchecked(edges[0].1.oriented(Orientation::Forward));
    let invalid = run(&counterfeit, &edges, x_zero(), &ProgressRange::default()).unwrap_err();
    assert!(matches!(
        invalid.cause,
        PlanarSectionCause::ShapeKind {
            actual: ShapeEnum::Edge,
            expected: ShapeEnum::Face
        }
    ));
}

#[test]
#[ignore = "explicit local Section latency and deterministic receipt"]
fn measured_block_section_replay() {
    use std::{hint::black_box, time::Instant};
    let face = setup();
    let edges = bound_edges(&face);
    let tool = x_zero();
    let progress = ProgressRange::default();
    let expected = run(&face, &edges, tool, &progress).unwrap();
    for _ in 0..12 {
        black_box(run(&face, &edges, tool, &progress).unwrap());
    }
    let mut samples_us = [0_u128; 100];
    for elapsed in &mut samples_us {
        let start = Instant::now();
        let result = black_box(run(&face, &edges, tool, &progress).unwrap());
        *elapsed = start.elapsed().as_micros();
        assert_eq!(result, expected);
    }
    samples_us.sort_unstable();
    println!(
        "{{\"section_samples\":100,\"p50_us\":{},\"p95_us\":{},\"max_us\":{}}}",
        samples_us[49], samples_us[94], samples_us[99]
    );
    println!("section_receipt={expected:?}");
}
