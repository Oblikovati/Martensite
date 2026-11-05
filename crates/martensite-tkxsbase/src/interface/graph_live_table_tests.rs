//! Live tables preserve owned aliases, while convenience snapshots stay immutable.
use super::test_norm::{fake_chain, fake_entity};
use super::{InterfaceGraph, InterfaceSharingTable, InterfaceSharingTableHandle};
use martensite_tkernel::{
    ncollection::List,
    standard::{Handle, Transient},
};
use std::sync::Arc;

fn append(table: &InterfaceSharingTableHandle, entity: i32, sharing: i32) {
    let mut values = table.get().unwrap().change_array1();
    values.change_value(entity).unwrap().append(sharing);
}

fn replace(table: &InterfaceSharingTableHandle, entity: i32, sharings: &[i32]) {
    let replacement: List<i32> = sharings.iter().copied().collect();
    table
        .get()
        .unwrap()
        .change_array1()
        .set_value(entity, replacement)
        .unwrap();
}

fn row(table: &InterfaceSharingTableHandle, entity: i32) -> Vec<i32> {
    let values = table.get().unwrap().array1();
    values.value(entity).unwrap().iter().copied().collect()
}

#[test]
fn public_table_ownership_types_meet_their_specific_send_sync_bounds() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<InterfaceSharingTableHandle>();
    assert_send_sync::<InterfaceSharingTable>();
}

#[test]
fn borrowed_getter_does_not_acquire_ownership_but_an_explicit_copy_does() {
    let (model, _) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let borrowed = graph.sharing_table_handle().unwrap();
    assert_eq!(borrowed.get().unwrap().transient_base().ref_count(), 1);
    let owned = borrowed.clone();
    assert_eq!(borrowed.get().unwrap().transient_base().ref_count(), 2);
    drop(owned);
    assert_eq!(borrowed.get().unwrap().transient_base().ref_count(), 1);
}

#[test]
fn adding_model_entities_after_evaluation_preserves_the_existing_checked_diagnostic() {
    let (model, _) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let added = fake_entity("new entity", &[]);
    model.add_entity(&added).unwrap();
    let error = graph.get_sharings(&added).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
    assert_eq!(error.message(), "InterfaceGraph: entity 5; expected [1, 4]");
}

#[test]
fn sharing_lists_retain_one_incremental_allocator_across_explicit_list_copies() {
    let (model, _) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let live = graph.sharing_table_handle().cloned().unwrap();
    let values = live.get().unwrap().array1();
    let first = values.value(1).unwrap();
    let second = values.value(2).unwrap();
    assert!(first.allocator() == second.allocator());
    assert!(first.allocator().incremental_extension().is_ok());
    let copied: List<i32> = first.clone();
    assert!(copied.allocator() == first.allocator());
    assert_eq!(copied.iter().copied().collect::<Vec<_>>(), vec![3]);
}

#[test]
fn returned_handle_writes_are_visible_in_original_copies_and_assignment() {
    let (model, entities) = fake_chain();
    let source = InterfaceGraph::new(&model, true).unwrap();
    let handle = source.sharing_table_handle().cloned().unwrap();
    let mut assigned = InterfaceGraph::new(&model, true).unwrap();
    let destination = std::ptr::from_ref(&assigned);
    let returned = std::ptr::from_mut(assigned.assign_from(&source).unwrap()).cast_const();
    assert_eq!(destination, returned);
    let copies = [
        InterfaceGraph::from_graph(&source, false).unwrap(),
        InterfaceGraph::from_graph(&source, true).unwrap(),
        assigned,
    ];
    assert_eq!(handle.get().unwrap().transient_base().ref_count(), 5);
    append(&handle, 1, 4);
    for graph in [&source].into_iter().chain(copies.iter()) {
        assert!(graph.sharing_table_handle().unwrap() == &handle);
        assert_eq!(
            graph.get_sharings(&entities[0]).unwrap().unwrap(),
            vec![entities[2].clone(), entities[3].clone()]
        );
        assert_eq!(
            graph.root_entities().unwrap().content(),
            vec![entities[3].clone()]
        );
    }
}

#[test]
fn row_replacement_preserves_duplicates_and_queries_observe_changed_roots() {
    let (model, entities) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let live = graph.sharing_table_handle().cloned().unwrap();
    replace(&live, 1, &[2, 2, 4]);
    append(&live, 4, 2);
    assert_eq!(
        graph.get_sharings(&entities[0]).unwrap().unwrap(),
        vec![
            entities[1].clone(),
            entities[1].clone(),
            entities[3].clone()
        ]
    );
    assert_eq!(
        graph.get_sharings(&entities[3]).unwrap().unwrap(),
        vec![entities[1].clone()]
    );
    assert_eq!(graph.root_entities().unwrap().nb_entities(), 0);
    assert_eq!(row(&live, 1), vec![2, 2, 4]);
}

