// SPDX-License-Identifier: GPL-2.0-only
//! A real cylindrical solid partitioned into a holed wall occurrence and its neighbouring patch.
use super::*;
use martensite_tkbrep::topods::{Edge, Vertex, Wire};
use martensite_tkmath::gp::{Axis1, Circle3, Direction2, Line2, Line3, Point2, StandardDirection2};
use martensite_tkprim::brep_prim::BRepPrimBuilder;
use std::f64::consts::PI;

const AXIAL: [f64; 2] = [0.5, 1.5];

fn point(u: f64, v: f64) -> Point3 {
    Point3::new(u.cos(), u.sin(), v)
}

fn vertex(builder: BRepPrimBuilder, [u, v]: [f64; 2]) -> Vertex {
    let mut vertex = Vertex::default();
    builder.make_vertex(&mut vertex, point(u, v)).unwrap();
    vertex
}

fn generator(builder: BRepPrimBuilder, face: &Face, u: f64) -> Edge {
    let mut edge = Edge::default();
    let z = Direction3::from_standard(StandardDirection3::Z);
    let v = Direction2::from_standard(StandardDirection2::Y);
    builder
        .make_edge_line(&mut edge, Line3::from_axis(Axis1::new(point(u, 0.), z)))
        .unwrap();
    builder
        .set_pcurve_line(&edge, face, Line2::new(Point2::new(u, 0.), v))
        .unwrap();
    edge
}

fn arc(builder: BRepPrimBuilder, face: &Face, v: f64) -> Edge {
    let mut edge = Edge::default();
    let z = StandardDirection3::Z;
    let x = StandardDirection3::X;
    let axes = Axis2::with_standard_frame(Point3::new(0., 0., v), z, x);
    let forward = Direction2::from_standard(StandardDirection2::X);
    builder
        .make_edge_circle(&mut edge, Circle3::new(axes, 1.).unwrap())
        .unwrap();
    builder
        .set_pcurve_line(&edge, face, Line2::new(Point2::new(0., v), forward))
        .unwrap();
    edge
}

fn install_side(
    builder: BRepPrimBuilder,
    wire: &mut Wire,
    mut edge: Edge,
    [first, last]: [&Vertex; 2],
    [low, high]: [f64; 2],
    forward: bool,
) {
    builder
        .add_edge_vertex(&mut edge, first, low, true)
        .unwrap();
    builder
        .add_edge_vertex(&mut edge, last, high, false)
        .unwrap();
    builder.builder().range(&edge, low, high, false).unwrap();
    builder.add_wire_edge(wire, &edge, forward).unwrap();
}

fn rectangle(face: &Face, angular: [f64; 2], axial: [f64; 2]) -> Wire {
    let builder = BRepPrimBuilder::new();
    let [u0, u1] = angular;
    let [v0, v1] = axial;
    let [a, b, c, d] = [[u0, v0], [u0, v1], [u1, v1], [u1, v0]].map(|uv| vertex(builder, uv));
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    install_side(
        builder,
        &mut wire,
        generator(builder, face, u0),
        [&a, &b],
        axial,
        true,
    );
    install_side(
        builder,
        &mut wire,
        arc(builder, face, v1),
        [&b, &c],
        angular,
        true,
    );
    install_side(
        builder,
        &mut wire,
        generator(builder, face, u1),
        [&d, &c],
        axial,
        false,
    );
    install_side(
        builder,
        &mut wire,
        arc(builder, face, v0),
        [&a, &d],
        angular,
        false,
    );
    builder.complete_wire(&wire).unwrap();
    wire
}

