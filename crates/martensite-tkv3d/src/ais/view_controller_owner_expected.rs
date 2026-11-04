//! Closed control-flow expectations are independent of callback storage and dispatch.
use super::view_controller_owner_fixture::*;

/// Expect genuine callback/reentry/retry observations; `selection_owner_expected(case, false)`.
pub(super) fn selection_owner_expected(
    case: SelectionOwnerCase,
    legacy_reentry_limit: bool,
) -> Vec<String> {
    let steps = expected_owner_steps(case.mode, legacy_reentry_limit);
    steps
        .iter()
        .map(|(phase, depth)| expected_owner_record(case, phase, *depth))
        .collect()
}
fn expected_owner_steps(
    mode: SelectionOwnerMode,
    legacy_reentry_limit: bool,
) -> &'static [(&'static str, usize)] {
    match mode {
        SelectionOwnerMode::Reenter if !legacy_reentry_limit => {
            &[("enter", 0), ("enter", 1), ("leave", 1), ("leave", 0)]
        }
        SelectionOwnerMode::Typed | SelectionOwnerMode::Unrelated => {
            &[("enter", 0), ("failure", 0), ("enter", 0), ("leave", 0)]
        }
        _ => &[("enter", 0), ("leave", 0)],
    }
}
fn expected_owner_record(case: SelectionOwnerCase, phase: &str, depth: usize) -> String {
    if phase == "failure" {
        return expected_owner_failure(case);
    }
    let action = if case.slot == SelectionOwnerSlot::Drag {
        case.action as i32
    } else {
        -1
    };
    format!(
        "{{\"case\":\"{}\",\"phase\":\"{phase}\",\"depth\":{depth},\"actual_arguments_match\":1,\"action\":{action}}}",
        case.name
    )
}
fn expected_owner_failure(case: SelectionOwnerCase) -> String {
    let (kind, label) = if case.mode == SelectionOwnerMode::Typed {
        ("Standard_ProgramError", "typed")
    } else {
        ("rust-panic", "unrelated")
    };
    format!(
        "{{\"case\":\"{}\",\"phase\":\"failure\",\"kind\":\"{kind}\",\"message\":\"selection callback kind={label}; expected success\"}}",
        case.name
    )
}
