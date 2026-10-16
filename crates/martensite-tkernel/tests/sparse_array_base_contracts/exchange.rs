//! Saved protected-state observers stay bound to their container across field exchange.
use super::fixture::{discard_integer, insert_integer, populated_sparse_base};
use super::resource_contracts::allocator_live_slots;
use martensite_tkernel::ncollection::{BaseAllocator, SparseArrayBase, SparseArrayBaseState};
use std::{cell::RefCell, ffi::c_void};

fn exchange_rows(actual: &str) {
    let expected = include_str!("../fixtures/oracle/tkernel_sparse_exchange_identity.expected");
    assert!(
        expected.lines().any(|line| line == actual),
        "row {actual:?}; expected native fixture row"
    );
}

#[test]
fn exchange_preserves_saved_protected_state_addresses() {
    let mut first = populated_sparse_base(4, 4, &[(0, 10)]);
    let mut second = populated_sparse_base(12, 9, &[(8, 80), (9, 90)]);
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let before = unsafe { [first.native_state(), second.native_state()] };
    let item = second.get_value(8).unwrap();
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let table = unsafe { (*before[1]).table };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { first.exchange(&mut second) };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let after = unsafe { [first.native_state(), second.native_state()] };
    assert_eq!(before, after);
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { exchange_field_identity(before, after) };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { exchange_contents(&first, before, item, table) };
}

unsafe fn exchange_field_identity(
    before: [*mut SparseArrayBaseState; 2],
    after: [*mut SparseArrayBaseState; 2],
) {
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let slots = |state: *mut SparseArrayBaseState| unsafe {
        (
            std::ptr::addr_of!((*state).size),
            std::ptr::addr_of!((*state).table),
        )
    };
    let (first, second) = (slots(before[0]), slots(before[1]));
    let (current_first, current_second) = (slots(after[0]), slots(after[1]));
    exchange_rows(&format!(
        "identity {} {} {} {}",
        u8::from(first.0 == current_first.0),
        u8::from(second.0 == current_second.0),
        u8::from(first.1 == current_first.1),
        u8::from(second.1 == current_second.1)
    ));
}

unsafe fn exchange_contents(
    first: &SparseArrayBase,
    before: [*mut SparseArrayBaseState; 2],
    item: *mut c_void,
    table: *mut *mut c_void,
) {
    exchange_rows(&format!(
        "contents {} {} {} {} {} {}",
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*before[0]).size },
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*before[1]).size },
        first.item_size(),
        first.block_size(),
        u8::from(first.get_value(8).unwrap() == item),
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        u8::from(unsafe { (*before[0]).table } == table)
    ));
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { exchange_clear_observers(first, before) };
}

unsafe fn exchange_clear_observers(
    first: &SparseArrayBase,
    before: [*mut SparseArrayBaseState; 2],
) {
    let mut trace = String::new();
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe {
        first.clear_with(|slot| {
            trace.push_str(&format!("{}:{},", *slot.cast::<i32>(), (*before[0]).size))
        })
    };
    exchange_rows(&format!(
        "clear_phase {trace} {} {}",
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*before[0]).size },
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*before[1]).size }
    ));
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExchangeDropObservation {
    value: i32,
    state: *const SparseArrayBaseState,
    trace: *const RefCell<String>,
}

unsafe fn copy_drop_observation(destination: *mut c_void, source: *mut c_void) {
    // The complete initialized item is copied, preserving borrowed observer provenance.
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe {
        destination
            .cast::<ExchangeDropObservation>()
            .write(*source.cast::<ExchangeDropObservation>())
    };
}

unsafe fn record_drop_observation(slot: *mut c_void) {
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let item = unsafe { *slot.cast::<ExchangeDropObservation>() };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { &*item.trace }
        .borrow_mut()
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        .push_str(&format!("{}:{},", item.value, unsafe {
            (*item.state).size
        }));
}

