//! Actual group nodes remain distinct and live across supported collection edits (#324).
use super::graphic3d_group_storage_contracts::{
    append_existing_group, group_contract_presentation,
};
use martensite_tkernel::{ncollection::SequenceNativeCursor, standard::*};
use martensite_tkservice::graphic3d::*;

fn start_group_cursor(presentation: &Graphic3dStructure) -> SequenceNativeCursor {
    let structure = presentation.cstructure();
    let fields = structure.cstructure_base().fields.read();
    SequenceNativeCursor::new(&fields.groups, true)
}
fn advance_group_cursor(presentation: &Graphic3dStructure, cursor: &mut SequenceNativeCursor) {
    let structure = presentation.cstructure();
    let fields = structure.cstructure_base().fields.read();
    cursor.next(&fields.groups).unwrap();
}
fn current_group_target(
    presentation: &Graphic3dStructure,
    cursor: &SequenceNativeCursor,
) -> HandleObserver<dyn Graphic3dGroup> {
    let structure = presentation.cstructure();
    let fields = structure.cstructure_base().fields.read();
    cursor
        .position()
        .value(&fields.groups)
        .unwrap_or_else(|| {
            panic!(
                "group cursor={:?}; expected an attached current node",
                cursor.position()
            )
        })
        .observe()
}
#[test]
fn duplicate_payloads_have_distinct_actual_nodes_without_extra_intrusive_owners() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    append_existing_group(&presentation, &group);
    let mut cursor = start_group_cursor(&presentation);
    let first_node = cursor.observer().current_node().unwrap();
    let first = current_group_target(&presentation, &cursor);
    advance_group_cursor(&presentation, &mut cursor);
    let second_node = cursor.observer().current_node().unwrap();
    let second = current_group_target(&presentation, &cursor);
    assert_ne!(first_node, second_node);
    assert_eq!(first.object_address(), second.object_address());
    assert_eq!(group.ref_count(), 3);
}
#[test]
fn earlier_removal_preserves_the_second_duplicate_node_as_successor() {
    let presentation = group_contract_presentation();
    let earlier = presentation.new_group().unwrap();
    let active = presentation.new_group().unwrap();
    append_existing_group(&presentation, &active);
    let mut cursor = start_group_cursor(&presentation);
    advance_group_cursor(&presentation, &mut cursor);
    let first_active_node = cursor.observer().current_node().unwrap();
    earlier.remove();
    advance_group_cursor(&presentation, &mut cursor);
    assert_ne!(first_active_node, cursor.observer().current_node().unwrap());
    assert_eq!(
        current_group_target(&presentation, &cursor).object_address(),
        active.object_address()
    );
    assert_eq!(active.ref_count(), 3);
}
#[test]
fn append_after_current_is_visible_to_the_same_detached_cursor() {
    let presentation = group_contract_presentation();
    let first = presentation.new_group().unwrap();
    let mut cursor = start_group_cursor(&presentation);
    let next = presentation.new_group().unwrap();
    advance_group_cursor(&presentation, &mut cursor);
    assert_eq!(
        current_group_target(&presentation, &cursor).object_address(),
        next.object_address()
    );
    assert_eq!((first.ref_count(), next.ref_count()), (2, 2));
}
#[test]
fn advancing_a_removed_current_node_returns_a_typed_failure() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    let mut cursor = start_group_cursor(&presentation);
    group.remove();
    let structure = presentation.cstructure();
    let fields = structure.cstructure_base().fields.read();
    let error = cursor.next(&fields.groups).unwrap_err();
    assert!(
        error.to_string().contains("Next"),
        "failure={error}; expected invalid current-node advancement context"
    );
}
#[test]
fn group_storage_and_detached_cursor_keep_the_declared_thread_bounds() {
    let fields = std::marker::PhantomData::<Graphic3dCStructureFields>;
    let cursor = std::marker::PhantomData::<SequenceNativeCursor>;
    let _: (&(dyn Send + Sync), &(dyn Send + Sync)) = (&fields, &cursor);
}
