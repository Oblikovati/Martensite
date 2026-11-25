// SPDX-License-Identifier: GPL-2.0-only
//! Owned actual native cylindrical projection fixtures.
use martensite_tkbo::surface_projection::*;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Builder, Edge, Face},
};
use martensite_tkernel::message::{
    ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCylindricalSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::{
    gp::{Axis3, Cylinder3, Direction3, Line3, Point3, Transform3, Vector3},
    predicates::roots::metric_bounds::cylindrical_projection::{
        CylinderProjectionError, NativeProjectionCurve, ProjectionSourceInterval,
    },
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
use std::{
    f64::consts::{FRAC_PI_2, TAU},
    num::NonZeroU64,
};

const OPTIONS: SurfaceProjectionOptions = SurfaceProjectionOptions {
    behavior: SurfaceProjectionBehavior::AxialCardinalV1,
    limits: ProjectionProofLimits {
        spatial: 1e-7,
        source_parameter: 1e-9,
        angle: 1e-9,
    },
};

fn source(x: f64, y: f64) -> Edge {
    let line = Line3::new(Point3::new(x, y, 0.), Direction3::Z);
    BRepBuilderAPIMakeEdge::from_lin_params(line, 1., 3.)
        .unwrap()
        .edge()
        .unwrap()
}

fn target(radial: Direction3, hole: bool) -> Face {
    let chart = Axis3::new(Point3::default(), Direction3::Z, radial).unwrap();
    let cylinder = Cylinder3::new(chart, 4.).unwrap();
    let mut face = BRepBuilderAPIMakeFace::from_cylinder_bounds(cylinder, (0., TAU, 0., 6.))
        .unwrap()
        .face()
        .unwrap();
    if hole {
        let patch = BRepBuilderAPIMakeFace::from_cylinder_bounds(
            cylinder,
            (FRAC_PI_2, 3. * FRAC_PI_2, 1.5, 2.5),
        )
        .unwrap()
        .face()
        .unwrap();
        let mut wires = Explorer::new(&patch, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
        let wire = Iterator::next(&mut wires).unwrap();
        for edge in Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            let edge = topods::edge(&edge).unwrap();
            let pcurve = BRepTool::curve_on_face(&edge, &patch).unwrap();
            assert!(pcurve.is_stored);
            let builder = BRepBuilder::default();
            builder
                .update_edge_pcurve_on_face(&edge, &pcurve.pcurve, &face, 0.)
                .unwrap();
            builder
                .range_on_face(&edge, &face, pcurve.first, pcurve.last)
                .unwrap();
        }
        Builder.add(&mut face, &wire.reversed()).unwrap();
    }
    face
}

#[test]
fn actual_located_reversed_occurrences_preserve_original_parametric_direction() {
    use martensite_tkg3d::top_abs::Orientation;
    use martensite_tkmath::toploc::Location;
    let pose = Location::new(Transform3::translation_vector(Vector3::new(10., 12., 20.)));
    let source = source(2., 0.).moved(&pose, false).unwrap();
    let source = topods::edge(&source.reversed()).unwrap();
    let face = target(Direction3::X, false).moved(&pose, false).unwrap();
    let face = topods::face(&face.reversed()).unwrap();
    let result = project(
        &source,
        &face,
        AxialProjectionMode::Forward(Direction3::X.reversed()),
    )
    .unwrap();
    let geometry = checked(&result);
    assert_eq!(geometry.source_orientation, Orientation::Reversed);
    assert_eq!(geometry.target_orientation, Orientation::Reversed);
    let edge = &geometry.edges[0];
    assert_eq!(edge.certificate.retained, interval(1., 3.));
    assert_eq!(edge.certificate.signed_travel, [6.; 2]);
    let NativeProjectionCurve::DegreeOne { poles, .. } = &edge.native.spatial else {
        panic!("expected actual translated native degree-one Section");
    };
    assert!(poles.iter().all(|p| p[0] == 6. && p[1] == 12.));
}

fn stamp(id: u64) -> SurfaceProjectionStamp {
    SurfaceProjectionStamp {
        document: *b"owned-projection",
        occurrence: NonZeroU64::new(id).unwrap(),
        revision: 9,
    }
}

fn boundary(face: &Face) -> Vec<(ProjectionBoundaryId, Edge)> {
    let mut edges: Vec<(ProjectionBoundaryId, Edge)> = Vec::new();
    for raw in Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
        if !edges.iter().any(|(_, edge)| edge.is_same(&raw)) {
            let id = ProjectionBoundaryId(NonZeroU64::new(100 + edges.len() as u64).unwrap());
            edges.push((id, topods::edge(&raw).unwrap()));
        }
    }
    edges
}

fn run(
    source: &Edge,
    target: &Face,
    target_edges: &[(ProjectionBoundaryId, Edge)],
    mode: AxialProjectionMode,
    progress: &ProgressRange,
) -> Result<SurfaceProjectionResult, SurfaceProjectionFailure> {
    let bindings: Vec<_> = target_edges
        .iter()
        .map(|(id, edge)| ProjectionBoundaryBinding { id: *id, edge })
        .collect();
    project_axial_line_to_cylinder(
        SurfaceProjectionSource {
            stamp: stamp(1),
            edge: source,
        },
        SurfaceProjectionTarget {
            stamp: stamp(2),
            face: target,
            boundary: &bindings,
        },
        mode,
        OPTIONS,
        progress,
    )
}

fn project(
    source: &Edge,
    target: &Face,
    mode: AxialProjectionMode,
) -> Result<SurfaceProjectionResult, SurfaceProjectionFailure> {
    run(
        source,
        target,
        &boundary(target),
        mode,
        &ProgressRange::default(),
    )
}

fn checked(result: &SurfaceProjectionResult) -> &SurfaceProjectionGeometry {
    assert_eq!(
        result.parents,
        SurfaceProjectionParents {
            source: stamp(1),
            target: stamp(2)
        }
    );
    let SurfaceProjectionOutcome::Projected(geometry) = &result.outcome else {
        panic!("expected native projection");
    };
    geometry
}

fn interval(a: f64, b: f64) -> ProjectionSourceInterval {
    ProjectionSourceInterval {
        endpoints: [[a; 2], [b; 2]],
        included: [true; 2],
    }
}

fn assert_generator(
    edge: &SurfaceProjectionEdge,
    x: f64,
    travel: f64,
    region: ProjectionSourceInterval,
) {
    let assert_radial = |[actual_x, y, _]: [f64; 3]| {
        assert!((actual_x - x).abs() <= 1e-13 && y.abs() <= 1e-13);
    };
    match &edge.native.spatial {
        NativeProjectionCurve::Line { origin, direction } => {
            assert_radial(*origin);
            assert_eq!([direction[0], direction[1]], [0.; 2]);
        }
        NativeProjectionCurve::DegreeOne { poles, .. } => {
            assert!(poles.len() > 4);
            poles.iter().copied().for_each(assert_radial);
        }
    }
    let proof = &edge.certificate;
    assert_eq!(proof.signed_travel, [travel; 2]);
    assert_eq!(proof.retained, region);
    assert_eq!(proof.selected[0].source, region);
    assert!(proof.spatial_error < 1e-13);
    assert!(proof.source_parameter_error < 1e-13);
    assert!(proof.angle_error < 1e-13);
}

#[test]
fn actual_forward_two_sided_closest_and_normal_own_the_native_roots() {
    use AxialProjectionMode::{Closest, Forward, Normal, TwoSided};
    let source = source(2., 0.);
    let direction = Direction3::X.reversed();
    for radial in [Direction3::X, Direction3::Y] {
        let face = target(radial, false);
        let forward = project(&source, &face, Forward(direction)).unwrap();
        let mut branches = &checked(&forward).edges;
        assert_eq!(branches.len(), 1);
        assert_generator(&branches[0], -4., 6., interval(1., 3.));
        let both = project(&source, &face, TwoSided(direction)).unwrap();
        branches = &checked(&both).edges;
        assert_generator(&branches[0], 4., -2., interval(1., 3.));
        assert_eq!(branches[0].seam.is_some(), radial == Direction3::X);
        assert_eq!(checked(&both).checked_edges, 2);
        let closest = project(&source, &face, Closest).unwrap();
        assert_generator(&checked(&closest).edges[0], 4., 2., interval(1., 3.));
        let normal = project(&source, &face, Normal).unwrap();
        let branches = &checked(&normal).edges;
        let [near, far] = if radial == Direction3::X {
            [&branches[0], &branches[1]]
        } else {
            [&branches[1], &branches[0]]
        };
        assert_generator(near, 4., 2., interval(1., 3.));
        assert_generator(far, -4., -6., interval(1., 3.));
    }
}

#[test]
fn actual_far_hole_qualifies_both_remaining_source_intervals_and_rims() {
    let face = target(Direction3::X, true);
    let bound = boundary(&face);
    let source = source(2., 0.);
    let mode = AxialProjectionMode::Forward(Direction3::X.reversed());
    let result = run(&source, &face, &bound, mode, &ProgressRange::default()).unwrap();
    let [before, after] = checked(&result).edges.as_slice() else {
        panic!("two actual clipped edges");
    };
    assert_generator(before, -4., 6., interval(1., 1.5));
    assert_generator(after, -4., 6., interval(2.5, 3.));
    assert_hole_rim(&bound, before.endpoints[1], 1.5);
    assert_hole_rim(&bound, after.endpoints[0], 2.5);
    let closest = project(&source, &face, AxialProjectionMode::Closest).unwrap();
    assert_generator(&checked(&closest).edges[0], 4., 2., interval(1., 3.));
}

fn assert_hole_rim(
    boundary: &[(ProjectionBoundaryId, Edge)],
    endpoint: ProjectionEndpointLineage,
    z: f64,
) {
    let ProjectionEndpointLineage::Hole(boundary_use) = endpoint else {
        panic!("expected original hole trim");
    };
    let (_, edge) = boundary
        .iter()
        .find(|(id, _)| *id == boundary_use.id)
        .unwrap();
    let (circle, _, _) = BRepTool::curve(edge).unwrap();
    let circle = circle.downcast_ref::<GeomCircle>().unwrap().circ().unwrap();
    assert_eq!(circle.location().z(), z);
}

#[test]
fn near_hole_uses_open_nearest_interval_and_leaves_closest_explicitly_incomplete() {
    let face = target(Direction3::X.reversed(), true);
    let source = source(2., 0.);
    let closest = project(&source, &face, AxialProjectionMode::Closest).unwrap_err();
    assert!(matches!(
        closest.cause,
        SurfaceProjectionCause::Numeric(CylinderProjectionError::ClosestTrim)
    ));
    let nearest = project(
        &source,
        &face,
        AxialProjectionMode::TwoSided(Direction3::X.reversed()),
    )
    .unwrap();
    let edges = &checked(&nearest).edges;
    assert_eq!(edges.len(), 3);
    assert_eq!(
        edges[0].certificate.selected[0].source,
        ProjectionSourceInterval {
            endpoints: [[1.5; 2], [2.5; 2]],
            included: [false; 2],
        }
    );
    assert_eq!(
        edges[0].certificate.selected[0].section_parameters,
        [[1.5; 2], [2.5; 2]]
    );
    assert_generator(&edges[1], 4., -2., interval(1., 1.5));
    assert_generator(&edges[2], 4., -2., interval(2.5, 3.));
}

#[test]
fn completed_empty_tangency_axial_overlap_and_tie_are_typed_independently() {
    use AxialProjectionMode::{Forward, TwoSided};
    let face = target(Direction3::X, false);
    let none = project(&source(5., -1.), &face, Forward(Direction3::Y)).unwrap();
    assert_eq!(
        none.outcome,
        SurfaceProjectionOutcome::Empty(CylinderProjectionEmpty::NoSupportRoot)
    );
    let tangent = project(&source(4., -1.), &face, Forward(Direction3::Y)).unwrap_err();
    assert!(matches!(
        tangent.cause,
        SurfaceProjectionCause::Numeric(CylinderProjectionError::Tangent)
    ));
    let overlap = project(&source(4., 0.), &face, Forward(Direction3::Z)).unwrap_err();
    assert!(matches!(
        overlap.cause,
        SurfaceProjectionCause::Numeric(CylinderProjectionError::Overlap)
    ));
    let tie = project(&source(0., 0.), &face, TwoSided(Direction3::X)).unwrap_err();
    assert!(matches!(
        tie.cause,
        SurfaceProjectionCause::Numeric(CylinderProjectionError::RootTie)
    ));
}

struct Cancel;
impl ProgressIndicatorHooks for Cancel {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        true
    }
}

