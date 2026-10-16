//! Owned behavior and workload baselines before the storage replacement (#82).
use super::CellIndex;

#[test]
fn inline_boundary_and_clone_isolation_hold_for_empty_and_heap_keys() {
    for length in [0, 10, 11, 24] {
        let source = vec![7; length];
        let original = CellIndex::from_slice(&source);
        let mut copy = original.clone();
        assert_eq!(original.heap.is_some(), length > 10);
        copy.as_mut_slice().fill(9);
        assert_eq!(original.as_slice(), source);
        assert_eq!(copy.as_slice(), vec![9; length]);
    }
}

#[test]
fn every_move_storage_pair_preserves_values_and_empties_the_donor() {
    for source_length in [0, 10, 11, 24] {
        for destination_length in [0, 10, 11, 24] {
            check_move_pair(source_length, destination_length);
        }
    }
}

fn check_move_pair(source_length: usize, destination_length: usize) {
    let mut source = CellIndex::from_slice(&vec![7; source_length]);
    let mut destination = CellIndex::from_slice(&vec![9; destination_length]);
    let old_destination = destination.data_ptr();
    let source_pointer = source.data_ptr();
    destination.move_from(&mut source);
    assert_eq!(destination.as_slice(), vec![7; source_length]);
    assert!(source.as_slice().is_empty());
    if source_length > 10 {
        assert_eq!(destination.data_ptr(), source_pointer);
        if destination_length > 10 {
            assert_eq!(source.data_ptr(), old_destination);
        }
    }
}

#[test]
fn cell_index_implements_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CellIndex>();
}

#[test]
#[ignore = "manual storage benchmark; run the same release binary workload before and after #82"]
fn cell_index_storage_workload() {
    let started = std::time::Instant::now();
    let mut checksum = 0_i64;
    for length in [0, 10, 11, 24] {
        checksum += storage_batch(length);
    }
    assert_eq!(checksum, 900_000);
    println!(
        "cell_index_workload rounds=20000 lengths=0,10,11,24 checksum={checksum} elapsed_ns={} index_bytes={}",
        started.elapsed().as_nanos(),
        std::mem::size_of::<CellIndex>()
    );
}

fn storage_batch(length: usize) -> i64 {
    let source = vec![1; length];
    let owners: Vec<_> = (0..20_000)
        .map(|_| CellIndex::from_slice(&source))
        .collect();
    let mut checksum = 0;
    for original in &owners {
        let mut copy = original.clone();
        let moved = CellIndex::move_construct(&mut copy);
        checksum += std::hint::black_box(moved.as_slice())
            .iter()
            .map(|&n| i64::from(n))
            .sum::<i64>();
    }
    checksum
}