#[test]
fn exchanged_stored_destructor_reads_the_destination_observer() {
    let trace = RefCell::new(String::new());
    let mut first = SparseArrayBase::new(size_of::<ExchangeDropObservation>(), 4, discard_integer);
    let mut second = SparseArrayBase::new(
        size_of::<ExchangeDropObservation>(),
        9,
        record_drop_observation,
    );
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    populate_drop_observations(&second, unsafe { first.native_state() }, &trace);
    insert_integer(&first, 0, 10);
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { first.exchange(&mut second) };
    drop(first);
    exchange_rows(&format!("drop_phase {}", trace.borrow()));
}

fn populate_drop_observations(
    second: &SparseArrayBase,
    state: *const SparseArrayBaseState,
    trace: &RefCell<String>,
) {
    let mut item = ExchangeDropObservation {
        value: 80,
        state,
        trace,
    };
    for (index, value) in [(8, 80), (9, 90)] {
        item.value = value;
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe {
            second.set_value(
                index,
                std::ptr::from_mut(&mut item).cast(),
                copy_drop_observation,
                copy_drop_observation,
            )
        };
    }
}

#[test]
fn repeated_exchange_keeps_state_slots_and_restores_tables() {
    let mut first = populated_sparse_base(4, 4, &[(0, 10)]);
    let mut second = populated_sparse_base(4, 9, &[(8, 80), (9, 90)]);
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let states = unsafe { [first.native_state(), second.native_state()] };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let tables = unsafe { [(*states[0]).table, (*states[1]).table] };
    for _ in 0..2 {
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { first.exchange(&mut second) };
        assert_eq!(
            // SAFETY: both sparse bases live for the whole test on this thread, so their state
            // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
            // wrote.
            unsafe { [first.native_state(), second.native_state()] },
            states
        );
    }
    exchange_rows(&format!(
        "round_trip {} {} {} {}",
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*states[0]).size },
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { (*states[1]).size },
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        u8::from(unsafe { (*states[0]).table } == tables[0]),
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        u8::from(unsafe { (*states[1]).table } == tables[1])
    ));
}

#[test]
fn exchange_transfers_distinct_allocation_resources_with_their_blocks() {
    let first_resource = BaseAllocator::accumulating(256).unwrap();
    let second_resource = BaseAllocator::accumulating(256).unwrap();
    let mut first = SparseArrayBase::with_allocator(4, 1, discard_integer, first_resource.clone());
    let mut second =
        SparseArrayBase::with_allocator(4, 9, discard_integer, second_resource.clone());
    insert_integer(&first, 0, 10);
    insert_integer(&second, 8, 80);
    assert!(
        allocator_live_slots(&first_resource) > 0 && allocator_live_slots(&second_resource) > 0
    );
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    unsafe { first.exchange(&mut second) };
    drop(first);
    assert_eq!(allocator_live_slots(&second_resource), 0);
    assert!(allocator_live_slots(&first_resource) > 0);
    drop(second);
    assert_eq!(allocator_live_slots(&first_resource), 0);
}

#[test]
fn self_exchange_preserves_state_storage_and_callbacks() {
    let mut first = populated_sparse_base(4, 4, &[(0, 10)]);
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let state = unsafe { first.native_state() };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let table = unsafe { (*state).table };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let destroy = unsafe { (*state).destroy };
    first.exchange_self();
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    let current = unsafe { first.native_state() };
    // SAFETY: both sparse bases live for the whole test on this thread, so their state
    // pointers, tables and blocks are live, and each slot read holds an i32 the fixture wrote.
    assert!(std::ptr::fn_addr_eq(destroy, unsafe { (*current).destroy }));
    exchange_rows(&format!(
        "self {} {} {} {}",
        u8::from(state == current),
        first.size(),
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        u8::from(table == unsafe { (*current).table }),
        // SAFETY: both sparse bases live for the whole test on this thread, so their state
        // pointers, tables and blocks are live, and each slot read holds an i32 the fixture
        // wrote.
        unsafe { *first.get_value(0).unwrap().cast::<i32>() }
    ));
}