#[test]
fn isolation_and_independent_concurrent_reads_preserve_owned_results() {
    fn shared<T: Send + Sync>() {}
    shared::<SurfaceProjectionResult>();
    let face = target(Direction3::X, true);
    let source = source(2., 0.);
    let mode = AxialProjectionMode::Normal;
    let bound = boundary(&face);
    let indicator = ProgressIndicator::new(Box::new(Cancel));
    let canceled = run(&source, &face, &bound, mode, &indicator.start()).unwrap_err();
    assert!(matches!(canceled.cause, SurfaceProjectionCause::Cancelled));
    let result = project(&source, &face, mode).unwrap();
    std::thread::scope(|scope| {
        for worker in [(); 3].map(|()| scope.spawn(|| project(&source, &face, mode).unwrap())) {
            assert_eq!(worker.join().unwrap(), result);
        }
    });
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let cylinder = surface.downcast_ref::<GeomCylindricalSurface>().unwrap();
    let current = cylinder.cylinder().unwrap();
    cylinder.set_cylinder(Cylinder3::new(current.position(), 8.).unwrap());
    assert_eq!(checked(&result).edges.len(), 3);
    assert!(project(&source, &face, mode).is_err());
}

#[test]
#[ignore = "explicit local projection latency and deterministic receipt"]
fn measured_holed_normal_projection_replay() {
    use std::{hint::black_box, time::Instant};
    let face = target(Direction3::X, true);
    let source = source(2., 0.);
    let bound = boundary(&face);
    let mode = AxialProjectionMode::Normal;
    let progress = ProgressRange::default();
    let expected = run(&source, &face, &bound, mode, &progress).unwrap();
    for _ in 0..12 {
        black_box(run(&source, &face, &bound, mode, &progress).unwrap());
    }
    let mut timings = [0_u128; 100];
    for elapsed in &mut timings {
        let start = Instant::now();
        let result = black_box(run(&source, &face, &bound, mode, &progress).unwrap());
        *elapsed = start.elapsed().as_micros();
        assert_eq!(result, expected);
    }
    timings.sort_unstable();
    println!(
        "{{\"samples\":100,\"p50_us\":{},\"p95_us\":{},\"max_us\":{}}}",
        timings[49], timings[94], timings[99]
    );
    println!("projection_receipt={expected:?}");
}
