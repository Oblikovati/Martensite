use super::super::*;
fn unpublished_array_slot(array: &mut DynamicArray<ReplacementItem>) -> *mut ReplacementItem {
    let mut storage = array.storage_extension();
    // SAFETY: the array's only value stays at its address while it leaves automatic ownership. Clear automatic Drop ownership before replacement may end this slot's lifetime.
    unsafe { storage.set_used_size(0).unwrap() };
    storage.change_block_at(0).unwrap()
}
fn initial_array() -> (DynamicArray<ReplacementItem>, ReplacementTrace) {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let mut array = DynamicArray::with_increment(2);
    array.append(ReplacementItem::new(1, &trace));
    (array, trace)
}
unsafe fn publish_value(array: &mut DynamicArray<ReplacementItem>, value: i32) {
    // SAFETY: Caller has reconstructed the one-slot prefix and transfers its Drop ownership back.
    unsafe { array.storage_extension().set_used_size(1).unwrap() };
    assert_eq!(array.value(0).unwrap().value, value);
}
fn replace_array(mode: ReplacementMode, expected_trace: &str) {
    let (mut array, trace) = initial_array();
    let pointer = unpublished_array_slot(&mut array);
    take_trace(&trace);
    // SAFETY: `pointer` is the live slot that left automatic ownership above, and this test
    // repairs it on failure.
    unsafe { apply_replacement(pointer, 2, mode, &trace) };
    assert_eq!(array.size(), 0);
    assert_eq!(take_trace(&trace), expected_trace);
    // SAFETY: Replacement succeeded, so the same physical slot is again a valid initialized prefix.
    unsafe { publish_value(&mut array, 2) };
    assert_eq!(
        array.storage_extension().change_block_at(0).unwrap(),
        pointer
    );
    drop(array);
    assert_eq!(take_trace(&trace), "D2");
}
#[test]
fn assignment_policy_updates_actual_array_slot_with_mutated_donor_drop() {
    replace_array(ReplacementMode::Assign, "C2,A2,D-7");
}
#[test]
fn noexcept_move_policy_reconstructs_actual_array_slot_in_native_order() {
    replace_array(ReplacementMode::Move, "C2,D1,M2,D-7");
}
#[test]
fn copy_preferred_policy_keeps_temporary_value_until_its_drop() {
    replace_array(ReplacementMode::Copy, "C2,D1,K2,D2");
}
#[test]
fn direct_policy_destroys_actual_array_slot_before_construction() {
    replace_array(ReplacementMode::Direct, "D1,C2");
}
#[test]
fn failed_transfer_leaves_array_slot_unpublished_until_explicit_repair() {
    let (mut array, trace) = initial_array();
    let pointer = unpublished_array_slot(&mut array);
    take_trace(&trace);
    // SAFETY: `pointer` is the live slot that left automatic ownership above, and this test
    // repairs it after the panic.
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        apply_replacement(pointer, 88, ReplacementMode::Copy, &trace);
    }));
    assert!(result.is_err());
    assert_eq!(array.size(), 0);
    assert_eq!(take_trace(&trace), "C88,D1,K88,D88");
    // SAFETY: the failed transfer left the slot dead, so constructing a fresh value revives it.
    unsafe { ReplacementStorage::construct_at(pointer, || ReplacementItem::new(5, &trace)) };
    // SAFETY: the slot holds the value constructed above.
    unsafe { publish_value(&mut array, 5) };
    drop(array);
    assert_eq!(take_trace(&trace), "C5,D5");
}
#[test]
fn throwing_assignment_keeps_published_target_live_with_native_prefix_change() {
    let (mut array, trace) = initial_array();
    let pointer = array.storage_extension().change_block_at(0).unwrap();
    take_trace(&trace);
    // SAFETY: `pointer` is the array's published slot, and an assignment failure leaves it
    // live.
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        apply_replacement(pointer, 77, ReplacementMode::Assign, &trace);
    }));
    assert!(result.is_err());
    assert_eq!(array.value(0).unwrap().value, 77);
    assert_eq!(take_trace(&trace), "C77,A77,D-7");
    drop(array);
    assert_eq!(take_trace(&trace), "D77");
}
