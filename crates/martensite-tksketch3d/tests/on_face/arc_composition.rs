// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tksketch3d::{ArcDefinition, ArcFitTolerance, ArcId, Edit, SketchArc};

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
