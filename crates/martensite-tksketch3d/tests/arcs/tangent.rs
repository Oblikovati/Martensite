// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::constraints::{policy, rid};
use martensite_tkmath::{elclib, gp::Axis1};
use martensite_tksketch3d::constraints::{
    ConstrainedSketch, ConstraintFailure, ContinuityFailure, CurvatureTolerance, LineArcJoin,
    Relation, RelationEdit, RelationSpec, RigidMotion,
};
use martensite_tksolve::preview::{Incomplete, Limit, PreviewError};

const START: [f64; 3] = [-4., 0., 0.];
const END: [f64; 3] = [4., 0., 0.];

fn fixture(height: f64, endpoint: u64, incoming: [f64; 3]) -> (Snapshot, LineArcJoin) {
    let source = create([START, END, [0., height, 0.]], through(), 1e-9).unwrap();
    let selected = if endpoint == 1 { START } else { END };
    let source = source
        .apply(
            &[
                seed(4, difference(selected, incoming), PointRole::Defining),
                line(9, endpoint, 4),
            ],
            || false,
        )
        .unwrap();
    (
        source,
        LineArcJoin {
            line: lid(9),
            arc: aid(),
            endpoint: pid(endpoint),
        },
    )
}

fn tangent_edit(join: LineArcJoin) -> RelationEdit {
    RelationEdit::Insert(rid(1), RelationSpec::Tangent(join.into()))
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| a[axis] - b[axis])
}

fn derivative(geometry: &Snapshot, join: LineArcJoin) -> [f64; 3] {
    let arc = geometry.arc(join.arc).unwrap();
    let at_start = join.endpoint == arc.recipe().endpoints[0];
    let parameter = arc.range()[usize::from(!at_start)];
    let (value, velocity) = elclib::circle_d1(parameter, arc.frame(), arc.radius());
    let endpoint = geometry.point(join.endpoint).unwrap().position;
    let distance2 = difference(*value.coordinates(), *endpoint.coordinates())
        .into_iter()
        .map(|delta| delta * delta)
        .sum::<f64>();
    assert!(distance2 <= 1e-18);
    velocity
        .xyz()
        .coordinates()
        .map(|value| if at_start { value } else { -value })
}

fn sine_and_dot(a: [f64; 3], b: [f64; 3]) -> (f64, f64) {
    let dot = |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| a[i] * b[i]).sum::<f64>();
    let cross = std::array::from_fn(|axis| {
        let i = (axis + 1) % 3;
        let j = (axis + 2) % 3;
        a[i] * b[j] - a[j] * b[i]
    });
    (
        (dot(cross, cross) / (dot(a, a) * dot(b, b))).sqrt(),
        dot(a, b),
    )
}

fn assert_tangent(geometry: &Snapshot, join: LineArcJoin) -> [f64; 3] {
    let selected = geometry.point(join.endpoint).unwrap().position;
    let other = geometry.point(pid(4)).unwrap().position;
    let direction = difference(*selected.coordinates(), *other.coordinates());
    let actual = derivative(geometry, join);
    let (sine, dot) = sine_and_dot(direction, actual);
    assert!(
        dot > 0. && sine <= policy().parallel_sine_tolerance,
        "sine={sine} dot={dot}"
    );
    actual
}

#[test]
fn exact_radius_five_tangents_keep_both_endpoint_traversals_and_sweep_branches() {
    for (height, start_x) in [(2., 3.), (8., -3.)] {
        for (endpoint, x) in [(1, start_x), (2, -start_x)] {
            let expected = [x, 4., 0.];
            let (geometry, join) = fixture(height, endpoint, expected);
            let seed = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
            let authored = seed.apply(&[tangent_edit(join)], || false).unwrap();
            assert_eq!(authored.geometry(), &geometry);
            assert_xyz(point(assert_tangent(authored.geometry(), join)), expected);
            let arc = authored.geometry().arc(join.arc).unwrap();
            assert!((arc.radius() - 5.).abs() <= 2e-14);
            assert_eq!(arc.range()[1] < PI, height < 4.);
            let rank = authored.inspect_rank().unwrap();
            assert_eq!(
                (rank.rank, rank.degrees_of_freedom, rank.equations),
                (3, 9, 2)
            );
            let archive = [(rid(1), Relation::Tangent(join.into()))];
            let restored =
                ConstrainedSketch::restore(geometry.clone(), &archive, policy(), || false).unwrap();
            assert_eq!(restored.geometry(), &geometry);
            assert_eq!(restored.relations().collect::<Vec<_>>(), archive);
        }
    }
}