#[test]
fn assignment_replaces_ownership_without_retargeting_outstanding_old_handles() {
    let (model, entities) = fake_chain();
    let source = InterfaceGraph::new(&model, true).unwrap();
    let mut target = InterfaceGraph::new(&model, true).unwrap();
    let old_copy = InterfaceGraph::from_graph(&target, false).unwrap();
    let old = target.sharing_table_handle().cloned().unwrap();
    target.assign(&source).unwrap();
    assert!(target.sharing_table_handle().unwrap() != &old);
    append(&old, 4, 1);
    assert_eq!(
        old_copy.get_sharings(&entities[3]).unwrap().unwrap(),
        vec![entities[0].clone()]
    );
    assert!(
        target
            .get_sharings(&entities[3])
            .unwrap()
            .unwrap()
            .is_empty()
    );
    append(source.sharing_table_handle().unwrap(), 1, 4);
    assert_eq!(target.get_sharings(&entities[0]).unwrap().unwrap().len(), 2);
    assert_eq!(
        old_copy.get_sharings(&entities[0]).unwrap().unwrap().len(),
        1
    );
}

#[test]
fn snapshots_refresh_after_writes_without_mutating_prior_snapshots() {
    let (model, entities) = fake_chain();
    let source = InterfaceGraph::new(&model, true).unwrap();
    let copy = InterfaceGraph::from_graph(&source, true).unwrap();
    let old = source.sharing_table().unwrap();
    append(copy.sharing_table_handle().unwrap(), 1, 4);
    let fresh = source.sharing_table().unwrap();
    assert_eq!(old[0], vec![3]);
    assert_eq!(fresh[0], vec![3, 4]);
    assert!(!Arc::ptr_eq(&old, &fresh));
    assert!(Arc::ptr_eq(&fresh, &copy.sharing_table().unwrap()));
    let mut local = fresh.clone();
    Arc::make_mut(&mut local)[0].clear();
    assert_eq!(source.get_sharings(&entities[0]).unwrap().unwrap().len(), 2);
}

#[test]
fn table_ownership_survives_graph_drop_and_ends_after_the_last_handle() {
    let (model, _) = fake_chain();
    let live = {
        InterfaceGraph::new(&model, true)
            .unwrap()
            .sharing_table_handle()
            .cloned()
            .unwrap()
    };
    let weak = live.downgrade();
    let alias = live.clone();
    drop(live);
    append(&alias, 1, 4);
    assert_eq!(row(&alias, 1), vec![3, 4]);
    drop(alias);
    assert!(weak.upgrade().is_null());
}

#[test]
fn explicit_array_copy_owns_independent_lists_and_local_handle_rebinding_is_local() {
    let (model, _) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let live = graph.sharing_table_handle().cloned().unwrap();
    let mut local = live.clone();
    let deep = Handle::new(live.get().unwrap().clone());
    append(&deep, 1, 4);
    assert_eq!(row(&live, 1), vec![3]);
    assert_eq!(row(&deep, 1), vec![3, 4]);
    local.nullify();
    assert!(graph.sharing_table_handle().unwrap() == &live);
    drop(graph);
    drop(live);
    append(&deep, 1, 2);
    assert_eq!(row(&deep, 1), vec![3, 4, 2]);
}

#[test]
fn self_assignment_retains_live_table_edits_and_the_existing_bitmap_ordering() {
    let (model, entities) = fake_chain();
    let mut graph = InterfaceGraph::new(&model, true).unwrap();
    graph.get_from_entity(&entities[0], false, 17).unwrap();
    let live = graph.sharing_table_handle().cloned().unwrap();
    append(&live, 1, 4);
    graph.assign_self();
    assert!(graph.sharing_table_handle().unwrap() == &live);
    assert_eq!(graph.get_sharings(&entities[0]).unwrap().unwrap().len(), 2);
    assert_eq!(graph.nb_statuses(), 0);
    assert!(graph.bit_map().value(1, 0).unwrap());
    assert!(!graph.is_present(1).unwrap());
}

#[test]
fn externally_changed_bounds_and_invalid_entity_numbers_keep_native_failure_classes() {
    let (model, entities) = fake_chain();
    let graph = InterfaceGraph::new(&model, true).unwrap();
    let live = graph.sharing_table_handle().cloned().unwrap();
    live.get().unwrap().change_array1().update_lower_bound(0);
    assert_eq!(
        graph
            .get_sharings(&entities[3])
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        graph.root_entities().err().unwrap().exception_type(),
        "Standard_OutOfRange"
    );
    live.get().unwrap().change_array1().update_lower_bound(1);
    replace(&live, 1, &[0]);
    assert_eq!(
        graph
            .get_sharings(&entities[0])
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
    live.get()
        .unwrap()
        .change_array1()
        .resize_bounds(1, 0, false)
        .unwrap();
    assert_eq!(graph.root_entities().unwrap().nb_entities(), 0);
    assert_eq!(
        graph
            .get_sharings(&entities[0])
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
}
