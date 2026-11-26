// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tksketch3d::{
    ArcDefinition, ArcFitTolerance, ArcId, Edit, SketchArc,
    constraints::{ArcBranch, ArcConstraintFailure, ConstraintFailure, Relation},
};

#[test]
fn fixed_arc_replays_with_an_independent_planar_attachment() {
    let arc = ArcId::new(key(80));
    let geometry = ordinary(false)
        .geometry()
        .apply(&arc_edits(arc), || false)
        .unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), policy())
        .unwrap()
        .apply(
            &[
                RelationEdit::Insert(rid(40), RelationSpec::FixPoint(pid(2))),
                RelationEdit::Insert(rid(81), RelationSpec::FixArc(arc)),
            ],
            || false,
        )
        .unwrap();
    let binding = proof(1, 0., 70);
    let source = binding.revision();
    let attached = attach(&sketch, binding.clone());
    let rank = attached.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (15, 2, 14)
    );
    let moved = attached
        .drag_preferred(pid(1), point([-1., 0., 0.]), 1e-8, || source, || false)
        .unwrap()
        .sketch;
    assert_eq!(location(&moved), [-1., 0., 0.]);
    assert_eq!(moved.geometry().arc(arc), geometry.arc(arc));
    assert_eq!(moved.inspect_rank().unwrap(), rank);
    assert_same_reopened_snapshot(&moved, binding);
    let detached = moved.detach(|| false).unwrap();
    assert_eq!(detached.geometry(), moved.geometry());
    assert_eq!(
        detached.relations().collect::<Vec<_>>(),
        sketch.relations().collect::<Vec<_>>()
    );
}

fn arc_edits(arc: ArcId) -> [Edit; 4] {
    [
        seed(10, [7., -4., 0.], PointRole::Defining),
        seed(11, [7., 4., 0.], PointRole::Defining),
        seed(12, [7., 0., 2.], PointRole::SketchMarker),
        Edit::InsertArc(SketchArc {
            id: arc,
            endpoints: [pid(10), pid(11)],
            definition: ArcDefinition::Through { witness: pid(12) },
            fit_tolerance: ArcFitTolerance::new(1e-9).unwrap(),
        }),
    ]
}

#[test]
fn a_face_attachment_cannot_couple_to_a_crossing_radius_arc() {
    let arc = ArcId::new(key(80));
    let mut edits = arc_edits(arc);
    edits[..3].clone_from_slice(&[
        seed(10, [-3., 0., 0.], PointRole::Defining),
        seed(11, [-1., 0., 0.], PointRole::Defining),
        seed(12, [-2., 1., 0.], PointRole::SketchMarker),
    ]);
    let geometry = ordinary(false).geometry().apply(&edits, || false).unwrap();
    let sketch = ConstrainedSketch::new(geometry.clone(), policy())
        .unwrap()
        .apply(
            &[RelationEdit::Insert(
                rid(81),
                RelationSpec::Radius(arc, length(1.)),
            )],
            || false,
        )
        .unwrap();
    let expected = Relation::Radius(arc, length(1.), ArcBranch::CrossHalfTurn);
    assert_eq!(sketch.relations().next(), Some((rid(81), expected)));
    let binding = proof(1, 0., 70);
    let result = sketch.attach_point_on_face(rid(90), pid(10), binding.clone(), || false);
    assert!(matches!(result, Err(Failure::Sketch(cause)) if matches!(
        *cause,
        ConstraintFailure::Arc(identity, ArcConstraintFailure::CrossingCoupledRelation(relation))
            if identity == arc && relation == rid(90)
    )));
    assert_eq!(sketch.geometry(), &geometry);
    let independent = attach(&sketch, binding);
    assert_eq!(
        independent.ordinary_relations().next(),
        Some((rid(81), expected))
    );
}
