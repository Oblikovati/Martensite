//! Assignment follows the source's field order, including self and status-free sources.
use super::InterfaceGraph;
use super::test_norm::fake_chain;
use martensite_tkernel::tcollection::AsciiString;
use std::sync::Arc;

#[test]
fn self_assignment_clears_statuses_but_retains_bitmap_model_and_sharing_identity() {
    let (model, entities) = fake_chain();
    let mut graph = InterfaceGraph::new(&model, true).unwrap();
    graph.get_from_entity(&entities[0], false, 17).unwrap();
    graph.thepresents = AsciiString::new("kept name");
    let table = graph.sharing_table().unwrap();
    let original_address = std::ptr::from_ref(&graph);
    assert_eq!(
        std::ptr::from_mut(graph.assign_self()).cast_const(),
        original_address
    );
    assert_eq!(graph.nb_statuses(), 0);
    assert_eq!(graph.status(1).unwrap(), 0);
    assert!(!graph.is_present(1).unwrap());
    assert!(graph.bit_map().value(1, 0).unwrap());
    assert!(graph.model() == &model);
    assert!(Arc::ptr_eq(&table, &graph.sharing_table().unwrap()));
    assert_eq!(graph.thepresents, AsciiString::new("kept name"));
}

#[test]
fn assignment_copies_statuses_and_bitmap_but_shares_the_model_and_sharing_table() {
    let (model, entities) = fake_chain();
    let mut source = InterfaceGraph::new(&model, true).unwrap();
    source.get_from_entity(&entities[1], false, 29).unwrap();
    source.thepresents = AsciiString::new("source name");
    let (other_model, _) = fake_chain();
    let mut target = InterfaceGraph::new(&other_model, true).unwrap();
    target.assign(&source).unwrap();
    assert!(target.model() == source.model());
    assert!(Arc::ptr_eq(
        &target.sharing_table().unwrap(),
        &source.sharing_table().unwrap()
    ));
    assert_eq!(target.thepresents, AsciiString::new("source name"));
    source.remove_item(2).unwrap();
    assert_eq!(target.status(2).unwrap(), 29);
    assert!(target.is_present(2).unwrap());
    assert_eq!(source.status(2).unwrap(), 0);
    assert!(!source.is_present(2).unwrap());
}

#[test]
fn assignment_from_a_status_free_source_keeps_the_destinations_old_bitmap() {
    let (model, entities) = fake_chain();
    let mut target = InterfaceGraph::new(&model, true).unwrap();
    target.get_from_entity(&entities[0], false, 17).unwrap();
    let source = InterfaceGraph::new(&model, false).unwrap();
    target.assign(&source).unwrap();
    assert_eq!(target.nb_statuses(), 0);
    assert_eq!(target.bit_map().length(), 4);
    assert!(target.bit_map().value(1, 0).unwrap());
    assert!(!target.is_present(1).unwrap());
    assert_eq!(target.status(1).unwrap(), 0);
    assert!(!source.bit_map().value(1, 0).unwrap());
}

#[test]
fn both_copy_modes_share_the_table_identity_and_copy_statuses() {
    let (model, entities) = fake_chain();
    let mut source = InterfaceGraph::new(&model, true).unwrap();
    source.get_from_entity(&entities[0], false, 17).unwrap();
    for copied in [false, true] {
        let copy = InterfaceGraph::from_graph(&source, copied).unwrap();
        assert!(Arc::ptr_eq(
            &copy.sharing_table().unwrap(),
            &source.sharing_table().unwrap()
        ));
        assert_eq!(copy.status(1).unwrap(), 17);
        assert!(copy.is_present(1).unwrap());
    }
}
