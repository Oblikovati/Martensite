// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{baseline, lid, pid};
use martensite_tksketch3d::{
    ArcId, BatchPhase, CurveId, CurveRole, Edit, EditFailure, EntityId, MinimumLength,
    constraints::{ConstrainedSketch, RelationEdit, RelationId, RelationSpec},
    wire::{DirectedLineUse, LineDirection, WireClosure, WireFailure},
};
use std::num::NonZeroU64;

#[test]
fn marking_a_line_reference_preserves_measurement_and_keeps_it_out_of_new_wires() {
    let original = baseline();
    let line = lid(5);
    let curve = CurveId::Line(line);
    let uses = [DirectedLineUse {
        line,
        direction: LineDirection::Forward,
    }];
    let accepted = original
        .wire_recipe(&uses, WireClosure::Open, 0, || false)
        .unwrap();
    let marked = original
        .apply(
            &[Edit::SetCurveRole {
                curve,
                role: CurveRole::Construction,
            }],
            || false,
        )
        .unwrap();
    assert_eq!(original.curve_role(curve), Some(CurveRole::Model));
    assert_eq!(marked.curve_role(curve), Some(CurveRole::Construction));
    assert_eq!(marked.line(line), original.line(line));
    assert!(marked.visible_ids().any(|key| key == EntityId::Line(line)));
    assert!(
        matches!(marked.wire_recipe(&uses, WireClosure::Open, 0, || false), Err(WireFailure::ConstructionLine(id)) if id == line)
    );
    assert!(accepted.materialize(|| false).is_ok());
    let restored = marked
        .apply(
            &[Edit::SetCurveRole {
                curve,
                role: CurveRole::Model,
            }],
            || false,
        )
        .unwrap();
    assert_eq!(restored, original);
    assert_eq!(
        restored
            .wire_recipe(&uses, WireClosure::Open, 0, || false)
            .unwrap(),
        accepted
    );
}

#[test]
fn a_reference_line_can_drive_a_length_in_the_real_solver() {
    let line = lid(5);
    let curve = CurveId::Line(line);
    let base = baseline()
        .apply(
            &[Edit::SetCurveRole {
                curve,
                role: CurveRole::Construction,
            }],
            || false,
        )
        .unwrap();
    let prepared = ConstrainedSketch::new(base, crate::constraints::policy()).unwrap();
    let id = RelationId::new(NonZeroU64::new(1).unwrap());
    let updated = prepared
        .apply(
            &[RelationEdit::Insert(
                id,
                RelationSpec::Length(line, MinimumLength::new(5.).unwrap()),
            )],
            || false,
        )
        .unwrap();
    assert!((updated.geometry().line(line).unwrap().length_estimate - 5.).abs() < 2e-7);
    assert_eq!(
        updated.geometry().curve_role(curve),
        Some(CurveRole::Construction)
    );
    assert_eq!(updated.inspect_rank().unwrap().equations, 1);
}

#[test]
fn curve_roles_are_typed_and_cancelled_or_removed_in_the_ordinary_batch() {
    let curve = CurveId::Line(lid(5));
    let original = baseline();
    let same_raw_arc = CurveId::Arc(ArcId::new(lid(5).get()));
    let wrong_type = Edit::SetCurveRole {
        curve: same_raw_arc,
        role: CurveRole::Construction,
    };
    assert_eq!(original.curve_role(same_raw_arc), None);
    assert_eq!(
        original.apply(&[wrong_type], || false),
        Err(EditFailure::MissingEditTarget(EntityId::Arc(ArcId::new(
            lid(5).get()
        ))))
    );
    let marking = Edit::SetCurveRole {
        curve,
        role: CurveRole::Construction,
    };
    let mut polls = 0;
    assert_eq!(
        original.apply(&[marking], || {
            polls += 1;
            polls == 3
        }),
        Err(EditFailure::Cancelled(BatchPhase::Validation))
    );
    assert_eq!(original.curve_role(curve), Some(CurveRole::Model));
    let removed = original
        .apply(&[marking, Edit::RemovePoint(pid(1))], || false)
        .unwrap();
    assert_eq!(removed.curve_role(curve), None);
    assert_eq!(
        removed.curve_role(CurveId::Line(lid(6))),
        Some(CurveRole::Model)
    );
}
