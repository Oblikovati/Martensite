// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::gp::{Axis3, Cylinder3};

const IDENTITY: [[f64; 4]; 3] = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];

fn wall() -> SegmentCylinderWall {
    SegmentCylinderWall::capture(
        Cylinder3::new(Axis3::default(), 1.).unwrap(),
        IDENTITY,
        [0., 2.],
    )
    .unwrap()
}

fn chord(y: f64, z: f64) -> [[f64; 3]; 2] {
    [[-3., y, z], [3., y, z]]
}

fn contacts(wall: &SegmentCylinderWall, source: [[f64; 3]; 2]) -> Vec<CylinderSegmentContact> {
    let CylinderSegmentIntersection::Isolated(contacts) = wall.intersect(source, || false).unwrap()
    else {
        panic!("expected isolated roots");
    };
    contacts
}

#[test]
fn owned_two_roots_keep_directed_order_and_exact_world_points() {
    let wall = wall();
    let source = chord(0., 1.);
    for (input, sense) in [(source, 1.), ([source[1], source[0]], -1.)] {
        let hits = contacts(&wall, input);
        let [a, b] = hits
            .as_slice()
            .try_into()
            .unwrap_or_else(|_| panic!("expected two hits"));
        let literal = [1.0_f64 / 3., 2.0_f64 / 3.];
        for (i, (hit, x)) in [a, b].into_iter().zip([-sense, sense]).enumerate() {
            assert_eq!(hit.root_ordinal, i as u8);
            assert_eq!(hit.line, CylinderLinePlace::Interior);
            assert_eq!(hit.wall, CylinderWallPlace::Interior);
            assert_eq!(hit.sense, CylinderContactSense::Transverse);
            let cut = hit.cut.unwrap();
            assert_eq!(cut.parameter, [literal[i], literal[i].next_up()]);
            assert_eq!(cut.position, [x, 0., 1.]);
            assert_eq!(cut.coordinates, [[x; 2], [0.; 2], [1.; 2]]);
            assert_eq!(cut.error_upper, 0.);
        }
    }
}

#[test]
fn exact_tangent_endpoint_and_axial_classification_are_independent_of_rounding() {
    let wall = wall();
    let tangent = contacts(&wall, chord(1., 1.));
    assert_eq!(tangent.len(), 1);
    assert_eq!(tangent[0].sense, CylinderContactSense::Tangent);
    assert_eq!(tangent[0].cut, None);
    assert_eq!(
        wall.intersect(chord(1.0_f64.next_up(), 1.), || false)
            .unwrap(),
        CylinderSegmentIntersection::Disjoint
    );
    let narrower = contacts(&wall, chord(1.0_f64.next_down(), 1.));
    assert_eq!(narrower.len(), 2);
    assert!(narrower[0].cut.unwrap().position[0] < 0.);
    assert!(narrower[1].cut.unwrap().position[0] > 0.);
    let mix = contacts(&wall, [[-1., 0., 1.], [3., 0., 1.]]);
    assert_eq!(mix[0].line, CylinderLinePlace::Start);
    assert!(mix[0].cut.is_none() && mix[1].cut.is_some());
    for hit in contacts(&wall, chord(0., 2.)) {
        assert_eq!(hit.wall, CylinderWallPlace::LastRim);
        assert!(hit.cut.is_some());
    }
    for hit in contacts(&wall, chord(0., 2.0_f64.next_up())) {
        assert_eq!(hit.wall, CylinderWallPlace::Above);
        assert!(hit.cut.is_none());
    }
}

#[test]
fn coincident_generator_requires_a_positive_retained_interval() {
    let wall = wall();
    let query = |low, high| {
        wall.intersect([[1., 0., low], [1., 0., high]], || false)
            .unwrap()
    };
    assert_eq!(query(-1., 3.), CylinderSegmentIntersection::Overlap);
    assert_eq!(query(3., 4.), CylinderSegmentIntersection::Disjoint);
    assert_eq!(
        query(-1., 0.),
        CylinderSegmentIntersection::GeneratorEndpoint {
            line: CylinderLinePlace::End,
            rim: CylinderWallPlace::FirstRim,
        }
    );
}

#[test]
fn irrational_crossing_is_bounded_and_replayed_on_the_actual_placed_chart() {
    let wall = SegmentCylinderWall::capture(
        Cylinder3::new(Axis3::default(), 1.).unwrap(),
        [[0., 0., 1., 4.], [1., 0., 0., -8.], [0., 1., 0., 16.]],
        [0., 2.],
    )
    .unwrap();
    let hits = contacts(&wall, [[5., -11., 16.5], [5., -5., 16.5]]);
    for (hit, sign) in hits.into_iter().zip([-1., 1.]) {
        let cut = hit.cut.unwrap();
        let analytic = -8. + sign * 3.0_f64.sqrt() * 0.5;
        assert!(cut.coordinates[1][0] <= analytic && analytic <= cut.coordinates[1][1]);
        assert!(cut.error_upper > 0. && cut.error_upper < 2e-15);
        assert_eq!(cut.position[0], 5.);
        assert_eq!(cut.position[2], 16.5);
    }
    let mut count = 0;
    assert!(matches!(
        wall.intersect([[5., -11., 16.5], [5., -5., 16.5]], || {
            count += 1;
            count == 4
        }),
        Err(CylinderSegmentError::Cancelled)
    ));
}
