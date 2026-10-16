//! `BaseAllocator::free_outcome` against the expected rows:
//! what each allocator class's free
//! does with an address another class produced.
#![allow(unsafe_code)]
use martensite_tkernel::ncollection::{BaseAllocator, FreeOutcome};
use std::ptr::NonNull;

const NATIVE: &str = include_str!("fixtures/tkernel_allocator_free_outcome.expected");

fn native(label: &str) -> Vec<&'static str> {
    let line = NATIVE
        .lines()
        .find(|line| line.starts_with(&format!("{label} ")))
        .unwrap_or_else(|| panic!("fixture row {label}"));
    line[label.len() + 1..].split(' ').collect()
}

/// The row spelling: a call that raised, or one that returned.
fn spelled(outcome: &FreeOutcome) -> &'static str {
    match outcome {
        FreeOutcome::Raises(_) => "raises",
        FreeOutcome::Released | FreeOutcome::Ignored => "returns",
        FreeOutcome::Undefined => "undefined",
    }
}

fn heap_kinds() -> [(&'static str, BaseAllocator); 3] {
    [
        ("common", BaseAllocator::common()),
        ("heap", BaseAllocator::global_heap()),
        ("aligned", BaseAllocator::aligned(16)),
    ]
}

/// Frees `address` through its own allocator after a query.
fn release(owner: &BaseAllocator, address: NonNull<u8>) {
    // SAFETY: `address` came from `owner` and is freed once.
    unsafe { owner.free_raw(address) }.unwrap();
}

/// #78: built-in heap classes share layout-bearing storage on every platform.
#[test]
fn builtin_heap_classes_classify_each_others_live_storage_as_released() {
    for (owner_name, owner) in heap_kinds() {
        let address = owner.allocate_raw(64).unwrap();
        for (freer_name, freer) in heap_kinds() {
            assert!(
                matches!(
                    freer.free_outcome(&owner, address.as_ptr()),
                    FreeOutcome::Released
                ),
                "owner={owner_name}, freer={freer_name}; expected shared heap release"
            );
        }
        release(&owner, address);
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
fn heap_classes_release_each_others_blocks_on_linux() {
    for (owner_name, owner) in heap_kinds() {
        for (freer_name, freer) in heap_kinds() {
            let address = owner.allocate_raw(64).unwrap();
            let outcome = freer.free_outcome(&owner, address.as_ptr());
            assert!(
                matches!(outcome, FreeOutcome::Released),
                "{owner_name} {freer_name}"
            );
            let row = native(&format!("pair {owner_name} {freer_name}"));
            assert_eq!(row, [spelled(&outcome)]);
            release(&owner, address);
        }
        let outcome = owner.free_outcome(&owner, std::ptr::null());
        assert!(matches!(outcome, FreeOutcome::Ignored));
        assert_eq!(native(&format!("null {owner_name}")), [spelled(&outcome)]);
    }
}

#[test]
fn incremental_free_is_ignored_for_every_address() {
    let incremental = BaseAllocator::incremental(4096);
    let common = BaseAllocator::common();
    let own = incremental.allocate_raw(32).unwrap();
    let outcome = incremental.free_outcome(&incremental, own.as_ptr());
    assert!(matches!(outcome, FreeOutcome::Ignored));
    assert_eq!(native("inc_own")[0], spelled(&outcome));
    let foreign = common.allocate_raw(16).unwrap();
    let outcome = incremental.free_outcome(&common, foreign.as_ptr());
    assert!(matches!(outcome, FreeOutcome::Ignored));
    assert_eq!(native("inc_foreign"), [spelled(&outcome)]);
    release(&common, foreign);
    let outcome = incremental.free_outcome(&incremental, std::ptr::null());
    assert_eq!(native("inc_null"), [spelled(&outcome)]);
}

#[test]
fn accumulating_free_releases_its_own_and_raises_for_others() {
    let accumulating = BaseAllocator::accumulating(1024).unwrap();
    let common = BaseAllocator::common();
    let incremental = BaseAllocator::incremental(4096);
    let own = accumulating.allocate_raw(32).unwrap();
    let outcome = accumulating.free_outcome(&accumulating, own.as_ptr());
    assert!(matches!(outcome, FreeOutcome::Released));
    assert_eq!(native("acc_own"), [spelled(&outcome)]);
    let foreign = common.allocate_raw(16).unwrap();
    let outcome = accumulating.free_outcome(&common, foreign.as_ptr());
    let FreeOutcome::Raises(failure) = &outcome else {
        panic!("expected Standard_ProgramError, got {outcome:?}");
    };
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert_eq!(native("acc_foreign"), [spelled(&outcome)]);
    release(&common, foreign);
    let outcome = accumulating.free_outcome(&accumulating, std::ptr::null());
    assert_eq!(native("acc_null"), [spelled(&outcome)]);
    let arena = incremental.allocate_raw(8).unwrap();
    let outcome = accumulating.free_outcome(&incremental, arena.as_ptr());
    assert_eq!(native("acc_inc"), [spelled(&outcome)]);
    release(&accumulating, own);
}

#[test]
fn a_heap_free_of_arena_storage_is_undefined() {
    let incremental = BaseAllocator::incremental(4096);
    let accumulating = BaseAllocator::accumulating(1024).unwrap();
    let inc_address = incremental.allocate_raw(8).unwrap();
    let acc_address = accumulating.allocate_raw(8).unwrap();
    for (_, freer) in heap_kinds() {
        let outcome = freer.free_outcome(&incremental, inc_address.as_ptr());
        assert!(matches!(outcome, FreeOutcome::Undefined));
        let outcome = freer.free_outcome(&accumulating, acc_address.as_ptr());
        assert!(matches!(outcome, FreeOutcome::Undefined));
    }
    release(&accumulating, acc_address);
}

#[test]
fn the_query_frees_nothing() {
    let accumulating = BaseAllocator::accumulating(1024).unwrap();
    let address = accumulating.allocate_raw(32).unwrap();
    let before = accumulating
        .accumulating_block(address.as_ptr() as usize)
        .unwrap();
    let _ = accumulating.free_outcome(&accumulating, address.as_ptr());
    let after = accumulating
        .accumulating_block(address.as_ptr() as usize)
        .unwrap();
    assert_eq!(before.alloc_count, after.alloc_count);
    release(&accumulating, address);
}
