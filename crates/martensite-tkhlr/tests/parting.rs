// SPDX-License-Identifier: GPL-2.0-only
//! Public native parting query controls with independent analytic geometry.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{ShapeIterator, Solid},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhlr::parting::{
    self, PartingCarrier, PartingCurve, PartingEdgeKind, PartingFailureKind, PartingOptions,
    PartingOrigin, PartingPositionEvidence, PartingShell,
};
use martensite_tkmath::gp::{Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use std::{
    collections::BTreeSet,
    sync::{Barrier, atomic::AtomicBool},
};
#[path = "parting/fixtures.rs"]
mod fixtures;
#[path = "parting/refusals.rs"]
mod refusals;
#[path = "parting/transforms.rs"]
mod transforms;
use fixtures::{Model, SCOPE, block, cylinder};

const SIZE: [f64; 3] = [60., 40., 10.];

#[test]
fn closed_box_uses_both_rims_and_exact_skew_incidence() {
    let model = Model::new(block());
    let skew = Direction3::new(1., 1., 1.).unwrap();
    for (pull, axis) in [
        (Direction3::X, Some(0)),
        (Direction3::Y, Some(1)),
        (Direction3::Z, Some(2)),
        (skew, None),
    ] {
        let curves = selected(&model, pull, PartingOptions::default());
        let kind = if axis.is_some() {
            PartingEdgeKind::ZeroBandRim
        } else {
            PartingEdgeKind::Opposite
        };
        require_edge_parents(&model, &curves, kind);
        assert_eq!(curve_points(&curves), expected_box(axis));
        assert_eq!(curves.len(), if axis.is_some() { 8 } else { 6 });
    }
}

#[test]
fn cylinder_smooth_generators_and_axial_rims_match_closed_form() {
    let model = Model::new(cylinder(5., 10.));
    let smooth = selected(&model, Direction3::X, PartingOptions::default());
    assert_generators(&smooth);
    let axial = selected(&model, Direction3::Z, PartingOptions::default());
    require_edge_parents(&model, &axial, PartingEdgeKind::ZeroBandRim);
    let mut levels: Vec<_> = axial
        .iter()
        .map(|curve| {
            let PartingCarrier::Circle(circle) = curve.carrier() else {
                panic!("expected native circular rim");
            };
            assert_eq!(circle.radius(), 5.);
            let [first, last] = curve.range();
            assert_eq!(last - first, std::f64::consts::TAU);
            circle.location().z()
        })
        .collect();
    levels.sort_by(f64::total_cmp);
    assert_eq!(levels, [0., 10.]);
}

fn assert_generators(curves: &[PartingCurve]) {
    assert_eq!(curves.len(), 2);
    let mut levels = Vec::new();
    for curve in curves {
        let PartingOrigin::Smooth { face, uv_ends } = curve.origin() else {
            panic!("expected a smooth face trace");
        };
        assert_eq!(face.shell, PartingShell::Exterior);
        assert_eq!(uv_ends[0][0], uv_ends[1][0]);
        assert_eq!(abs_uv_height(uv_ends), [0., 10.]);
        assert!(
            matches!(curve.position_evidence(), PartingPositionEvidence::CylindricalGenerator {
            upper_bound: 0., source_tolerance,
        } if source_tolerance.is_finite() && source_tolerance >= 0.)
        );
        let mut ends = curve.endpoints().map(coords);
        ends.sort_by(|a, b| a[2].total_cmp(&b[2]));
        let y = ends[0][1].signum() * 5.;
        for (actual, expected_z) in ends.into_iter().zip([0., 10.]) {
            for (value, expected) in actual.into_iter().zip([0., y, expected_z]) {
                assert!(
                    (value - expected).abs() <= 2e-14,
                    "{value} versus {expected}"
                );
            }
        }
        levels.push(y);
    }
    levels.sort_by(f64::total_cmp);
    assert_eq!(levels, [-5., 5.]);
}

#[test]
fn exclusion_filters_participating_faces_and_allows_completed_empty() {
    let model = Model::new(block());
    let options = PartingOptions {
        exclude_straight: true,
        ..Default::default()
    };
    assert!(selected(&model, Direction3::Z, options).is_empty());
    let cylinder = Model::new(cylinder(5., 10.));
    let generators = selected(&cylinder, Direction3::X, options);
    assert_generators(&generators);
    let PartingOrigin::Smooth { face, .. } = generators[0].origin() else {
        panic!("expected smooth face");
    };
    let options = PartingOptions {
        exclude_faces: &[face.id],
        ..Default::default()
    };
    assert!(selected(&cylinder, Direction3::X, options).is_empty());
    let original = selected(&model, Direction3::Z, PartingOptions::default());
    let PartingOrigin::Shared { faces, .. } = original[0].origin() else {
        panic!("expected native edge");
    };
    let excluded = faces[0].face.id;
    let expected: Vec<_> = original
        .iter()
        .filter(|curve| {
            let PartingOrigin::Shared { faces, .. } = curve.origin() else {
                panic!("expected native edge");
            };
            faces.iter().all(|side| side.face.id != excluded)
        })
        .map(signature)
        .collect();
    let options = PartingOptions {
        exclude_faces: &[excluded],
        ..Default::default()
    };
    let actual = selected(&model, Direction3::Z, options);
    assert_eq!(actual.iter().map(signature).collect::<Vec<_>>(), expected);
    assert!(actual.len() < original.len());
}

#[test]
fn internal_exclusion_removes_only_enclosed_shell_faces() {
    let solid = fixtures::cavity_box(Point3::new(4., 3., 2.), Point3::new(8., 7., 8.));
    let model = Model::new(solid);
    let all = selected(&model, Direction3::Z, PartingOptions::default());
    let outside = selected(
        &model,
        Direction3::Z,
        PartingOptions {
            exclude_internal: true,
            ..Default::default()
        },
    );
    let cavity: Vec<_> = all
        .iter()
        .filter(|curve| parent_role(curve) == PartingShell::Cavity)
        .collect();
    assert_eq!((all.len(), outside.len(), cavity.len()), (16, 8, 8));
    assert!(
        outside
            .iter()
            .all(|curve| parent_role(curve) == PartingShell::Exterior)
    );
    for curve in cavity {
        let [a, b] = curve.endpoints();
        assert_eq!(a.z(), b.z());
        assert!(a.z() == 2. || a.z() == 8.);
    }
}

#[test]
fn planar_through_hole_is_exterior() {
    let model = Model::new(fixtures::through_hole());
    let all = selected(&model, Direction3::Z, PartingOptions::default());
    let exterior = selected(
        &model,
        Direction3::Z,
        PartingOptions {
            exclude_internal: true,
            ..Default::default()
        },
    );
    assert_eq!(all.len(), 16);
    assert_eq!(
        all.iter().map(signature).collect::<Vec<_>>(),
        exterior.iter().map(signature).collect::<Vec<_>>()
    );
    assert!(
        exterior
            .iter()
            .all(|curve| parent_role(curve) == PartingShell::Exterior)
    );
}

#[test]
fn location_orientation_and_binding_order_preserve_provenance() {
    let mut transform = Transform3::default();
    transform
        .set_values([[0., -1., 0., 80.], [1., 0., 0., 20.], [0., 0., 1., 2.]])
        .unwrap();
    let located = Solid::unchecked(block().moved(&Location::new(transform), true).unwrap());
    let model = Model::new(located);
    let (mut faces, mut edges) = model.bindings();
    let first = parting::query(
        model.source(&faces, &edges),
        Direction3::Z,
        PartingOptions::default(),
    )
    .unwrap();
    faces.reverse();
    edges.reverse();
    let second = parting::query(
        model.source(&faces, &edges),
        Direction3::Z,
        PartingOptions::default(),
    )
    .unwrap();
    assert_eq!(
        first.curves().iter().map(signature).collect::<Vec<_>>(),
        second.curves().iter().map(signature).collect::<Vec<_>>()
    );
    require_edge_parents(&model, second.curves(), PartingEdgeKind::ZeroBandRim);
    let expected = expected_box(Some(2))
        .into_iter()
        .map(|ends| {
            let mut transformed = ends.map(|point| {
                coords(Point3::new(point[0], point[1], point[2]).transformed(transform))
            });
            transformed.sort_by(point_order);
            transformed
        })
        .collect::<Vec<_>>();
    assert_eq!(curve_points(second.curves()), sorted(expected));
}

#[test]
fn published_result_survives_source_mutation_and_cancellation() {
    let mut model = Model::new(block());
    let shell = ShapeIterator::new(&model.solid, true, true)
        .value()
        .unwrap()
        .clone();
    shell.set_free(true).unwrap();
    let result = selected(&model, Direction3::Z, PartingOptions::default());
    let before = curve_points(&result);
    let flag = AtomicBool::new(true);
    let (faces, edges) = model.bindings();
    let err = parting::query(
        model.source(&faces, &edges),
        Direction3::Z,
        PartingOptions {
            cancelled: Some(&flag),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert_eq!(err.source, SCOPE);
    assert!(matches!(err.kind, PartingFailureKind::Cancelled));
    assert_eq!(
        curve_points(&selected(&model, Direction3::Z, PartingOptions::default())),
        before
    );
    assert!(shell.free().unwrap());
    let edge = &model.edges[0];
    BRepBuilder::default()
        .range(edge, -100., -99., true)
        .unwrap();
    assert_eq!(BRepTool::range(edge).unwrap(), (-100., -99.));
    model.solid.nullify();
    drop(model);
    assert_eq!(curve_points(&result), before);
}

#[test]
fn concurrent_same_and_distinct_sources_match_serial_detached_bits() {
    let same = Model::new(cylinder(5., 10.));
    let other = Model::new(cylinder(3., 17.));
    let planar = Model::new(block());
    let work = [
        (&same, Direction3::X),
        (&same, Direction3::Z),
        (&other, Direction3::X),
        (&planar, Direction3::Y),
    ];
    let capture = |model, pull| {
        selected(model, pull, PartingOptions::default())
            .iter()
            .map(signature)
            .collect::<Vec<_>>()
    };
    let serial = work.map(|(model, pull)| capture(model, pull));
    for _ in 0..8 {
        let ready = Barrier::new(work.len());
        let concurrent = std::thread::scope(|scope| {
            work.map(|(model, pull)| {
                let ready = &ready;
                scope.spawn(move || {
                    ready.wait();
                    capture(model, pull)
                })
            })
            .map(|task| task.join().unwrap())
        });
        assert_eq!(serial, concurrent);
    }
}

fn selected(model: &Model, pull: Direction3, options: PartingOptions<'_>) -> Vec<PartingCurve> {
    let (faces, edges) = model.bindings();
    let report = parting::query(model.source(&faces, &edges), pull, options).unwrap();
    assert_eq!(report.source(), SCOPE);
    assert_eq!(report.pull(), pull);
    assert_eq!(report.coverage(), parting::PartingCoverage::AnalyticSolidV1);
    report.curves().to_vec()
}

fn require_edge_parents(model: &Model, curves: &[PartingCurve], expected: PartingEdgeKind) {
    let (face_bindings, edge_bindings) = model.bindings();
    let mut retained = BTreeSet::new();
    for curve in curves {
        let PartingOrigin::Shared {
            edge,
            orientation,
            faces,
            kind,
        } = curve.origin()
        else {
            panic!("expected native shared edge");
        };
        assert!(retained.insert(edge));
        assert_eq!(kind, expected);
        assert_eq!(
            curve.position_evidence(),
            PartingPositionEvidence::NativeCarrier
        );
        let native = edge_bindings
            .iter()
            .find(|binding| binding.id == edge)
            .unwrap();
        assert_eq!(orientation, native.occurrence.orientation());
        assert!(faces[0].face.id < faces[1].face.id);
        assert_ne!(faces[0].edge_orientation, faces[1].edge_orientation);
        for parent in faces {
            let bound = face_bindings
                .iter()
                .find(|binding| binding.id == parent.face.id)
                .unwrap();
            assert_eq!(parent.face.orientation, bound.occurrence.orientation());
            let use_in_face = Explorer::new(bound.occurrence, ShapeEnum::Edge, ShapeEnum::Shape)
                .unwrap()
                .find(|use_in_face| use_in_face.is_same(native.occurrence))
                .unwrap();
            assert_eq!(parent.edge_orientation, use_in_face.orientation());
        }
    }
}

fn expected_box(axis: Option<usize>) -> Vec<[[f64; 3]; 2]> {
    let mut expected = Vec::new();
    for varying in 0..3 {
        let [a, b] = [(varying + 1) % 3, (varying + 2) % 3];
        for [high_a, high_b] in [[false, false], [false, true], [true, false], [true, true]] {
            if axis.map_or(high_a == high_b, |pull| pull == varying) {
                continue;
            }
            let mut first = [0.; 3];
            first[a] = if high_a { SIZE[a] } else { 0. };
            first[b] = if high_b { SIZE[b] } else { 0. };
            let mut last = first;
            last[varying] = SIZE[varying];
            expected.push([first, last]);
        }
    }
    sorted(expected)
}

fn curve_points(curves: &[PartingCurve]) -> Vec<[[f64; 3]; 2]> {
    sorted(
        curves
            .iter()
            .map(|curve| {
                let mut points = curve.endpoints().map(coords);
                points.sort_by(point_order);
                points
            })
            .collect(),
    )
}

fn sorted(mut curves: Vec<[[f64; 3]; 2]>) -> Vec<[[f64; 3]; 2]> {
    curves.sort_by(|a, b| point_order(&a[0], &b[0]).then_with(|| point_order(&a[1], &b[1])));
    curves
}

fn point_order(a: &[f64; 3], b: &[f64; 3]) -> std::cmp::Ordering {
    a[0].total_cmp(&b[0])
        .then_with(|| a[1].total_cmp(&b[1]))
        .then_with(|| a[2].total_cmp(&b[2]))
}

fn signature(curve: &PartingCurve) -> (PartingOrigin, [u64; 2], [[u64; 3]; 2]) {
    (
        curve.origin(),
        curve.range().map(f64::to_bits),
        curve
            .endpoints()
            .map(|point| coords(point).map(f64::to_bits)),
    )
}

fn coords(point: Point3) -> [f64; 3] {
    [point.x(), point.y(), point.z()]
}

fn parent_role(curve: &PartingCurve) -> PartingShell {
    let PartingOrigin::Shared { faces, .. } = curve.origin() else {
        panic!("expected native edge");
    };
    assert_eq!(faces[0].face.shell, faces[1].face.shell);
    faces[0].face.shell
}

fn abs_uv_height(ends: [[f64; 2]; 2]) -> [f64; 2] {
    let mut values = [ends[0][1], ends[1][1]];
    values.sort_by(f64::total_cmp);
    values
}