fn partitioned_native(angular: [f64; 2], axial: [f64; 2]) -> (Solid, Face) {
    let (body, mut face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let hole = rectangle(&face, angular, axial);
    let builder = BRepPrimBuilder::new();
    face.set_free(true).unwrap();
    builder.add_face_wire(&mut face, &hole).unwrap();
    face.set_free(false).unwrap();
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let mut patch = Face::default();
    builder
        .builder()
        .make_face_on_surface(&mut patch, &surface, 1e-7)
        .unwrap();
    let patch_wire = topods::wire(&hole.reversed()).unwrap();
    builder.add_face_wire(&mut patch, &patch_wire).unwrap();
    let shells = Explorer::new(&body, ShapeEnum::Shell, ShapeEnum::Shape).unwrap();
    let mut shell = topods::shell(shells.current()).unwrap();
    shell.set_free(true).unwrap();
    builder.add_shell_face(&mut shell, &patch).unwrap();
    shell.set_free(false).unwrap();
    assert!(BRepTool::is_closed(&shell).unwrap());
    (body, face)
}

fn diameter(z: f64) -> Snapshot {
    source([[0., -3., z], [0., 3., z]])
}

fn trimmed(angular: [f64; 2], axial: [f64; 2]) -> CylindricalSplitDomain {
    let (body, face) = partitioned_native(angular, axial);
    capture(&body, &face)
}

#[test]
fn keeps_both_cuts_or_only_the_author_ordered_retained_root() {
    let small = trimmed([0.5, 2.5], AXIAL);
    let two = split(&diameter(0.25), &small);
    assert_eq!(two.cuts.len(), 2);
    assert_eq!(two.cuts[0].position.position, [0., -1., 0.25]);
    assert_eq!(two.cuts[1].position.position, [0., 1., 0.25]);
    assert_eq!(two.fragments.len(), 3);
    let one = split(&diameter(1.), &small);
    let Hit::Isolated(contacts) = &one.report.contact else {
        panic!("expected both source roots");
    };
    assert_eq!(contacts[0].wall, WallPlace::Interior);
    assert_eq!(contacts[1].wall, WallPlace::HoleInterior);
    assert_eq!(one.cuts.len(), 1);
    assert_eq!(one.cuts[0].root_ordinal, 0);
    assert_eq!(one.cuts[0].position.position, [0., -1., 1.]);
    assert_eq!(one.fragments.len(), 2);
    assert!(one.report.trim_trace.upper_bound < 1e-14);
    assert_eq!(
        one.fragments[0].source_parameters[1],
        one.fragments[1].source_parameters[0]
    );

    let reverse = split(&source([[0., 3., 1.], [0., -3., 1.]]), &small);
    assert_eq!(reverse.cuts[0].root_ordinal, 1);
    assert_eq!(reverse.cuts[0].position.position, [0., -1., 1.]);
}

#[test]
fn wide_hole_reports_both_trimmed_out_and_retains_both_closed_horizontal_rims() {
    let wall = trimmed([0.5, 5.5], AXIAL);
    let Outcome::Unchanged(report) = query(&diameter(1.), &wall).unwrap() else {
        panic!("expected both crossings in removed material");
    };
    let Hit::Isolated(contacts) = report.contact else {
        panic!("expected both support roots");
    };
    assert_eq!(contacts.len(), 2);
    assert!(
        contacts
            .iter()
            .all(|hit| hit.wall == WallPlace::HoleInterior && hit.cut.is_none())
    );
    for z in AXIAL {
        let edit = split(&diameter(z), &wall);
        assert_eq!(edit.cuts.len(), 2);
        assert!(
            edit.cuts
                .iter()
                .all(|cut| cut.location == WallPlace::HoleBoundary)
        );
        let below_or_above = if z == AXIAL[0] {
            z.next_down()
        } else {
            z.next_up()
        };
        let outside = split(&diameter(below_or_above), &wall);
        assert!(
            outside
                .cuts
                .iter()
                .all(|cut| cut.location == WallPlace::Interior)
        );
    }
    let no_support = source([[0., -3., 1.], [0., -2., 1.]]);
    let Outcome::Unchanged(empty) = query(&no_support, &wall).unwrap() else {
        panic!("expected true disjoint");
    };
    assert_eq!(empty.contact, Hit::Disjoint);
}

#[test]
fn literal_pi_endpoints_separate_the_exact_negative_x_ray() {
    let before = trimmed([PI, 5.5], AXIAL);
    let past = trimmed([PI.next_up(), 5.5], AXIAL);
    let input = source(DIA);
    let older = split(&input, &before);
    let newer = split(&input, &past);
    assert_eq!(older.cuts.len(), 1);
    assert_eq!(older.cuts[0].root_ordinal, 1);
    assert_eq!(newer.cuts.len(), 2);
    assert_eq!(newer.cuts[0].position.position, [-1., 0., 1.]);
}

#[test]
fn a_coincident_generator_can_be_empty_or_retain_only_existing_hole_rim_endpoints() {
    let domain = trimmed([0.5, 2.5], AXIAL);
    for ([first, last], endpoints) in [
        ([0.75, 1.25], [false, false]),
        ([0.5, 1.], [true, false]),
        ([1.5, 0.5], [true, true]),
    ] {
        let input = source([[0., 1., first], [0., 1., last]]);
        let Outcome::Unchanged(report) = query(&input, &domain).unwrap() else {
            panic!("a coincident generator must keep its authored endpoints");
        };
        assert_eq!(report.contact, Hit::GeneratorHole { endpoints });
    }
    let retained = source([[0., 1., 0.25], [0., 1., 1.]]);
    assert_eq!(
        query(&retained, &domain),
        Err(Failure::Overlap {
            line: lid(3),
            face: STAMP
        })
    );
}

#[test]
fn captured_hole_replays_and_binds_its_actual_angular_boundary() {
    let (body, face) = partitioned_native([0.5, 2.5], AXIAL);
    let captured = capture(&body, &face);
    let source = diameter(1.);
    let expected = query(&source, &captured).unwrap();
    let generator = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|edge| topods::edge(&edge).unwrap())
        .find_map(|edge| {
            let trace = BRepTool::curve_on_face(&edge, &face).unwrap();
            (trace.first == 0.5 && trace.last == 1.5).then_some(trace.pcurve)
        })
        .unwrap();
    generator
        .get()
        .unwrap()
        .translate(martensite_tkmath::gp::Vector2::new(0.125, 0.))
        .unwrap();
    assert_eq!(query(&source, &captured).unwrap(), expected);
    assert!(matches!(
        capture_cylindrical_split_domain(
            &body,
            &face,
            STAMP,
            FaceCaptureLimits::bounded(),
            BOUND,
            || false
        ),
        Err(FaceError::UnsupportedTrim { .. })
    ));
}