#[test]
fn tangent_authoring_holds_the_line_then_rotated_drag_moves_its_free_endpoint() {
    let (geometry, join) = fixture(2., 1, [3., 4., 0.2]);
    let before = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    let authored = before.apply(&[tangent_edit(join)], || false).unwrap();
    assert_eq!(
        geometry.line(join.line),
        authored.geometry().line(join.line)
    );
    assert_ne!(geometry.arc(join.arc), authored.geometry().arc(join.arc));
    assert_tangent(authored.geometry(), join);
    let rotated = rotate(&authored);
    let rotated_direction = assert_tangent(rotated.geometry(), join);
    assert!(rotated_direction[2] > rotated_direction[1].abs());
    let mut target = *rotated
        .geometry()
        .point(pid(4))
        .unwrap()
        .position
        .coordinates();
    target[1] -= 0.03;
    let moved = rotated
        .drag_preferred(pid(4), point(target), 1e-8, || false)
        .unwrap();
    assert_ne!(
        moved.update.sketch.geometry().point(pid(4)),
        rotated.geometry().point(pid(4))
    );
    assert!(moved.preference.scaled_cursor_error < 1e-7);
    assert_tangent(moved.update.sketch.geometry(), join);
    assert_eq!(moved.update.rank.degrees_of_freedom, 9);
    assert!(
        moved
            .update
            .sketch
            .geometry()
            .arc(join.arc)
            .unwrap()
            .range()[1]
            < PI
    );
}

fn rotate(source: &ConstrainedSketch) -> ConstrainedSketch {
    let axis = Axis1::new(point([0.; 3]), Direction3::new(1., 0., 0.).unwrap());
    source
        .transform_points(
            &[pid(1), pid(2), pid(3), pid(4)],
            RigidMotion::Rotate {
                axis,
                radians: FRAC_PI_2,
            },
            || false,
        )
        .unwrap()
}

#[test]
fn arc_tangent_preserves_the_existing_limits_and_refuses_aliases_and_backward_branches() {
    let (geometry, join) = fixture(2., 1, [3., 4., 0.]);
    let geometry = geometry.apply(&[line(10, 1, 2)], || false).unwrap();
    let bound = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    let invalid = LineArcJoin {
        line: lid(10),
        ..join
    };
    assert!(matches!(bound.apply(&[tangent_edit(invalid)], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::RepeatedControl(point))) if point == pid(2)));
    let missing = LineArcJoin {
        endpoint: pid(3),
        ..join
    };
    assert!(matches!(bound.apply(&[tangent_edit(missing)], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::UnsharedEndpoint(j))) if j == missing.into()));
    let edit = RelationEdit::Insert(
        rid(1),
        RelationSpec::G2(join.into(), CurvatureTolerance::new(1e-9).unwrap()),
    );
    assert!(matches!(bound.apply(&[edit], || false),
        Err(ConstraintFailure::Continuity(_, ContinuityFailure::ArcCurvatureMismatch(arc))) if arc == aid()));

    let mut narrow = policy();
    narrow.preview.block_inputs = 11;
    let narrow = ConstrainedSketch::new(geometry, narrow).unwrap();
    assert!(matches!(
        narrow.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Preview(
            _,
            PreviewError::Incomplete(Incomplete::Limit {
                quantity: Limit::BlockInputs,
                ..
            })
        ))
    ));

    let (wrong, join) = fixture(2., 1, [-3., -4., 0.]);
    let wrong = ConstrainedSketch::new(wrong, policy()).unwrap();
    assert!(matches!(
        wrong.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Continuity(
            _,
            ContinuityFailure::BackwardTangent
        ))
    ));
}

