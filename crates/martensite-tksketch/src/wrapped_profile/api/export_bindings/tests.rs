//! Synthetic tables isolate semantic association from geometry construction (#519).
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkbrep::{brep::BRepBuilder, topods::Vertex};

fn storage(id: u64) -> BindingKey {
    BindingKey {
        source: Source::Region(SurfaceRegionId(id)),
        role: HistoryRole::Solid,
    }
}
fn name(id: u64) -> WrappedHistoryKey {
    WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(id)),
        role: HistoryRole::Solid,
    }
}
fn shape() -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default().make_vertex(&mut vertex);
    vertex.into_shape()
}

#[test]
fn reordered_decoded_bindings_keep_authored_identity() {
    let first = shape();
    let second = shape();
    let mut entities = BTreeMap::new();
    append(
        &[storage(1), storage(2)],
        &[name(11), name(12)],
        vec![(storage(2), second.clone()), (storage(1), first.clone())],
        &mut entities,
    )
    .unwrap();
    assert!(entities[&name(11)].is_same(&first));
    assert!(entities[&name(12)].is_same(&second));
}

#[test]
fn equal_count_wrong_key_refuses_without_mutating_existing_names() {
    let existing = shape();
    let mut entities = BTreeMap::from([(name(99), existing.clone())]);
    let refusal = append(
        &[storage(1), storage(2)],
        &[name(11), name(12)],
        vec![(storage(1), shape()), (storage(3), shape())],
        &mut entities,
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::MissingBinding(key) if key == storage(2)));
    assert_eq!(entities.len(), 1);
    assert!(entities[&name(99)].is_same(&existing));
}

#[test]
fn duplicate_decoded_storage_keys_refuse() {
    let refusal = append(
        &[storage(1), storage(2)],
        &[name(11), name(12)],
        vec![(storage(1), shape()), (storage(1), shape())],
        &mut BTreeMap::new(),
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::DuplicateBinding(key) if key == storage(1)));
}

#[test]
fn duplicate_expected_storage_keys_refuse() {
    let refusal = append(
        &[storage(1), storage(1)],
        &[name(11), name(12)],
        vec![(storage(1), shape()), (storage(2), shape())],
        &mut BTreeMap::new(),
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::DuplicateBinding(key) if key == storage(1)));
}

#[test]
fn duplicate_public_names_within_member_refuse() {
    let mut entities = BTreeMap::new();
    let refusal = append(
        &[storage(1), storage(2)],
        &[name(11), name(11)],
        vec![(storage(1), shape()), (storage(2), shape())],
        &mut entities,
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::DuplicateName(key) if key == name(11)));
    assert!(entities.is_empty());
}

#[test]
fn duplicate_public_names_across_members_preserve_first_member() {
    let existing = shape();
    let mut entities = BTreeMap::from([(name(11), existing.clone())]);
    let refusal = append(
        &[storage(2)],
        &[name(11)],
        vec![(storage(2), shape())],
        &mut entities,
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::DuplicateName(key) if key == name(11)));
    assert!(entities[&name(11)].is_same(&existing));
}

#[test]
fn missing_and_extra_decoded_entries_refuse() {
    for bindings in [
        Vec::new(),
        vec![(storage(1), shape()), (storage(2), shape())],
    ] {
        let refusal =
            append(&[storage(1)], &[name(11)], bindings, &mut BTreeMap::new()).unwrap_err();
        assert!(matches!(
            refusal,
            ExportBindingFailure::Count {
                table: "decoded bindings",
                expected: 1,
                ..
            }
        ));
    }
}

#[test]
fn inconsistent_name_count_refuses() {
    let refusal = append(
        &[storage(1)],
        &[],
        vec![(storage(1), shape())],
        &mut BTreeMap::new(),
    )
    .unwrap_err();
    assert!(matches!(
        refusal,
        ExportBindingFailure::Count {
            table: "history names",
            actual: 0,
            expected: 1
        }
    ));
}

#[test]
fn empty_export_is_completed_without_names() {
    let mut entities = BTreeMap::new();
    append(&[], &[], Vec::new(), &mut entities).unwrap();
    assert!(entities.is_empty());
}

#[test]
fn ordered_duplicate_expected_and_decoded_keys_refuse_without_publishing() {
    let mut entities = BTreeMap::new();
    let refusal = append(
        &[storage(1), storage(1)],
        &[name(11), name(12)],
        vec![(storage(1), shape()), (storage(1), shape())],
        &mut entities,
    )
    .unwrap_err();
    assert!(matches!(refusal, ExportBindingFailure::DuplicateBinding(key) if key == storage(1)));
    assert!(entities.is_empty());
}