#[test]
fn exact_axial_hole_membership_precedes_rounding_at_a_literal_one_third_rim() {
    let rem = 1_f64 / 3.;
    let inside = trimmed([0.5, 5.5], [rem, 1.25]);
    let outside = trimmed([0.5, 5.5], [rem.next_up(), 1.25]);
    let input = source([[0., -3., 0.], [0., 3., 1.]]);
    let Outcome::Unchanged(report) = query(&input, &inside).unwrap() else {
        panic!("the exact crossing lies higher than the literal lower rim");
    };
    let Hit::Isolated(contacts) = report.contact else {
        panic!("expected both support roots");
    };
    assert!(
        contacts
            .iter()
            .all(|hit| hit.wall == WallPlace::HoleInterior)
    );
    let edit = split(&input, &outside);
    assert_eq!(edit.cuts.len(), 1);
    assert_eq!(edit.cuts[0].position.position, [0., -1., rem]);
    assert_eq!(edit.cuts[0].location, WallPlace::Interior);
    assert!(edit.cuts[0].position.error_upper > 0.);
}

#[test]
fn native_hole_spatial_trace_is_bound_independently_of_its_exact_chart() {
    let (body, face) = partitioned_native([0.5, 2.5], AXIAL);
    let old = capture(&body, &face);
    let line = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|edge| topods::edge(&edge).unwrap())
        .find_map(|edge| {
            let (curve, first, last) = BRepTool::curve(&edge).unwrap();
            (first == 0.5 && last == 1.5).then_some(curve)
        })
        .unwrap();
    line.get()
        .unwrap()
        .translate(Vector3::new(0.125, 0., 0.))
        .unwrap();
    assert_eq!(split(&diameter(1.), &old).cuts.len(), 1);
    assert!(matches!(
        capture_cylindrical_split_domain(&body, &face, STAMP, FaceCaptureLimits::bounded(), BOUND, || false),
        Err(FaceError::TraceError { upper_bound, limit: BOUND }) if upper_bound >= 0.125
    ));
}

#[test]
fn frozen_holed_wall_has_equal_four_worker_replays() {
    let wall = trimmed([0.5, 2.5], AXIAL);
    let input = diameter(1.);
    let expected = query(&input, &wall).unwrap();
    std::thread::scope(|threads| {
        let tasks = (0..4).map(|_| {
            threads.spawn(|| {
                for _ in 0..5 {
                    assert_eq!(query(&input, &wall).unwrap(), expected);
                }
            })
        });
        for task in tasks.collect::<Vec<_>>() {
            task.join().unwrap();
        }
    });
}

#[test]
fn independent_processes_replay_identical_hole_cuts_and_trace() {
    let run = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "split_cylinder::hole::emit_hole_split",
                "--nocapture",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .find(|line| line.starts_with("hole-receipt "))
            .unwrap()
            .to_owned()
    };
    assert_eq!(run(), run());
}

#[test]
fn emit_hole_split() {
    let input = diameter(1.);
    let wall = trimmed([0.5, 2.5], AXIAL);
    let edit = split(&input, &wall);
    println!(
        "hole-receipt {:?} {:?} {:?}",
        edit.report, edit.cuts, edit.fragments
    );
}
