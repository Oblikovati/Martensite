// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_cmath::atan2;
use martensite_tksketch3d::{
    PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, ConstraintFailure, LineAngle, Relation, RelationEdit, RelationSpec,
    },
};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[test]
fn supported_small_and_large_angles_solve_with_rank_one_and_keep_a_free_endpoint() {
    for radians in [0.01, FRAC_PI_4, FRAC_PI_2, PI - 0.01] {
        let target = LineAngle::new(radians).unwrap();
        let sketch = ConstrainedSketch::new(lines([2.4, 3.2, 0.]), policy())
            .unwrap()
            .apply(&fixed_reference_angle(target), || false)
            .unwrap();
        let rank = sketch.inspect_rank().unwrap();
        assert_eq!(
            (rank.rank, rank.degrees_of_freedom, rank.equations),
            (10, 2, 10)
        );
        assert!((independent_angle(sketch.geometry()) - radians).abs() < 2e-8);
        assert!(
            sketch
                .relations()
                .any(|(_, relation)| relation == Relation::Angle([lid(10), lid(11)], target))
        );
        let [start, end] = sketch.geometry().line(lid(11)).unwrap().points;
        let destination = point(std::array::from_fn(|i| {
            start.coordinates()[i] + 1.02 * (end.coordinates()[i] - start.coordinates()[i])
        }));
        let moved = sketch
            .drag(pid(4), destination, || false)
            .unwrap_or_else(|error| {
                let source = sketch.geometry().point(pid(4)).unwrap().position;
                panic!("angle {radians} from {source:?} to {destination:?}: {error}");
            });
        assert_eq!(moved.rank, rank);
        assert!((independent_angle(moved.sketch.geometry()) - radians).abs() < 2e-8);
        assert!(
            moved
                .sketch
                .geometry()
                .point(pid(4))
                .unwrap()
                .position
                .distance(&destination)
                < 2e-8
        );
    }
}

#[test]
fn open_interval_angle_and_signed_singular_seeds_have_typed_refusals() {
    for value in [0., -0., -0.2, PI, PI.next_up(), f64::INFINITY, f64::NAN] {
        assert_eq!(
            LineAngle::new(value).unwrap_err().radians.to_bits(),
            value.to_bits()
        );
    }
    let edit = RelationEdit::Insert(
        rid(3),
        RelationSpec::Angle([lid(10), lid(11)], LineAngle::new(FRAC_PI_4).unwrap()),
    );
    for seed in [[4., 0., 0.], [-4., 0., 0.], [4., 1e-9, 0.], [-4., 1e-9, 0.]] {
        let geometry = lines(seed);
        let base = ConstrainedSketch::new(geometry.clone(), policy()).unwrap();
        assert!(
            matches!(base.apply(&[edit],|| false),Err(ConstraintFailure::AngleSeedSingular(id,expected,tolerance)) if id==rid(3)&&expected==[lid(10),lid(11)]&&tolerance==policy().parallel_sine_tolerance)
        );
        assert_eq!(base.geometry(), &geometry);
    }
    let base = ConstrainedSketch::new(lines([2.4, 3.2, 0.]), policy()).unwrap();
    for radians in [1e-9, PI.next_down()] {
        let angle = LineAngle::new(radians).unwrap();
        let edit = RelationEdit::Insert(rid(3), RelationSpec::Angle([lid(10), lid(11)], angle));
        assert!(
            matches!(base.apply(&[edit],|| false),Err(ConstraintFailure::AngleTargetSingular(id,actual,_)) if id==rid(3)&&actual==angle)
        );
    }
}

#[test]
fn loose_numeric_angle_and_singular_pointer_intent_leave_the_prior_geometry_owned() {
    let angle = LineAngle::new(FRAC_PI_4).unwrap();
    let mut loose = policy();
    loose.solve.relative_tolerance = 0.5;
    let base = ConstrainedSketch::new(lines([2.4, 3.2, 0.]), loose).unwrap();
    let edits = fixed_reference_angle(angle);
    assert!(
        matches!(base.apply(&edits,|| false),Err(ConstraintFailure::Infeasible(id,Relation::Angle(..))) if id==rid(3))
    );
    let sketch = ConstrainedSketch::new(lines([2.4, 3.2, 0.]), policy())
        .unwrap()
        .apply(&edits, || false)
        .unwrap();
    let old = sketch.geometry().clone();
    let singular = point([1., 5., 7.]);
    assert!(matches!(
        sketch.drag(pid(4), singular, || false),
        Err(ConstraintFailure::Preview(..))
    ));
    assert_eq!(sketch.geometry(), &old);
}

