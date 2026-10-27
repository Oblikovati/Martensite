//! Entry-only clear policy: unlocked ordered destruction, retained new groups and explicit panic behavior.
#[path = "support/group_clear_retirement.rs"]
mod group_clear_retirement;
use super::graphic3d_group_storage_contracts::group_contract_presentation;
use group_clear_retirement::*;
use martensite_tkernel::{
    ncollection::{BaseAllocator, Sequence},
    standard::*,
};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

fn retirement_scene() -> (Handle<Graphic3dStructure>, Arc<GroupClearTrace>) {
    let presentation = group_contract_presentation();
    let structure = presentation.cstructure();
    let mut fields = structure.cstructure_base().fields.write();
    fields.groups = Sequence::with_allocator(BaseAllocator::fresh_heap());
    fields.is_2d_text = true;
    fields.is_for_highlight = true;
    drop(fields);
    let trace = GroupClearTrace::new(&presentation);
    (presentation, trace)
}
fn install_entry_batch(trace: &Arc<GroupClearTrace>, panics: bool) {
    install_retiring_group(trace, "A", GroupDropAction::Append);
    let action = if panics {
        GroupDropAction::AppendThenPanic
    } else {
        GroupDropAction::Observe
    };
    install_retiring_group(trace, "B", action);
    install_retiring_group(trace, "C", GroupDropAction::Observe);
    install_retiring_group(trace, "D", GroupDropAction::Observe);
}
fn assert_live_created_groups(trace: &GroupClearTrace, expected: usize) {
    let fields = trace.structure.cstructure_base().fields.read();
    let actual: Vec<usize> = fields
        .groups
        .iter()
        .map(|group| group.object_address() as usize)
        .collect();
    assert_eq!(actual, *trace.created.read());
    assert_eq!(actual.len(), expected);
    assert!(fields.groups.iter().all(|group| group.ref_count() == 1));
    assert_eq!(fields.groups.allocator(), &trace.allocator);
}
fn assert_retirement_rows(trace: &GroupClearTrace, panics: bool) {
    let rows = trace.rows.read();
    assert_eq!(
        rows.iter().map(|row| row.role).collect::<Vec<_>>(),
        ["A", "B", "C", "D"]
    );
    let after_b = if panics { 2 } else { 1 };
    let expected = [0, 1, after_b, after_b];
    for (row, count) in rows.iter().zip(expected) {
        assert_eq!(
            row.state,
            Some((count, true, true, true)),
            "actual destructor role={}; expected reachable entry-free collection, old flags and same allocator",
            row.role
        );
    }
}
#[test]
fn clear_retires_only_entry_groups_outside_guard_before_resetting_flags() {
    let (presentation, trace) = retirement_scene();
    install_entry_batch(&trace, false);
    presentation.cstructure().clear();
    assert_retirement_rows(&trace, false);
    assert_live_created_groups(&trace, 1);
    let fields = trace.structure.cstructure_base().fields.read();
    assert_eq!((fields.is_2d_text, fields.is_for_highlight), (false, false));
}
#[test]
fn clear_panic_preserves_new_groups_and_old_flags_while_finishing_ordered_retirement() {
    let (presentation, trace) = retirement_scene();
    install_entry_batch(&trace, true);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        presentation.cstructure().clear()
    }));
    let payload =
        outcome.expect_err("clear returned success; expected the deliberate B destructor panic");
    assert_eq!(
        payload.downcast_ref::<EntryGroupDropPanic>(),
        Some(&EntryGroupDropPanic { role: "B" })
    );
    assert_retirement_rows(&trace, true);
    assert_live_created_groups(&trace, 2);
    let fields = trace.structure.cstructure_base().fields.read();
    assert_eq!((fields.is_2d_text, fields.is_for_highlight), (true, true));
}
