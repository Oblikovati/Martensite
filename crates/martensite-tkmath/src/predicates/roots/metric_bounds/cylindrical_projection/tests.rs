// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::gp::{Axis3, Point3};
use std::f64::consts::{FRAC_PI_2, PI};

const IDENTITY: ProjectionPlacement = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];

fn cylinder(u: Direction3, hole: bool) -> ProjectionCylinder {
    let frame = Axis3::new(Point3::default(), Direction3::Z, u).unwrap();
    ProjectionCylinder {
        cylinder: Cylinder3::new(frame, 4.).unwrap(),
        placement: IDENTITY,
        axial: [0., 6.],
        hole: hole.then_some(ProjectionHole {
            angle: [FRAC_PI_2, 3. * FRAC_PI_2],
            axial: [1.5, 2.5],
        }),
    }
}

fn line(x: f64, y: f64) -> AxialProjectionLine {
    AxialProjectionLine {
        line: Line3::new(Point3::new(x, y, 0.), Direction3::Z),
        placement: IDENTITY,
        range: [1., 3.],
    }
}

fn regular(target: ProjectionCylinder, mode: AxialProjectionMode) -> Box<AxialProjectionWitness> {
    let plan = plan_axial_cylinder_projection(target, line(2., 0.), mode, || false).unwrap();
    let CylinderProjectionPlan::Regular(witness) = plan else {
        panic!("expected regular projection");
    };
    witness
}

fn interval(a: f64, b: f64) -> ProjectionSourceInterval {
    ProjectionSourceInterval {
        endpoints: [[a, a], [b, b]],
        included: [true; 2],
    }
}

#[test]
fn signed_modes_select_different_named_and_retained_intervals() {
    use AxialProjectionMode::{Closest, Forward, Normal, TwoSided};
    use CylinderProjectionBranch::{NegativeU, PositiveU};
    let target = cylinder(Direction3::X, true);
    let minus_x = Direction3::X.reversed();
    let forward = regular(target, Forward(minus_x)).branch_plans().unwrap();
    assert_eq!(forward.len(), 1);
    assert_eq!(forward[0].branch, NegativeU);
    assert_eq!(forward[0].signed_travel, [6.; 2]);
    assert_eq!(forward[0].selected, [interval(1., 1.5), interval(2.5, 3.)]);

    let both = regular(target, TwoSided(minus_x)).branch_plans().unwrap();
    assert_eq!(both[0].branch, PositiveU);
    assert_eq!(both[0].signed_travel, [-2.; 2]);
    assert_eq!(both[0].selected, [interval(1., 3.)]);
    assert!(both[1].selected.is_empty());

    let closest = regular(target, Closest).branch_plans().unwrap();
    assert_eq!(closest[0].selected, [interval(1., 3.)]);
    assert!(closest[1].selected.is_empty());
    let normals = regular(target, Normal).branch_plans().unwrap();
    assert_eq!(normals[0].selected, [interval(1., 3.)]);
    assert_eq!(normals[1].selected, forward[0].selected);
}

#[test]
fn removed_closest_and_directional_switch_remain_different_outcomes() {
    use AxialProjectionMode::{Closest, TwoSided};
    let target = cylinder(Direction3::X.reversed(), true);
    let closest =
        plan_axial_cylinder_projection(target, line(2., 0.), Closest, || false).unwrap_err();
    assert_eq!(closest, CylinderProjectionError::ClosestTrim);
    let both = regular(target, TwoSided(Direction3::X.reversed()))
        .branch_plans()
        .unwrap();
    assert_eq!(
        both[0].selected,
        [ProjectionSourceInterval {
            endpoints: [[1.5; 2], [2.5; 2]],
            included: [false, false],
        }]
    );
    assert_eq!(both[1].selected, [interval(1., 1.5), interval(2.5, 3.)]);
}

#[test]
fn exact_empty_tangent_overlap_and_equal_travel_are_distinct() {
    use AxialProjectionMode::{Forward, Normal, TwoSided};
    use CylinderProjectionError::{Overlap, RootTie, Tangent};
    let target = cylinder(Direction3::X, false);
    let query = |source, mode| plan_axial_cylinder_projection(target, source, mode, || false);
    assert!(matches!(
        query(line(5., -1.), Forward(Direction3::Y)),
        Ok(CylinderProjectionPlan::Empty(
            CylinderProjectionEmpty::NoSupportRoot
        ))
    ));
    assert_eq!(
        query(line(4., -1.), Forward(Direction3::Y)).unwrap_err(),
        Tangent
    );
    assert_eq!(
        query(line(4., 0.), Forward(Direction3::Z)).unwrap_err(),
        Overlap
    );
    assert_eq!(query(line(0., 0.), Normal).unwrap_err(), Overlap);
    assert_eq!(
        query(line(0., 0.), TwoSided(Direction3::X)).unwrap_err(),
        RootTie
    );
}

#[test]
fn qualified_root_checks_the_prism_and_whole_retained_native_span() {
    let witness = regular(
        cylinder(Direction3::X, false),
        AxialProjectionMode::Forward(Direction3::X.reversed()),
    );
    let sweep = witness.sweep().unwrap();
    witness
        .verify_prism(sweep, line(2., 0.), Direction3::X, [-sweep.length, 0.])
        .unwrap();
    let exact = NativeProjectionEdge {
        range: [1., 3.],
        spatial: NativeProjectionCurve::Line {
            origin: [-4., 0., 0.],
            direction: [0., 0., 1.],
        },
        placement: IDENTITY,
        prism: NativeProjectionCurve::Line {
            origin: [0., -6.],
            direction: [1., 0.],
        },
        target: NativeProjectionCurve::Line {
            origin: [PI, 0.],
            direction: [0., 1.],
        },
        second_target: None,
        vertices: [1., 3.].map(|z| ProjectionVertex {
            point: [-4., 0., z],
            placement: IDENTITY,
        }),
    };
    let cap = ProjectionProofLimits {
        spatial: 1e-7,
        source_parameter: 1e-9,
        angle: 1e-9,
    };
    let proof = witness
        .certify_native_edges(sweep, std::slice::from_ref(&exact), cap, || false)
        .unwrap();
    assert_eq!(proof[0].1.selected[0].source, interval(1., 3.));
    assert_eq!(
        proof[0].1.selected[0].section_parameters,
        [[1.; 2], [3.; 2]]
    );
    let distorted = NativeProjectionEdge {
        spatial: NativeProjectionCurve::DegreeOne {
            knots: vec![1., 2., 3.],
            poles: vec![[-4., 0., 1.], [-4.01, 0., 2.], [-4., 0., 3.]],
        },
        ..exact
    };
    assert_eq!(
        witness
            .certify_native_edges(sweep, &[distorted], cap, || false)
            .unwrap_err(),
        CylinderProjectionError::Coverage
    );
}