#[test]
fn exactly_aligned_authored_arc_still_obeys_the_native_endpoint_angular_limit() {
    use martensite_tkmath::predicates::{
        compare_arc_endpoint_tangent_3d, compare_directed_line_sine_3d,
    };
    use std::cmp::Ordering::{Greater, Less};

    let half = (1_u64 << 27) as f64;
    let start = [1., 0., 0.];
    let other = [1. + 2. * half, 0., 0.];
    let witness = [1. + half, 1., 0.];
    // The exact tangent's x:y ratio has the odd 54-bit numerator 2^54−1.
    // The difference of the two line endpoints retains it, while no binary64 jet can.
    let incoming = [[2. - ((1_u64 << 54) as f64), -2. * half, 0.], start];
    let geometry = create([start, other, witness], through(), 8.)
        .unwrap()
        .apply(
            &[seed(4, incoming[0], PointRole::Defining), line(9, 1, 4)],
            || false,
        )
        .unwrap();
    let join = LineArcJoin {
        line: lid(9),
        arc: aid(),
        endpoint: pid(1),
    };
    let mut limits = policy();
    limits.parallel_sine_tolerance = f64::from_bits(1);
    let cap = limits.parallel_sine_tolerance;
    assert_eq!(
        compare_arc_endpoint_tangent_3d([start, other, witness], incoming, cap),
        Ok((Greater, Less))
    );
    let arc = geometry.arc(join.arc).unwrap();
    let (_, native) = elclib::circle_d1(0., arc.frame(), arc.radius());
    assert_eq!(
        compare_directed_line_sine_3d(incoming, [[0.; 3], *native.xyz().coordinates()], cap),
        Ok((Greater, Greater))
    );
    let relation = [(rid(1), Relation::Tangent(join.into()))];
    let before = ConstrainedSketch::new(geometry.clone(), limits).unwrap();
    assert!(matches!(
        before.apply(&[tangent_edit(join)], || false),
        Err(ConstraintFailure::Infeasible(id, failed)) if (id, failed) == relation[0]
    ));
    assert!(matches!(
        ConstrainedSketch::restore(geometry.clone(), &relation, limits, || false),
        Err(ConstraintFailure::Infeasible(id, failed)) if (id, failed) == relation[0]
    ));
    assert_eq!(before.geometry(), &geometry);
}

#[test]
fn cancelled_arc_tangent_adoption_keeps_the_seed_revision() {
    let (geometry, join) = fixture(2., 1, [3., 4., 0.2]);
    let accepted = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
    let mut polls = 0;
    assert!(matches!(
        accepted.apply(&[tangent_edit(join)], || {
            polls += 1;
            polls == 9
        }),
        Err(ConstraintFailure::Cancelled)
    ));
    assert_eq!(accepted.geometry(), &geometry);
    assert_eq!(accepted.relations().count(), 0);
}

#[test]
fn preferred_arc_tangent_drag_refreshes_the_line_frame_before_its_pole() {
    let (geometry, join) = fixture(2., 1, [3., 4., 0.]);
    let edits = [
        tangent_edit(join),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(join.endpoint)),
    ];
    let mut sketch = ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(&edits, || false)
        .unwrap();
    for step in 1..=26 {
        let angle = f64::from(step) * FRAC_PI_2 / 32.;
        let (sine, cosine) = martensite_cmath::sincos(angle);
        let target = point(difference(START, [3. * cosine, 4. * cosine, 5. * sine]));
        let moved = sketch
            .drag_preferred(pid(4), target, 1e-8, || false)
            .unwrap();
        assert!(moved.preference.scaled_cursor_error < 1e-7);
        assert_eq!(moved.update.rank.degrees_of_freedom, 6);
        assert_tangent(moved.update.sketch.geometry(), join);
        sketch = moved.update.sketch;
    }
    // With the original +Z frame reference this direction is past the trial-domain cutoff.
    let line = sketch.geometry().line(join.line).unwrap();
    assert!(line.carrier.direction().coordinates()[2].abs() > 0.95);
    assert!(sketch.geometry().arc(join.arc).unwrap().range()[1] < PI);
}
