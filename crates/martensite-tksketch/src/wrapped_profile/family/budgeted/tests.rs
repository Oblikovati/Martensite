//! The budget is declared before construction and checked before the only completion handoff.
use super::super::tests::requests;
use super::*;
#[test]
fn composed_family_retains_the_same_encoding_history_and_budget_receipt() {
    let inputs = requests();
    let result = construct_budgeted(&inputs, CylindricalMeasureLimit::Refuse(1e-4))
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(result.budget.members.len(), 2);
    assert!(result.budget.volume.discrepancy <= 1e-4);
    assert!(result.budget.volume.representative_error <= 1e-4);
    assert_eq!(
        result.family.history.records.len(),
        result.family.keys.len()
    );
    let (root, bindings) = pipeline::decode(&result.family.bytes, &result.family.keys).unwrap();
    assert_eq!(
        crate::surface_sketch::encoding::encode(&root, &bindings).unwrap(),
        result.family.bytes
    );
    fn send_sync<T: Send + Sync>() {}
    send_sync::<BudgetedFamily>();
}
#[test]
fn invalid_volume_policy_refuses_before_any_construction_checkpoint() {
    let mut checkpoints = 0;
    let result = construct_budgeted_with_cancel(
        &requests(),
        CylindricalMeasureLimit::Refuse(f64::NAN),
        |_| {
            checkpoints += 1;
            false
        },
    );
    assert!(matches!(result, Err(FamilyFailure::Budget(_))));
    assert_eq!(checkpoints, 0);
}
#[test]
fn an_unmet_volume_allowance_never_returns_a_budgeted_family() {
    let result = construct_budgeted(&requests(), CylindricalMeasureLimit::Refuse(0.));
    assert!(matches!(result, Err(FamilyFailure::Budget(cause))
        if matches!(*cause, publication_budget::BudgetFailure::Limit { region: None, .. })));
}
#[test]
fn cancellation_at_either_budget_boundary_refuses_handoff() {
    for limit in [1, 2] {
        cancel_budget(limit);
    }
}
fn cancel_budget(limit: usize) {
    let mut polls = 0;
    let mut completed = false;
    let result =
        construct_budgeted_with_cancel(&requests(), CylindricalMeasureLimit::Report, |point| {
            completed |= point.stage == FamilyStage::Completion;
            if point.stage == FamilyStage::Budget {
                polls += 1;
            }
            point.stage == FamilyStage::Budget && polls == limit
        });
    assert_budget_cancelled(result);
    assert_eq!(polls, limit);
    assert!(!completed);
}
#[test]
fn final_completion_checkpoint_follows_successful_budget_validation() {
    let mut saw_budget = false;
    let result =
        construct_budgeted_with_cancel(&requests(), CylindricalMeasureLimit::Report, |point| {
            saw_budget |= point.stage == FamilyStage::Budget;
            if point.stage == FamilyStage::Completion {
                assert!(saw_budget);
                return true;
            }
            false
        });
    assert!(matches!(
        result,
        Err(FamilyFailure::Cancelled(Checkpoint {
            stage: FamilyStage::Completion,
            ..
        }))
    ));
}
#[test]
fn reordered_selection_has_the_same_budgeted_geometry_and_metadata() {
    let mut inputs = requests();
    let first = construct_budgeted(&inputs, CylindricalMeasureLimit::Report)
        .unwrap_or_else(|e| panic!("{e}"));
    inputs.reverse();
    let second = construct_budgeted(&inputs, CylindricalMeasureLimit::Report)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(first.family.bytes, second.family.bytes);
    assert_eq!(first.family.keys, second.family.keys);
    assert_eq!(first.family.history, second.family.history);
    assert_eq!(first.budget, second.budget);
}

fn assert_budget_cancelled(result: Checked<BudgetedFamily>) {
    assert!(matches!(
        result,
        Err(FamilyFailure::Cancelled(Checkpoint {
            stage: FamilyStage::Budget,
            ..
        }))
    ));
}
