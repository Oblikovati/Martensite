//! Ordered snapshots and exact ownership pin the public group-storage contract for #324.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{graphic3d::*, null_driver::NullGraphicDriver};

/// Build an isolated actual group owner; `group_contract_presentation()`.
pub(super) fn group_contract_presentation() -> Handle<Graphic3dStructure> {
    let driver = NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    Graphic3dStructure::new(&manager, &Handle::default()).unwrap()
}
/// Retain another actual entry without copying its payload; `append_existing_group(prs, group)`.
pub(super) fn append_existing_group(
    presentation: &Graphic3dStructure,
    group: &Handle<dyn Graphic3dGroup>,
) {
    // Two collection entries may own the same payload; neither is a copied group object.
    presentation
        .cstructure()
        .cstructure_base()
        .fields
        .write()
        .groups
        .append(group.clone());
}
#[test]
fn group_snapshots_preserve_duplicate_order_and_each_intrusive_owner() {
    let presentation = group_contract_presentation();
    let first = presentation.new_group().unwrap();
    let second = presentation.new_group().unwrap();
    append_existing_group(&presentation, &first);
    let snapshot = presentation.groups();
    assert_eq!(snapshot, vec![first.clone(), second.clone(), first.clone()]);
    assert_eq!((first.ref_count(), second.ref_count()), (5, 3));
    drop(snapshot);
    assert_eq!((first.ref_count(), second.ref_count()), (3, 2));
}
#[test]
fn mutating_a_snapshot_keeps_the_live_collection_and_owners_independent() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    append_existing_group(&presentation, &group);
    let mut snapshot = presentation.groups();
    assert_eq!(group.ref_count(), 5);
    drop(snapshot.pop());
    assert_eq!(group.ref_count(), 4);
    assert_eq!(presentation.number_of_groups(), 2);
    drop(snapshot);
    assert_eq!(group.ref_count(), 3);
}
#[test]
fn driver_removal_releases_only_first_matching_group_entry() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    append_existing_group(&presentation, &group);
    presentation.cstructure().remove_group(&group);
    assert_eq!(presentation.groups(), vec![group.clone()]);
    assert_eq!(group.ref_count(), 2);
    assert!(!group.is_deleted());
}
#[test]
fn clearing_live_groups_keeps_existing_duplicate_snapshots_alive() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    append_existing_group(&presentation, &group);
    let snapshot = presentation.groups();
    presentation.cstructure().clear();
    assert_eq!(presentation.number_of_groups(), 0);
    assert_eq!(group.ref_count(), 3);
    assert_eq!(snapshot, vec![group.clone(), group.clone()]);
    drop(snapshot);
    assert_eq!(group.ref_count(), 1);
}
#[test]
fn copying_fields_retains_payloads_without_sharing_collection_edits() {
    let presentation = group_contract_presentation();
    let group = presentation.new_group().unwrap();
    let structure = presentation.cstructure();
    let mut copied = structure.cstructure_base().fields.snapshot();
    assert_eq!(group.ref_count(), 3);
    copied.groups.clear(None);
    assert_eq!(group.ref_count(), 2);
    assert_eq!(presentation.groups(), vec![group.clone()]);
}
