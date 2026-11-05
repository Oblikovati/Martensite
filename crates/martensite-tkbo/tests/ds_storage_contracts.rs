//! Source-defined segmented storage remains retained across clearing.
use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkernel::ncollection::DynamicArray;

#[test]
fn interference_storage_grows_one_record_per_block_and_clear_retains_blocks() {
    let mut ds = DataStructure::default();
    assert_growth(ds.interf_vv());
    assert_growth(ds.interf_ve());
    assert_growth(ds.interf_vf());
    assert_growth(ds.interf_ee());
    assert_growth(ds.interf_ef());
    assert_growth(ds.interf_ff());
    assert_growth(ds.interf_vz());
    assert_growth(ds.interf_ez());
    assert_growth(ds.interf_fz());
    assert_growth(ds.interf_zz());
}

fn assert_growth<T: Default>(array: &mut DynamicArray<T>) {
    array.append(T::default());
    array.append(T::default());
    assert_eq!(array.storage_extension().block_count(), 2);
    array.clear(false);
    assert_eq!(array.len(), 0);
    assert_eq!(array.storage_extension().block_count(), 2);
    array.append(T::default());
    assert_eq!(array.storage_extension().block_count(), 2);
}

#[test]
fn data_structure_clear_keeps_empty_array_storage_reusable() {
    let mut ds = DataStructure::default();
    assert_growth(ds.interf_vv());
    ds.clear();
    assert_eq!(ds.interf_vv().len(), 0);
    assert_eq!(ds.interf_vv().storage_extension().block_count(), 2);
    ds.interf_vv().append(Default::default());
    assert_eq!(ds.interf_vv().storage_extension().block_count(), 2);
}
