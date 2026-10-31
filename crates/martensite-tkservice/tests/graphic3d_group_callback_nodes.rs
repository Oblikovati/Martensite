//! A selected group remains the same node when its Clear callback removes an earlier node (#324).
#[path = "support/earlier_group_clear.rs"]
mod earlier_group_clear;
use super::graphic3d_group_storage_contracts::group_contract_presentation;
use earlier_group_clear::EarlierGroupClear;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

fn append_clearing_group(
    presentation: &Handle<Graphic3dStructure>,
    earlier: &Handle<dyn Graphic3dGroup>,
) -> Handle<dyn Graphic3dGroup> {
    let active = EarlierGroupClear::new(&presentation.cast(), earlier.observe());
    presentation
        .cstructure()
        .cstructure_base()
        .fields
        .write()
        .groups
        .append(active.cast());
    active.cast()
}
#[test]
fn clearing_selected_group_removes_that_node_after_earlier_callback_mutation() {
    let presentation = group_contract_presentation();
    let earlier = presentation.new_group().unwrap();
    let active = append_clearing_group(&presentation, &earlier);
    let next = presentation.new_group().unwrap();
    presentation.cstructure().remove_group(&active);
    assert_eq!(presentation.groups(), vec![next.clone()]);
    assert_eq!(
        (earlier.ref_count(), active.ref_count(), next.ref_count()),
        (1, 1, 2)
    );
    assert!(earlier.is_deleted());
}
