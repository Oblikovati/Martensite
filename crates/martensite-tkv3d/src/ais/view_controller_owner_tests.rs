//! Canonical Arc tests expose the routing gap; legacy reentry remains explicitly limited.
use super::AisDragAction;
use super::view_controller_owner_expected::selection_owner_expected;
use super::view_controller_owner_fixture::*;
use super::view_controller_owner_replay::{SelectionOwnerInterface, selection_owner_replay};

fn assert_selection_owner(case: SelectionOwnerCase, interface: SelectionOwnerInterface) {
    let actual = selection_owner_replay(case, interface);
    let limited = !matches!(interface, SelectionOwnerInterface::Arc);
    let expected = selection_owner_expected(case, limited);
    println!(
        "OWNER_TRACE_BEGIN {}\n{}\nOWNER_TRACE_END {}",
        case.name,
        actual.join("\n"),
        case.name
    );
    assert_eq!(
        actual, expected,
        "callback case={}; expected witnessed owner/retry trace (legacy reentry explicitly partial)",
        case.name
    );
}
macro_rules! owner_control {
    ($test:ident, $name:literal, $slot:ident, $mode:ident, $action:ident, $interface:ident) => {
        #[test]
        fn $test() {
            let case = SelectionOwnerCase {
                name: $name,
                slot: SelectionOwnerSlot::$slot,
                mode: SelectionOwnerMode::$mode,
                action: AisDragAction::$action,
            };
            assert_selection_owner(case, SelectionOwnerInterface::$interface);
        }
    };
}
owner_control!(
    arc_selection_direct,
    "selection-direct",
    Selection,
    Direct,
    Start,
    Arc
);
owner_control!(
    arc_selection_delegate,
    "selection-delegate",
    Selection,
    Delegate,
    Start,
    Arc
);
owner_control!(
    arc_selection_reenter,
    "selection-reenter",
    Selection,
    Reenter,
    Start,
    Arc
);
owner_control!(
    arc_selection_typed,
    "selection-typed",
    Selection,
    Typed,
    Start,
    Arc
);
owner_control!(
    arc_selection_unrelated,
    "selection-unrelated",
    Selection,
    Unrelated,
    Start,
    Arc
);
owner_control!(arc_drag_direct, "drag-direct", Drag, Direct, Start, Arc);
owner_control!(
    arc_drag_delegate,
    "drag-delegate",
    Drag,
    Delegate,
    Start,
    Arc
);
owner_control!(arc_drag_reenter, "drag-reenter", Drag, Reenter, Start, Arc);
owner_control!(arc_drag_typed, "drag-typed", Drag, Typed, Start, Arc);
owner_control!(
    arc_drag_unrelated,
    "drag-unrelated",
    Drag,
    Unrelated,
    Start,
    Arc
);
owner_control!(
    arc_subview_direct,
    "subview-direct",
    Subview,
    Direct,
    Start,
    Arc
);
owner_control!(
    arc_subview_delegate,
    "subview-delegate",
    Subview,
    Delegate,
    Start,
    Arc
);
owner_control!(
    arc_subview_reenter,
    "subview-reenter",
    Subview,
    Reenter,
    Start,
    Arc
);
owner_control!(
    arc_subview_typed,
    "subview-typed",
    Subview,
    Typed,
    Start,
    Arc
);
owner_control!(
    arc_subview_unrelated,
    "subview-unrelated",
    Subview,
    Unrelated,
    Start,
    Arc
);
owner_control!(
    arc_drag_confirmed,
    "drag-confirmed",
    Drag,
    Direct,
    Confirmed,
    Arc
);
owner_control!(arc_drag_update, "drag-update", Drag, Direct, Update, Arc);
owner_control!(arc_drag_stop, "drag-stop", Drag, Direct, Stop, Arc);
owner_control!(arc_drag_abort, "drag-abort", Drag, Direct, Abort, Arc);
owner_control!(
    legacy_selection_direct,
    "selection-direct",
    Selection,
    Direct,
    Start,
    Legacy
);
owner_control!(
    legacy_selection_delegate,
    "selection-delegate",
    Selection,
    Delegate,
    Start,
    Legacy
);
owner_control!(
    legacy_selection_reenter,
    "selection-reenter",
    Selection,
    Reenter,
    Start,
    Legacy
);
owner_control!(
    legacy_selection_typed,
    "selection-typed",
    Selection,
    Typed,
    Start,
    Legacy
);
owner_control!(
    legacy_selection_unrelated,
    "selection-unrelated",
    Selection,
    Unrelated,
    Start,
    Legacy
);
owner_control!(
    legacy_drag_direct,
    "drag-direct",
    Drag,
    Direct,
    Start,
    Legacy
);
owner_control!(
    legacy_drag_delegate,
    "drag-delegate",
    Drag,
    Delegate,
    Start,
    Legacy
);
owner_control!(
    legacy_drag_reenter,
    "drag-reenter",
    Drag,
    Reenter,
    Start,
    Legacy
);
owner_control!(legacy_drag_typed, "drag-typed", Drag, Typed, Start, Legacy);
owner_control!(
    legacy_drag_unrelated,
    "drag-unrelated",
    Drag,
    Unrelated,
    Start,
    Legacy
);
owner_control!(
    legacy_subview_direct,
    "subview-direct",
    Subview,
    Direct,
    Start,
    Legacy
);
owner_control!(
    legacy_subview_delegate,
    "subview-delegate",
    Subview,
    Delegate,
    Start,
    Legacy
);
owner_control!(
    legacy_subview_reenter,
    "subview-reenter",
    Subview,
    Reenter,
    Start,
    Legacy
);
owner_control!(
    legacy_subview_typed,
    "subview-typed",
    Subview,
    Typed,
    Start,
    Legacy
);
owner_control!(
    legacy_subview_unrelated,
    "subview-unrelated",
    Subview,
    Unrelated,
    Start,
    Legacy
);
owner_control!(
    legacy_drag_confirmed,
    "drag-confirmed",
    Drag,
    Direct,
    Confirmed,
    Legacy
);
owner_control!(
    legacy_drag_update,
    "drag-update",
    Drag,
    Direct,
    Update,
    Legacy
);
owner_control!(legacy_drag_stop, "drag-stop", Drag, Direct, Stop, Legacy);
owner_control!(legacy_drag_abort, "drag-abort", Drag, Direct, Abort, Legacy);
owner_control!(
    coexisting_selection,
    "selection-direct",
    Selection,
    Direct,
    Start,
    LegacyWithDefaultArc
);
owner_control!(
    coexisting_drag,
    "drag-direct",
    Drag,
    Direct,
    Start,
    LegacyWithDefaultArc
);
owner_control!(
    coexisting_subview,
    "subview-direct",
    Subview,
    Direct,
    Start,
    LegacyWithDefaultArc
);