fn fixed_reference_angle(target: LineAngle) -> [RelationEdit; 3] {
    [
        RelationEdit::Insert(rid(1), RelationSpec::FixLine(lid(10))),
        RelationEdit::Insert(rid(2), RelationSpec::FixPoint(pid(3))),
        RelationEdit::Insert(rid(3), RelationSpec::Angle([lid(10), lid(11)], target)),
    ]
}

fn lines(second: [f64; 3]) -> Snapshot {
    Snapshot::empty(length(0.01))
        .apply(
            &[
                seed(1, [1., 2., 3.], PointRole::Defining),
                seed(2, [3., 2., 3.], PointRole::Defining),
                seed(3, [-3., 5., 7.], PointRole::Defining),
                seed(
                    4,
                    [-3. + second[0], 5. + second[1], 7. + second[2]],
                    PointRole::Defining,
                ),
                line(10, 1, 2),
                line(11, 3, 4),
            ],
            || false,
        )
        .unwrap()
}

fn independent_angle(snapshot: &Snapshot) -> f64 {
    let [a, b] = snapshot.line(lid(11)).unwrap().points;
    let [x, y, z] = std::array::from_fn(|i| b.coordinates()[i] - a.coordinates()[i]);
    atan2((y * y + z * z).sqrt(), x)
}

#[test]
#[ignore = "release p95 fixture for native angle preview and independent adoption"]
fn measured_native_angle_drag_with_radial_and_azimuth_motion() {
    for count in [10, 20] {
        let (mut sketch, offset) = angle_components(count);
        let mut elapsed = Vec::with_capacity(100);
        let mut work = 0;
        let mut maximum_error = 0.0_f64;
        for i in 0..100 {
            let phase = |period: i32| {
                1.0 - 4.0 * (f64::from((i + 1) % period) / f64::from(period) - 0.5).abs()
            };
            let (azimuth_sine, azimuth_cosine) = martensite_cmath::sincos(0.05 * phase(23));
            let scale = 1.0 + 0.02 * phase(31);
            let target = point([
                offset + 3. * scale,
                2. + 4. * scale * azimuth_cosine,
                2. + 4. * scale * azimuth_sine,
            ]);
            let start = std::time::Instant::now();
            let update = sketch.drag(pid(4 * count), target, || false).unwrap();
            elapsed.push(start.elapsed().as_nanos());
            assert_eq!(
                (update.rank.rank, update.rank.degrees_of_freedom),
                (10 * count as usize, 2 * count as usize)
            );
            work = work.max(update.residual_work_bound);
            maximum_error = maximum_error.max(update.solver_hard_error);
            sketch = update.sketch;
        }
        elapsed.sort_unstable();
        println!(
            "{{\"coordinates\":{},\"samples\":100,\"p95_ns\":{},\"work\":{work},\"maximum_scaled_error\":{maximum_error}}}",
            12 * count,
            elapsed[94]
        );
    }
}

fn angle_components(count: u64) -> (ConstrainedSketch, f64) {
    let mut geometry = Vec::new();
    let mut relations = Vec::new();
    let mut offset = 0.;
    let target = LineAngle::new(atan2(4., 3.)).unwrap();
    for i in 0..count {
        offset = 20. * f64::from(u32::try_from(i).unwrap());
        let p = 4 * i + 1;
        let a = 1000 + 2 * i;
        let b = a + 1;
        let r = 3 * i + 1;
        geometry.extend([
            seed(p, [offset, 0., 0.], PointRole::Defining),
            seed(p + 1, [offset + 5., 0., 0.], PointRole::Defining),
            seed(p + 2, [offset, 2., 2.], PointRole::Defining),
            seed(p + 3, [offset + 3., 6., 2.], PointRole::Defining),
            line(a, p, p + 1),
            line(b, p + 2, p + 3),
        ]);
        relations.extend([
            RelationEdit::Insert(rid(r), RelationSpec::FixLine(lid(a))),
            RelationEdit::Insert(rid(r + 1), RelationSpec::FixPoint(pid(p + 2))),
            RelationEdit::Insert(rid(r + 2), RelationSpec::Angle([lid(a), lid(b)], target)),
        ]);
    }
    let geometry = Snapshot::empty(length(0.01))
        .apply(&geometry, || false)
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(&relations, || false)
        .unwrap();
    (sketch, offset)
}
